//! Icon resources.
//! See:
//! - https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-448.html
//! - https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-101.html#HEADING101-48
//! - https://preterhuman.net/macstuff/insidemac/MoreToolbox/MoreToolbox-269.html

mod color;
mod io;
mod list_1bit;
mod masked_1bit;
mod paletted;
mod single_1bit;

pub use color::IconColorMaskedOldest;
pub use io::IconIO;
pub use list_1bit::{Icon1BitLargeMasked, Icon1BitSmallMaskedOldest};
pub use masked_1bit::{Icon1BitMiniMasked, Icon1BitSmallMasked};
pub use paletted::{
    Icon4BitLarge, Icon4BitMini, Icon4BitSmall, Icon8BitLarge, Icon8BitMini, Icon8BitSmall,
};
pub use single_1bit::Icon1BitLargeOldest;
