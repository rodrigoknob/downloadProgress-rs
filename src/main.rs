mod app_view;
mod tests;

use std::env::args;
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
    // /// Abosulte path of the uninstall file (.exe)
    // #[clap(short, long)]
    // uninstall_file_path: String,

    // /// Abosulte path of the installation file (.exe)
    // #[clap(short, long)]
    // install_file_path: String,

    /// Application process PID
    #[clap(short, long)]
    application_pid: u32,

    /// Show GUI of application
    #[clap(short, long)]
    show_gui: bool
}

fn main() {
    let args = Args::parse();
    // let install_file = args.install_file_path;
    // let uninstall_file = args.uninstall_file_path;
    let application_pid = args.application_pid;
    let show_gui = args.show_gui;

    // let install_file = "/home/rodrigo/Documentos/install.sh";
    let install_file = "C:\\Users\\rodrigo\\Downloads\\InCardio-Installer-WIN\\InCardioDuo-Installer.exe";
    // let uninstall_file = "/home/rodrigo/Documentos/uninstall.sh";
    let uninstall_file = "C:\\Users\\rodrigo\\AppData\\Local\\InCardioDuo\\unins000.exe";

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
    let thread = std::thread::spawn(move ||{

        close_application(application_pid);

        let output = Command::new(uninstall_file)
            .arg("/SILENT")
            .output()
            .expect("Failed to execute command");

        let output = Command::new(install_file)
            .output()
            .expect("Failed to execute command");

        std::process::exit(0x0100);

        if show_gui {
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
        }

    });

    if show_gui {
        let mut native_options = NativeOptions::default();
        native_options.initial_window_size = Some(Vec2::new(400.0, 150.0));

        eframe::run_native(Box::new(app), native_options);
    } else {
        thread.join().unwrap();
    }
}

fn close_application(pid: u32) {
    let mut count = 1;
    let time_limit = 10;
    let mut application_process_exists = false;

    while count < time_limit {
        application_process_exists = check_if_application_process_exists(pid);

        if !application_process_exists {
            return
        }

        std::thread::sleep(Duration::from_millis(500));
        count += 1
    }

    if application_process_exists {
        kill_application_process(pid)
    }
}

fn check_if_application_process_exists(pid: u32) -> bool {
    let pid_query = format!("PID eq {}", pid);

    let mut cmd_output = Command::new("cmd")
        .arg("/K")
        .arg("tasklist")
        .arg("/FI")
        .arg(pid_query)
        .output()
        .expect("Failed to execute command");

    let cmd_output_as_string = String::from_utf8_lossy(&cmd_output.stdout.as_mut_slice()).to_string();

    let pid_exists = cmd_output_as_string.contains(format!("{}", pid).as_str());

    println!("output : {}", String::from_utf8_lossy(&cmd_output.stdout.as_mut_slice()).to_string());
    println!("PID EXISTS??? {}", pid_exists);
    return pid_exists

}

fn kill_application_process(pid: u32) {
    Command::new("cmd")
        .arg("/K")
        .arg("taskkill")
        .arg("/PID")
        .arg(format!("{}", pid))
        .arg("/f")
        .output()
        .expect("Failed to execute command");
}


