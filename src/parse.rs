use events::KeyEvent;
use libevdev::InputEvent;

pub fn parse_event(ev: &InputEvent, buf: &mut Vec<char> ) {
    let key = KeyEvent.from_u16(ev.code);
    println!("Got key: {:?}", key.to_char());
}
