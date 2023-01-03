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
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            padding: Rect::from_points(15.0, 15.0, 15.0, 15.0),
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "title",
        Style {
            font_family: "bold",
            font_size: 30.0,
            size: Size {
                width: Dimension::Auto,
                height: Points(20.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "description",
        Style {
            font_size: 14.0,
            line_height: 20.0,
            size: Size {
                width: Dimension::Auto,
                height: Points(30.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "summary",
        Style {
            flex_direction: FlexDirection::Row,
            gap: Size {
                width: Points(3.0),
                height: Points(0.0),
            },
            size: Size {
                width: Dimension::Auto,
                height: Points(60.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "summary-title",
        Style {
            font_family: "bold",
            flex_grow: 1.0,
            size: Size {
                width: Dimension::Auto,
                height: Points(13.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "photo",
        Style {
            size: Size {
                width: Dimension::Points(100.0),
                height: Dimension::Auto,
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "ingredients",
        Style {
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceAround,
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "ingredient",
        Style {
            border_color: Some("#a3928b"),
            margin: Rect::from_points(0.0, 0.0, 0.0, 1.0),
            border: Rect {
                left: Dimension::Undefined,
                top: Dimension::Undefined,
                bottom: Dimension::Points(0.3),
                right: Dimension::Undefined,
            },
            size: Size {
                width: Dimension::Auto,
                height: Points(12.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "steps",
        Style {
            margin: Rect::from_points(0.0, 0.0, 20.0, 0.0),
            flex_direction: FlexDirection::Row,
            border_color: Some("#a3928b"),
            gap: Size {
                width: Points(5.0),
                height: Points(5.0),
            },
            size: Size {
                width: Dimension::Auto,
                height: Points(80.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "step",
        Style {
            flex_grow: 1.0,
            font_size: 14.0,
            line_height: 20.0,
            border_color: Some("#a3928b"),
            border: Rect {
                left: Dimension::Undefined,
                bottom: Dimension::Undefined,
                top: Dimension::Undefined,
                right: Dimension::Points(0.3),
            },
            padding: Rect::from_points(0.0, 5.0, 0.0, 0.0),
            size: Size {
                width: Dimension::Auto,
                height: Points(70.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "last-step",
        Style {
            flex_grow: 1.0,
            border_color: Some("#a3928b"),
            font_size: 14.0,
            line_height: 20.0,
            size: Size {
                width: Dimension::Auto,
                height: Points(70.0),
            },
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
    assets.images.add(
        "food",
        image::open("examples/assets/images/food.jpg").unwrap(),
    );
    assets
}

fn main() {
    let root = dom::parse(&fs::read_to_string("examples/assets/xml/recipe.xml").unwrap()).unwrap();
    let buf = &mut BufWriter::new(File::create("dist/recipe.pdf").unwrap());
    let doc = Document::new(&root, build_assets())
        .unwrap()
        .build()
        .unwrap();
    doc.save(buf).unwrap();
}
