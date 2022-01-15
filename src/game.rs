use crate::map::Map;
use crate::screen::Screen;
use crate::widget::{Widget, InteractiveWidget, OutputWidget, Window, HorizBar};
use crate::layout::{Justify, Aligned};
use crate::player::Player;
use crate::input;
use crate::gameui::StartMenu;

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

    pub fn start_menu(&mut self)
    {
        let mut menu = StartMenu::new(
            0, 0, 3, 14 + 2,
            &vec!["Start New Game", "Load Game", "Exit"]
        );
        self.screen.add_widget(&menu);
        menu.align_centres(&self.main_frame);
        menu.adjust_pos(0, -2);
        menu.set_zindex(2);
        menu.show();

        self.screen.draw();
        self.screen.refresh();

        let mut output = None;
        for e in std::io::stdin().events() {
            menu.process_event(e.unwrap());
            self.screen.draw();
            self.screen.refresh();
            output = menu.try_get_output();
            if let Some(_) = output {
                break;
            }
        }

        self.screen.rm_widget(&menu);
    }

    pub fn splash_screen(&mut self) {
        let mut dialog = Window::new(
            0, 0,
            6 + (self.main_frame.content_height() as f64 * 0.2) as usize,
            self.main_frame.content_width()
        );
        self.screen.add_widget(&dialog);
        dialog.align_centres(&self.main_frame);
        dialog.set_zindex(2);
        dialog.show();

        dialog.printj(Justify::HCentre(0), " ____              ____                      ");
        dialog.printj(Justify::HCentre(1), "|  _ \\ ___   __ _ / ___| __ _ _ __ ___   ___ ");
        dialog.printj(Justify::HCentre(2), "| |_) / _ \\ / _` | |  _ / _` | '_ ` _ \\ / _ \\");
        dialog.printj(Justify::HCentre(3), "|  _ < (_) | (_| | |_| | (_| | | | | | |  __/");
        dialog.printj(Justify::HCentre(4), "|_| \\_\\___/ \\__, |\\____|\\__,_|_| |_| |_|\\___|");
        dialog.printj(Justify::HCentre(5), "            |___/                            ");
        dialog.printj(Justify::BottomCentre, "Press any key to continue.");
        self.screen.draw();
        self.screen.refresh();

        input::getkey();

        self.screen.rm_widget(&dialog);
    }

    pub fn start(&mut self) {} // TODO
}
