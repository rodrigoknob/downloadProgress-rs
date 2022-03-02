extern crate dirs;

use std::process::Command;
use std::sync::{Arc, LockResult, Mutex};
use std::thread::sleep;
use std::time::Duration;

use dirs::home_dir;
use eframe::{epi, NativeOptions};
use eframe::egui::{CentralPanel, Pos2, ProgressBar, RichText, WidgetText};
use eventuals::{Eventual, EventualWriter, Value};

use crate::epi::{Frame, IconData, Storage};
use crate::epi::egui::{Context, Rgba, Ui, Vec2};


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

}

fn update_value<T>(now: &Eventual<T>, writer: &mut EventualWriter<T>, update: impl FnOnce(&mut T)) where T: Value {
    if let Some(mut value) = now.value_immediate() {
        update(&mut value);
        writer.write(value);
    }

}

fn get_script_path() -> Option<String> {
    let relative_path = String::from("Downloads");
    let script_name = String::from("test_script.sh");

    if let Some(user_home_path_buf) = dirs::home_dir() {
        let user_home_path = user_home_path_buf.display().to_string();
        let print_path = format!("{}/{}/{}", user_home_path, relative_path, script_name);
        return Some(print_path);
    } else {
        None
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


        if let Some(script_path) = get_script_path() {
            let output = Command::new(script_path)
                .output()
                .expect("Failed");

            update_value(&event, &mut writer, move |mut value| {
                value.text = String::from_utf8(output.stdout).unwrap();
            });
        }


        // let output = Command::new(script_path)
        //     .output()
        //     .expect("Failed");

        // let output = Command::new("ls")
        //     // .current_dir("$HOME")
        //     // .arg("/Downloads/test_script.sh")
        //     .output()
        //     .expect("Failed to execute command");

        // update_value(&event, &mut writer, move |mut value| {
        //     value.progress += 0.1;
        //     value.text = String::from_utf8(output.stdout).unwrap();
        // });

        // println!("{}", String::from_utf8(output.stdout).unwrap());
        // update_value(&event, &mut writer, move |mut value| {
        //     value.text = format!("size: {}", size);
        // });
    });


    eframe::run_native(Box::new(app), native_options);
}

