use std::{fs::File, io::BufWriter};

use markup_pdf_rs::{assets::Assets, document::Document, dom, style::Style};
use taffy::{
    prelude::{Rect, Size},
    style::{
        Dimension::{self, Points},
        FlexDirection, JustifyContent,
    },
};

fn main() {
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
        "title",
        Style {
            font_size: 20.0,
            size: Size {
                width: Dimension::Auto,
                height: Points(10.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "paragraph",
        Style {
            size: Size {
                width: Dimension::Auto,
                height: Points(15.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "blockquote",
        Style {
            background_color: Some("#eeeeee"),
            size: Size {
                width: Dimension::Auto,
                height: Points(23.0),
            },
            ..Style::DEFAULT
        },
    );
    assets.styles.add(
        "blockquote-text",
        Style {
            flex_grow: 1.0,
            margin: Rect::from_points(5.0, 5.0, 5.0, 5.0),
            size: Size {
                width: Dimension::Auto,
                height: Points(20.0),
            },
            ..Style::DEFAULT
        },
    );

    assets.fonts.add(
        "default",
        include_bytes!("assets/fonts/Roboto-Regular.ttf").as_slice(),
    );

    let root = dom::parse(r#"
        <Document title="recipe">
            <Page style="page">
                <Layer style="main">
                    <Text style="title">Lorem Ipsum</Text>
                    <Text style="paragraph">Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras sollicitudin mollis ipsum, eu aliquam est consequat vitae.</Text>
                    <Layer style="blockquote">
                        <Text style="blockquote-text">Donec nunc erat, porttitor eu massa blandit, pulvinar aliquam turpis. Maecenas augue justo, auctor ornare eleifend sed, suscipit ut mauris.</Text>
                    </Layer>
                    <Text style="paragraph">Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras sollicitudin mollis ipsum, eu aliquam est consequat vitae.</Text>
                    <Text style="paragraph">Mauris in mattis lectus. Nulla eget iaculis ligula, eu pulvinar leo.</Text>
                    <Text style="paragraph">vitae laoreet erat mauris sed ipsum.</Text>
                </Layer>
            </Page>
        </Document>
    "#).unwrap();

    let doc = Document::new(&root, assets).unwrap().build().unwrap();

    let buf = &mut BufWriter::new(File::create("dist/simple.pdf").unwrap());
    doc.save(buf).unwrap();
}
