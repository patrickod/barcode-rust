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

pub enum LibevdevReadFlag {
    Sync = 1, // < Process data in sync mode */
    Normal = 2, // < Process data in normal mode */
    ForceSync = 3, // < Pretend the next event is a SYN_DROPPED and require the caller to sync */
    Blocking = 4 // < The fd is not in O_NONBLOCK and a read may block */
}

pub enum LibevdevGrabMode {
	Grab = 3,	// < Grab the device if not currently grabbed
	UnGrab = 4	// < Ungrab the device if currently grabbed
}
