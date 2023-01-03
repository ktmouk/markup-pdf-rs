use printpdf::PdfPageReference;

use crate::{
    assets::Assets,
    border::Border,
    dom::validate_name,
    error::Error,
    hex_color::HexColor,
    image::Image,
    layout::{LayoutNode, LayoutTree},
    rect::Rect,
    text::Text,
};

pub struct Layer<'a> {
    node: &'a LayoutNode<'a>,
    layout_tree: &'a LayoutTree<'a>,
    assets: &'a Assets<'a>,
}

impl<'a> Layer<'a> {
    pub fn new(
        node: &'a LayoutNode,
        layout_tree: &'a LayoutTree,
        assets: &'a Assets<'a>,
    ) -> Result<Self, Error> {
        validate_name(node.element, "Layer")?;
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
        self.draw(pdf_page, &rect)?;

        for node in &self.node.children {
            self.add_child(node, pdf_page, &rect)?;
        }
        Ok(())
    }

    fn add_child(
        &self,
        node: &LayoutNode,
        pdf_page: &PdfPageReference,
        rect: &Rect,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match node.element.name.as_str() {
            "Layer" => {
                let layer = Layer::new(node, self.layout_tree, self.assets)?;
                layer.build(pdf_page, Some(rect))?;
                Ok(())
            }
            "Image" => {
                let image = Image::new(node, self.layout_tree, self.assets)?;
                image.build(pdf_page, Some(rect))?;
                Ok(())
            }
            "Text" => {
                let text = Text::new(node, self.layout_tree, self.assets)?;
                text.build(pdf_page, Some(rect))?;
                Ok(())
            }
            _ => Err(Box::new(Error::UnknownChild(node.element.name.to_string()))),
        }
    }

    fn draw(
        &self,
        pdf_page: &PdfPageReference,
        rect: &Rect,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pdf_layer = pdf_page.add_layer("");
        let style = self.node.style;
        let rect = self.layout_tree.build_bottom_left_base_rect(rect)?;

        if let Some(background_color) = &style.background_color {
            pdf_layer.set_outline_color(HexColor::new(background_color).into());
            pdf_layer.set_fill_color(HexColor::new(background_color).into());
            pdf_layer.add_shape(rect.to_line());
        }

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
}
