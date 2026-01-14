use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    style::Color,
    symbols::border,
    widgets::Block,
};
use ratatui_tools::tilemap::{Char, TileMap};

pub fn main() -> std::io::Result<()> {
    ratatui::run(|t| App::default().run(t))
}

#[derive(Default)]
pub struct App {
    should_exit: bool,
}

impl App {
    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> std::io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| {
                let block = Block::bordered().title("Tilemap").border_set(border::THICK);

                let tile_map = TileMap::default()
                    .block(block)
                    .clear_character(Char::new('#', Color::White).bg(Color::Gray))
                    .paint(|tiles| {
                        tiles.set_tile(10, 10, Char::new('@', Color::Black));
                    });

                frame.render_widget(tile_map, frame.area());
            })?;

            if let Event::Key(key_event) = event::read()?
                && key_event.kind == KeyEventKind::Press
                && key_event.code == KeyCode::Esc
            {
                self.should_exit = true;
            }
        }

        Ok(())
    }
}
