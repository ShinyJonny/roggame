//use termion::event::Event;

use crate::widget::{
    InputLine,
    Widget,
    //InteractiveWidget,
    //OutputWidget,
    InnerWidget,
    Window,
};
use crate::layout::{Aligned, Align, Justify};
//use crate::misc::PoisonError;
//use crate::sub_impl_aligned;

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
            input_win.share_inner().add_subwidget(inl.share_inner());

            inputs.push(inl)
        }

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
