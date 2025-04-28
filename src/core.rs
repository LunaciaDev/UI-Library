use std::{cell::RefCell, rc::Rc};

use crate::data_type::*;

impl ElementConfig {
    pub fn new(config: ElementConfig) -> Rc<RefCell<ElementConfig>> {
        Rc::new(RefCell::new(config))
    }

    pub fn new_from(config: Rc<RefCell<ElementConfig>>) -> Rc<RefCell<ElementConfig>> {
        Rc::new(RefCell::new(config.borrow().clone()))
    }
}

impl SizingConfig {
    pub fn grow(min: f32, max: f32) -> SizingConfig {
        SizingConfig {
            sizing_type: SizingType::Grow,
            min_val: min,
            max_val: max,
            ..Default::default()
        }
    }

    pub fn fixed(val: f32) -> SizingConfig {
        SizingConfig {
            sizing_type: SizingType::Fixed,
            min_val: val,
            max_val: val,
            ..Default::default()
        }
    }

    pub fn percent(percent: f32) -> SizingConfig {
        SizingConfig {
            sizing_type: SizingType::Percent,
            percent,
            ..Default::default()
        }
    }

    pub fn fit() -> SizingConfig {
        SizingConfig::default()
    }
}

impl PaddingConfig {
    pub fn same_padding(padding: f32) -> PaddingConfig {
        PaddingConfig {
            left: padding,
            right: padding,
            top: padding,
            bottom: padding,
        }
    }

    pub fn axis_padding(top_bottom: f32, left_right: f32) -> PaddingConfig {
        PaddingConfig {
            left: left_right,
            right: left_right,
            top: top_bottom,
            bottom: top_bottom,
        }
    }

    pub fn individual_padding(top: f32, right: f32, bottom: f32, left: f32) -> PaddingConfig {
        PaddingConfig {
            top,
            right,
            bottom,
            left,
        }
    }
}

impl AlignmentConfig {
    pub fn new(align_x: HorizontalAlignment, align_y: VerticalAlignment) -> AlignmentConfig {
        AlignmentConfig { align_y, align_x }
    }
}

impl Element {
    pub fn new(id: u64, element_config: Rc<RefCell<ElementConfig>>) -> Element {
        Element {
            dimensions: Dimensions::default(),
            positions: Positions::default(),
            childs: Vec::new(),
            id,
            element_config,
            child_position_offset: 0.,
        }
    }
}

impl LayoutContext {
    pub fn create_context(width: f32, height: f32) -> LayoutContext {
        LayoutContext {
            root_dimensions: Dimensions { width, height },
            ..Default::default()
        }
    }

    pub fn begin_layout(&mut self) {
        self.element_stack.clear();
        self.top_id = 1;
        self.element_stack.push_back(Element::new(
            0,
            Rc::new(RefCell::new(ElementConfig {
                width: SizingConfig::fixed(self.root_dimensions.width),
                height: SizingConfig::fixed(self.root_dimensions.height),
                ..Default::default()
            })),
        ));
    }

    pub fn end_layout(&mut self) {
        let mut root_element = self
            .element_stack
            .pop_back()
            .expect("Root Element must always be there.");

        // configuring root element
        {
            let root_config = root_element.element_config.borrow();

            // root element always sized as Fixed.
            root_element.dimensions.width = root_config.width.max_val;
            root_element.dimensions.height = root_config.height.max_val;
        }

        LayoutContext::recursive_dbg(&root_element);
    }

    fn recursive_dbg(element: &Element) {
        println!(
            "Element id: {} has x: {}, y: {}, width: {}, height: {}",
            element.id,
            element.positions.x,
            element.positions.y,
            element.dimensions.width,
            element.dimensions.height
        );

        for child in &element.childs {
            LayoutContext::recursive_dbg(child);
        }
    }

    fn open_element(&mut self, element_config: Rc<RefCell<ElementConfig>>) {
        self.element_stack
            .push_back(Element::new(self.top_id, element_config));
        self.top_id += 1;
    }

    fn close_element(&mut self) {
        let mut current_element = self
            .element_stack
            .pop_back()
            .expect("There must be an element here.");

        let mut parent_element = self
            .element_stack
            .pop_back()
            .expect("It must have a parent element.");

        // borrow scoping
        {
            let current_config = &current_element.element_config.borrow();

            if let SizingType::Fixed = current_config.width.sizing_type {
                current_element.dimensions.width = current_config.width.max_val
            }

            if let SizingType::Fixed = current_config.height.sizing_type {
                current_element.dimensions.height = current_config.height.max_val
            }

            let parent_config = &parent_element.element_config.borrow();

            match parent_config.layout_direction {
                LayoutDirection::LeftToRight => {
                    parent_element.dimensions.width += current_element.dimensions.width;
                    parent_element.dimensions.height = parent_element
                        .dimensions
                        .height
                        .max(current_element.dimensions.height);
                    current_element.positions.x =
                        parent_element.positions.x + parent_element.child_position_offset;
                    current_element.positions.y = parent_element.positions.y;
                    parent_element.child_position_offset += current_element.dimensions.width;
                }
                LayoutDirection::TopToBottom => {
                    parent_element.dimensions.width = parent_element
                        .dimensions
                        .width
                        .max(current_element.dimensions.width);
                    parent_element.dimensions.height += current_element.dimensions.height;
                    current_element.positions.x = parent_element.positions.x;
                    current_element.positions.y =
                        parent_element.positions.y + parent_element.child_position_offset;
                    parent_element.child_position_offset += current_element.dimensions.height;
                }
            }
        }

        parent_element.childs.push(current_element);
        self.element_stack.push_back(parent_element);
    }

    pub fn add_element(
        &mut self,
        element_config: Rc<RefCell<ElementConfig>>,
        inner_layout: fn(&mut LayoutContext),
    ) {
        self.open_element(element_config);
        inner_layout(self);
        self.close_element();
    }
}
