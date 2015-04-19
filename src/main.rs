extern crate libc;

use std::fs::File;
use std::path::Path;
use std::default::Default;
use std::os::unix::io::AsRawFd;

const EAGAIN: i16 = 11;

struct InputEvent {
    time: libc::timeval,
    event_type: u16,
    code: u16,
    value: u32
}

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

enum LibEvdevReadFlag {
    SYNC = 1, // < Process data in sync mode */
    NORMAL = 2, // < Process data in normal mode */
    FORCE_SYNC = 3, // < Pretend the next event is a SYN_DROPPED and require the caller to sync */
    BLOCKING = 4 // < The fd is not in O_NONBLOCK and a read may block */
}

struct libevdev;

#[link(name = "evdev")]
extern {
    fn libevdev_new() -> *mut libevdev;
    fn libevdev_new_from_fd(fd: i32, dev: *mut libevdev) -> u32;
    fn libevdev_next_event(dev: *mut libevdev, flag: i16, ev: *mut InputEvent) -> i16;
    fn libevdev_free(dev: *mut libevdev);
    fn libevdev_event_type_get_name(t: u16) -> String;
    fn libevdev_event_code_get_name(t: u16, code: u16) -> String;
}

fn main() {
    let mut f = File::open("/dev/input/event18");

    match f {
        Err(e) => panic!("got error {:?}", e),
        _ => ()
    }

    unsafe {
        let fd = f.unwrap().as_raw_fd();
        let device = libevdev_new();
        let dev = libevdev_new_from_fd(fd, device);
        let mut rc = 1;

        let mut ev: InputEvent = Default::default();

        while {
            rc == 1 || rc == 0 || rc == -EAGAIN
        } {
            rc = libevdev_next_event(device, LibEvdevReadFlag::NORMAL as i16, &mut ev);
            if (rc == 0) {
                println!("Event {:?} {:?} {:?}",
                    libevdev_event_type_get_name(ev.event_type),
                    libevdev_event_code_get_name(ev.event_type, ev.code),
                    ev.value);
            }
        }
    }
}
