#![allow(dead_code)]

extern crate getopts;
extern crate scanner;
extern crate libc;

use std::ffi::CString;
use std::ffi::CStr;
use std::env;
use std::str;
use getopts::Options;

use scanner::parse;
use scanner::libevdev::{InputEvent,Libevdev,LibevdevGrabMode,NORMAL,BLOCKING};

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

fn listen(file: String) {
    unsafe {
        let f = libc::open(CString::new(file).unwrap().as_ptr(), libc::O_RDONLY, 0);
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

        let mut buf = vec![];

        while rc == 1 || rc == 0 || rc == -libc::EAGAIN {
            rc = libevdev_next_event(device, (NORMAL | BLOCKING).bits(), &mut ev);
            if rc == 0 {
                parse::parse_event(&ev, &mut buf);
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
