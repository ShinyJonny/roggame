use termion::input::TermRead;
use termion::event::{Event, Key};

pub fn getc() -> char
{
    let mut ret = '\0';

    for e in std::io::stdin().events() {
        if let Event::Key(Key::Char(c)) = e.unwrap() {
            ret = c;
            break;
        }
    }

    return ret;
}

pub fn getkey() -> Key
{
    let mut ret = Key::Esc;

    for e in std::io::stdin().events() {
        if let Event::Key(k) = e.unwrap() {
            ret = k;
            break;
        }
    }

    return ret;
}
