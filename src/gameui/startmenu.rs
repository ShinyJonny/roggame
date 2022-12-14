use cwinui::style::OwnedStyledText;
use termion::event::Event;

use cwinui::widget::{
    Widget,
    InteractiveWidget,
    OutputWidget,
    InnerWidget,
    Window,
    Menu,
};
use cwinui::layout::{
    Aligned,
    Alignable,
    Align,
};
use cwinui::widget::PoisonError;
use cwinui::sub_impl_aligned;
use cwinui::sub_impl_alignable;

pub struct StartMenu {
    win: Window,
    menu: Menu,
}

impl StartMenu {
    pub fn new(
        y: u32,
        x: u32,
        height: Option<usize>,
        width: Option<usize>,
        items: &[&str]
    ) -> Self
    {
        let h = if let Some(height) = height { height } else { items.len() + 1 + 1 };
        let w = if let Some(width) = width {
            width
        } else {
            let mut max = 0;
            for i in 0..items.len() {
                let l = items[i].chars().count();
                if l > max {
                    max = l;
                }
            }

            max + 3 + 3
        };

        let mut menu = Menu::new(y + 1, x + 1, Some((h - 1, w - 4)), items);
        menu.set_theme(
            |item| {
                let mut line = OwnedStyledText::from("  ");
                line.content.push_str(item);
                line
            },
            |item| {
                let mut line = OwnedStyledText::from("* ");
                line.content.push_str(item);
                line
            },
        );
        menu.show();

        let win = Window::new(y, x, h, w);
        win.share_inner().add_subwidget(menu.share_inner());

        let mut ret = Self {
            win,
            menu,
        };
        ret.draw_decoration();

        ret
    }

    fn draw_decoration(&mut self)
    {
        // self.win.print(0, 0, "@");
        for x in 0..self.win.content_width() {
            self.win.print(0, x as u32, "=");
        }
        //for y in 0..self.win.content_height() {
        //    self.win.print(y as u32, 0, "|");
        //}
    }
}

impl Widget for StartMenu {
    fn share_inner(&self) -> InnerWidget
    {
        self.win.share_inner()
    }
}

impl InteractiveWidget for StartMenu {
    fn process_event(&mut self, e: Event)
    {
        self.menu.process_event(e);
    }
}

impl OutputWidget<usize> for StartMenu {
    fn try_get_output(&self) -> Option<usize>
    {
        self.menu.try_get_output()
    }

    fn get_output(&self) -> Result<usize, PoisonError<usize>>
    {
        self.menu.get_output()
    }
}

sub_impl_aligned!(StartMenu, win);
sub_impl_alignable!(StartMenu, win, [menu]);
