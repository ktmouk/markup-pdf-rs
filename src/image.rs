use printpdf::{ImageTransform, ImageXObject, Mm, PdfPageReference, Px};

use crate::{
    assets::Assets,
    dom::{get_required_attr, validate_name},
    error::Error,
    layout::{LayoutNode, LayoutTree},
    rect::Rect,
};

pub struct Image<'a> {
    node: &'a LayoutNode<'a>,
    layout_tree: &'a LayoutTree<'a>,
    assets: &'a Assets<'a>,
}

impl<'a> Image<'a> {
    const BASE_DPI: f64 = 300.0;

    pub fn new(
        node: &'a LayoutNode,
        layout_tree: &'a LayoutTree,
        assets: &'a Assets<'a>,
    ) -> Result<Self, Error> {
        validate_name(node.element, "Image")?;
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

    fn get_image(&self) -> Result<printpdf::Image, Error> {
        let key: String = get_required_attr(self.node.element, "src")?;
        self.assets
            .images
            .get(&key)
            .ok_or_else(|| Error::ImageAssetNotFound(key.to_string()))
    }

    fn to_mm(px: usize) -> f64 {
        Mm::from(Px(px).into_pt(Self::BASE_DPI)).0
    }

    fn get_scale(rect: &Rect, image: &ImageXObject) -> f64 {
        let scale_x = rect.width / Self::to_mm(image.width.0);
        let scale_y = rect.height / Self::to_mm(image.height.0);
        scale_x.min(scale_y)
    }

    fn draw(&self, pdf_page: &PdfPageReference, rect: Rect) -> Result<(), Error> {
        let image = self.get_image()?;
        let scale = Self::get_scale(&rect, &image.image);
        let rect = rect.set_height(Self::to_mm(image.image.height.0) * scale);
        let rect = self.layout_tree.build_bottom_left_base_rect(&rect)?;

        let transform = ImageTransform {
            translate_x: Some(Mm(rect.x)),
            translate_y: Some(Mm(rect.y)),
            rotate: None,
            scale_x: Some(scale),
            scale_y: Some(scale),
            dpi: Some(Self::BASE_DPI),
        };

        let pdf_layer = pdf_page.add_layer("");
        image.add_to_layer(pdf_layer, transform);
        Ok(())
    }
}
