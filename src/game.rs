use crate::map::Map;
use crate::screen::Screen;
use crate::widget::{Widget, InteractiveWidget, OutputWidget, Window, HorizBar, Prompt};
use crate::layout::{Justify, Aligned};
use crate::player::Player;

extern crate termion;
use termion::input::TermRead;

const WIDTH: usize  = 80;
const HEIGHT: usize = 24;

pub struct Game {
    screen: Screen,
    maps: Vec<Map>,
    window: Window,
    main_frame: Window,
    bar: HorizBar,
    status_bar: Window,
    player: Player,
}

impl Game {
    pub fn new() -> Self
    {
        let mut screen = Screen::init(HEIGHT, WIDTH);

        let mut window =  Window::new(0, 0, HEIGHT, WIDTH);
        let mut main_frame = Window::new(1, 1, HEIGHT - 2 - 2, WIDTH - 2);
        let mut bar = HorizBar::new(HEIGHT as u32 - 1 - 2, 1, WIDTH - 2);
        let mut status_bar = Window::new(HEIGHT as u32 - 1 - 1, 1, 1, WIDTH - 2);

        window.set_border(('#', '#', '#', '#', '#', '#'));
        window.toggle_border().unwrap();
        window.set_zindex(0);
        bar.set_style(('#', '#', '#'));

        screen.add_widget(&window);
        screen.add_widget(&main_frame);
        screen.add_widget(&status_bar);
        screen.add_widget(&bar);

        window.show();
        main_frame.show();
        status_bar.show();
        bar.show();

        Self {
            screen,
            maps: Vec::new(),
            window,
            main_frame,
            bar,
            status_bar,
            player: Player::new()
        }
    }

    pub fn load_maps(&mut self, map_paths: Vec<&str>)
    {
        for m_path in &map_paths {
            self.maps.push(
                Map::from_file(m_path).unwrap()
            )
        }
    }

    pub fn init_player(&mut self) // TODO
    {
        let mut dialog = Window::new(0, 0, 5, 30);
        self.screen.add_widget(&dialog);
        dialog.align_centres(&self.main_frame);

        dialog.printj(Justify::TopCentre, "What is your name?");
        dialog.show();

        let (dgy, dgx) = dialog.inner_start_yx();
        let (dgh, dgw) = (dialog.inner_height(), dialog.inner_width());
        let mut prompt = Prompt::new(dgy + dgh as u32 - 1, dgx, dgw);
        self.screen.add_widget(&prompt);
        prompt.show();
        self.screen.draw();
        self.screen.refresh();

        let stdin = std::io::stdin();
        let mut events = stdin.lock().events();

        let mut output_ready = prompt.try_get_output();
        while output_ready == None {
            if let Some(e) = events.next() {
                let e = e.unwrap();
                prompt.process_event(e);
                output_ready = prompt.try_get_output();
                self.screen.draw();
                self.screen.refresh();
            }
        }
        self.player.name = output_ready.unwrap();

        self.screen.rm_widget(&prompt);
        self.screen.rm_widget(&dialog);
        self.main_frame.printj(Justify::Centre, format!("Welcome, {}.", &self.player.name).as_str());
    }

    pub fn start(&mut self) // TODO
    {
        for _ in 0..60 {
            self.screen.draw();
            self.screen.refresh();
        }
    }
}
