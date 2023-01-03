use std::vec;

use taffy::{
    error::TaffyError,
    prelude::{Node, Size},
    style::Dimension,
    Taffy,
};

use crate::{assets::StyleAsset, dom, error::Error, rect::Rect, style::Style};

pub struct LayoutNode<'a> {
    pub element: &'a dom::Element,
    pub taffy_node: Node,
    pub parent_taffy_node: Option<Node>,
    pub style: &'a Style<'a>,
    pub children: Vec<LayoutNode<'a>>,
}

impl LayoutNode<'_> {
    pub fn get_fixed_size(&self) -> Option<Size<f32>> {
        if let (Dimension::Points(width), Dimension::Points(height)) =
            (self.style.size.width, self.style.size.height)
        {
            Some(Size { width, height })
        } else {
            None
        }
    }
}

pub struct LayoutTree<'a> {
    pub root: LayoutNode<'a>,
    taffy: Taffy,
}

impl<'a> LayoutTree<'a> {
    pub fn new(taffy: Taffy, root: LayoutNode<'a>) -> LayoutTree<'a> {
        Self { taffy, root }
    }

    fn build_rect(&self, taffy_node: Node) -> Result<Rect, TaffyError> {
        let taffy_layout = self.taffy.layout(taffy_node)?;
        let x: f64 = taffy_layout.location.x.into();
        let y: f64 = taffy_layout.location.y.into();
        let width: f64 = taffy_layout.size.width.into();
        let height: f64 = taffy_layout.size.height.into();
        Ok(Rect::new(x, y, width, height))
    }

    pub fn build_absolute_rect(
        &self,
        node: &LayoutNode,
        parent_rect: Option<&Rect>,
    ) -> Result<Rect, TaffyError> {
        let rect = self.build_rect(node.taffy_node)?;

        if let Some(parent_rect) = parent_rect {
            Ok(rect.add_position(&parent_rect))
        } else {
            Ok(rect)
        }
    }

    // We want to use a relative position from top-left same as CSS.
    // However, Printpdf use a relative position from bottom-left
    // so we need to recalculate it.
    pub fn build_bottom_left_base_rect(&self, rect: &Rect) -> Result<Rect, Error> {
        let root_size = self.get_root_size()?;
        Ok(Rect::new(
            rect.x,
            f64::from(root_size.height) - rect.y - rect.height,
            rect.width,
            rect.height,
        ))
    }

    pub fn get_root_size(&self) -> Result<Size<f32>, Error> {
        self.root.get_fixed_size().ok_or(Error::UndefinedPageSize())
    }
}

#[derive(Debug)]
pub struct LayoutTreeBuilder<'a> {
    pub styles: &'a StyleAsset<'a>,
}

impl<'a> LayoutTreeBuilder<'a> {
    pub fn new(styles: &'a StyleAsset<'a>) -> LayoutTreeBuilder {
        Self { styles }
    }

    fn walk(
        &'a self,
        current: &'a dom::Element,
        taffy: &mut Taffy,
        parent_taffy_node: Option<Node>,
    ) -> Result<LayoutNode, TaffyError> {
        let style = self.find_style(current);
        let taffy_node = taffy.new_leaf(style.into())?;

        if let dom::Children::Elements(children) = &current.children {
            let children: Vec<LayoutNode> = children
                .iter()
                .map(|child| self.walk(child, taffy, Some(taffy_node)))
                .collect::<Result<Vec<_>, _>>()?;
            let child_taffy_nodes: Vec<Node> =
                children.iter().map(|child| child.taffy_node).collect();
            taffy.set_children(taffy_node, &child_taffy_nodes)?;
            Ok(LayoutNode {
                element: current,
                taffy_node,
                parent_taffy_node,
                children,
                style,
            })
        } else {
            Ok(LayoutNode {
                element: current,
                taffy_node,
                parent_taffy_node,
                children: vec![],
                style,
            })
        }
    }

    fn find_style(&self, element: &dom::Element) -> &Style {
        if let Some(key) = element.attributes.get("style") {
            self.styles.get(key).unwrap_or(&Style::DEFAULT)
        } else {
            &Style::DEFAULT
        }
    }

    pub fn build(&'a self, root: &'a dom::Element) -> Result<LayoutTree, TaffyError> {
        let mut taffy = Taffy::new();
        let node = self.walk(root, &mut taffy, None)?;
        taffy.compute_layout(node.taffy_node, Size::MAX_CONTENT)?;
        Ok(LayoutTree::new(taffy, node))
    }
}
