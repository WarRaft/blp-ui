use eframe::egui::TextEdit;

pub trait TextEditLikeButtonChain {
    fn like_button(self) -> Self;
}

impl<'t> TextEditLikeButtonChain for TextEdit<'t> {
    fn like_button(self) -> Self {
        // Простая заглушка - возвращаем сам TextEdit
        self
    }
}
