use printpdf::{Line, Mm, Point};

#[derive(Debug)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    // We want to use a relative position from top-left same as CSS.
    // However, Printpdf use a relative position from bottom-left
    // so we need to recalculate it.
    pub fn to_bottom_left_base(&self, page_height: f64) -> Self {
        Self::new(
            self.x,
            page_height - self.y - self.height,
            self.width,
            self.height,
        )
    }

    pub fn add_position(&self, rect: &Rect) -> Self {
        Self::new(self.x + rect.x, self.y + rect.y, self.width, self.height)
    }

    pub fn to_points(&self) -> Vec<(Point, bool)> {
        vec![
            (Point::new(Mm(self.x), Mm(self.y)), false),
            (Point::new(Mm(self.right()), Mm(self.y)), false),
            (Point::new(Mm(self.right()), Mm(self.bottom())), false),
            (Point::new(Mm(self.x), Mm(self.bottom())), false),
        ]
    }

    pub fn to_line(&self) -> Line {
        Line {
            points: self.to_points(),
            is_closed: true,
            has_fill: true,
            has_stroke: true,
            is_clipping_path: false,
        }
    }

    pub fn set_height(&self, height: f64) -> Self {
        Self::new(self.x, self.y, self.width, height)
    }

    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }
}
