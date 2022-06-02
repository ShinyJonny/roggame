//use termion::event::Event;

use crate::widget::{
    InputLine,
    Widget,
    //InteractiveWidget,
    //OutputWidget,
    InnerWidget,
    Window,
};
use crate::layout::{
    Aligned,
    Alignable,
    Align,
    Justify
};
//use crate::misc::PoisonError;
use crate::sub_impl_aligned;

const FIELD_CAPACITY: usize = 1024;

pub struct CharacterCreationForm {
    win: Window,
    label_win: Window,
    input_win: Window,
    labels: Vec<String>,
    inputs: Vec<InputLine>,
}

impl CharacterCreationForm {
    pub fn new(
        y: u32,
        x: u32,
        height: usize,
        width: usize,
        entries: &[&str]
    ) -> Self
    {
        let win = Window::new(y, x, height, width);

        let mut labels = Vec::with_capacity(entries.len());
        for e in entries {
            labels.push(String::from(*e));
        }

        // FIXME: check that the dimensions are sufficient.

        let mut label_win_width = labels[0].len();
        for e in &entries[1..] {
            if e.len() > label_win_width {
                label_win_width = e.len()
            }
        }

        let label_win_x = x + 3;
        let input_win_x = label_win_x + label_win_width as u32 + 1;
        let input_win_width = width - (input_win_x - 3) as usize - 3;

        let mut label_win = Window::new(
            y + 1,
            label_win_x,
            height - 1,
            label_win_width
        );
        let mut input_win = Window::new(
            y + 1,
            input_win_x,
            height - 1,
            input_win_width
        );

        win.share_inner().add_subwidget(label_win.share_inner());
        win.share_inner().add_subwidget(input_win.share_inner());
        label_win.show();
        input_win.show();

        let mut inputs = Vec::new();
        for line in 0..entries.len() {
            let mut inl = InputLine::new(0, 0, input_win_width);
            inl.align_to_inner(&input_win, Align::TopLeft);
            inl.adjust_pos(line as i32, 0);
            inl.show();
            inl.set_inactive();
            input_win.share_inner().add_subwidget(inl.share_inner());

            inputs.push(inl)
        }

        inputs[0].set_active();

        let mut form = Self {
            win,
            label_win,
            input_win,
            labels,
            inputs,
        };
        form.draw();

        form
    }

    fn draw(&mut self)
    {
        self.draw_labels();
        self.draw_border();
    }

    fn draw_labels(&mut self)
    {
        for i in 0..self.labels.len() {
            self.label_win.printj(Justify::Right(i as u32), &self.labels[i]);
        }
    }

    fn draw_border(&mut self)
    {
        for i in 0..self.win.content_width() {
            self.win.putc(0, i as u32, '=');
        }
    }
}

impl Widget for CharacterCreationForm {
    fn share_inner(&self) -> InnerWidget
    {
        self.win.share_inner()
    }
}

sub_impl_aligned!(CharacterCreationForm, win);

impl Alignable for CharacterCreationForm {
    fn align_centres<T: Aligned>(&mut self, anchor: &T)
    {
        let (wy, wx) = self.win.outer_start_yx();
        self.win.align_centres(anchor);
        let (new_wy, new_wx) = self.win.outer_start_yx();
        let dy: i32 = new_wy as i32 - wy as i32;
        let dx: i32 = new_wx as i32 - wx as i32;

        self.label_win.adjust_pos(dy, dx);
        self.input_win.adjust_pos(dy, dx);
        for input in &mut self.inputs {
            input.adjust_pos(dy, dx);
        }
    }

    fn align_to_inner<T: Aligned>(&mut self, anchor: &T, a: Align)
    {
        let (wy, wx) = self.win.outer_start_yx();
        self.win.align_to_inner(anchor, a);
        let (new_wy, new_wx) = self.win.outer_start_yx();
        let dy: i32 = new_wy as i32 - wy as i32;
        let dx: i32 = new_wx as i32 - wx as i32;

        self.label_win.adjust_pos(dy, dx);
        self.input_win.adjust_pos(dy, dx);
        for input in &mut self.inputs {
            input.adjust_pos(dy, dx);
        }
    }

    fn align_to_outer<T: Aligned>(&mut self, anchor: &T, a: Align)
    {
        let (wy, wx) = self.win.outer_start_yx();
        self.win.align_to_outer(anchor, a);
        let (new_wy, new_wx) = self.win.outer_start_yx();
        let dy: i32 = new_wy as i32 - wy as i32;
        let dx: i32 = new_wx as i32 - wx as i32;

        self.label_win.adjust_pos(dy, dx);
        self.input_win.adjust_pos(dy, dx);
        for input in &mut self.inputs {
            input.adjust_pos(dy, dx);
        }
    }

    fn adjust_pos(&mut self, y: i32, x: i32)
    {
        self.win.adjust_pos(y, x);
        self.label_win.adjust_pos(y, x);
        self.input_win.adjust_pos(y, x);
        for input in &mut self.inputs {
            input.adjust_pos(y, x);
        }
    }

    fn change_pos(&mut self, y: u32, x: u32)
    {
        let (wy, wx) = self.win.outer_start_yx();
        self.win.change_pos(y, x);
        let (new_wy, new_wx) = self.win.outer_start_yx();
        let dy: i32 = new_wy as i32 - wy as i32;
        let dx: i32 = new_wx as i32 - wx as i32;

        self.label_win.adjust_pos(dy, dx);
        self.input_win.adjust_pos(dy, dx);
        for input in &mut self.inputs {
            input.adjust_pos(dy, dx);
        }
    }
}
