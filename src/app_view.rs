use std::sync::{Arc, LockResult, Mutex};
use std::thread::sleep;
use std::time::Duration;

use eframe::{epi, NativeOptions};
use eframe::egui::{CentralPanel, Checkbox, Pos2, ProgressBar, RichText, Spinner, WidgetText};
use eventuals::{Eventual, EventualWriter, Value};

use crate::epi::{Frame, IconData, Storage};
use crate::epi::egui::{Context, Rgba, Ui, Vec2};

#[derive(PartialEq, Clone)]
pub struct AppState {
    pub text: String,
    // pub progress: f32,
    pub closing_app_complete: bool,
    pub install_complete: bool,
    pub uninstall_complete: bool
}

impl Eq for AppState {}

pub struct App {
    state: Eventual<AppState>
}

impl App {
    pub fn new(state: Eventual<AppState>) -> App {
        App { state }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &Context, frame: &Frame) {
        if let Some(mut state) = self.state.value_immediate() {

            CentralPanel::default().show(ctx, |ui: &mut Ui| {


                ui.vertical_centered_justified(|ui| {
                    ui.checkbox(&mut state.closing_app_complete, "Application Closed");
                    ui.checkbox(&mut state.uninstall_complete, "Application uninstalled");
                    ui.checkbox(&mut state.install_complete, "Application installed");
                    ui.add(Spinner::new().size(25.0));
                });

                // ui.checkbox(&mut state.closing_app_complete, "Application Closed");
                // ui.checkbox(&mut state.uninstall_complete, "Application uninstalled");
                // ui.checkbox(&mut state.install_complete, "Application installed");

            });
        }
        ctx.request_repaint();
    }

    fn name(&self) -> &str {
        "Hello"
    }

}

pub fn update_value<T>(now: &Eventual<T>, writer: &mut EventualWriter<T>, update: impl FnOnce(&mut T)) where T: Value {
    if let Some(mut value) = now.value_immediate() {
        update(&mut value);
        writer.write(value);
    }

}
