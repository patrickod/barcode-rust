use events::KeyEvent;
use libevdev::InputEvent;
use std::io::stdout;
use std::io::Write;

use num::FromPrimitive;

pub fn parse_event(ev: &InputEvent, buf: &mut Vec<u8> ) {
    // Only deal with KEY_DOWN events
    if ev.event_type == 1 && ev.value == 1 {
        let key = KeyEvent::from_u16(ev.code).unwrap();
        if !key.to_char().is_empty() {
            buf.push(key.to_char().parse::<u8>().unwrap());
        }
    }
    // Flush out the buffer if we've taken a newline
    if buf.len() > 0 && *buf.last().unwrap() == KeyEvent::KEY_END as u8 {
        let mut stdout = stdout();
        stdout.write(buf);
        stdout.flush();
        buf.clear();
    }
}
