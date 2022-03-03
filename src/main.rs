mod app_view;

use std::process::Command;
use std::sync::{Arc, LockResult, Mutex};
use std::thread::{sleep, Thread};
use std::time::Duration;

use eframe::{epi, NativeOptions};
use eframe::egui::{CentralPanel, Pos2, ProgressBar, RichText, WidgetText};
use eventuals::{Eventual, EventualWriter, Value};

use crate::epi::{Frame, IconData, Storage};
use crate::epi::egui::{Context, Rgba, Ui, Vec2};

use clap::Parser;
use crate::app_view::{App, AppState, update_value};

/// Program to uninstall old version of InCardio and install new version of InCardio
#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// Abosulte path of the uninstall file (.exe)
    #[clap(short, long)]
    uninstall_file_path: String,

    /// Abosulte path of the installation file (.exe)
    #[clap(short, long)]
    install_file_path: String,
}

fn main() {
    let args = Args::parse();
    let install_file = args.install_file_path;
    let uninstall_file = args.uninstall_file_path;

    println!("uninstall path {} ---- install path {}", uninstall_file, install_file);

    let (mut writer, event) = Eventual::<AppState>::new();
    writer.write(AppState {
        text: String::from("Hello"),
        // progress: 0.0,
        closing_app_complete: false,
        install_complete: false,
        uninstall_complete: false
    });

    let app = App::new(event.clone());
    std::thread::spawn(move ||{
        sleep(Duration::new(2, 30));

        update_value(&event, &mut writer, |mut value| {
            value.closing_app_complete = true;
        });

        let output = Command::new(uninstall_file)
            .output()
            .expect("Failed to execute command");

        update_value(&event, &mut writer, |mut value| {
            value.uninstall_complete = true;
        });

        let output = Command::new(install_file)
            .output()
            .expect("Failed to execute command");

        update_value(&event, &mut writer, |mut value| {
            value.install_complete = true;
        });

        // println!("{}", String::from_utf8(output.stdout).unwrap());
        // update_value(&event, &mut writer, move |mut value| {
        //     value.text = format!("size: {}", size);
        // });
    });

    let mut native_options = NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(400.0, 150.0));

    eframe::run_native(Box::new(app), native_options);
}

