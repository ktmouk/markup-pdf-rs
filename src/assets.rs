use printpdf::{image_crate::DynamicImage, Image, IndirectFontRef, PdfDocumentReference};
use rusttype::Font;
use std::{collections::HashMap, error::Error, io::Cursor};

use crate::style::Style;

#[derive(Debug, Default)]
pub struct Assets<'a> {
    pub fonts: FontAsset<'a>,
    pub images: ImageAsset<'a>,
    pub styles: StyleAsset<'a>,
}

impl<'a> Assets<'a> {
    pub fn prepare(&mut self, pdf_doc: &PdfDocumentReference) -> Result<(), Box<dyn Error>> {
        self.fonts.prepare(pdf_doc)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct FontAssetItem<'a> {
    pub printpdf: IndirectFontRef,
    pub rusttype: Font<'a>,
}

impl<'a> FontAssetItem<'a> {
    pub fn new(
        pdf_doc: &PdfDocumentReference,
        font_bytes: &'a [u8],
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            printpdf: pdf_doc.add_external_font(Cursor::new(font_bytes))?,
            rusttype: Font::try_from_bytes(font_bytes)
                .ok_or("Failed to load the font as rusttype")?,
        })
    }
}

#[derive(Debug, Default)]
pub struct FontAsset<'a> {
    fonts: HashMap<&'a str, &'a [u8]>,
    cache: HashMap<&'a str, FontAssetItem<'a>>,
}

impl<'a> FontAsset<'a> {
    pub fn add(&mut self, key: &'a str, font_bytes: &'a [u8]) {
        self.fonts.insert(key, font_bytes);
    }

    pub fn get(&self, key: &'a str) -> Option<&FontAssetItem> {
        self.cache.get(key)
    }

    pub fn prepare(&mut self, pdf_doc: &PdfDocumentReference) -> Result<(), Box<dyn Error>> {
        for (key, font_bytes) in &self.fonts {
            self.cache
                .insert(key, FontAssetItem::new(pdf_doc, font_bytes)?);
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ImageAsset<'a> {
    images: HashMap<&'a str, DynamicImage>,
}

impl<'a> ImageAsset<'a> {
    pub fn get(&self, key: &str) -> Option<Image> {
        self.images.get(key).map(Image::from_dynamic_image)
    }

    pub fn add(&mut self, key: &'a str, style: DynamicImage) {
        self.images.insert(key, style);
    }
}

#[derive(Debug, Default)]
pub struct StyleAsset<'a> {
    styles: HashMap<&'a str, Style<'a>>,
}

impl<'a> StyleAsset<'a> {
    pub fn get(&self, key: &str) -> Option<&Style> {
        self.styles.get(key)
    }

    pub fn add(&mut self, key: &'a str, style: Style<'a>) {
        self.styles.insert(key, style);
    }
}
