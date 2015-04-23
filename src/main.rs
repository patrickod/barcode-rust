extern crate libc;
extern crate getopts;

use std::default::Default;
use std::ffi::CString;
use std::ffi::CStr;
use std::env;
use std::str;
use getopts::Options;

#[repr(C)]
struct InputEvent {
    time: libc::timeval,
    event_type: u16,
    code: u16,
    value: u32
}

#[repr(C)]
struct Libevdev;

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

#[warn(dead_code)]
enum LibevdevReadFlag {
    Sync = 1, // < Process data in sync mode */
    Normal = 2, // < Process data in normal mode */
    ForceSync = 3, // < Pretend the next event is a SYN_DROPPED and require the caller to sync */
    Blocking = 4 // < The fd is not in O_NONBLOCK and a read may block */
}

#[warn(dead_code)]
enum LibevdevGrabMode {
	Grab = 3,	// < Grab the device if not currently grabbed
	UnGrab = 4	// < Ungrab the device if currently grabbed
}

#[link(name = "evdev")]
extern {
    fn libevdev_new() -> *mut Libevdev;
    fn libevdev_set_fd(dev: *mut Libevdev, fd: i32) -> i32;
    fn libevdev_next_event(dev: *mut Libevdev, flag: u32, ev: *mut InputEvent) -> i32;
    fn libevdev_free(dev: *mut Libevdev);
    fn libevdev_event_type_get_name(t: u16) -> *const libc::c_char;
    fn libevdev_event_code_get_name(t: u16, code: u16) -> *const libc::c_char;
    fn libevdev_grab(dev: *mut Libevdev, grab: u32) -> i32;
}

fn print_event(ev: &InputEvent) {
    // Ignore keyup events
    if ev.value != 1 {
        return;
    }
    unsafe {
        let type_slice = CStr::from_ptr(libevdev_event_type_get_name(ev.event_type));
        let code_slice = CStr::from_ptr(libevdev_event_code_get_name(ev.event_type, ev.code));
        println!("Event {:?} {:?} {:?}",
            str::from_utf8(type_slice.to_bytes()).unwrap(),
            str::from_utf8(code_slice.to_bytes()).unwrap(),
            ev.value);
    }
}

fn listen(file: String) {
    unsafe {
        let f = libc::open(CString::new(file).unwrap().as_ptr(), libc::O_RDONLY | libc::O_NONBLOCK, 0);
        let device = libevdev_new();
        let err = libevdev_set_fd(device, f);

        let mut rc = 1;
        let mut ev: InputEvent = InputEvent::default();

        if err < 0 {
            let error_string = libc::strerror(-err);
            let slice = CStr::from_ptr(error_string);
            println!("Get error {:?}", str::from_utf8(slice.to_bytes()).unwrap());
            libevdev_free(device);
            return;
        }

        // Grab the device and prevent it writing to STDIN
        if libevdev_grab(device, LibevdevGrabMode::Grab as u32) != 0 {
            println!("Unable to grab device");
            libevdev_free(device);
            return;
        };

        while rc == 1 || rc == 0 || rc == -libc::EAGAIN {
            rc = libevdev_next_event(device, LibevdevReadFlag::Normal as u32, &mut ev);
            if rc == 0 {
                print_event(&ev);
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
