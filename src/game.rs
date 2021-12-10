use crate::map::Map;
use crate::screen::{Screen, Widget};

const WIDTH: usize  = 80;
const HEIGHT: usize = 24;

pub struct Game {
    screen: Screen,
    maps: Vec<Map>,
    main_frame: Widget,
    display: Widget,
    spacer: Widget,
    status_bar: Widget,
}

impl Game {
    pub fn init() -> Self
    {
        let mut screen = Screen::new(HEIGHT, WIDTH);

        let mut main_frame =  screen.add_widget(0, 0, HEIGHT, WIDTH);
        main_frame.set_border(('#', '#', '#', '#', '#', '#'));
        main_frame.toggle_border();
        main_frame.set_zindex(0);

        let display = screen.add_widget(1, 1, HEIGHT - 2 - 2, WIDTH - 2);
        let mut spacer = screen.add_widget(HEIGHT as u32 - 1 - 2, 1, 1, WIDTH - 2);
        let status_bar = screen.add_widget(HEIGHT as u32 - 1 - 1, 1, 1, WIDTH - 2);

        spacer.set_border(('#', '\0', '#', '#', '\0', '\0'));
        spacer.toggle_border();

        Self {
            screen,
            maps: Vec::new(),
            main_frame,
            display,
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
        let msg = "What is your name?";

        let h: u32;
        let w: u32;

        {
            let d = self.display.borrow();
            h = d.height as u32;
            w = d.width as u32;
        }

        self.display.print(
            (h - 5) / 2 + 2,
            (w - msg.len() as u32) / 2,
            msg
        );

        let msg = ">_____________________________";

        self.display.print(
            (h - 5) / 2 + 4 + 2,
            (w - msg.len() as u32) / 2,
            msg
        );
    }

    pub fn start(&mut self) // TODO
    {
        self.screen.draw();
        self.screen.refresh();
    }
}
