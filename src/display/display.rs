use std::ops::RangeInclusive;

use chrono::{DateTime, Utc};
use egui::{Color32, RichText, Slider, Stroke, Ui, Vec2b};
use egui_plot::{GridMark, Line, Plot, PlotPoint, PlotPoints, PlotUi, Text};
use log::debug;
use reqwest::blocking::Client;

use crate::{screen::Screen, utils::label_input};

use super::requests::get_team_times;

#[derive(Debug)]
pub struct DisplayScreen {
    id_field: String,
    name_field: String,
    client: Client,
    spacing: f64,
    width: f32,
    endpoint: String,
    players: Vec<(Player, Vec<TeamTime>)>,
}

#[derive(Debug)]
pub struct Player {
    id: String,
    name: String,
}

impl Player {
    pub fn new(id: String, name: String) -> Self {
        Self { name, id }
    }
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
        ui.horizontal(|ui| {
            ui.vertical(|ui| self.update_player_manager(ui));
            ui.separator();
            ui.vertical(|ui| self.update_plot_controls(ui));
        });

        self.update_plot(ui);
        None
    }
}

impl DisplayScreen {
    pub fn boxed(client: Client, endpoint: String) -> Box<Self> {
        Box::new(Self {
            id_field: String::new(),
            name_field: String::new(),
            endpoint,
            client,
            width: 10.,
            spacing: 0.,
            players: Vec::new(),
        })
    }

    fn update_player_manager(&mut self, ui: &mut Ui) {
        ui.label("Player Manager");

        label_input(ui, "Player Id", &mut self.id_field);
        label_input(ui, "Custom Name", &mut self.name_field);

        if ui.button("Add Player").clicked() {
            self.add_player();
        }

        self.update_watch_list(ui);
    }

    fn add_player(&mut self) {
        let body = get_team_times(&mut self.client, &self.endpoint, self.id_field.clone());
        let player = Player::new(self.id_field.clone(), self.name_field.clone());
        self.players.push((player, body));
    }

    fn update_watch_list(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let mut marked = vec![];

            for player in &self.players {
                let text = format!("{} {} - {}", player.0.name, player.0.id, player.1.len());
                if ui.button(&text).clicked() {
                    marked.push(player.0.id.clone());
                }
            }

            for mark in marked {
                let idx = self.players.iter().position(|p| p.0.id == mark).unwrap();
                self.players.remove(idx);
                debug!("Removed {} from watch list", mark);
            }
        });
    }

    fn update_plot_controls(&mut self, ui: &mut Ui) {
        ui.label("Plot Controls");

        let spacing_slider = Slider::new(&mut self.spacing, 0.0..=0.1)
            .clamp_to_range(false)
            .text("Spacing");
        ui.add(spacing_slider);

        let width_slider = Slider::new(&mut self.width, 0.0..=100.0)
            .clamp_to_range(false)
            .text("Width");
        ui.add(width_slider);
    }

    fn update_plot(&mut self, ui: &mut Ui) {
        Plot::new("team_times")
            .auto_bounds(Vec2b::new(true, false))
            .x_axis_formatter(x_axis_formatter)
            .show_grid(Vec2b::new(true, false))
            .show(ui, |plot_ui| {
                let mut offset = 0.0;
                for player in &self.players {
                    self.lines_for(offset, player, plot_ui);
                    offset = offset + self.spacing;
                }
            });
    }

    fn lines_for(&self, offset: f64, player: &(Player, Vec<TeamTime>), ui: &mut PlotUi) {
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

            let transform = ui.transform();
            let bounds = transform.bounds();
            let min_x = bounds.min()[0];
            let max_x = bounds.max()[0];
            let ten_percent = (max_x - min_x) * 0.1;
            let x = bounds.min()[0] + ten_percent;
            let point = PlotPoint::new(x, offset);

            // ui.zoom_bounds(zoom_factor, center)
            let widget = RichText::new(&player.0.name).strong().size(10.0);
            let text = Text::new(point, widget);

            ui.add(text);
            ui.add(line);
        }
    }
}

fn x_axis_formatter(mark: GridMark, _: &RangeInclusive<f64>) -> String {
    let timestamp = mark.value as i64;
    let datetime = DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap();

    const SECONDS_IN_A_DAY: f64 = 24. * 60. * 60.;
    if mark.step_size > SECONDS_IN_A_DAY {
        datetime.format("%d %b %y").to_string()
    } else {
        datetime.format("%H:%M").to_string()
    }
}
