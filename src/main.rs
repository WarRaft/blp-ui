pub mod app;
mod error;
mod ext;
mod ui;
mod paint_bg_maze;

use crate::error::UiError;
use app::app::App;
use blp::image;
use eframe::egui::{IconData, ViewportBuilder, vec2};
use eframe::NativeOptions;
use std::path::PathBuf;
use std::sync::Arc;

#[inline]
fn report_error(msg: &str) {
    // stderr (visible if launched from a terminal)
    eprintln!("blp: {msg}");

    // Native dialog (requires `rfd`, which you already include under the `ui` feature)
    let _ = rfd::MessageDialog::new()
        .set_level(rfd::MessageLevel::Error)
        .set_title("blp - error")
        .set_description(msg)
        .show();
}

fn run_native(path: Option<PathBuf>) -> Result<(), UiError> {
    eframe::run_native(
        "blp",
        NativeOptions {
            viewport: ViewportBuilder::default()
                .with_title("blp")
                .with_app_id("org.warraft.blp")
                .with_inner_size(vec2(800.0, 680.0))
                .with_icon(Arc::new({
                    let img = image::load_from_memory(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png")))?.into_rgba8();
                    let (w, h) = img.dimensions();
                    IconData { rgba: img.into_raw(), width: w, height: h }
                })),
            ..Default::default()
        },
        Box::new(move |cc| -> Result<Box<dyn eframe::App>, _> {
            let mut app = App::new(&cc.egui_ctx);
            if let Err(e) = app.pick_from_file(path.clone()) {
                report_error(&format!("Failed to open file: {}", e));
                app.error = Some(e);
            }
            Ok(Box::new(app))
        }),
    )
    .map_err(|err| {
        report_error(&format!("Failed to launch UI: {}", err));
        UiError::new("ui-run-native").with_arg("msg", err.to_string())
    })
}

fn main() {
    if let Err(e) = run_native(None) {
        eprintln!("UI failed: {:?}", e);
        std::process::exit(1);
    }
}
