use std::fmt::Debug;

use bytes::{BufMut, BytesMut};
pub use hidapi_rusb::HidApi;
use hidapi_rusb::{HidDevice, HidError};
use thiserror::Error;

use crate::constants::{FRAME_SIZE, INTERFACE_NO, PRODUCT_ID, VENDOR_ID};
use crate::inputs::{Input, NotEnoughBytesError};

pub struct SteamDeckHIDReader {
    device: HidDevice,
    buf: [u8; FRAME_SIZE],
    bytes: BytesMut,
}

impl SteamDeckHIDReader {
    pub fn open(api: &HidApi) -> Result<Self, HIDOpenError> {
        for device in api.device_list() {
            if device.vendor_id() != VENDOR_ID {
                continue;
            }

            if device.product_id() != PRODUCT_ID {
                continue;
            }

            if device.interface_number() != INTERFACE_NO {
                continue;
            }

            let device = device.open_device(api)?;
            let reader = Self {
                device,
                buf: [0; FRAME_SIZE],
                bytes: BytesMut::with_capacity(FRAME_SIZE),
            };

            return Ok(reader);
        }

        Err(HIDOpenError::DeviceNotFound)
    }

    pub fn read_inputs(&mut self) -> Result<Input, ReadError> {
        self.device.read(&mut self.buf)?;

        self.bytes.clear();
        self.bytes.put(&self.buf[..]);

        let inputs = Input::from_bytes(&mut self.bytes)?;
        Ok(inputs)
    }
}

impl Debug for SteamDeckHIDReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SteamDeckHIDReader")
            .field("buf", &self.buf)
            .field("bytes", &self.bytes)
            .finish()
    }
}

#[derive(Debug, Error)]
pub enum HIDOpenError {
    #[error("HID device not found")]
    DeviceNotFound,

    #[error(transparent)]
    HidError(#[from] HidError),
}

#[derive(Debug, Error)]
pub enum ReadError {
    #[error(transparent)]
    NotEnoughBytesError(#[from] NotEnoughBytesError),

    #[error(transparent)]
    HidError(#[from] HidError),
}
