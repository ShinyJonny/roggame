use crate::map::Map;
use crate::screen::{Screen, WidgetHandle};

const WIDTH: u32  = 80;
const HEIGHT: u32 = 24;

pub struct Game {
    screen: Screen,
    maps: Vec<Map>,
    view: WidgetHandle,
    spacer: WidgetHandle,
    status_bar: WidgetHandle,
}

impl Game {
    pub fn init() -> Self
    {
        let mut screen = Screen::new(HEIGHT, WIDTH);

        let mut main_frame =  screen.add_widget(0, 0, HEIGHT, WIDTH);
        main_frame.set_border(('#', '#', '#', '#', '#', '#'));
        main_frame.toggle_border();
        main_frame.set_zindex(0);

        let view = screen.add_widget(1, 1, HEIGHT - 2 - 2, WIDTH - 2);
        let mut spacer = screen.add_widget(HEIGHT - 1 - 2, 1, 1, WIDTH - 2);
        let status_bar = screen.add_widget(HEIGHT - 1 - 1, 1, 1, WIDTH - 2);

        spacer.set_border(('#', '\0', '#', '#', '\0', '\0'));
        spacer.toggle_border();

        Self {
            screen,
            maps: Vec::new(),
            view,
            spacer,
            status_bar,
        }
    }

    pub fn load_maps(&mut self, map_paths: Vec<&str>)
    {
        for m_path in &map_paths
        {
            self.maps.push(
                Map::from_file(m_path).unwrap()
            )
        }
    }

    pub fn init_player(&mut self) // TODO
    {
    }

    pub fn start(&mut self) // TODO
    {
        self.screen.draw();
        self.screen.refresh();
    }
}
