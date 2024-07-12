use std::env;

use egui::Ui;
use log::{debug, Log};
use reqwest::{blocking::ClientBuilder, Url};
use serde_derive::{Deserialize, Serialize};

use crate::{
    display::display::DisplayScreen,
    screen::{OptScreen, Screen},
};

#[derive(Debug, Default)]
pub struct LoginScreen {
    endpoint: String,
    username: String,
    password: String,
    message: String,
}

impl LoginScreen {
    pub fn boxed() -> Box<Self> {
        Box::new(Self::default())
    }
}

impl Screen for LoginScreen {
    fn update(&mut self, ui: &mut Ui) -> OptScreen {
        label_input(ui, "Endpoint: ", &mut self.endpoint);
        label_input(ui, "Username: ", &mut self.username);
        label_input(ui, "Password: ", &mut self.password);
        ui.label(&self.message);
        if ui.button("Login").clicked() {
            return self.attempt_login();
        }

        None
    }
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    result: bool,
}

impl LoginScreen {
    fn attempt_environment_login(&mut self) -> OptScreen {
        let endpoint = env::var("ANALYSER_ENDPOINT").ok()?;
        let username = env::var("ANALYSER_USERNAME").ok()?;
        let password = env::var("ANALYSER_PASSWORD").ok()?;

        Some(
            self.attempt_environment_login()
                .expect("Failed to log in via environment variables"),
        )
    }

    fn attempt_login(&mut self) -> OptScreen {
        let client = ClientBuilder::new()
            .cookie_store(true)
            .build()
            .expect("Failed to build client");

        let url_string = format!("{}/api/login", self.endpoint);
        let Ok(login_url) = Url::parse(&url_string) else {
            self.message = "Invalid URL".to_string();
            return None;
        };

        let login_data = LoginRequest {
            username: self.username.clone(),
            password: self.password.clone(),
        };

        let Ok(response) = client.post(login_url).json(&login_data).send() else {
            self.message = "Sending request failed".to_string();
            return None;
        };

        let parsed_response = response
            .json::<LoginResponse>()
            .expect("Failed to parse response");
        if !parsed_response.result {
            self.message = "Invalid credentials".to_string();
            return None;
        }

        debug!("Successfully logged in");
        Some(DisplayScreen::boxed(client))
    }
}

fn label_input(ui: &mut Ui, label: &str, text: &mut String) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(text);
    });
}
