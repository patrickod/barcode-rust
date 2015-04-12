struct InputEvent;
struct libevdev;

#[link(name = "seccomp")]
extern {
    fn libevdev_new() -> *mut libevdev;
    fn libevdev_new_from_fd(fd: u32, dev: *mut libevdev) -> u32;
    fn libevdev_next_event(dev: *mut libevdev, flag: u32, ev: *mut InputEvent);
    fn libevdev_free(dev: *mut libevdev);
}

fn main() {
    println!("Hello world from the barcode scanner");
}
