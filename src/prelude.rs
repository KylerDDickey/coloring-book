pub use crate::error::Error;

// Generic Result type for this application containing the application-scope
// Error type.
pub type Result<T> = core::result::Result<T, Error>;

// Wrapper type for applying traits to core or foreign types.
pub struct W<T>(pub T);
