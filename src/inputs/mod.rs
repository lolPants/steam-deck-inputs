mod debug;
mod from_bytes;
mod parsed;
mod raw;

pub use from_bytes::NotEnoughBytesError;
pub use parsed::{Accelerometer, DPad, Gyro, MenuButtons, SideButtons, Stick, Trackpad, ABXY};
pub use raw::Inputs;
