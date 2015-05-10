use events::KeyEvent;
use libevdev::InputEvent;
use num::FromPrimitive;

pub fn parse_event(ev: &InputEvent, buf: &mut Vec<char> ) {
    if ev.event_type == 1 && ev.value == 1 {
        let key = KeyEvent::from_u16(ev.code).unwrap();
        println!("Got key: {:?}", key.to_char());
    }
}
