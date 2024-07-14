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
    TeamSwitch(DateTime<Utc>, Team),
    Disconnect(DateTime<Utc>),
}

impl IntermediateTeamTime {
    pub fn time(&self) -> &DateTime<Utc> {
        match self {
            IntermediateTeamTime::TeamSwitch(t, _) => t,
            IntermediateTeamTime::Disconnect(t) => t,
        }
    }
}

pub fn get_team_times(client: &mut Client, endpoint: &str, id: String) -> Vec<TeamTime> {
    let url_text = format!("{}/api/get_historical_logs", endpoint);
    let url = Url::parse(&url_text).expect("Failed to turn into URL");
    let body = HistoricalLogsRequest::by_id(id);
    let response = client
        .post(url)
        .json(&body)
        .send()
        .expect("Request failed")
        .json::<HistoricalLogsResponse>()
        .expect("Failed to deserialize json");

    let filtered_logs = response
        .result
        .into_iter()
        .filter(|p| p.kind == "DISCONNECTED" || p.kind == "CONNECTED")
        .collect::<Vec<_>>();

    let intermediate = historical_log_into_intermediate(filtered_logs);
    extract_team_times(intermediate)
}

fn historical_log_into_intermediate(logs: Vec<HistoricalLog>) -> Vec<IntermediateTeamTime> {
    let mut times = vec![];

    for historical_log in logs {
        if historical_log.kind == "DISCONNECTED" {
            times.push(IntermediateTeamTime::Disconnect(historical_log.time()));
        }
        if historical_log.kind == "CONNECTED" {
            /*
            let re = Regex::new(r"-> ([^)]*)\)").unwrap();
            let mut last_match = None;
            for mat in re.find_iter(&historical_log.raw) {
                last_match = Some(mat);
            }
            */

            // TODO: CRCON has a bug where it wont send the actual logs so we have to do this.
            // let team_match = last_match.unwrap().as_str();
            let team_match = "Allies";
            let team = match team_match {
                "Allies" => Team::Allies,
                _ => Team::Axis,
            };

            times.push(IntermediateTeamTime::TeamSwitch(
                historical_log.time(),
                team,
            ));
        }
    }

    times
}

fn extract_team_times(mut inters: Vec<IntermediateTeamTime>) -> Vec<TeamTime> {
    if inters.is_empty() {
        return Vec::new();
    }
    inters.sort_by(|a, b| a.time().cmp(b.time()));

    let mut team_times = Vec::new();
    let mut last = inters.first().unwrap();
    for inter in &inters.iter().skip(1).collect::<Vec<_>>() {
        let time = match (last, inter) {
            (
                IntermediateTeamTime::TeamSwitch(old_time, old_team),
                IntermediateTeamTime::TeamSwitch(new_time, _),
            ) => Some(TeamTime::new(old_time, new_time, old_team)),
            (
                IntermediateTeamTime::TeamSwitch(start, team),
                IntermediateTeamTime::Disconnect(end),
            ) => Some(TeamTime::new(start, end, team)),
            (IntermediateTeamTime::Disconnect(_), IntermediateTeamTime::TeamSwitch(_, _)) => None,
            (IntermediateTeamTime::Disconnect(_), IntermediateTeamTime::Disconnect(_)) => None,
        };

        if let Some(time) = time {
            team_times.push(time);
        }

        last = inter;
    }

    team_times
}
