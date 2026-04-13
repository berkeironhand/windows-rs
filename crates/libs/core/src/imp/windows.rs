#[cfg(not(feature = "kernel"))]
mod factory_cache;
#[cfg(not(feature = "kernel"))]
pub use factory_cache::*;

#[cfg(not(feature = "kernel"))]
mod generic_factory;
#[cfg(not(feature = "kernel"))]
pub use generic_factory::*;

#[cfg(not(feature = "kernel"))]
mod marshaler;
#[cfg(not(feature = "kernel"))]
pub use marshaler::*;

#[cfg(not(feature = "kernel"))]
mod array_proxy;
#[cfg(not(feature = "kernel"))]
pub use array_proxy::*;
