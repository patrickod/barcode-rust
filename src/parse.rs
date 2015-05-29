use libevdev::InputEvent;
use std::io::stdout;
use std::io::Write;

use isbn::isbn_13_valid;
use events::KeyEvent;

use num::FromPrimitive;

pub fn parse_event(ev: &InputEvent, buf: &mut Vec<u8> ) {
    if !should_parse_event(ev) {
        return;
    }

    let key = KeyEvent::from_u16(ev.code).unwrap();
    if !key.to_char().is_empty() {
        match key.to_char().parse::<u8>() {
            Ok(v) => buf.push(v),
            Err(_) => ()
        }
    }

    // Flush out the buffer if we've taken a newline
    if buf.len() > 0 && key == KeyEvent::KEY_ENTER {
        let mut stdout = stdout();

        if isbn_13_valid(buf) {
            for &c in buf.iter() {
                stdout.write(c.to_string().as_bytes());
            }
            stdout.write(key.to_char().as_bytes());
        }
        stdout.flush();
        buf.clear();
    }
}

fn should_parse_event(ev: &InputEvent) -> bool {
    return ev.event_type == 1 && ev.value == 1;
}
