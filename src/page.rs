use printpdf::{Mm, PdfDocumentReference};

use crate::{
    assets::Assets,
    dom::{validate_name, Element},
    error::Error,
    layer::Layer,
    layout::LayoutTreeBuilder,
};

pub struct Page<'a> {
    element: &'a Element,
    assets: &'a Assets<'a>,
}

impl<'a> Page<'a> {
    pub fn new(element: &'a Element, assets: &'a Assets) -> Result<Self, Error> {
        validate_name(element, "Page")?;
        Ok(Self { element, assets })
    }

    pub fn build(&self, doc: &PdfDocumentReference) -> Result<(), Box<dyn std::error::Error>> {
        let layout_tree_builder = LayoutTreeBuilder::new(&self.assets.styles);
        let layout_tree = layout_tree_builder.build(self.element)?;

        let size = &layout_tree.get_root_size()?;
        let (page_index, _) = doc.add_page(Mm(size.width.into()), Mm(size.height.into()), "");
        let pdf_page = &doc.get_page(page_index);

        for node in &layout_tree.root.children {
            let layer = Layer::new(node, &layout_tree, self.assets)?;
            layer.build(pdf_page, None)?;
        }
        Ok(())
    }
}
