use printpdf::{Mm, PdfPageReference};
use rusttype::{Font, Scale};

use crate::{
    assets::{Assets, FontAssetItem},
    border::Border,
    dom::{validate_name, Children},
    error::Error,
    hex_color::HexColor,
    layout::{LayoutNode, LayoutTree},
    rect::Rect,
};

struct MultiLine<'a> {
    text: &'a str,
    rusttype_font: &'a Font<'a>,
    max_width: f64,
    font_size: f32,
}

fn to_mm_size(font_size: f64) -> f64 {
    let pt = printpdf::Pt(font_size);
    Mm::from(pt).0
}

impl<'a> MultiLine<'a> {
    pub fn split_text_to_multi_lines(&self) -> Vec<&str> {
        let mut from = 0;
        let mut sum_width = 0.0;
        let mut lines = Vec::new();
        let widths = self.compute_char_widths();

        for (index, (to, _)) in self.text.char_indices().enumerate() {
            if widths.len() - 1 <= index {
                break;
            }
            if (self.max_width) <= ((sum_width + widths[index + 1]).ceil()) {
                lines.push(&self.text[from..to]);
                sum_width = 0.0;
                from = to;
            }
            sum_width += widths[index];
        }

        if from != self.text.len() - 1 {
            lines.push(&self.text[from..]);
        }
        lines
    }

    pub fn compute_char_widths(&self) -> Vec<f64> {
        self.rusttype_font
            .glyphs_for(self.text.chars())
            .map(|g| {
                g.scaled(self.get_scale(self.rusttype_font))
                    .h_metrics()
                    .advance_width
            })
            .map(|w| to_mm_size((w * self.font_size) as f64))
            .collect()
    }

    fn get_scale(&self, font: &Font) -> Scale {
        let units_per_em = f32::from(font.units_per_em());
        let v_metrics = font.v_metrics_unscaled();
        let glyph_height = (v_metrics.ascent - v_metrics.descent) / units_per_em;
        Scale::uniform(glyph_height)
    }
}

pub struct Text<'a> {
    node: &'a LayoutNode<'a>,
    layout_tree: &'a LayoutTree<'a>,
    assets: &'a Assets<'a>,
}

impl<'a> Text<'a> {
    pub fn new(
        node: &'a LayoutNode,
        layout_tree: &'a LayoutTree,
        assets: &'a Assets<'a>,
    ) -> Result<Self, Error> {
        validate_name(node.element, "Text")?;
        Ok(Self {
            node,
            layout_tree,
            assets,
        })
    }

    pub fn build(
        &self,
        pdf_page: &PdfPageReference,
        parent_rect: Option<&Rect>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let rect = self
            .layout_tree
            .build_absolute_rect(self.node, parent_rect)?;
        self.draw(pdf_page, rect)?;
        Ok(())
    }

    fn load_font(&self) -> Result<&FontAssetItem, Error> {
        let key = self.node.style.font_family;
        self.assets
            .fonts
            .get(key)
            .ok_or_else(|| Error::FontAssetNotFound(key.to_string()))
    }

    fn draw(&self, pdf_page: &PdfPageReference, rect: Rect) -> Result<(), Error> {
        if let Children::Text(text) = &self.node.element.children {
            self.draw_border(pdf_page, &rect)?;
            self.draw_text(text, pdf_page, &rect)?;
            Ok(())
        } else {
            Err(Error::InvalidChildren(String::from("Text")))
        }
    }

    fn draw_border(&self, pdf_page: &PdfPageReference, rect: &Rect) -> Result<(), Error> {
        let style = self.node.style;
        let rect = self.layout_tree.build_bottom_left_base_rect(&rect)?;

        if let Some(border_color) = &style.border_color {
            Border {
                border: &style.border,
                border_color: &HexColor::new(border_color).into(),
                rect: &rect,
                pdf_page,
            }
            .draw();
        }
        Ok(())
    }

    fn draw_text(&self, text: &str, pdf_page: &PdfPageReference, rect: &Rect) -> Result<(), Error> {
        let font_data = self.load_font()?;
        let font_size = self.node.style.font_size;

        let rect = rect.set_height(to_mm_size(font_size));
        let rect = self.layout_tree.build_bottom_left_base_rect(&rect)?;

        let multi_line = MultiLine {
            text,
            rusttype_font: &font_data.rusttype,
            max_width: rect.width,
            font_size: font_size as f32,
        };
        let lines = multi_line.split_text_to_multi_lines();

        let pdf_layer = pdf_page.add_layer("");
        pdf_layer.begin_text_section();
        pdf_layer.set_font(&font_data.printpdf, font_size);
        pdf_layer.set_text_cursor(Mm(rect.x), Mm(rect.y));
        pdf_layer.set_line_height(self.node.style.line_height);

        for text in lines.iter() {
            pdf_layer.write_text(*text, &font_data.printpdf);
            pdf_layer.add_line_break();
        }
        pdf_layer.end_text_section();
        Ok(())
    }
}
