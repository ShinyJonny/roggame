use std::collections::HashMap;
use termion::event::{Event, Key};

use cwinui::widget::{
    InputLine,
    Widget,
    InteractiveWidget,
    OutputWidget,
    InnerWidget,
    Window,
    PoisonError,
};
use cwinui::layout::{
    Aligned,
    Alignable,
    Align,
    Justify
};
use cwinui::sub_impl_aligned;

const FIELD_CAPACITY: usize = 1024;

pub struct CharacterCreationForm {
    win: Window,
    label_win: Window,
    spacer_win: Window,
    input_win: Window,
    labels: Vec<String>,
    inputs: Vec<InputLine>,
    selected: usize,
    output_ready: bool,
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
        let spacer_win_x = label_win_x + label_win_width as u32;
        let spacer_win_width = 2 as usize;
        let input_win_x = label_win_x + label_win_width as u32 + spacer_win_width as u32;
        let input_win_width = width - 3 - label_win_width - spacer_win_width as usize - 3;

        let mut label_win = Window::new(
            y + 1,
            label_win_x,
            height - 1,
            label_win_width
        );
        let mut spacer_win = Window::new(
            y + 1,
            spacer_win_x,
            height - 1,
            spacer_win_width
        );
        let mut input_win = Window::new(
            y + 1,
            input_win_x,
            height - 1,
            input_win_width
        );

        win.share_inner().add_subwidget(label_win.share_inner());
        win.share_inner().add_subwidget(spacer_win.share_inner());
        win.share_inner().add_subwidget(input_win.share_inner());
        label_win.show();
        spacer_win.show();
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
            spacer_win,
            input_win,
            labels,
            inputs,
            selected: 0,
            output_ready: false,
        };
        form.draw();

        form
    }

    fn draw(&mut self)
    {
        self.draw_border();
        self.draw_labels();
        self.draw_spacer();
    }

    fn draw_labels(&mut self)
    {
        for i in 0..self.labels.len() {
            self.label_win.printj(&self.labels[i], Justify::Right(i as u32));
        }
    }

    fn draw_spacer(&mut self)
    {
        for i in 0..self.labels.len() {
            self.spacer_win.putc(i as u32, 0, ':');
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

impl InteractiveWidget for CharacterCreationForm {
    fn process_event(&mut self, e: Event)
    {
        match e {
            Event::Key(Key::Char('\t')) |
            Event::Key(Key::Down)=> {
                if self.selected + 1 != self.inputs.len() {
                    self.inputs[self.selected].set_inactive();
                    self.selected += 1;
                    self.inputs[self.selected].set_active();
                }
            },
            Event::Key(Key::BackTab) |
            Event::Key(Key::Up) => {
                if self.selected != 0 {
                    self.inputs[self.selected].set_inactive();
                    self.selected -= 1;
                    self.inputs[self.selected].set_active();
                }
            },
            Event::Key(Key::Char('\n')) => {
                self.output_ready = true;
            },
            _ => {
                self.inputs[self.selected].process_event(e);
            }
        }
    }
}

impl OutputWidget<HashMap<String, String>> for CharacterCreationForm {
    fn try_get_output(&self) -> Option<HashMap<String, String>>
    {
        if !self.output_ready {
            return None;
        }

        let mut map = HashMap::with_capacity(self.labels.len());
        for i in 0..self.labels.len() {
            let key = self.labels[i].clone();
            let val = self.inputs[i].get_output().unwrap_err().into_inner();
            map.insert(key, val);
        }

        Some(map)
    }

    fn get_output(&self) -> Result<HashMap<String, String>, PoisonError<HashMap<String, String>>>
    {
        let mut map = HashMap::with_capacity(self.labels.len());
        for i in 0..self.labels.len() {
            let key = self.labels[i].clone();
            let val = self.inputs[i].get_output().unwrap_err().into_inner();
            map.insert(key, val);
        }


        if !self.output_ready {
            return Err(PoisonError::new(map));
        }

        Ok(map)
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
        self.spacer_win.adjust_pos(dy, dx);
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
        self.spacer_win.adjust_pos(dy, dx);
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
        self.spacer_win.adjust_pos(dy, dx);
        self.input_win.adjust_pos(dy, dx);
        for input in &mut self.inputs {
            input.adjust_pos(dy, dx);
        }
    }

    fn adjust_pos(&mut self, y: i32, x: i32)
    {
        self.win.adjust_pos(y, x);
        self.label_win.adjust_pos(y, x);
        self.spacer_win.adjust_pos(y, x);
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
        self.spacer_win.adjust_pos(dy, dx);
        self.input_win.adjust_pos(dy, dx);
        for input in &mut self.inputs {
            input.adjust_pos(dy, dx);
        }
    }
}
