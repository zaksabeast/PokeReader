use core::fmt;
use num_enum::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Gender {
    Male = 0,
    Female = 1,
    Genderless = 2,
    #[num_enum(default)]
    Unknown = 3,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
