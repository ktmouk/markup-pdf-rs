use std::{
    fs::{self, File},
    io::BufWriter,
};

use markup_pdf_rs::{assets::Assets, document::Document, dom, style::Style};
use taffy::{
    prelude::{Rect, Size},
    style::{
        Dimension::{self, Points},
        FlexDirection, JustifyContent,
    },
};

fn build_assets<'a>() -> Assets<'a> {
    let mut assets = Assets::default();
    assets.styles.add(
        "page",
        Style {
            size: Size {
                width: Points(210.0),
                height: Points(297.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "main",
        Style {
            gap: Size {
                width: Dimension::Auto,
                height: Points(6.0),
            },
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            padding: Rect::from_points(15.0, 15.0, 15.0, 15.0),
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "header",
        Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            size: Size {
                width: Dimension::Auto,
                height: Points(14.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "title",
        Style {
            flex_grow: 1.0,
            font_family: "bold",
            font_size: 20.0,
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "date",
        Style {
            size: Size {
                width: Points(40.0),
                height: Points(20.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "birthdate-input",
        Style {
            flex_grow: 1.0,
            size: Size {
                width: Dimension::Auto,
                height: Points(17.0),
            },
            border: Rect {
                left: Dimension::Undefined,
                top: Dimension::Points(0.6),
                bottom: Dimension::Points(0.6),
                right: Dimension::Undefined,
            },
            padding: Rect::from_points(15.0, 0.0, 4.0, 0.0),
            border_color: Some("#000000"),
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "profile",
        Style {
            flex_direction: FlexDirection::Row,
            gap: Size {
                width: Points(5.0),
                height: Points(0.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "photo",
        Style {
            flex_direction: FlexDirection::Column,
            border: Rect::from_points(0.3, 0.3, 0.3, 0.3),
            border_color: Some("#000000"),
            padding: Rect::from_points(4.0, 4.0, 4.0, 4.0),
            size: Size {
                width: Points(30.0),
                height: Points(40.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "photo-text",
        Style {
            flex_grow: 1.0,
            font_size: 10.0,
            size: Size {
                width: Dimension::Auto,
                height: Points(8.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "profile-form",
        Style {
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            border: Rect::from_points(0.8, 0.8, 0.8, 0.8),
            border_color: Some("#000000"),
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "label",
        Style {
            flex_grow: 1.0,
            size: Size {
                width: Dimension::Auto,
                height: Points(8.0),
            },
            padding: Rect::from_points(1.0, 0.0, 0.0, 0.0),
            border: Rect {
                left: Dimension::Undefined,
                top: Dimension::Undefined,
                bottom: Dimension::Points(0.3),
                right: Dimension::Undefined,
            },
            border_color: Some("#000000"),
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "label-text",
        Style {
            flex_grow: 1.0,
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "input",
        Style {
            flex_grow: 1.0,
            size: Size {
                width: Dimension::Auto,
                height: Points(20.0),
            },
            padding: Rect::from_points(1.0, 0.0, 0.0, 0.0),
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "input-text",
        Style {
            flex_grow: 1.0,
            size: Size {
                width: Dimension::Auto,
                height: Points(16.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "table",
        Style {
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            border: Rect::from_points(0.8, 0.8, 0.8, 0.8),
            border_color: Some("#000000"),
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "table-header",
        Style {
            size: Size {
                width: Dimension::Auto,
                height: Points(7.0),
            },
            padding: Rect::from_points(1.0, 0.0, 0.0, 0.0),
            border_color: Some("#000000"),
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "table-body",
        Style {
            flex_grow: 1.0,
            padding: Rect::from_points(1.0, 0.0, 0.0, 0.0),
            border: Rect {
                left: Dimension::Undefined,
                bottom: Dimension::Undefined,
                top: Dimension::Points(0.3),
                right: Dimension::Undefined,
            },
            border_color: Some("#000000"),
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "table-date",
        Style {
            size: Size {
                width: Points(20.0),
                height: Dimension::Auto,
            },
            margin: Rect::from_points(1.0, 2.0, 0.0, 0.0),
            border: Rect {
                left: Dimension::Undefined,
                top: Dimension::Undefined,
                bottom: Dimension::Undefined,
                right: Dimension::Points(0.2),
            },
            border_color: Some("#000000"),
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "table-detail",
        Style {
            flex_grow: 1.0,
            ..Style::DEFAULT
        },
    );
    assets.fonts.add(
        "default",
        include_bytes!("assets/fonts/ShipporiMincho-Regular.ttf").as_slice(),
    );
    assets.fonts.add(
        "bold",
        include_bytes!("assets/fonts/ShipporiMincho-Bold.ttf").as_slice(),
    );
    assets
}

fn main() {
    let root = dom::parse(&fs::read_to_string("examples/assets/xml/resume.xml").unwrap()).unwrap();
    let buf = &mut BufWriter::new(File::create("dist/resume.pdf").unwrap());
    let doc = Document::new(&root, build_assets())
        .unwrap()
        .build()
        .unwrap();
    doc.save(buf).unwrap();
}
