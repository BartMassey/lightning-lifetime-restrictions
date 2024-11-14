#[cfg(not(feature = "libfix"))]
mod lib {
    mod lib_restrict;
    pub use lib_restrict::*;
}

#[cfg(feature = "libfix")]
mod lib {
    mod lib_fixed;
    pub use lib_fixed::*;
}

pub use lib::*;
