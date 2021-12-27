use crate::map::Map;
use crate::terminal::Terminal;
use crate::widget::Widget;

const WIDTH: usize  = 80;
const HEIGHT: usize = 24;

pub struct Game {
    terminal: Terminal,
    maps: Vec<Map>,
    window: Widget,
    main_frame: Widget,
    spacer: Widget,
    status_bar: Widget,
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
        let w = self.main_frame.content_width() as u32;
        let h = self.main_frame.content_height() as u32;

        let msg = "What   is   your name?";

        self.main_frame.print(
            (h - 5) / 2 + 2,
            (w - msg.len() as u32) / 2,
            msg,
        );

        self.prompt(
            (h - 5) / 2 + 4 + 2,
            (w - 30) / 2 + 1,
            30,
        );
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
