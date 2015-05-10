use events::KeyEvent;
use libevdev::InputEvent;
use std::io::stdout;
use std::io::Write;

use num::FromPrimitive;

pub fn parse_event(ev: &InputEvent, buf: &mut Vec<u8> ) {
    if ev.event_type == 1 && ev.value == 1 {
        let key = KeyEvent::from_u16(ev.code).unwrap();
        if !key.to_char().is_empty() {
            buf.push(key.to_char().parse::<u8>().unwrap());
        }
    }
    if buf.last().unwrap() == &"\n".parse::<u8>().unwrap() {
        let mut stdout = stdout();
        stdout.write(buf);
        stdout.flush();
        buf.clear();
    }
}
