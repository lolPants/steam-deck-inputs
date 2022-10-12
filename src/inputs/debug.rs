use std::fmt::Debug;

use super::Inputs;

impl Debug for Inputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Input")
            .field("increment", &self.increment)
            .field("dpad", &self.dpad())
            .field("abxy", &self.abxy())
            .field("menu_buttons", &self.menu_buttons())
            .field("left_buttons", &self.left_buttons())
            .field("left_stick", &self.left_stick())
            .field("left_trackpad", &self.left_trackpad())
            .field("right_buttons", &self.right_buttons())
            .field("right_stick", &self.right_stick())
            .field("right_trackpad", &self.right_trackpad())
            .field("gyro", &self.gyro())
            .field("accelerometer", &self.accelerometer())
            .finish()
    }
}
