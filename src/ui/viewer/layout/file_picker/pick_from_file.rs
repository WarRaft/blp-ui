use crate::ui::viewer::layout::file_picker::file_pick_input::FilePickInput;
use crate::error::UiError;
use crate::ext::path::ensure_readable::EnsureReadable;
use crate::app::app::App;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

impl App {
    pub(crate) fn pick_from_file(&mut self, path: Option<PathBuf>) -> Result<(), UiError> {
        let Some(path) = path else {
            return Ok(()); // ничего не выбрано
        };

        // Проверяем, что путь доступен для чтения
        path.as_path().ensure_readable()?;

        // Если это директория, обрабатываем как папку
        if path.is_dir() {
            // TODO: Implement directory scanning for BLP files
            return Err(UiError::new("directory-not-supported")
                .with_arg("path", path.display().to_string()));
        }

        // Обрабатываем как файл
        self.picked_file = Some(path.clone());
        self.blp = None;
        self.mip_textures.fill_with(|| None);

        let (tx, rx) = mpsc::sync_channel(1);
        self.decode_rx = Some(rx);
        self.loading = true;

        // поток для декодирования
        thread::spawn(move || {
            let res = FilePickInput::Path(path).decode();
            let _ = tx.send(res);
        });

        Ok(())
    }
}
