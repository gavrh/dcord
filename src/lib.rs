pub mod constants;
pub mod prelude;

#[cfg(feature="builder")]
pub mod builder;
#[cfg(feature="cache")]
pub mod cache;
#[cfg(feature="client")]
pub mod client;
#[cfg(feature="gateway")]
pub mod gateway;
#[cfg(feature="http")]
pub mod http;
#[cfg(feature="utils")]
pub mod utils;


mod error;

// For the procedual macros
pub use async_trait::async_trait;

/// Special module that re-exports most public items from this crate.
/// 
/// Useful, because you don't have to remember the full paths of dcrs items.
/// 
/// Not exported:
/// - [`none`]
pub mod any {
    #[cfg(feature="builder")]
    #[doc(no_inline)]
    pub use crate::builder::*;
    #[cfg(feature="cache")]
    #[doc(no_inline)]
    pub use crate::cache::*;
    #[cfg(feature="client")]
    #[doc(no_inline)]
    pub use crate::client::*;
    #[cfg(feature="gateway")]
    #[doc(no_inline)]
    pub use crate::gateway::*;
    #[cfg(feature="http")]
    #[doc(no_inline)]
    pub use crate::http::*;
    #[cfg(feature="utils")]
    #[doc(no_inline)]
    pub use crate::utils::*;
}