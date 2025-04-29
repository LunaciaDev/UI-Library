use std::{cell::RefCell, rc::Rc};

use crate::data_type::*;

impl ElementConfig {
    pub fn new(config: ElementConfig) -> Rc<RefCell<ElementConfig>> {
        Rc::new(RefCell::new(config))
    }

    pub fn new_from(config: Rc<RefCell<ElementConfig>>) -> Rc<RefCell<ElementConfig>> {
        Rc::new(RefCell::new(*config.borrow()))
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
            remaining_dimensions: Dimensions::default(),
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

    fn fit_sizing(&mut self, x_axis: bool) {
        for element in &self.element_chain_bottomup {
            let mut element = element.borrow_mut();
            let config = element.element_config.borrow();

            if x_axis {
                let width_config = config.width;
                let layout_direction = config.layout_direction;
                drop(config);

                if let SizingType::Fit = width_config.sizing_type {
                    match layout_direction {
                        LayoutDirection::LeftToRight => {
                            let mut width_accumulator = 0.;

                            for child in &element.childs {
                                width_accumulator += child.borrow().dimensions.width;
                            }

                            element.dimensions.width = width_accumulator;
                        }
                        LayoutDirection::TopToBottom => {
                            let mut max_width: f32 = 0.;
                            
                            for child in &element.childs {
                                max_width = max_width.max(child.borrow().dimensions.width);
                            }

                            element.dimensions.width = max_width;
                        }
                    }
                }
            } else {
                let height_config = config.height;
                let layout_direction = config.layout_direction;
                drop(config);

                if let SizingType::Fit = height_config.sizing_type {
                    match layout_direction {
                        LayoutDirection::LeftToRight => {
                            let mut max_height: f32 = 0.;

                            for child in &element.childs {
                                max_height = max_height.max(child.borrow().dimensions.height);
                            }

                            element.dimensions.height = max_height;
                        }
                        LayoutDirection::TopToBottom => {
                            let mut height_accumulator = 0.;
                            
                            for child in &element.childs {
                                height_accumulator += child.borrow().dimensions.height;
                            }

                            element.dimensions.height = height_accumulator;
                        }
                    }
                }
            }
        }
    }

    pub fn position_element(&mut self) {
        for element in (self.element_chain_bottomup).iter().rev() {
            let element = element.borrow_mut();
            let element_config = element.element_config.borrow();
            
            let padding_config = element_config.padding;
            let child_gap = element_config.gap;
            let layout_direction = element_config.layout_direction;
            let mut child_offset = Positions::default();

            drop(element_config);

            child_offset.x = padding_config.left;
            child_offset.y = padding_config.right;

            for child in &element.childs {
                let mut child = child.borrow_mut();

                child.positions.x = element.positions.x + child_offset.x;
                child.positions.y = element.positions.y + child_offset.y;

                match layout_direction {
                    LayoutDirection::LeftToRight => {
                        child_offset.x += child_gap + child.dimensions.width;
                    }
                    LayoutDirection::TopToBottom => {
                        child_offset.y += child_gap + child.dimensions.height;
                    }
                }
            }
        }
    }

    pub fn end_layout(&mut self) {
        let root_element = Rc::new(RefCell::new(
            self.element_stack
                .pop_back()
                .expect("Root Element must always be there."),
        ));

        let root_element_clone = Rc::clone(&root_element);

        self.element_chain_bottomup.push(root_element);

        // process the configuration

        // Step 1: Fit Sizing Width
        self.fit_sizing(true);

        // Step 2: Grow Width

        // Step 3: Wrap Text

        // Step 4: Fit Sizing Height
        self.fit_sizing(false);

        // Step 5: Grow Height

        // Step 6: Positions
        self.position_element();

        LayoutContext::recursive_dbg(root_element_clone);
    }

    fn recursive_dbg(element: Rc<RefCell<Element>>) {
        let element = element.borrow();

        println!(
            "Element id: {} has x: {}, y: {}, width: {}, height: {}",
            element.id,
            element.positions.x,
            element.positions.y,
            element.dimensions.width,
            element.dimensions.height
        );

        for child in &element.childs {
            LayoutContext::recursive_dbg(Rc::clone(child));
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

        // Configure constant values
        {
            let element_config = current_element.element_config.borrow();

            // Fixed Sizing + Padding content limitation
            if let SizingType::Fixed = element_config.width.sizing_type {
                current_element.dimensions.width = element_config.width.max_val;
                current_element.remaining_dimensions.width = current_element.dimensions.width
                    - element_config.padding.left
                    - element_config.padding.right;
            }
            if let SizingType::Fixed = element_config.height.sizing_type {
                current_element.dimensions.height = element_config.height.max_val;
                current_element.remaining_dimensions.height = current_element.dimensions.height
                    - element_config.padding.top
                    - element_config.padding.bottom;
            }
        }

        let current_element = Rc::new(RefCell::new(current_element));

        parent_element.childs.push(Rc::clone(&current_element));
        self.element_chain_bottomup.push(current_element);
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
