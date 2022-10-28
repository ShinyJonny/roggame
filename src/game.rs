use std::collections::HashMap;
use cwinui::style::{Color, TextStyle};
use cwinui::style::WithStyle;
use termion::input::TermRead;
use termion::event::{Event, Key};

use cwinui::screen::Screen;
use cwinui::widget::{
    Widget,
    InteractiveWidget,
    OutputWidget,
    Window,
    HorizBar,
};
use cwinui::layout::{
    Justify,
    Alignable,
};
use crate::input;
use cwinui::pos;
use crate::gameui::{StartMenu, CharacterCreationForm};
use crate::player::Player;
use crate::map::Map;

const HEIGHT: usize = 24;
const WIDTH: usize  = 80;
const MAP_HEIGHT: usize = HEIGHT - 4;
const MAP_WIDTH: usize = WIDTH - 2;

type RgbValue = (u8, u8, u8);

const ACCENT_COLOR: RgbValue = (0x00, 0xd4, 0xaa);

enum StartMenuOption {
    NewGame,
    LoadGame,
    Exit,
}

struct Ui {
    window: Window,
    main_frame: Window,
    bar: HorizBar,
    status_bar: Window,
}

pub struct Game {
    screen: Screen,
    ui: Ui,
    state: GameState,
}

impl Game {
    pub fn new() -> Self
    {
        let mut screen = Screen::init(HEIGHT, WIDTH);

        let mut window =  Window::new(0, 0, HEIGHT, WIDTH);
        let mut main_frame = Window::new(1, 1, HEIGHT - 2 - 2, WIDTH - 2);
        let mut bar = HorizBar::new(HEIGHT as u32 - 1 - 2, 1, WIDTH - 2)
            .theme('#', '#', '#');
        let mut status_bar = Window::new(HEIGHT as u32 - 1 - 1, 1, 1, WIDTH - 2);

        window.set_theme('#', '#', '#', '#', '#', '#', '#', '#');
        window.toggle_border().unwrap();
        window.set_zindex(0);

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
            ui: Ui {
                window,
                main_frame,
                bar,
                status_bar,
            },
            state: GameState {
                player: Player::new(),
                map: Map::new(0, 0)
            },
        }
    }

    pub fn run(&mut self)
    {
        self.splash_screen();

        let option = self.start_screen();

        match option {
            StartMenuOption::NewGame => self.character_create(),
            StartMenuOption::LoadGame => self.character_select(),
            StartMenuOption::Exit => return,
        }

        self.init_map();
        self.start();
    }

    fn splash_screen(&mut self) {
        let mut logo = Window::new(0, 0, 6, self.ui.main_frame.content_width());
        let mut message = Window::new(0, 0, 1, self.ui.main_frame.content_width());
        self.screen.add_widget(&logo);
        self.screen.add_widget(&message);

        logo.align_centres(&self.ui.main_frame);
        logo.adjust_pos(-1, 0);
        logo.set_zindex(2);
        logo.show();
        message.align_centres(&logo);
        message.adjust_pos(7, 0);
        message.set_zindex(2);
        message.show();

        logo.show();
        message.show();

        logo.printj(" ____              ____                      ", Justify::HCentre(0));
        logo.printj("|  _ \\ ___   __ _ / ___| __ _ _ __ ___   ___ ", Justify::HCentre(1));
        logo.printj("| |_) / _ \\ / _` | |  _ / _` | '_ ` _ \\ / _ \\", Justify::HCentre(2));
        logo.printj("|  _ < (_) | (_| | |_| | (_| | | | | | |  __/", Justify::HCentre(3));
        logo.printj("|_| \\_\\___/ \\__, |\\____|\\__,_|_| |_| |_|\\___|", Justify::HCentre(4));
        logo.printj("            |___/                            ", Justify::HCentre(5));
        message.printj("Press any key to continue.", Justify::BottomCentre);
        self.screen.refresh();

        input::getkey();

        self.screen.rm_widget(&logo);
        self.screen.rm_widget(&message);
    }

    fn start_screen(&mut self) -> StartMenuOption
    {
        let start_menu_opts = [
            "Start New Game",
            "Load Saved Game",
            "Exit",
        ];

        let mut menu = StartMenu::new(0, 0, None, None, &start_menu_opts);
        self.screen.add_widget(&menu);
        menu.align_centres(&self.ui.main_frame);
        menu.set_zindex(2);
        menu.show();

        self.screen.refresh();

        let mut output = None;

        for e in std::io::stdin().events() {
            menu.process_event(e.unwrap());

            self.screen.refresh();

            if let Some(o) = menu.try_get_output() {
                output = Some(o);
                break;
            }
        }

        self.screen.rm_widget(&menu);

        match output {
            Some(0) => StartMenuOption::NewGame,
            Some(1) => StartMenuOption::LoadGame,
            Some(2) => StartMenuOption::Exit,
            _ => unreachable!("invalid start menu option."),
        }
    }

    fn character_create(&mut self)
    {
        let character_create_entries = [
            "Name".with_style(|f|
                f.fg_color(Color::Rgb(ACCENT_COLOR))
                 .text_style(TextStyle::BOLD | TextStyle::UNDERLINE)
            ),
        ];
        let mut form = CharacterCreationForm::new(0, 0, 4, 25, &character_create_entries);
        self.screen.add_widget(&form);
        form.align_centres(&self.ui.main_frame);
        form.set_zindex(2);
        form.show();

        self.screen.refresh();

        let mut output = HashMap::default();

        for e in std::io::stdin().events() {
            form.process_event(e.unwrap());

            self.screen.refresh();

            if let Some(o) = form.try_get_output() {
                output = o;
                break;
            }
        }

        self.state.player.name = output.remove("Name").unwrap();
        self.state.player.pos.y = MAP_HEIGHT as u32 / 2;
        self.state.player.pos.x = MAP_WIDTH as u32 / 2;

        self.screen.rm_widget(&form);
        self.screen.refresh();
    }

    fn character_select(&self) { todo!() } // TODO

    fn init_map(&mut self)
    {
        let map_name = "map.dat";
        let mut f = std::fs::File::open(map_name)
            .expect(format!("Couldn't open the map: {}", map_name).as_str());
        self.state.map = Map::from_reader(&mut f)
            .expect(format!("Couldn't read the map: {}", map_name).as_str());
    }

    fn start(&mut self) {
        self.update_map();
        self.screen.refresh();

        let mut input = std::io::stdin().events();

        loop {
            let next = input.next();

            // NOTE: for now, all of the operations, including redrawing and updating the game
            // state, are in lock-step with the user input. Nothing happens if there is no input.
            // Therefore, all of the operations happen in the block where a new event has been
            // received.

            if let Some(Ok(event)) = next {
                let mut recognised_event = true;

                match event {
                    Event::Key(Key::Right)
                    | Event::Key(Key::Char('l')) => self.state.player_move(Direction::Right),
                    Event::Key(Key::Left)
                    | Event::Key(Key::Char('h')) => self.state.player_move(Direction::Left),
                    Event::Key(Key::Up)
                    | Event::Key(Key::Char('k')) => self.state.player_move(Direction::Up),
                    Event::Key(Key::Down)
                    | Event::Key(Key::Char('j')) => self.state.player_move(Direction::Down),
                    Event::Key(Key::Char('y')) => self.state.player_move(Direction::TopLeft),
                    Event::Key(Key::Char('u')) => self.state.player_move(Direction::TopRight),
                    Event::Key(Key::Char('b')) => self.state.player_move(Direction::DownLeft),
                    Event::Key(Key::Char('n')) => self.state.player_move(Direction::DownRight),
                    Event::Key(Key::Char('q')) => break,
                    _ => recognised_event = false,
                }

                if recognised_event {
                    self.update_map();

                    self.screen.refresh();
                }
            } else if let Some(Err(_)) = next {
                // TODO: log the error
            }
        }
    }

    fn update_map(&mut self)
    {
        let map_height = self.state.map.height();
        let map_width = self.state.map.width();

        // Draw the map itsef
        for y in 0..map_height {
            for x in 0..map_width {
                let c = self.state.map.grid[pos!(map_width, y, x)].0 as char;

                self.ui.main_frame.putc(
                    y as u32,
                    x as u32,
                    c,
                )
            }
        }

        // Draw the player
        self.ui.main_frame.putc(
            self.state.player.pos.y,
            self.state.player.pos.x,
            '@'.with_style(|s| s.fg_color(Color::Rgb(ACCENT_COLOR)))
        );
    }
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
    TopRight,
    TopLeft,
    DownRight,
    DownLeft,
}

struct GameState {
    player: Player,
    map: Map,
}

impl GameState {
    /// Player action: move.
    /// Does not imply eny explicit movement type, this depends on the conditions of the player
    /// character (e.g. can result in attacking, jumping, etc.).
    pub fn player_move(&mut self, direction: Direction)
    {
        let (y_adjust, x_adjust): (i32, i32) = match direction {
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::TopRight => (-1, 1),
            Direction::TopLeft => (-1, -1),
            Direction::DownRight => (1, 1),
            Direction::DownLeft => (1, -1),
        };

        let new_y = self.player.pos.y as i32 + y_adjust;
        let new_x = self.player.pos.x as i32 + x_adjust;

        if new_y < 0 || new_x < 0 {
            return;
        }

        if new_y >= self.map.height() as i32 || new_x >= self.map.width() as i32 {
            return;
        }

        // TODO: more checks, resulting in more actions.

        self.player.pos.y = new_y as u32;
        self.player.pos.x = new_x as u32;
    }
}
