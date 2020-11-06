//! Attribute macros for [`smol`](https://github.com/stjepang/smol).

#[doc(hidden)]
pub use async_io;
#[cfg(feature = "auto")]
pub use num_cpus;
#[doc(hidden)]
pub use std;

pub use smol_potat_macro::{bench, main, test};
