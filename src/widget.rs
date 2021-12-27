use std::cmp::Ordering;
use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;
use crate::pos;

pub struct Widget {
    w: Rc<RefCell<InnerWidget>>,
}

impl Widget {
    pub fn new(start_y: u32, start_x: u32, height: usize, width: usize) -> Self
    {
        Self {
            w: Rc::new(RefCell::new(
                InnerWidget {
                    buffer: vec!['\0'; width * height],
                    start_y,
                    start_x,
                    height,
                    width,
                    z_index: 1,
                    has_border: false,
                    border_style: (' ', ' ', ' ', ' ', ' ', ' '),
                }
            ))
        }
    }

    pub fn share(&self) -> Self
    {
        Widget { w: self.w.clone() }
    }

    pub fn content_width(&self) -> usize
    {
        let w = self.borrow();

        if w.has_border {
            w.width - 2
        } else {
            w.width
        }
    }

    pub fn content_height(&self) -> usize
    {
        let w = self.borrow();

        if w.has_border {
            w.height - 2
        } else {
            w.height
        }
    }

    pub fn content_yx(&self) -> (u32, u32)
    {
        let w = self.borrow();

        if w.has_border {
            (w.start_y, w.start_x)
        } else {
            (w.start_y + 1, w.start_x + 1)
        }
    }

    // (horizontal bars, vertical bars, top-left corner, top-right corner, bottom-left corner,
    // bottom-right corner)
    pub fn set_border(&mut self, border: (char, char, char, char, char, char))
    {
        self.borrow_mut().border_style = border;
    }

    pub fn toggle_border(&mut self) -> Result<(), ()>
    {
        let mut w = self.borrow_mut();

        if w.width < 2 || w.height < 2 {
            return Err(());
        }

        if w.has_border {
            w.has_border = false;
        } else {
            w.has_border = true;
        }

        Ok(())
    }

    pub fn set_zindex(&mut self, z_index: u32)
    {
        self.borrow_mut().z_index = z_index;
    }

    pub fn print(&mut self, mut y: u32, mut x: u32, line: &str)
    {
        let mut w = self.borrow_mut();

        let mut width = w.width;
        let mut height = w.height;

        if w.has_border {
            if width < 2 || height < 2 {
                return;
            }

            y += 1;
            x += 1;
            width -= 2;
            height -= 2;
        } else {
            if width < 1 || height < 1 {
                return;
            }
        }

        if x as usize >= width || y as usize >= height {
            return;
        }

        let ww = w.width;

        for (i, c) in line.chars().enumerate() {
            if x as usize + i >= width as usize {
                break;
            }

            w.buffer[pos![ww, y as usize, x as usize + i]] = c;
        }
    }

    pub fn putc(&mut self, mut y: u32, mut x: u32, c: char)
    {
        let mut w = self.borrow_mut();

        let mut width = w.width;
        let mut height = w.height;

        if w.has_border {
            if width < 2 || height < 2 {
                return;
            }

            y += 1;
            x += 1;
            width -= 2;
            height -= 2;
        } else {
            if width < 1 || height < 1 {
                return;
            }
        }

        if x as usize >= width || y as usize >= height {
            return;
        }

        let ww = w.width;
        w.buffer[pos![ww, y as usize, x as usize]] = c;
    }

    pub fn clear(&mut self)
    {
        for c in self.borrow_mut().buffer.iter_mut() {
            *c = '\0';
        }
    }
}

impl Deref for Widget {
    type Target = Rc<RefCell<InnerWidget>>;

    fn deref(&self) -> &Self::Target
    {
        &self.w
    }
}

pub struct InnerWidget {
    pub buffer: Vec<char>,
    pub start_y: u32,
    pub start_x: u32,
    pub width: usize,
    pub height: usize,
    pub z_index: u32,
    pub has_border: bool,
    pub border_style: (char, char, char, char, char, char),
}

// Widgets are sorted based on their z_index.
// This simplifies the mechanisms for drawing or calculating which parts to draw.

impl PartialOrd for Widget {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.borrow().z_index.cmp(&other.borrow().z_index))
    }
}

impl Ord for Widget {
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.borrow().z_index.cmp(&other.borrow().z_index)
    }
}

impl PartialEq for Widget {
    fn eq(&self, other: &Self) -> bool
    {
        self.borrow().z_index == other.borrow().z_index
    }
}

impl Eq for Widget {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widget_handles_equal()
    {
        let a = Widget::new(0, 0, 0, 0);
        let b = Widget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 1;
        b.borrow_mut().z_index = 1;

        assert!(a == b);
    }

    #[test]
    fn widget_handles_not_equal()
    {
        let a = Widget::new(0, 0, 0, 0);
        let b = Widget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 1;
        b.borrow_mut().z_index = 2;

        assert!(a != b);
    }

    #[test]
    fn widget_handles_greater()
    {
        let a = Widget::new(0, 0, 0, 0);
        let b = Widget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 2;
        b.borrow_mut().z_index = 1;

        assert!(a > b);
    }

    #[test]
    fn widget_handles_smaller_or_eq()
    {
        let a = Widget::new(0, 0, 0, 0);
        let b = Widget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 0;
        b.borrow_mut().z_index = 1;

        assert!(a <= b);
    }

    #[test]
    fn widget_handles_sort()
    {
        let a = Widget::new(0, 0, 0, 0);
        let b = Widget::new(0, 0, 0, 0);
        let c = Widget::new(0, 0, 0, 0);
        let d = Widget::new(0, 0, 0, 0);
        let e = Widget::new(0, 0, 0, 0);
        let f = Widget::new(0, 0, 0, 0);
        let g = Widget::new(0, 0, 0, 0);
        a.borrow_mut().z_index = 0;
        b.borrow_mut().z_index = 1;
        c.borrow_mut().z_index = 7;
        d.borrow_mut().z_index = 3;
        e.borrow_mut().z_index = 9;
        f.borrow_mut().z_index = 4;
        g.borrow_mut().z_index = 2;

        let mut vector = vec![a, b, c, d, e, f, g];
        vector.sort();

        let mut last_z = 0;

        for w in vector {
            assert!(last_z <= w.borrow().z_index);
            last_z = w.borrow_mut().z_index;
        }
    }
}
