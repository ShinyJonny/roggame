use termion::input::TermRead;

use crate::screen::Screen;
use crate::widget::{
    Widget,
    InteractiveWidget,
    OutputWidget,
    Window,
    HorizBar,
};
use crate::layout::{
    Justify,
    Alignable,
};
use crate::input;
use crate::gameui::{StartMenu, CharacterCreationForm};

const WIDTH: usize  = 80;
const HEIGHT: usize = 24;

const START_MENU_OPTS: &[&str] = &[
    "Start New Game",
    "Load Saved Game",
    "Exit",
];

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

struct GameState {}

pub struct Game {
    screen: Screen,
    ui: Ui,
    game_state: GameState,
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
            ui: Ui {
                window,
                main_frame,
                bar,
                status_bar,
            },
            game_state: GameState {},
        }
    }

    pub fn run(&mut self)
    {
        self.splash_screen();
        let option = self.start_screen();

        match option {
            StartMenuOption::NewGame => self.character_create(),
            StartMenuOption::LoadGame => self.character_select(),
            StartMenuOption::Exit => self.finish(),
        }
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

        logo.printj(Justify::HCentre(0), " ____              ____                      ");
        logo.printj(Justify::HCentre(1), "|  _ \\ ___   __ _ / ___| __ _ _ __ ___   ___ ");
        logo.printj(Justify::HCentre(2), "| |_) / _ \\ / _` | |  _ / _` | '_ ` _ \\ / _ \\");
        logo.printj(Justify::HCentre(3), "|  _ < (_) | (_| | |_| | (_| | | | | | |  __/");
        logo.printj(Justify::HCentre(4), "|_| \\_\\___/ \\__, |\\____|\\__,_|_| |_| |_|\\___|");
        logo.printj(Justify::HCentre(5), "            |___/                            ");
        message.printj(Justify::BottomCentre, "Press any key to continue.");
        self.screen.draw();
        self.screen.refresh();

        input::getkey();

        self.screen.rm_widget(&logo);
        self.screen.rm_widget(&message);
    }

    fn start_screen(&mut self) -> StartMenuOption
    {
        let mut menu = StartMenu::new(0, 0, None, None, START_MENU_OPTS);
        self.screen.add_widget(&menu);
        menu.align_centres(&self.ui.main_frame);
        menu.set_zindex(2);
        menu.show();

        self.screen.draw();
        self.screen.refresh();

        let mut output = None;

        for e in std::io::stdin().events() {
            menu.process_event(e.unwrap());

            self.screen.draw();
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
            _ => panic!("unreachable: invalid start menu option."),
        }
    }

    fn character_create(&mut self)
    {
        let character_create_entries = [
            "Name",
            "Gender",
            "Class",
        ];
        let mut form = CharacterCreationForm::new(0, 0, 4, 25, &character_create_entries);
        self.screen.add_widget(&form);
        form.align_centres(&self.ui.main_frame);
        form.set_zindex(2);
        form.show();

        self.screen.draw();
        self.screen.refresh();

        input::getkey();
    }

    fn character_select(&self) {} // TODO

    fn start(&self) {} // TODO

    fn finish(&self) {} // TODO
}
