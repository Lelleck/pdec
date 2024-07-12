use std::{collections::HashMap, time::Instant};

use reqwest::blocking::Client;

use crate::screen::Screen;

#[derive(Debug)]
pub struct DisplayScreen {
    client: Client,
    players: HashMap<String, Vec<TeamTime>>,
}

#[derive(Debug)]
pub enum Team {
    Axis,
    Allies,
}

#[derive(Debug)]
pub struct TeamTime {
    pub start: Instant,
    pub end: Instant,
    pub team: Team,
}

impl Screen for DisplayScreen {
    fn update(&mut self, ui: &mut egui::Ui) -> Option<Box<dyn Screen>> {
        ui.label("Hello");
        None
    }
}

impl DisplayScreen {
    pub fn boxed(client: Client) -> Box<Self> {
        Box::new(Self { client })
    }
}
