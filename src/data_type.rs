use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

// TYPE DEFINITION

pub(crate) type ElementReference = Rc<RefCell<Element>>;
pub type TextMeasureFunction = dyn Fn(&str, u32, u16) -> TextMeasurement;

// STRUCT DEFINITION

#[derive(Clone)]
pub(crate) struct Element {
    pub id: u64,
    pub dimensions: Dimensions,
    pub position: Position,
    pub child_elements: Vec<ElementReference>,
    pub element_config: TypeConfig, // element will own this, so no lifetime problem

    // metadata
    pub grow_on_percent_mark: bool,
}

pub struct LayoutContext {
    pub(crate) element_stack: VecDeque<Element>,
    pub(crate) top_id: u64, // temporary, will be replaced with proper ID generation
    pub(crate) root_dimensions: Dimensions,
    pub(crate) element_tree_post_order: Vec<ElementReference>,
    pub(crate) measure_text_fn: Box<TextMeasureFunction>,
    pub(crate) measure_text_cache: HashMap<String, TextMeasurement>,
}

#[derive(Clone, Copy)]
pub struct ElementConfig {
    pub width: DimensionConfig,
    pub height: DimensionConfig,
    pub padding: PaddingConfig,
    pub child_gap: f32,
    pub child_alignment: AlignmentConfig,
    pub child_layout_direction: LayoutDirection,
    pub color: Color,
}

#[derive(Clone)]
pub struct RenderCommand {
    pub position: Position,
    pub render_data: RenderData,
}

#[derive(Clone)]
pub enum RenderData {
    Text(TextRenderData),
    Rectangle(RectangleRenderData),
    Image(ImageRenderData),
    Border(BorderRenderData),

    // TODO: figure out whether it is possible to inject arbitrary data
    // Otherwise, allow user to pass an ID and detect that ID for each
    // custom element.
    Custom,
}

#[derive(Clone)]
pub struct TextRenderData {
    pub font_id: u32,
    pub text: Rc<str>,
    pub font_size: u16,
    pub font_color: Color,
}

#[derive(Clone, Copy)]
pub struct RectangleRenderData {
    pub dimenions: Dimensions,
    pub color: Color,
}

#[derive(Clone, Copy)]
pub struct ImageRenderData {
    pub image_id: u32,
    pub dimensions: Dimensions,
}

#[derive(Clone, Copy)]
pub struct BorderRenderData {}

#[derive(Clone)]
pub(crate) enum TypeConfig {
    Rectangle(Rc<ElementConfig>),
    Text(InternalTextConfig),
}

#[derive(Clone)]
pub(crate) struct InternalTextConfig {
    pub width: DimensionConfig,
    pub height: DimensionConfig,
    pub font_id: u32,
    pub break_word: bool,
    pub text: Rc<str>,
    pub font_size: u16,
    pub font_color: Color,
    pub text_lines: Vec<Rc<Element>>,
}

#[derive(Clone, Copy)]
pub struct TextConfig {
    pub width: DimensionConfig,
    pub height: DimensionConfig,
    pub font_id: u32,
    pub break_word: bool,
    pub font_size: u16,
    pub font_color: Color,
}

#[derive(Clone, Copy)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy)]
pub enum DimensionConfig {
    Fixed(FixedDimensionConfig),
    Fit(FitSizingConfig),
    Grow(GrowDimensionConfig),
    Percent(PercentDimenionConfig),
}

#[derive(Clone, Copy)]
pub struct FitSizingConfig {
    pub min_size: f32,
}

#[derive(Clone, Copy)]
pub struct FixedDimensionConfig {
    pub size: f32,
}

#[derive(Clone, Copy)]
pub struct GrowDimensionConfig {
    pub min_size: f32,
    pub max_size: f32,
}

#[derive(Clone, Copy)]
pub struct PercentDimenionConfig {
    pub percent: f32,
}

#[derive(Clone, Copy)]
pub struct PaddingConfig {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Clone, Copy)]
pub struct AlignmentConfig {
    pub align_x: HorizontalAlignment,
    pub align_y: VerticalAlignment,
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
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Copy)]
pub struct TextMeasurement {
    pub width: f32,
    pub height: f32,

    // offsets, automatically added to position during layout phase.
    pub x_offset: f32,
    pub y_offset: f32,
}

// DEFAULT VALUES

impl Default for ElementConfig {
    fn default() -> Self {
        ElementConfig {
            width: DimensionConfig::fit(),
            height: DimensionConfig::fit(),
            padding: PaddingConfig::same_padding(0.),
            child_gap: 0.,
            child_alignment: AlignmentConfig::default(),
            child_layout_direction: LayoutDirection::LeftToRight,
            color: Color::default(),
        }
    }
}

impl Default for AlignmentConfig {
    fn default() -> Self {
        AlignmentConfig {
            align_x: HorizontalAlignment::Left,
            align_y: VerticalAlignment::Top,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 249,
            g: 4,
            b: 225,
            a: 255,
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

impl Default for Position {
    fn default() -> Self {
        Position { x: 0., y: 0. }
    }
}
