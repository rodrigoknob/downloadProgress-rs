use std::sync::{Arc, LockResult, Mutex};
use std::thread::sleep;
use std::time::Duration;

use eframe::{epi, NativeOptions};
use eframe::egui::{CentralPanel, Pos2, ProgressBar, RichText, WidgetText};
use eventuals::{Eventual, EventualWriter, Value};

use crate::epi::{Frame, IconData, Storage};
use crate::epi::egui::{Context, Rgba, Ui, Vec2};

#[derive(PartialEq, Clone)]
pub struct AppState {
    pub text: String,
    pub progress: f32
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

pub fn update_value<T>(now: &Eventual<T>, writer: &mut EventualWriter<T>, update: impl FnOnce(&mut T)) where T: Value {
    if let Some(mut value) = now.value_immediate() {
        update(&mut value);
        writer.write(value);
    }

}

pub fn get_native_options() -> NativeOptions {
    return NativeOptions {
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
    }
}