use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub(crate) struct Element {
    pub dimensions: Dimensions,
    pub positions: Positions,
    pub childs: Vec<Rc<RefCell<Element>>>,
    pub id: u64,
    pub element_config: Rc<RefCell<ElementConfig>>,
    pub grow_on_percent_mark: bool,
}

#[derive(Default)]
pub struct LayoutContext {
    // User need an instance of this struct in order to do anything, but all member must be hidden
    pub(crate) element_stack: VecDeque<Element>,
    pub(crate) top_id: u64,
    pub(crate) root_dimensions: Dimensions,
    pub(crate) element_chain_bottomup: Vec<Rc<RefCell<Element>>>,
}

#[derive(Clone, Copy)]
pub struct ElementConfig {
    pub width: SizingConfig,
    pub height: SizingConfig,
    pub padding: PaddingConfig,
    pub gap: f32,
    pub child_alignment: AlignmentConfig,
    pub layout_direction: LayoutDirection,
}

#[derive(Clone, Copy)]
pub struct PaddingConfig {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Clone, Copy)]
pub struct SizingConfig {
    pub sizing_type: SizingType,
    pub min_val: f32,
    pub max_val: f32,
    pub percent: f32,
}

#[derive(Clone, Copy)]
pub enum SizingType {
    Fixed,
    Fit,
    Grow,
    Percent,
}

#[derive(Clone, Copy)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

#[derive(Clone, Copy)]
pub enum LayoutDirection {
    LeftToRight,
    TopToBottom,
}

#[derive(Clone, Copy)]
pub struct AlignmentConfig {
    pub align_y: VerticalAlignment,
    pub align_x: HorizontalAlignment,
}

#[derive(Clone, Copy)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy)]
pub struct Positions {
    pub x: f32,
    pub y: f32,
}

impl Default for ElementConfig {
    fn default() -> Self {
        ElementConfig {
            width: SizingConfig {
                ..Default::default()
            },
            height: SizingConfig {
                ..Default::default()
            },
            padding: PaddingConfig {
                ..Default::default()
            },
            gap: 0.,
            child_alignment: AlignmentConfig {
                ..Default::default()
            },
            layout_direction: LayoutDirection::LeftToRight,
        }
    }
}

impl Default for Dimensions {
    fn default() -> Self {
        Dimensions {
            width: 0.,
            height: 0.,
        }
    }
}

impl Default for Positions {
    fn default() -> Self {
        Positions { x: 0., y: 0. }
    }
}

impl Default for AlignmentConfig {
    fn default() -> Self {
        AlignmentConfig {
            align_y: VerticalAlignment::Top,
            align_x: HorizontalAlignment::Left,
        }
    }
}

impl Default for PaddingConfig {
    fn default() -> Self {
        PaddingConfig {
            left: 0.,
            right: 0.,
            top: 0.,
            bottom: 0.,
        }
    }
}

impl Default for SizingConfig {
    fn default() -> Self {
        SizingConfig {
            sizing_type: SizingType::Fit,
            min_val: 0.,
            max_val: 0.,
            percent: 0.,
        }
    }
}
