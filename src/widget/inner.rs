use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;
use crate::pos;

pub struct Cursor {
    pub y: u32,
    pub x: u32,
    pub hidden: bool,
}

pub struct InnerWidgetBody {
    pub buffer: Vec<char>,
    pub start_y: u32,
    pub start_x: u32,
    pub width: usize,
    pub height: usize,
    pub cursor: Cursor,
    pub z_index: u32,
    pub hidden: bool,
    pub subwidgets: Vec<InnerWidget>,
}

pub struct InnerWidget(Rc<RefCell<InnerWidgetBody>>);

impl InnerWidget {
    pub fn new(start_y: u32, start_x: u32, height: usize, width: usize) -> Self
    {
        Self (
            Rc::new(RefCell::new(
                InnerWidgetBody {
                    buffer: vec!['\0'; width * height],
                    start_y,
                    start_x,
                    height,
                    width,
                    cursor: Cursor { y: 0, x: 0, hidden: true },
                    z_index: 1,
                    hidden: true,
                    subwidgets: Vec::new(),
                }
            ))
        )
    }

    pub fn share(&self) -> Self
    {
        InnerWidget(Rc::clone(&self))
    }

    pub fn add_subwidget(&mut self, sub: InnerWidget)
    {
        self.borrow_mut().subwidgets.push(sub);
    }

    pub fn print(&mut self, y: u32, x: u32, line: &str)
    {
        let y = y as usize;
        let x = x as usize;

        let mut body = self.borrow_mut();

        if x >= body.width || y >= body.height {
            return;
        }

        // TODO: support printing with newlines (and other non-standard whitespace).
        // FIXME: check for variable-length characters.
        // FIXME: check for non-printable characters.

        let mut print_len = line.chars().count();
        if x + print_len > body.width {
            print_len = body.width - x;
        }

        let w = body.width;
        let mut chars = line.chars();
        for i in 0..print_len {
            body.buffer[pos![w, y, x + i]] = chars.next().unwrap();
        }
    }

    pub fn putc(&mut self, y: u32, x: u32, c: char)
    {
        let mut body = self.borrow_mut();

        if x as usize >= body.width || y as usize >= body.height {
            return;
        }

        let w = body.width;
        body.buffer[pos![w, y as usize, x as usize]] = c;
    }

    pub fn clear(&mut self)
    {
        for c in self.borrow_mut().buffer.iter_mut() {
            *c = '\0';
        }
    }

    pub fn show_cursor(&mut self)
    {
        self.borrow_mut().cursor.hidden = false;
    }

    pub fn hide_cursor(&mut self)
    {
        self.borrow_mut().cursor.hidden = true;
    }

    pub fn move_cursor(&mut self, y: u32, x: u32)
    {
        let mut body = self.borrow_mut();

        if y as usize >= body.height || x as usize >= body.width {
            return;
        }

        body.cursor.y = y;
        body.cursor.x = x;
    }

    pub fn advance_cursor(&mut self, steps: i32)
    {
        let mut body = self.borrow_mut();

        if steps < 0 {
            if (-steps) as u32 > body.cursor.x {
                return;
            }
        } else if steps as u32 + body.cursor.x >= body.width as u32 {
            return;
        }

        body.cursor.x = (body.cursor.x as i32 + steps) as u32;
    }
}

impl Deref for InnerWidget {
    type Target = Rc<RefCell<InnerWidgetBody>>;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}
