use printpdf::{Color, PdfPageReference};
use taffy::style::Dimension;

use crate::rect::Rect;

pub struct Border<'a> {
    pub border: &'a taffy::geometry::Rect<Dimension>,
    pub border_color: &'a Color,
    pub rect: &'a Rect,
    pub pdf_page: &'a PdfPageReference,
}

impl<'a> Border<'a> {
    pub fn draw(&self) {
        if let Dimension::Points(left) = self.border.left {
            if left > 0.0 {
                self.draw_border(Rect::new(
                    self.rect.x,
                    self.rect.y,
                    left.into(),
                    self.rect.height,
                ));
            }
        }
        if let Dimension::Points(right) = self.border.right {
            let right: f64 = right.into();
            if right > 0.0 {
                self.draw_border(Rect::new(
                    self.rect.right() - right,
                    self.rect.y,
                    right,
                    self.rect.height,
                ));
            }
        }
        if let Dimension::Points(bottom) = self.border.bottom {
            if bottom > 0.0 {
                self.draw_border(Rect::new(
                    self.rect.x,
                    self.rect.y,
                    self.rect.width,
                    bottom.into(),
                ));
            }
        }
        if let Dimension::Points(top) = self.border.top {
            let top: f64 = top.into();
            if top > 0.0 {
                self.draw_border(Rect::new(
                    self.rect.x,
                    self.rect.bottom() - top,
                    self.rect.width,
                    top,
                ));
            }
        }
    }

    pub fn draw_border(&self, rect: Rect) {
        let pdf_layer = self.pdf_page.add_layer("");
        pdf_layer.set_fill_color(self.border_color.clone());
        pdf_layer.set_outline_color(self.border_color.clone());
        pdf_layer.set_outline_thickness(0.0);
        pdf_layer.add_shape(rect.to_line());
    }
}
