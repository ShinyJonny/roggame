use crate::map::Map;
use crate::screen::Screen;
use crate::widget::Widget;

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
    pub fn new() -> Self
    {
        let mut screen = Screen::init(HEIGHT, WIDTH);

        let mut main_frame =  screen.add_widget(0, 0, HEIGHT, WIDTH);
        main_frame.set_border(('#', '#', '#', '#', '#', '#'));
        main_frame.toggle_border().unwrap();
        main_frame.set_zindex(0);

        let display = screen.add_widget(1, 1, HEIGHT - 2 - 2, WIDTH - 2);
        let mut spacer = screen.add_widget(HEIGHT as u32 - 1 - 2, 1, 1, WIDTH - 2);
        let status_bar = screen.add_widget(HEIGHT as u32 - 1 - 1, 1, 1, WIDTH - 2);

        for i in 0..spacer.content_width() as u32 {
            spacer.putc(0, i, '#');
        }

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
        let w = self.display.content_width() as u32;
        let h = self.display.content_height() as u32;

        let msg = "What is your name?";

        self.display.print(
            (h - 5) / 2 + 2,
            (w - msg.len() as u32) / 2,
            msg,
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
        for _ in 0..60 {
            self.screen.draw();
            self.screen.refresh();
        }

        std::thread::sleep(std::time::Duration::from_millis(3000));
    }
}
