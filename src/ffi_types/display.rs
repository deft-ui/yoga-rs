use strum_macros::{Display, EnumString};
use crate::internal;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Display, EnumString)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[strum(ascii_case_insensitive, serialize_all= "kebab-case")]
pub enum Display {
    Flex = 0,
    None = 1,
}

impl From<Display> for internal::YGDisplay {
    fn from(d: Display) -> internal::YGDisplay {
        match d {
            Display::Flex => internal::YGDisplay::YGDisplayFlex,
            Display::None => internal::YGDisplay::YGDisplayNone,
        }
    }
}
