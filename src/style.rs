use taffy::{
    prelude::{Rect, Size},
    style::{
        AlignContent, AlignItems, AlignSelf, Dimension, Display, FlexDirection, FlexWrap,
        JustifyContent, PositionType,
    },
};

#[derive(Debug)]
pub struct Style<'a> {
    pub display: Display,
    pub position_type: PositionType,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub align_content: AlignContent,
    pub justify_content: JustifyContent,
    pub position: Rect<Dimension>,
    pub margin: Rect<Dimension>,
    pub padding: Rect<Dimension>,
    pub border: Rect<Dimension>,
    pub gap: Size<Dimension>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,
    pub size: Size<Dimension>,
    pub min_size: Size<Dimension>,
    pub max_size: Size<Dimension>,
    pub aspect_ratio: Option<f32>,
    pub background_color: Option<&'a str>,
    pub border_color: Option<&'a str>,
    pub font_size: f64,
    pub font_family: &'a str,
    pub line_height: f64,
}

impl<'a> From<&Style<'a>> for taffy::style::Style {
    fn from(item: &Style) -> Self {
        taffy::style::Style {
            display: item.display,
            position_type: item.position_type,
            flex_direction: item.flex_direction,
            flex_wrap: item.flex_wrap,
            align_items: item.align_items,
            align_self: item.align_self,
            align_content: item.align_content,
            justify_content: item.justify_content,
            position: item.position,
            margin: item.margin,
            padding: item.padding,
            border: item.border,
            gap: item.gap,
            flex_grow: item.flex_grow,
            flex_shrink: item.flex_shrink,
            flex_basis: item.flex_basis,
            size: item.size,
            min_size: item.min_size,
            max_size: item.max_size,
            aspect_ratio: item.aspect_ratio,
        }
    }
}

impl<'a> Style<'a> {
    const DEFAULT_FONT_FAMILY: &str = "default";

    pub const DEFAULT: Style<'a> = Style {
        display: Display::Flex,
        position_type: PositionType::Relative,
        flex_direction: FlexDirection::Row,
        flex_wrap: FlexWrap::NoWrap,
        align_items: AlignItems::Stretch,
        align_self: AlignSelf::Auto,
        align_content: AlignContent::Stretch,
        justify_content: JustifyContent::FlexStart,
        position: Rect::UNDEFINED,
        margin: Rect::UNDEFINED,
        padding: Rect::UNDEFINED,
        border: Rect::UNDEFINED,
        gap: Size::UNDEFINED,
        flex_grow: 0.0,
        flex_shrink: 1.0,
        flex_basis: Dimension::Auto,
        size: Size::AUTO,
        min_size: Size::AUTO,
        max_size: Size::AUTO,
        aspect_ratio: None,
        background_color: None,
        border_color: None,
        font_size: 14.0,
        line_height: 16.0,
        font_family: Self::DEFAULT_FONT_FAMILY,
    };
}
