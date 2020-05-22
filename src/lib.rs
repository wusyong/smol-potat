pub use smol_potat_derive::{main, bench, test};
#[cfg(feature="auto")]
pub use num_cpus;
pub use smol::*;