use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Deserialize;

pub const CHAR_STANDARD: &str = "Standard";

// We need to store SongDifficulty in slint data structures, therefore
// conversion from/to a primitive is needed.

#[repr(i32)]
#[derive(Clone, Copy, Deserialize, Eq, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive)]
pub enum SongDifficulty {
    Easy,
    Normal,
    Hard,
    Expert,
    ExpertPlus,
}
