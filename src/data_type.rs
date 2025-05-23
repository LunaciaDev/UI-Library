use std::{cell::RefCell, collections::{HashMap, VecDeque}, rc::Rc};

pub(crate) struct Element {
    pub dimensions: Dimensions,
    pub positions: Positions,
    pub childs: Vec<Rc<RefCell<Element>>>,
    pub id: u64,
    pub element_config: Rc<ElementConfig>,
    pub grow_on_percent_mark: bool,

    // text data
    pub text_config: Option<Rc<TextConfig>>,
    pub text: Option<String>,
    // store broken-down texts childs.
    // should be positioned relative to the parent.
    pub text_childs: Vec<Rc<Element>>, 
}

#[derive(Clone, Copy, Default)]
pub struct TextMeasurement {
    pub width: f32,
    pub height: f32,
    pub x_offset: f32,
    pub y_offset: f32,
}

pub type TextMeasureFunction = dyn Fn(&str, u32, u16) -> TextMeasurement;

pub struct LayoutContext {
    // User need an instance of this struct in order to do anything, but all member must be hidden
    pub(crate) element_stack: VecDeque<Element>,
    pub(crate) top_id: u64,
    pub(crate) root_dimensions: Dimensions,
    pub(crate) element_chain_bottomup: Vec<Rc<RefCell<Element>>>,
    pub(crate) measure_text: Box<TextMeasureFunction>,
    pub(crate) measurement_cache: HashMap<String, TextMeasurement>,
}

#[derive(Clone, Copy)]
pub struct ElementConfig {
    pub width: SizingConfig,
    pub height: SizingConfig,
    pub padding: PaddingConfig,
    pub gap: f32,
    pub child_alignment: AlignmentConfig,
    pub layout_direction: LayoutDirection,
    pub color: Color,
}

#[derive(Clone, Copy)]
pub struct TextConfig {
    pub font_id: u32,
    pub font_size: u16,
    pub color: Color,
    pub break_word: bool,
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
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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

pub struct RenderCommand {
    pub dimension: Dimensions,
    pub position: Positions,
    pub color: Color,
    pub text: Option<String>,
    pub text_config: Option<TextConfig>,
}

impl Default for ElementConfig {
    fn default() -> Self {
        ElementConfig {
            width: SizingConfig::default(),
            height: SizingConfig::default(),
            padding: PaddingConfig::default(),
            gap: 0.,
            child_alignment: AlignmentConfig::default(),
            layout_direction: LayoutDirection::LeftToRight,
            color: Color::default(),
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

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}
