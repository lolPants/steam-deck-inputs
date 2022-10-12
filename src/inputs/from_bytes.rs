use bytes::{Buf, BytesMut};
use thiserror::Error;

use super::raw::{Buttons1, Buttons2};
use super::Input;
use crate::constants::FRAME_SIZE;

#[derive(Debug, Clone, Copy, Error)]
#[error("not enough bytes to decode input")]
pub struct NotEnoughBytesError;

impl Input {
    pub fn from_bytes(bytes: &mut BytesMut) -> Result<Self, NotEnoughBytesError> {
        if bytes.remaining() < FRAME_SIZE {
            return Err(NotEnoughBytesError);
        }

        let input = Input {
            header: bytes.get_u32_le(),
            increment: bytes.get_u32_le(),
            buttons_1: Buttons1::from_bits_truncate(bytes.get_u32_le()),
            buttons_2: Buttons2::from_bits_truncate(bytes.get_u32_le()),
            left_trackpad_x: bytes.get_i16_le(),
            left_trackpad_y: bytes.get_i16_le(),
            right_trackpad_x: bytes.get_i16_le(),
            right_trackpad_y: bytes.get_i16_le(),
            accel_axis_right_to_left: bytes.get_i16_le(),
            accel_axis_top_to_bottom: bytes.get_i16_le(),
            accel_axis_front_to_back: bytes.get_i16_le(),
            gyro_axis_right_to_left: bytes.get_i16_le(),
            gyro_axis_top_to_bottom: bytes.get_i16_le(),
            gyro_axis_front_to_back: bytes.get_i16_le(),
            unknown_1: bytes.get_i16_le(),
            unknown_2: bytes.get_i16_le(),
            unknown_3: bytes.get_i16_le(),
            unknown_4: bytes.get_i16_le(),
            l2_analog: bytes.get_i16_le(),
            r2_analog: bytes.get_i16_le(),
            left_stick_x: bytes.get_i16_le(),
            left_stick_y: bytes.get_i16_le(),
            right_stick_x: bytes.get_i16_le(),
            right_stick_y: bytes.get_i16_le(),
            left_trackpad_push_force: bytes.get_i16_le(),
            right_trackpad_push_force: bytes.get_i16_le(),
            left_stick_touch_coverage: bytes.get_i16_le(),
            right_stick_touch_coverage: bytes.get_i16_le(),
        };

        Ok(input)
    }
}
