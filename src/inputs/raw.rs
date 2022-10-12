use bitflags::bitflags;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Input {
    pub(crate) header: u32,
    pub(crate) increment: u32,
    pub(crate) buttons_1: Buttons1,
    pub(crate) buttons_2: Buttons2,
    pub(crate) left_trackpad_x: i16,
    pub(crate) left_trackpad_y: i16,
    pub(crate) right_trackpad_x: i16,
    pub(crate) right_trackpad_y: i16,
    pub(crate) accel_axis_right_to_left: i16,
    pub(crate) accel_axis_top_to_bottom: i16,
    pub(crate) accel_axis_front_to_back: i16,
    pub(crate) gyro_axis_right_to_left: i16,
    pub(crate) gyro_axis_top_to_bottom: i16,
    pub(crate) gyro_axis_front_to_back: i16,
    pub(crate) unknown_1: i16,
    pub(crate) unknown_2: i16,
    pub(crate) unknown_3: i16,
    pub(crate) unknown_4: i16,
    pub(crate) l2_analog: i16,
    pub(crate) r2_analog: i16,
    pub(crate) left_stick_x: i16,
    pub(crate) left_stick_y: i16,
    pub(crate) right_stick_x: i16,
    pub(crate) right_stick_y: i16,
    pub(crate) left_trackpad_push_force: i16,
    pub(crate) right_trackpad_push_force: i16,
    pub(crate) left_stick_touch_coverage: i16,
    pub(crate) right_stick_touch_coverage: i16,
}

bitflags! {
    pub(crate) struct Buttons1: u32 {
        const R2 = 1 << 0;
        const L2 = 1 << 1;
        const R1 = 1 << 2;
        const L1 = 1 << 3;
        const Y = 1 << 4;
        const B = 1 << 5;
        const X = 1 << 6;
        const A = 1 << 7;
        const DPAD_UP = 1 << 8;
        const DPAD_RIGHT = 1 << 9;
        const DPAD_LEFT = 1 << 10;
        const DPAD_DOWN = 1 << 11;
        const SELECT = 1 << 12;
        const STEAM = 1 << 13;
        const START = 1 << 14;
        const L5 = 1 << 15;
        const R5 = 1 << 16;
        const LEFT_TRACKPAD_CLICK = 1 << 17;
        const RIGHT_TRACKPAD_CLICK = 1 << 18;
        const LEFT_TRACKPAD_TOUCH = 1 << 19;
        const RIGHT_TRACKPAD_TOUCH = 1 << 20;
        const FLAG_21 = 1 << 21;
        const L3_CLICK = 1 << 22;
        const FLAG_23 = 1 << 23;
        const FLAG_24 = 1 << 24;
        const FLAG_25 = 1 << 25;
        const R3_CLICK = 1 << 26;
        const FLAG_27 = 1 << 27;
        const FLAG_28 = 1 << 28;
        const FLAG_29 = 1 << 29;
        const FLAG_30 = 1 << 30;
        const FLAG_31 = 1 << 31;
    }

    pub(crate) struct Buttons2: u32 {
        const FLAG_0 = 1 << 0;
        const FLAG_1 = 1 << 1;
        const FLAG_2 = 1 << 2;
        const FLAG_3 = 1 << 3;
        const FLAG_4 = 1 << 4;
        const FLAG_5 = 1 << 5;
        const FLAG_6 = 1 << 6;
        const FLAG_7 = 1 << 7;
        const FLAG_8 = 1 << 8;
        const L4 = 1 << 9;
        const R4 = 1 << 10;
        const FLAG_11 = 1 << 11;
        const FLAG_12 = 1 << 12;
        const FLAG_13 = 1 << 13;
        const L3_TOUCH = 1 << 14;
        const R3_TOUCH = 1 << 15;
        const FLAG_16 = 1 << 16;
        const FLAG_17 = 1 << 17;
        const QUICK_ACCESS = 1 << 18;
        const FLAG_19 = 1 << 19;
        const FLAG_20 = 1 << 20;
        const FLAG_21 = 1 << 21;
        const FLAG_22 = 1 << 22;
        const FLAG_23 = 1 << 23;
        const FLAG_24 = 1 << 24;
        const FLAG_25 = 1 << 25;
        const FLAG_26 = 1 << 26;
        const FLAG_27 = 1 << 27;
        const FLAG_28 = 1 << 28;
        const FLAG_29 = 1 << 29;
        const FLAG_30 = 1 << 30;
        const FLAG_31 = 1 << 31;
    }
}
