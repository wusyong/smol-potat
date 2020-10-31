//! Attribute macros for [`smol`](https://github.com/stjepang/smol).

#[doc(hidden)]
pub use async_channel;
#[doc(hidden)]
pub use async_executor;
#[doc(hidden)]
pub use easy_parallel;
#[doc(hidden)]
pub use futures_lite;
#[cfg(feature = "auto")]
pub use num_cpus;
#[doc(hidden)]
pub use smol::block_on;

pub use smol_potat_macro::{bench, main, test};
