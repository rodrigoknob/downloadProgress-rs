mod app_view;

use std::process::Command;
use std::sync::{Arc, LockResult, Mutex};
use std::thread::sleep;
use std::time::Duration;

use eframe::{epi, NativeOptions};
use eframe::egui::{CentralPanel, Pos2, ProgressBar, RichText, WidgetText};
use eventuals::{Eventual, EventualWriter, Value};

use crate::epi::{Frame, IconData, Storage};
use crate::epi::egui::{Context, Rgba, Ui, Vec2};

use clap::Parser;
use crate::app_view::{App, AppState, get_native_options, update_value};

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

    println!("uninstall path {} ---- install path {}", args.uninstall_file_path, args.install_file_path);

    let native_options = get_native_options();
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

