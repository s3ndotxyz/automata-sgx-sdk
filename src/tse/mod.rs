#[cfg(all(feature = "tse", target_vendor = "teaclave"))]
mod api_trusted;
#[cfg(all(feature = "tse", target_vendor = "teaclave"))]
pub use api_trusted::*;


#[cfg(any(
    not(feature = "tse"),
    all(feature = "tse", not(target_vendor = "teaclave"))
))]
mod api_untrusted;
#[cfg(any(
    not(feature = "tse"),
    all(feature = "tse", not(target_vendor = "teaclave"))
))]
pub use api_untrusted::*;
