use ratatui_core::style::Color;
use ratatui_widgets::canvas::{Painter, Shape};

/// A [Shape] which allows to draw points using cell indices
/// rather than floats. This does not cause render errors due to rounding,
/// which is desired when rendering tile maps.
pub struct Cells<'a> {
    cells: &'a [(isize, isize)],
    color: Color,
}

impl<'a> Cells<'a> {
    /// Create new [Cells] for the given cell indices.
    pub const fn new(
        cells: &'a [(isize, isize)],
        color: Color,
    ) -> Self {
        Self { cells, color }
    }
}

impl<'a> Shape for Cells<'a> {
    fn draw(
        &self,
        painter: &mut Painter,
    ) {
        let bounds = painter.bounds();
        let left = bounds.0[0];
        let top = bounds.1[1];

        for (x, y) in self.cells {
            let x = x - left as isize;
            let y = top as isize - y;

            if x >= 0 && y >= 0 {
                painter.paint(x as usize, y as usize, self.color);
            }
        }
    }
}
