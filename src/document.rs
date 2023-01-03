use printpdf::{PdfDocument, PdfDocumentReference};

use crate::{
    assets::Assets,
    dom::{get_attr, validate_name, Children, Element},
    error::Error,
    page::Page,
};

#[derive(Debug)]
pub struct Document<'a> {
    root: &'a Element,
    assets: Assets<'a>,
}

impl<'a> Document<'a> {
    pub fn new(root: &'a Element, assets: Assets<'a>) -> Result<Self, Error> {
        validate_name(root, "Document")?;
        Ok(Self { root, assets })
    }

    pub fn build(&mut self) -> Result<PdfDocumentReference, Box<dyn std::error::Error>> {
        let pdf_doc = self.build_pdf_doc();
        self.assets.prepare(&pdf_doc)?;

        if let Children::Elements(children) = &self.root.children {
            for element in children {
                let page = Page::new(element, &self.assets)?;
                page.build(&pdf_doc)?;
            }
        }
        Ok(pdf_doc)
    }

    fn build_pdf_doc(&self) -> PdfDocumentReference {
        let title = get_attr(self.root, "title").unwrap_or_else(|| String::from("Untitled"));
        PdfDocument::empty(title)
    }
}
