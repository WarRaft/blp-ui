use crate::ext::path::to_abs_string_with_macros::PathMacrosExt;
use crate::app::app::App;
use crate::ui::viewer::layout::file_picker::hotkey_pressed::hotkey_pressed;
use crate::ui::i18n::shortcut::platform_cmd_shortcut;
use crate::ui::i18n::lng_list::LngList;
use crate::ui::i18n::prefs::save_lang;
use eframe::egui::{self, ComboBox, Context, Key, TextEdit, TopBottomPanel};

impl App {
    pub(crate) fn draw_file_picker(&mut self, ctx: &Context) {
        for f in ctx.input(|i| i.raw.dropped_files.clone()) {
            if let Some(path) = f.path {
                if let Err(e) = self.pick_from_file(Some(path)) {
                    self.error = Some(e);
                }
            }
        }

        let open_hotkey = hotkey_pressed(ctx, Key::O);
        let paste_hotkey = hotkey_pressed(ctx, Key::V);

        let mut click_select = false;
        let mut click_paste = false;

        TopBottomPanel::top("file_picker_bar")
            .show_separator_line(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(!self.loading, |ui| {
                        // Open
                        if ui
                            .button(self.tr("open"))
                            .on_hover_text(format!("{}
{}", self.tr("open-hint"), platform_cmd_shortcut("O")))
                            .clicked()
                        {
                            click_select = true;
                        }

                        // Paste
                        if ui
                            .button(self.tr("paste"))
                            .on_hover_text(format!("{}
{}", self.tr("paste-hint"), platform_cmd_shortcut("V")))
                            .clicked()
                        {
                            click_paste = true;
                        }
                    });

                    // Языковой переключатель
                    ComboBox::from_id_salt("menu_lng")
                        .selected_text(self.lng.name())
                        .show_ui(ui, |ui| {
                            for cand in [LngList::Uk, LngList::Ru, LngList::En, LngList::Zh, LngList::Tc] {
                                let sel = self.lng == cand;
                                if ui.selectable_label(sel, cand.name()).clicked() && !sel {
                                    self.lng = cand;
                                    let _ = save_lang(self.lng);
                                }
                            }
                        });

                    if let Some(path) = self.picked_file.clone() {
                        let mut s = path.to_abs_string_with_macros();
                        ui.add_sized(
                            [ui.available_width(), ui.spacing().interact_size.y],
                            TextEdit::singleline(&mut s)
                                .cursor_at_end(true)
                                .interactive(false),
                        );
                    } else {
                        let s = self.tr(if self.blp.is_some() { "pasted-image" } else { "drop-hint" });
                        ui.label(s);
                    }
                });
            });

        // Действия
        if open_hotkey || click_select {
            self.file_dialog_open();
        }
        if click_paste || paste_hotkey {
            if let Err(e) = self.pick_from_clipboard() {
                self.error = Some(e);
            }
        }

        // Отображение ошибок через toast
        if let Some(err) = self.error.take() {
            let error_msg = format!("{}", err);
            let error_title = self.tr("error").to_string();
            let close_label = self.tr("close").to_string();
            
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("error_toast"),
                egui::ViewportBuilder::default()
                    .with_title(&error_title)
                    .with_inner_size([400.0, 200.0])
                    .with_decorations(true)
                    .with_resizable(true),
                |ctx, _class| {
                    let mut close_clicked = false;
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.vertical(|ui| {
                            ui.heading(format!("⚠ {}", error_title));
                            ui.separator();
                            ui.label(&error_msg);
                            ui.add_space(10.0);
                            if ui.button(&close_label).clicked() {
                                close_clicked = true;
                            }
                        });
                    });
                    if close_clicked {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                },
            );
        }
    }
}
