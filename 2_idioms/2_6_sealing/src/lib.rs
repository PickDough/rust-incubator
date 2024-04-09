#![allow(dead_code, unused)]
mod my_error;
mod my_iterator_ext;

use crate::my_error::MyError;
use std::fmt::Display;
use std::fmt::Formatter;

///
///```compile_fail
/// mod my_iterator_ext;
///
/// use crate::my_iterator_ext::MyIteratorExt;
///
/// struct Foo;
///
/// impl Iterator for Foo {
///     type Item = ();
///
///     fn next(&mut self) -> Option<Self::Item> {
///         todo!()
///     }
/// }
///
/// impl MyIteratorExt for Foo {
///     type Item = ();
///
///     fn next(&mut self) -> Option<Self::Item> {
///         todo!()
///     }
/// }
/// fn main() { }
///```
#[derive(Debug)]
struct Bar;

impl Display for Bar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}

impl MyError for Bar {
    fn source(&self) -> Option<&(dyn MyError + 'static)> {
        todo!()
    }
}

///```compile_fail
/// mod my_error;
///
/// use crate::my_error::MyError;
/// use std::fmt::{Debug, Display, Formatter};
///
/// struct Bar;
/// impl Display for Bar {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         todo!()
///     }
/// }
///
/// impl MyError for Bar {
///     fn type_id(&self, _: crate::my_error::private::Sealed) -> TypeId where Self: 'static {
///         todo!()
///     }
/// }
/// fn main() { }
///```
struct Foo;
