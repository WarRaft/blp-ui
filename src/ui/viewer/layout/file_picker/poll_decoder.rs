use crate::error::UiError;
use crate::app::app::App;
use eframe::egui::Context;
use std::sync::mpsc::TryRecvError;

impl App {
    pub(crate) fn poll_decoder(&mut self, ctx: &Context) {
        if !self.loading {
            return;
        }
        // Чтобы UI не "замерзал", просим перерисовку
        ctx.request_repaint();

        // ВАЖНО: сначала вынуть rx из self, чтобы дальше можно было менять self.*
        let Some(rx) = self.decode_rx.take() else {
            // канала нет — считать, что загрузка сорвалась
            self.loading = false;
            return;
        };

        match rx.try_recv() {
            // === успех ===
            Ok(Ok(any_image)) => {
                // Заливка текстур только для существующих уровней
                self.mip_visible.fill(false);
                // Extract Blp if it's a BLP format
                use blp::AnyImageData;
                if let AnyImageData::Blp(ref blp) = any_image.data {
                    self.blp = Some(blp.clone());
                }
                self.image = Some(any_image);
                self.loading = false;
                // rx дропаем — декодер завершён
            }

            // === ошибка из воркера (AppErr) ===
            Ok(Err(err)) => {
                    // Вкладываем как причину в "внешний" ключ, если нужен контекст
                    self.error = Some(UiError::new("error-poll-decoder").push_blp(err));
                self.blp = None;
                self.loading = false;
                // rx дропаем
            }

            // === канал пуст — оставляем rx и ждём следующего кадра ===
            Err(TryRecvError::Empty) => {
                self.decode_rx = Some(rx); // вернуть канал назад
                // self.loading остаётся true
            }

            // === воркер умер — фиксируем явную ошибку ===
            Err(TryRecvError::Disconnected) => {
                self.error = Some(UiError::new("blp.decode-thread-disconnected").with_arg("msg", "decoder thread disconnected"));
                self.blp = None;
                self.loading = false;
                // rx дропаем
            }
        }
    }
}
