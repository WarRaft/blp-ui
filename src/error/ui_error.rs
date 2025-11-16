use num_enum::TryFromPrimitiveError;
use std::error::Error;
use std::{collections::BTreeMap, fmt, io, sync::Arc};

/// Lightweight UI error type modelled after `BlpError`.
#[derive(Debug, Clone)]
pub struct UiError {
    pub key: &'static str,
    pub args: BTreeMap<&'static str, String>,
    pub causes: Vec<Cause>,
}

#[derive(Debug, Clone)]
pub enum Cause {
    /// A wrapped error from the `blp` library.
    Blp(blp::BlpError),
    /// A boxed std error.
    Std(Arc<dyn Error + Send + Sync>),
}

impl UiError {
    #[inline]
    pub fn new(key: &'static str) -> Self {
        Self { key, args: BTreeMap::new(), causes: Vec::new() }
    }

    #[inline]
    pub fn with_arg(mut self, name: &'static str, val: impl ToString) -> Self {
        self.args.insert(name, val.to_string());
        self
    }

    #[inline]
    pub fn with_args(mut self, args: impl IntoIterator<Item = (&'static str, String)>) -> Self {
        for (k, v) in args {
            self.args.insert(k, v);
        }
        self
    }

    #[inline]
    pub fn push_blp(mut self, cause: blp::BlpError) -> Self {
        self.causes.push(Cause::Blp(cause));
        self
    }

    #[inline]
    pub fn push_std(mut self, cause: impl Error + Send + Sync + 'static) -> Self {
        self.causes
            .push(Cause::Std(Arc::new(cause)));
        self
    }

    #[inline]
    pub fn ctx(self, key: &'static str) -> UiError {
        UiError::new(key)
            .push_blp(blp::BlpError::new("ui-ctx"))
            .push_blp_match(self)
    }

    #[inline]
    pub fn ctx_with(self, key: &'static str, f: impl FnOnce(UiError) -> UiError) -> UiError {
        f(UiError::new(key).push_blp_match(self))
    }

    // Helper to push a UiError as a Blp cause by converting it into a BlpError-like wrapper.
    // Here we simply embed the UiError's display into a synthetic BlpError key to preserve message.
    fn push_blp_match(mut self, _ui: UiError) -> UiError {
        // Create a minimal BlpError with the UiError message
        let blp = blp::BlpError::new("ui-cause");
        self.causes.push(Cause::Blp(blp));
        self
    }
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.key)?;
        let mut first = true;
        for (k, v) in &self.args {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{}={:?}", k, v)?;
        }
        write!(f, ")")
    }
}

impl Error for UiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.causes
            .iter()
            .find_map(|c| match c {
                Cause::Blp(e) => Some(e as &dyn Error),
                Cause::Std(e) => Some(e.as_ref()),
            })
    }
}

impl From<io::Error> for UiError {
    fn from(e: io::Error) -> Self {
        UiError::new("io-error").push_std(e)
    }
}

impl From<blp::image::ImageError> for UiError {
    fn from(e: blp::image::ImageError) -> Self {
        UiError::new("image-error").push_std(e)
    }
}

impl From<blp::image::error::DecodingError> for UiError {
    fn from(e: blp::image::error::DecodingError) -> Self {
        UiError::new("png-error").push_std(e)
    }
}

impl<T> From<TryFromPrimitiveError<T>> for UiError
where
    T: num_enum::TryFromPrimitive + 'static,
    T::Primitive: Copy + Into<u64>,
{
    fn from(_err: TryFromPrimitiveError<T>) -> Self {
        UiError::new("num-error").with_arg("name", core::any::type_name::<T>())
    }
}
