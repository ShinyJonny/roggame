use crate::widget::{Widget, InteractiveWidget, OutputWidget, InnerWidget, Window};
use crate::layout::{Aligned, Align};
use crate::misc::PoisonError;

extern crate termion;
use termion::event::{Event, Key};

pub struct StartMenu<'a> {
    win: Window,
    active_item: u32,
    items: Vec<&'a str>,
    output: Option<u32>,
}

impl<'a> StartMenu<'a> {
    pub fn new(y: u32, x: u32, height: usize, width: usize, items: &[&'a str]) -> Self
    {
        let mut win = Window::new(y, x, height, width);

        // TODO: list overflow handling.
        if items.len() > 0 {
            win.print(0, 0, "*");
            win.print(0, 2, items[0]);
            for y in 1..items.len() {
                win.print(y as u32, 2, items[y]);
            }
        }

        Self {
            win,
            active_item: 0,
            items: Vec::from(items),
            output: None,
        }
    }
}

impl<'a> StartMenu<'a> {
    fn set_item(&mut self, i: u32) {
        self.win.clearln(i as usize);
        self.win.print(i, 0, "*");
        self.win.print(i, 2, self.items[i as usize]);
    }
    fn unset_item(&mut self, i: u32) {
        self.win.clearln(i as usize);
        self.win.print(i, 2, self.items[i as usize]);
    }
}

impl<'a> Widget for StartMenu<'a> {
    fn share_inner(&self) -> InnerWidget
    {
        self.win.share_inner()
    }
}

impl<'a> InteractiveWidget for StartMenu<'a> {
    fn process_event(&mut self, e: Event)
    {
        match e {
            Event::Key(Key::Up) => {
                if self.active_item > 0 {
                    self.unset_item(self.active_item);
                    self.active_item -= 1;
                    self.set_item(self.active_item);
                }
            },
            Event::Key(Key::Down) => {
                if self.active_item + 1 < self.items.len() as u32 {
                    self.unset_item(self.active_item);
                    self.active_item += 1;
                    self.set_item(self.active_item);
                }
            },
            Event::Key(Key::Char('\n')) | Event::Key(Key::Char(' ')) => {
                self.output = Some(self.active_item);
            },
            // TODO: Esc | End
            // TODO: mouse support
            _ => (),
        }
    }
}

impl<'a> OutputWidget<u32> for StartMenu<'a> {
    fn try_get_output(&mut self) -> Option<u32>
    {
        self.output
    }

    fn get_output(self) -> Result<u32, PoisonError<u32>>
    {
        if let Some(o) = self.output {
            Ok(o)
        } else {
            // FIXME: is this really the correct way to do this???
            Err(PoisonError::new(0))
        }
    }
}

impl<'a> Aligned for StartMenu<'a> {
    fn inner_width(&self) -> usize
    {
        self.win.inner_width()
    }

    fn inner_height(&self) -> usize
    {
        self.win.inner_height()
    }

    fn inner_start_yx(&self) -> (u32, u32)
    {
        self.win.inner_start_yx()
    }

    fn outer_width(&self) -> usize
    {
        self.win.outer_width()
    }

    fn outer_height(&self) -> usize
    {
        self.win.outer_height()
    }

    fn outer_start_yx(&self) -> (u32, u32)
    {
        self.win.outer_start_yx()
    }

    fn centre(&self) -> (u32, u32)
    {
        self.win.centre()
    }

    fn align_centres<T: Aligned>(&mut self, anchor: &T)
    {
        self.win.align_centres(anchor);
    }

    fn align_to_inner<T: Aligned>(&mut self, anchor: &T, a: Align)
    {
        self.win.align_to_inner(anchor, a);
    }

    fn align_to_outer<T: Aligned>(&mut self, anchor: &T, a: Align)
    {
        self.win.align_to_outer(anchor, a);
    }

    fn adjust_pos(&mut self, y: i32, x: i32)
    {
        self.win.adjust_pos(y, x);
    }

    fn change_pos(&mut self, y: u32, x: u32)
    {
        self.win.change_pos(y, x);
    }
}
