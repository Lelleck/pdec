use std::vec;

use chrono::{DateTime, Utc};
use regex::Regex;
use reqwest::{blocking::Client, Url};
use serde_derive::{Deserialize, Serialize};

use super::display::{Team, TeamTime};

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoricalLogsRequest {
    player_name: String,
    log_type: String,
    player_id: String,
    from: Option<String>,
    till: Option<String>,
    limit: u32,
    time_sort: String,
    exact_player: bool,
    exact_action: bool,
    server_filter: String,
    output: Option<String>,
}

impl HistoricalLogsRequest {
    pub fn by_id(id: String) -> Self {
        Self {
            player_name: "".to_string(),
            log_type: "".to_string(),
            player_id: id,
            from: None,
            till: None,
            limit: 9999999,
            time_sort: "desc".to_string(),
            exact_player: false,
            exact_action: false,
            server_filter: "".to_string(),
            output: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalLog {
    #[serde(alias = "type")]
    pub kind: String,
    pub raw: String,
}

impl HistoricalLog {
    pub fn time(&self) -> DateTime<Utc> {
        let re = Regex::new(r#"\b\d{6,}\b"#).unwrap();
        let res = re.find(&self.raw).unwrap();
        let num = res.as_str().parse().unwrap();
        DateTime::<Utc>::from_timestamp(num, 0).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalLogsResponse {
    pub result: Vec<HistoricalLog>,
}

#[derive(Debug)]
pub enum IntermediateTeamTime {
    Join(DateTime<Utc>),
    Leave(DateTime<Utc>),
    Kill(DateTime<Utc>, Team),
}

impl IntermediateTeamTime {
    pub fn time(&self) -> &DateTime<Utc> {
        match self {
            IntermediateTeamTime::Join(t) => t,
            IntermediateTeamTime::Leave(t) => t,
            IntermediateTeamTime::Kill(t, _) => t,
        }
    }
}

pub fn get_team_times(client: &mut Client, endpoint: &str, id: String) -> Vec<TeamTime> {
    let url_text = format!("{}/api/get_historical_logs", endpoint);
    let url = Url::parse(&url_text).expect("Failed to turn into URL");
    let body = HistoricalLogsRequest::by_id(id.clone());
    let response = client
        .post(url)
        .json(&body)
        .send()
        .expect("Request failed")
        .json::<HistoricalLogsResponse>()
        .expect("Failed to deserialize json");

    let filtered_logs = response.result;
    let intermediate = historical_log_into_intermediate(&id, filtered_logs);
    extract_team_times(intermediate)
}

fn historical_log_into_intermediate(
    id: &str,
    logs: Vec<HistoricalLog>,
) -> Vec<IntermediateTeamTime> {
    let mut times = vec![];
    let re_str = format!(r"\((?P<team>[^/()]+)/{}\)", id);
    let re = Regex::new(&re_str).unwrap();

    for historical_log in logs {
        if historical_log.kind == "DISCONNECTED" {
            times.push(IntermediateTeamTime::Leave(historical_log.time()));
        }
        if historical_log.kind == "CONNECTED" {
            times.push(IntermediateTeamTime::Join(historical_log.time()));
        }
        if historical_log.kind == "KILL" {
            if let Some(caps) = re.captures(&historical_log.raw) {
                if let Some(team) = caps.name("team") {
                    times.push(IntermediateTeamTime::Kill(
                        historical_log.time(),
                        Team::from_str(team.as_str()),
                    ));
                }
            }
        }
    }

    times
}

#[derive(Debug, Clone)]
enum ProcessingState {
    Absent,
    Present(DateTime<Utc>, Option<Team>),
}

fn extract_team_times(mut intermediaries: Vec<IntermediateTeamTime>) -> Vec<TeamTime> {
    if intermediaries.is_empty() {
        return Vec::new();
    }
    intermediaries.sort_by(|a, b| a.time().cmp(b.time()));

    let mut state = ProcessingState::Absent;
    let mut times = vec![];

    for intermediary in &intermediaries {
        let mut new_state = state.clone();
        let time = match (&state, intermediary) {
            (ProcessingState::Absent, IntermediateTeamTime::Join(join_time)) => {
                // Player joins
                new_state = ProcessingState::Present(join_time.clone(), None);
                None
            }
            (
                ProcessingState::Present(old_join_time, team),
                IntermediateTeamTime::Join(new_join_time),
            ) => {
                // Player joins, again?!
                new_state = ProcessingState::Present(new_join_time.clone(), None);
                Some(TeamTime::new(old_join_time, new_join_time, team.clone()))
            }
            (
                ProcessingState::Present(join_time, team),
                IntermediateTeamTime::Leave(leave_time),
            ) => {
                // Player leaves
                new_state = ProcessingState::Absent;
                Some(TeamTime::new(join_time, leave_time, team.clone()))
            }
            (
                ProcessingState::Present(join_time, old_team),
                IntermediateTeamTime::Kill(kill_time, new_team),
            ) => {
                // Player kills
                if old_team.is_none() {
                    // We now know the team of the player
                    new_state = ProcessingState::Present(join_time.clone(), Some(new_team.clone()));
                    None
                } else {
                    // We need to check whether the player has switched team
                    match *old_team.as_ref().unwrap() == *new_team {
                        // Player is still in the same team
                        true => None,
                        false => {
                            // The player has switched team
                            new_state =
                                ProcessingState::Present(kill_time.clone(), Some(new_team.clone()));
                            Some(TeamTime::new(join_time, kill_time, old_team.clone()))
                        }
                    }
                }
            }
            // (ProcessingState::Disconnected, IntermediateTeamTime::Disconnect(_)) -> Player leaves twice
            // (ProcessingState::Absent, IntermediateTeamTime::Kill(time, team)) -> Player kills without being here
            _ => None,
        };

        if let Some(time) = time {
            times.push(time);
        };

        state = new_state;
    }

    times
}
