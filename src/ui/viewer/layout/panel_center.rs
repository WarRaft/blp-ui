use crate::app::app::App;
use eframe::egui::{self, Align, CentralPanel, Frame, Image, Layout, Margin, RichText, ScrollArea, Sense, vec2};

impl App {
    pub(crate) fn draw_panel_center(&mut self, ctx: &egui::Context) {
        CentralPanel::default()
            .frame(Frame::default())
            .show(ctx, |ui| {
                if self.loading {
                    return self.draw_spinner(ctx);
                }

                let any_visible = self
                    .mip_visible
                    .iter()
                    .any(|visible| *visible);
                if !any_visible {
                    ui.centered_and_justified(|ui| {
                        ui.label(RichText::new(self.tr("no-visible-mip-hint")));
                    });
                    return;
                }
                ScrollArea::vertical()
                    .id_salt("right_scroll_mips")
                    .show(ui, |ui| {
                        let spy = ui.spacing().item_spacing.y;

                        ui.add_space(spy * 2.0);
                        let pad_lr: i8 = ui.spacing().item_spacing.x.round() as i8;
                        for i in 0..16 {
                            if !self.mip_visible[i] {
                                continue;
                            }

                            let tex_opt = self.mip_textures.get(i).cloned().flatten();
                            let (w, h) = tex_opt.as_ref()
                                .map(|t| (t.size()[0] as u32, t.size()[1] as u32))
                                .unwrap_or((0, 0));

                            // внешний горизонтальный паддинг
                            Frame::default().inner_margin(Margin { left: pad_lr, right: pad_lr, top: 0, bottom: 0 }).show(ui, |ui| {
                                let title = format!("#{i:02} {w}×{h}");

                                ui.horizontal(|ui| {
                                    let noim = RichText::new(self.tr("no-image"));

                                    if let Some(tex) = &tex_opt {
                                        let tex_size = tex.size_vec2();
                                        if tex_size.x > 0.0 && tex_size.y > 0.0 {
                                            let avail_w = ui.available_width();
                                            let draw_w = avail_w.min(tex_size.x);
                                            let draw_h = draw_w * (tex_size.y / tex_size.x);
                                            ui.add(Image::from_texture((tex.id(), tex_size)).fit_to_exact_size(vec2(draw_w, draw_h)));
                                        } else {
                                            ui.label(noim.clone());
                                        }
                                    } else {
                                        ui.label(noim.clone());
                                    }

                                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                                        ui.label(RichText::new(title).monospace());
                                    });
                                });
                            });

                            ui.add_space(spy);
                        }

                        // растяжка-строка
                        let _ = ui.allocate_exact_size(vec2(ui.available_width(), 0.0), Sense::hover());
                    });
            });
    }
}
