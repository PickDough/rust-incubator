/// Basic expectations for error values.
///
/// Simplified version of [`std::error::Error`].
use std::{
    any::TypeId,
    fmt::{Debug, Display},
};

use crate::my_error::private::Sealed;

/// Basic expectations for error values.
pub trait MyError: Debug + Display {
    /// The lower-level source of this error, if any.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    ///
    /// use step_2_6::MyError;
    ///
    /// #[derive(Debug)]
    /// struct SuperError {
    ///     source: SuperErrorSideKick,
    /// }
    ///
    /// impl fmt::Display for SuperError {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "SuperError is here!")
    ///     }
    /// }
    ///
    /// impl MyError for SuperError {
    ///     fn source(&self) -> Option<&(dyn MyError + 'static)> {
    ///         Some(&self.source)
    ///     }
    /// }
    ///
    /// #[derive(Debug)]
    /// struct SuperErrorSideKick;
    ///
    /// impl fmt::Display for SuperErrorSideKick {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "SuperErrorSideKick is here!")
    ///     }
    /// }
    ///
    /// impl MyError for SuperErrorSideKick {}
    ///
    /// fn get_super_error() -> Result<(), SuperError> {
    ///     Err(SuperError { source: SuperErrorSideKick })
    /// }
    ///
    /// fn main() {
    ///     match get_super_error() {
    ///         Err(e) => {
    ///             println!("Error: {e}");
    ///             println!("Caused by: {}", e.source().unwrap());
    ///         }
    ///         _ => println!("No error"),
    ///     }
    /// }
    /// ```
    fn source(&self) -> Option<&(dyn MyError + 'static)> {
        None
    }

    /// Gets the `TypeId` of `self`.
    ///
    /// __This is memory-unsafe to override in user code.__
    #[doc(hidden)]
    fn type_id(&self, _: Sealed) -> TypeId
        where
            Self: 'static,
    {
        TypeId::of::<Self>()
    }
}

mod private {
    pub struct Sealed;
}

impl<'a, T: MyError + ?Sized> MyError for &'a T {
    fn source(&self) -> Option<&(dyn MyError + 'static)> {
        MyError::source(&**self)
    }
}
