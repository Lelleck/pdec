use chrono::{DateTime, Utc};
use egui::{Color32, Slider, Stroke, Ui, Vec2b};
use egui_plot::{Line, Plot, PlotPoints, PlotUi};
use log::debug;
use reqwest::blocking::Client;

use crate::{screen::Screen, utils::label_input};

use super::requests::get_team_times;

#[derive(Debug)]
pub struct DisplayScreen {
    id_field: String,
    client: Client,
    offset: f64,
    width: f32,
    endpoint: String,
    players: Vec<(String, Vec<TeamTime>)>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Team {
    Axis,
    Allies,
}

#[derive(Debug)]
pub struct TeamTime {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub team: Team,
}

impl TeamTime {
    pub fn new(start: &DateTime<Utc>, end: &DateTime<Utc>, team: &Team) -> Self {
        Self {
            start: start.clone(),
            end: end.clone(),
            team: team.clone(),
        }
    }
}

impl Screen for DisplayScreen {
    fn update(&mut self, ui: &mut egui::Ui) -> Option<Box<dyn Screen>> {
        self.update_player_manager(ui);
        self.update_plot(ui);
        None
    }
}

impl DisplayScreen {
    pub fn boxed(client: Client, endpoint: String) -> Box<Self> {
        Box::new(Self {
            id_field: String::new(),
            endpoint,
            client,
            width: 10.,
            offset: 0.,
            players: Vec::new(),
        })
    }

    fn update_player_manager(&mut self, ui: &mut Ui) {
        label_input(ui, "Player Id", &mut self.id_field);
        if ui.button("Add Player").clicked() {
            self.add_player();
        }

        self.update_watch_list(ui);
    }

    fn add_player(&mut self) {
        let body = get_team_times(&mut self.client, &self.endpoint, self.id_field.clone());
        self.players.push((self.id_field.clone(), body));
    }

    fn update_watch_list(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let mut marked = vec![];

            for player in &self.players {
                let text = format!("{} - {}", player.0, player.1.len());
                if ui.button(&text).clicked() {
                    marked.push(player.0.clone());
                }
            }

            for mark in marked {
                let idx = self.players.iter().position(|p| p.0 == mark).unwrap();
                self.players.remove(idx);
                debug!("Removed {} from watch list", mark);
            }
        });
    }

    fn update_plot(&mut self, ui: &mut Ui) {
        let slider = Slider::new(&mut self.offset, 0.0..=0.1);
        ui.add(slider);

        let s2 = Slider::new(&mut self.width, 0.0..=100.0);
        ui.add(s2);

        Plot::new("team_times")
            .auto_bounds(Vec2b::new(true, false))
            .show(ui, |plot_ui| {
                let mut offset = 0.0;
                for player in &self.players {
                    self.lines_for(offset, player, plot_ui);
                    offset = offset + self.offset;
                }
            });
    }

    fn lines_for(&self, offset: f64, player: &(String, Vec<TeamTime>), ui: &mut PlotUi) {
        for team_time in &player.1 {
            let x_start = team_time.start.timestamp() as f64;
            let x_end = team_time.end.timestamp() as f64;
            let points = PlotPoints::new(vec![[x_start, offset], [x_end, offset]]);
            let color = if team_time.team == Team::Axis {
                Color32::RED
            } else {
                Color32::BLUE
            };
            let line = Line::new(points)
                .style(egui_plot::LineStyle::Solid)
                .width(self.width)
                .color(Color32::RED)
                .stroke(Stroke::new(self.width, color));

            ui.add(line);
        }
    }
}
