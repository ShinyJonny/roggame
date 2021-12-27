use crate::map::Map;
use crate::terminal::Terminal;
use crate::widget::Widget;
use crate::layout::{Justify, Align};
use crate::player::Player;

const WIDTH: usize  = 80;
const HEIGHT: usize = 24;

pub struct Game {
    terminal: Terminal,
    maps: Vec<Map>,
    window: Widget,
    main_frame: Widget,
    spacer: Widget,
    status_bar: Widget,
    player: Player,
}

impl Game {
    pub fn new() -> Self
    {
        let mut terminal = Terminal::new(HEIGHT, WIDTH);

        let mut window =  terminal.screen.add_widget(0, 0, HEIGHT, WIDTH);
        window.set_border(('#', '#', '#', '#', '#', '#'));
        window.toggle_border().unwrap();
        window.set_zindex(0);

        let main_frame = terminal.screen.add_widget(1, 1, HEIGHT - 2 - 2, WIDTH - 2);
        let mut spacer = terminal.screen.add_widget(HEIGHT as u32 - 1 - 2, 1, 1, WIDTH - 2);
        let status_bar = terminal.screen.add_widget(HEIGHT as u32 - 1 - 1, 1, 1, WIDTH - 2);

        for i in 0..spacer.content_width() as u32 {
            spacer.putc(0, i, '#');
        }

        Self {
            terminal,
            maps: Vec::new(),
            window,
            main_frame,
            spacer,
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
        let mut dialog = self.terminal.screen.add_widget(0, 0, 5, 30);
        dialog.align_to(self.main_frame.share(), Align::Center);

        dialog.print_just(Justify::TopCenter, "What is your name?");
        let (dgy, dgx) = dialog.content_yx();
        self.player.name = self.prompt(dgy + 5 - 1, dgx, 30);

        dialog.clear();
        dialog.print_just(Justify::Center, format!("Welcome, {}.", self.player.name).as_str());
    }

    pub fn start(&mut self) // TODO
    {
        for _ in 0..60 {
            self.terminal.screen.draw();
            self.terminal.screen.refresh();
        }
    }

    fn prompt(&mut self, y: u32, x: u32, length: usize) -> String
    {
        self.terminal.input_field(y, x, length)
    }
}
