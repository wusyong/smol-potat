pub use smol_potat_macro::{main, bench, test};
#[cfg(feature="auto")]
pub use num_cpus;
#[doc(hidden)]
pub use smol::{run, block_on};