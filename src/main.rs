use std::borrow::Cow;
use std::fmt::format;
use std::fs::File;
use std::ops::Deref;
use std::process::Command;
use std::sync::{Arc, LockResult, Mutex};
use std::thread::sleep;
use std::time::Duration;
use eframe::egui::{CentralPanel, Pos2, ProgressBar, RichText, WidgetText};
use eframe::{epi, NativeOptions};
use eframe::epaint::util::Float;
use eventuals::{Eventual, EventualWriter, Value};
use crate::epi::egui::{Context, Rgba, Ui, Vec2};
use crate::epi::{Frame, IconData, Storage};

use reqwest::blocking::*;

#[derive(PartialEq, Clone)]
struct AppState {
    text: String,
    progress: f32
}

impl Eq for AppState {}

struct App {
    state: Eventual<AppState>
}

impl App {
    fn new(state: Eventual<AppState>) -> App {
        App { state }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &Context, frame: &Frame) {
        if let Some(state) = self.state.value_immediate() {
            CentralPanel::default().show(ctx, |ui: &mut Ui| {
                ui.vertical(|ui|{
                    if ui.button(state.text).clicked() {
                    }
                    ui.add(ProgressBar::new(state.progress))
                })
            });
        }
        ctx.request_repaint();
    }

    fn name(&self) -> &str {
        "Hello"
    }

    // fn clear_color(&self) -> Rgba {
    //     Rgba::from_rgb(255.0, 255.0, 255.0)
    // }
}

fn update_value<T>(now: &Eventual<T>, writer: &mut EventualWriter<T>, update: impl FnOnce(&mut T)) where T: Value {
    if let Some(mut value) = now.value_immediate() {
        update(&mut value);
        writer.write(value);
    }

}

fn main() {
    let native_options = NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: false,
        icon_data: Some(IconData{height: 30u32, width: 30u32, rgba: vec![255, 255, 255]}),
        initial_window_pos: Some(Pos2::new(200.0, 300.0)),
        initial_window_size: Some(Vec2::new(200.0, 300.0)),
        min_window_size: Some(Vec2::new(200.0, 300.0)),
        max_window_size: Some(Vec2::new(200.0, 300.0)),
        resizable: false,
        transparent: false
    };

    let (mut writer, event) = Eventual::<AppState>::new();
    writer.write(AppState { text: String::from("0.0"), progress: 0.0});
    let app = App::new(event.clone());
    std::thread::spawn(move ||{
        for _ in 0..10 {
            update_value(&event, &mut writer, |mut value| {
                value.progress += 0.1;
            });
            sleep(Duration::from_millis(250));
        }

        let output = Command::new("ls")
            .arg("-la")
            .output()
            .expect("Failed to execute command");

        update_value(&event, &mut writer, move |mut value| {
            value.progress += 0.1;
            value.text = String::from_utf8(output.stdout).unwrap();
        });

        let client = reqwest::blocking::ClientBuilder::default().build().unwrap();

        let download_url = "https://releases.ubuntu.com/20.04.3/ubuntu-20.04.3-desktop-amd64.iso";
        let get_size_request = client.head(download_url)
            .build().unwrap();

        let size: u64 = client.execute(get_size_request).unwrap()
            .headers().get(reqwest::header::CONTENT_LENGTH).unwrap()
            .to_str().unwrap()
            .parse().unwrap();

        let download_path = std::path::Path::new("/tmp/ubuntu");
        if download_path.exists() {
            std::fs::remove_file(download_path).unwrap();
        }

        // client.execute(client.get(download_url)
        //     .build().unwrap()).unwrap().;

        update_value(&event, &mut writer, move |mut value| {
            value.text = format!("size: {}", size);
        });
    });
    eframe::run_native(Box::new(app), native_options);
}
