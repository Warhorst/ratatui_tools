use ratatui_core::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Color,
    widgets::Widget,
};
use ratatui_widgets::block::Block;

pub struct TileMap<'a, F>
where
    F: Fn(&mut Tiles),
{
    clear_character: Option<Character>,
    block: Option<Block<'a>>,
    paint: Option<F>,
}

impl<'a, F> Default for TileMap<'a, F> where F: Fn(&mut Tiles) {
    fn default() -> Self {
        TileMap {
            clear_character: None,
            block: None,
            paint: None
        }
    }
}

impl<'a, F> TileMap<'a, F>
where
    F: Fn(&mut Tiles),
{
    pub fn clear_character(
        mut self,
        character: Character,
    ) -> Self {
        self.clear_character = Some(character);
        self
    }

    pub fn block(
        mut self,
        block: Block<'a>,
    ) -> Self {
        self.block = Some(block);
        self
    }

    pub fn paint(
        mut self,
        paint: F,
    ) -> Self {
        self.paint = Some(paint);
        self
    }
}

impl<'a, F> Widget for TileMap<'a, F>
where
    F: Fn(&mut Tiles),
{
    fn render(
        self,
        mut area: Rect,
        buf: &mut Buffer,
    ) where
        Self: Sized,
    {
        if let Some(block) = &self.block {
            block.render(area, buf);
            area = block.inner(area);
        }

        if let Some(Character { c, fg, bg }) = self.clear_character {
            area.positions().for_each(|p| {
                buf[p].set_char(c).set_fg(fg).set_bg(bg);
            });
        }

        if let Some(paint) = &self.paint {
            let mut tiles = Tiles::default();
            paint(&mut tiles);

            tiles
                .0
                .into_iter()
                .filter(|(p, _)| area.contains(*p))
                .for_each(|(p, Character { c, fg, bg })| {
                    buf[p].set_char(c).set_fg(fg).set_bg(bg);
                });
        }
    }
}

#[derive(Default)]
pub struct Tiles(Vec<(Position, Character)>);

impl Tiles {
    pub fn set_tile(
        &mut self,
        x: u16,
        y: u16,
        c: char,
        fg: Color,
        bg: Color,
    ) {
        self.0.push((Position::new(x, y), Character { c, fg, bg }));
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Character {
    c: char,
    fg: Color,
    bg: Color,
}

impl Character {
    pub fn new(
        c: char,
        fg: Color,
        bg: Color,
    ) -> Self {
        Character { c, fg, bg }
    }
}
