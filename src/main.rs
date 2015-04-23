extern crate libc;
extern crate getopts;

use std::default::Default;
use std::ffi::CString;
use std::ffi::CStr;
use std::env;
use std::str;
use getopts::Options;

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

struct Libevdev;

#[link(name = "evdev")]
extern {
    fn libevdev_new() -> *mut Libevdev;
    fn libevdev_new_from_fd(fd: i32, dev: *mut Libevdev) -> i32;
    fn libevdev_next_event(dev: *mut Libevdev, flag: i16, ev: *mut InputEvent) -> i16;
    fn libevdev_free(dev: *mut Libevdev);
    fn libevdev_event_type_get_name(t: u16) -> String;
    fn libevdev_event_code_get_name(t: u16, code: u16) -> String;
}

fn listen(file: String) {
    unsafe {
        let f = libc::open(CString::new(file).unwrap().as_ptr(), libc::O_RDONLY | libc::O_NONBLOCK, 0);
        let mut device = libevdev_new();
        let err = libevdev_new_from_fd(f, device);
        let mut rc = 1;

        let mut ev: InputEvent = Default::default();

        if err < 0 {
            let error_string = libc::strerror(-err);
            let slice = CStr::from_ptr(error_string);
            println!("Get error {:?}", str::from_utf8(slice.to_bytes()).unwrap());
            libevdev_free(device);
            return;
        }

        while rc == 1 || rc == 0 || rc == -EAGAIN {
            rc = libevdev_next_event(device, LibEvdevReadFlag::NORMAL as i16, &mut ev);
            if rc == 0 {
                println!("Event {:?} {:?} {:?}",
                    libevdev_event_type_get_name(ev.event_type),
                    libevdev_event_code_get_name(ev.event_type, ev.code),
                    ev.value);
            }
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optopt("f", "", "device to listen on", "NAME");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") || !matches.opt_present("f") {
        print_usage(&program, opts);
        return;
    }

    let file = matches.opt_str("f");
    listen(file.unwrap());
}
