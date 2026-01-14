use ratatui_core::{
    buffer::{Buffer, Cell},
    layout::{Position, Rect},
    style::Color,
    widgets::Widget,
};
use ratatui_widgets::block::Block;

pub struct TileMap<'a, F>
where
    F: Fn(&mut Tiles),
{
    clear_character: Option<Char>,
    block: Option<Block<'a>>,
    paint: Option<F>,
}

impl<'a, F> Default for TileMap<'a, F>
where
    F: Fn(&mut Tiles),
{
    fn default() -> Self {
        TileMap {
            clear_character: None,
            block: None,
            paint: None,
        }
    }
}

impl<'a, F> TileMap<'a, F>
where
    F: Fn(&mut Tiles),
{
    pub fn clear_character(
        mut self,
        character: Char,
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

        if let Some(char) = self.clear_character {
            area.positions()
                .for_each(|p| char.draw_on_cell(&mut buf[p]));
        }

        if let Some(paint) = &self.paint {
            let mut tiles = Tiles::default();
            paint(&mut tiles);

            tiles
                .0
                .into_iter()
                .map(|(p, c)| (Position::new(area.x + p.x, area.y + p.y), c))
                .filter(|(p, _)| area.contains(*p))
                .for_each(|(p, c)| c.draw_on_cell(&mut buf[p]));
        }
    }
}

#[derive(Default)]
pub struct Tiles(Vec<(Position, Char)>);

impl Tiles {
    pub fn set_tile(
        &mut self,
        x: u16,
        y: u16,
        char: Char,
    ) {
        self.0.push((Position::new(x, y), char));
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Char {
    c: char,
    fg: Color,
    bg: Option<Color>,
}

impl Char {
    pub fn new(
        c: char,
        fg: Color,
    ) -> Self {
        Char { c, fg, bg: None }
    }

    pub fn bg(
        mut self,
        bg: Color,
    ) -> Self {
        self.bg = Some(bg);
        self
    }

    fn draw_on_cell(
        &self,
        cell: &mut Cell,
    ) {
        cell.set_char(self.c).set_fg(self.fg);

        if let Some(bg) = self.bg {
            cell.set_bg(bg);
        }
    }
}
