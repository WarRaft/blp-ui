use blp::AnyImage;
use crate::ext::path::ensure_readable::EnsureReadable;
use std::path::PathBuf;

pub enum FilePickInput {
    Path(PathBuf),
    Bytes(Vec<u8>),
}

impl FilePickInput {
    fn into_bytes(self) -> Result<Vec<u8>, blp::BlpError> {
        match self {
            FilePickInput::Path(path) => {
                path.as_path().read_all()
                    .map_err(|_e| blp::BlpError::new("ui-read-error"))
            }
            FilePickInput::Bytes(data) => Ok(data),
        }
    }

    pub fn decode(self) -> Result<AnyImage, blp::BlpError> {
        let data = self.into_bytes()?;
        let img = AnyImage::from_buffer(&data)?;
        Ok(img)
    }
}
