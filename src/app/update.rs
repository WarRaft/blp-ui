pub(crate) use crate::app::app::App;
use eframe::egui::{self};

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(target_os = "macos")]
        {
            use crate::ui::viewer::layout::file_picker::macos_paste_event::{take_cmdv_event, tick_ensure_cmdv_event};
            tick_ensure_cmdv_event();
            if take_cmdv_event() {
                if let Err(e) = self.pick_from_clipboard() {
                    self.error = Some(e);
                }
            }
        }

        self.draw_file_picker(ctx);
        if self.blp.is_some() || self.loading {
            self.draw_panel_left(ctx);
            self.draw_panel_right(ctx);
            self.draw_panel_center(ctx);
        }
        self.poll_decoder(ctx);
    }
}
