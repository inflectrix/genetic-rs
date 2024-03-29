pub mod prelude {
    pub use genetic_rs_common::prelude::*;

    #[cfg(feature = "derive")]
    pub use genetic_rs_macros::*;
}

pub use genetic_rs_common::*;

#[cfg(feature = "derive")]
pub use genetic_rs_macros::*;
