extern crate libc;

#[repr(C)]
pub struct InputEvent {
    time: libc::timeval,
    pub event_type: u16,
    pub code: u16,
    pub value: u32
}

#[repr(C)]
pub struct Libevdev;

impl Default for InputEvent {
    fn default() -> InputEvent {
        InputEvent {
            time: libc::timeval{ tv_sec: 0, tv_usec: 0 },
            event_type: 0,
            code: 0,
            value: 0
        }
    }
}

bitflags! {
    flags LibevdevReadFlag: u32 {
        const SYNC =      1,
        const NORMAL =    2,
        const FORCE_SYNC = 4,
        const BLOCKING =  8
    }
}

pub enum LibevdevGrabMode {
	Grab = 3,	// < Grab the device if not currently grabbed
	UnGrab = 4	// < Ungrab the device if currently grabbed
}
