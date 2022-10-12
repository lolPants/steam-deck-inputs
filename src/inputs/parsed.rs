use derive_getters::Getters;

use super::raw::{Buttons1, Buttons2};
use super::Inputs;
use crate::utils::convert_range;

impl Inputs {
    pub fn increment(&self) -> u32 {
        self.increment
    }

    pub fn dpad(&self) -> DPad {
        DPad {
            up: self.buttons_1.contains(Buttons1::DPAD_UP),
            down: self.buttons_1.contains(Buttons1::DPAD_DOWN),
            left: self.buttons_1.contains(Buttons1::DPAD_LEFT),
            right: self.buttons_1.contains(Buttons1::DPAD_RIGHT),
        }
    }

    pub fn abxy(&self) -> ABXY {
        ABXY {
            a: self.buttons_1.contains(Buttons1::A),
            b: self.buttons_1.contains(Buttons1::B),
            x: self.buttons_1.contains(Buttons1::X),
            y: self.buttons_1.contains(Buttons1::Y),
        }
    }

    pub fn menu_buttons(&self) -> MenuButtons {
        MenuButtons {
            start: self.buttons_1.contains(Buttons1::STEAM),
            select: self.buttons_1.contains(Buttons1::SELECT),
            steam: self.buttons_1.contains(Buttons1::STEAM),
            quick_access: self.buttons_2.contains(Buttons2::QUICK_ACCESS),
        }
    }

    pub fn left_buttons(&self) -> SideButtons {
        SideButtons {
            shoulder: self.buttons_1.contains(Buttons1::L1),
            trigger_value: self.l2_analog,
            trigger_full: self.buttons_1.contains(Buttons1::L2),
            four: self.buttons_2.contains(Buttons2::L4),
            five: self.buttons_1.contains(Buttons1::L5),
        }
    }

    pub fn left_stick(&self) -> Stick {
        Stick {
            touch: self.buttons_2.contains(Buttons2::L3_TOUCH),
            click: self.buttons_1.contains(Buttons1::L3_CLICK),
            coverage: self.left_stick_touch_coverage,
            x: self.left_stick_x,
            y: self.left_stick_y,
        }
    }

    pub fn left_trackpad(&self) -> Trackpad {
        Trackpad {
            touch: self.buttons_1.contains(Buttons1::LEFT_TRACKPAD_TOUCH),
            click: self.buttons_1.contains(Buttons1::LEFT_TRACKPAD_CLICK),
            force: self.left_trackpad_push_force,
            x: self.left_trackpad_x,
            y: self.left_trackpad_y,
        }
    }

    pub fn right_buttons(&self) -> SideButtons {
        SideButtons {
            shoulder: self.buttons_1.contains(Buttons1::R1),
            trigger_value: self.r2_analog,
            trigger_full: self.buttons_1.contains(Buttons1::R2),
            four: self.buttons_2.contains(Buttons2::R4),
            five: self.buttons_1.contains(Buttons1::R5),
        }
    }

    pub fn right_stick(&self) -> Stick {
        Stick {
            touch: self.buttons_2.contains(Buttons2::R3_TOUCH),
            click: self.buttons_1.contains(Buttons1::R3_CLICK),
            coverage: self.right_stick_touch_coverage,
            x: self.right_stick_x,
            y: self.right_stick_y,
        }
    }

    pub fn right_trackpad(&self) -> Trackpad {
        Trackpad {
            touch: self.buttons_1.contains(Buttons1::RIGHT_TRACKPAD_TOUCH),
            click: self.buttons_1.contains(Buttons1::RIGHT_TRACKPAD_CLICK),
            force: self.right_trackpad_push_force,
            x: self.right_trackpad_x,
            y: self.right_trackpad_y,
        }
    }

    pub fn gyro(&self) -> Gyro {
        Gyro {
            right_to_left: self.gyro_axis_right_to_left,
            top_to_bottom: self.gyro_axis_top_to_bottom,
            front_to_back: self.gyro_axis_front_to_back,
        }
    }

    pub fn accelerometer(&self) -> Accelerometer {
        Accelerometer {
            right_to_left: self.accel_axis_right_to_left,
            top_to_bottom: self.accel_axis_top_to_bottom,
            front_to_back: self.accel_axis_front_to_back,
        }
    }
}

#[derive(Debug, Clone, Copy, Getters)]
#[allow(clippy::upper_case_acronyms)]
pub struct ABXY {
    a: bool,
    b: bool,
    x: bool,
    y: bool,
}

#[derive(Debug, Clone, Copy, Getters)]
pub struct DPad {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[derive(Debug, Clone, Copy, Getters)]
pub struct MenuButtons {
    start: bool,
    select: bool,

    steam: bool,
    quick_access: bool,
}

#[derive(Debug, Clone, Copy, Getters)]
pub struct SideButtons {
    shoulder: bool,

    trigger_value: i16,
    trigger_full: bool,

    four: bool,
    five: bool,
}

impl SideButtons {
    #[inline]
    pub fn trigger_value_small(&self) -> u8 {
        convert_range(self.trigger_value)
    }
}

#[derive(Debug, Clone, Copy, Getters)]
pub struct Stick {
    touch: bool,
    click: bool,
    coverage: i16,

    x: i16,
    y: i16,
}

impl Stick {
    #[inline]
    pub fn coverage_small(&self) -> u8 {
        convert_range(self.coverage)
    }

    #[inline]
    pub fn x_small(&self) -> u8 {
        convert_range(self.x)
    }

    #[inline]
    pub fn y_small(&self) -> u8 {
        convert_range(self.y)
    }
}

#[derive(Debug, Clone, Copy, Getters)]
pub struct Trackpad {
    touch: bool,
    click: bool,
    force: i16,

    x: i16,
    y: i16,
}

impl Trackpad {
    #[inline]
    pub fn force_small(&self) -> u8 {
        convert_range(self.force)
    }

    #[inline]
    pub fn x_small(&self) -> u8 {
        convert_range(self.x)
    }

    #[inline]
    pub fn y_small(&self) -> u8 {
        convert_range(self.y)
    }
}

#[derive(Debug, Clone, Copy, Getters)]
pub struct Gyro {
    right_to_left: i16,
    top_to_bottom: i16,
    front_to_back: i16,
}

#[derive(Debug, Clone, Copy, Getters)]
pub struct Accelerometer {
    right_to_left: i16,
    top_to_bottom: i16,
    front_to_back: i16,
}
