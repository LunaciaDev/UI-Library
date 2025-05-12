use std::{cell::RefCell, cmp::Ordering, collections::VecDeque, rc::Rc};

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
    pub fn grow_clamped(min: f32, max: f32) -> SizingConfig {
        SizingConfig {
            sizing_type: SizingType::Grow,
            min_val: min,
            max_val: max,
            ..Default::default()
        }
    }

    pub fn grow() -> SizingConfig {
        SizingConfig {
            sizing_type: SizingType::Grow,
            min_val: 0.,
            max_val: 0.,
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
            grow_on_percent_mark: false,
        }
    }
}

impl LayoutContext {
    pub fn create_context(width: f32, height: f32) -> LayoutContext {
        LayoutContext {
            root_dimensions: Dimensions { width, height },
            element_stack: VecDeque::new(),
            top_id: 0,
            element_chain_bottomup: Vec::new(),
            measure_text: Box::new(|_| -> TextMeasurement { panic!("No text measurement was supplied") }),
        }
    }

    pub fn set_measurement_fn(&mut self, function: impl Fn(&str) -> TextMeasurement + 'static) {
        self.measure_text = Box::new(function);
    }

    pub fn begin_layout(&mut self) {
        self.element_stack.clear();
        self.element_chain_bottomup.clear();
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
            let layout_direction = element.element_config.borrow().layout_direction;

            if x_axis {
                let sizing_config = element.element_config.borrow().width;

                if matches!(sizing_config.sizing_type, SizingType::Fit) {
                    match layout_direction {
                        LayoutDirection::LeftToRight => {
                            let mut width_accumulator = 0.;

                            for child in &element.childs {
                                width_accumulator += child.borrow().dimensions.width;
                            }

                            element.dimensions.width = width_accumulator;

                            let gaps = element.element_config.borrow().gap;
                            element.dimensions.width += (element.childs.len() - 1) as f32 * gaps;
                        },
                        LayoutDirection::TopToBottom => {
                            let mut max_width: f32 = 0.;

                            for child in &element.childs {
                                max_width = max_width.max(child.borrow().dimensions.width);
                            }

                            element.dimensions.width = max_width;
                        },
                    }

                    let padding_width = element.element_config.borrow().padding.left + element.element_config.borrow().padding.right;
                    element.dimensions.width += padding_width;
                }
            }
            else {
                let sizing_config = element.element_config.borrow().height;

                if matches!(sizing_config.sizing_type, SizingType::Fit) {
                    match layout_direction {
                        LayoutDirection::LeftToRight => {
                            let mut max_height: f32 = 0.;

                            for child in &element.childs {
                                max_height = max_height.max(child.borrow().dimensions.height);
                            }

                            element.dimensions.height = max_height;
                        },
                        LayoutDirection::TopToBottom => {
                            let mut height_accumulator = 0.;

                            for child in &element.childs {
                                height_accumulator += child.borrow().dimensions.height;
                            }

                            element.dimensions.height = height_accumulator;

                            let gap = element.element_config.borrow().gap;

                            element.dimensions.height += (element.childs.len() - 1) as f32 * gap
                        },
                    }

                    let padding_height = element.element_config.borrow().padding.top + element.element_config.borrow().padding.bottom;
                    element.dimensions.height += padding_height;
                }
            }
        }
    }

    fn position_element(&mut self) {
        for element in (self.element_chain_bottomup).iter().rev() {
            let parent = element.borrow_mut();

            let padding_config = parent.element_config.borrow().padding;
            let child_gap = parent.element_config.borrow().gap;
            let layout_direction = parent.element_config.borrow().layout_direction;
            let horizontal_alignment = parent.element_config.borrow().child_alignment.align_x;
            let vertical_alignment = parent.element_config.borrow().child_alignment.align_y;

            /*
               On Alignments:

               When we are aligning along the layout direction, all element act as one singular large element.
               When we are aligning against the layout direction however, each element align individually.
            */

            let mut child_bounding_box = Dimensions::default();

            for child in &parent.childs {
                let mut child = child.borrow_mut();
                let child_dimensions = child.dimensions;

                match layout_direction {
                    LayoutDirection::LeftToRight => {
                        child_bounding_box.width += child_dimensions.width;

                        match vertical_alignment {
                            VerticalAlignment::Top => {
                                child.positions.y = parent.positions.y + padding_config.top;
                            }
                            VerticalAlignment::Bottom => {
                                child.positions.y = parent.positions.y + parent.dimensions.height
                                    - padding_config.bottom
                                    - child.dimensions.height;
                            }
                            VerticalAlignment::Center => {
                                let height_offset = (parent.dimensions.height
                                    - child.dimensions.height
                                    - padding_config.top
                                    - padding_config.bottom)
                                    / 2.;
                                child.positions.y = parent.positions.y + height_offset;
                            }
                        }
                    }
                    LayoutDirection::TopToBottom => {
                        child_bounding_box.height += child_dimensions.height;

                        match horizontal_alignment {
                            HorizontalAlignment::Left => {
                                child.positions.x = parent.positions.x + padding_config.left;
                            }
                            HorizontalAlignment::Right => {
                                child.positions.x = parent.positions.x + parent.dimensions.width
                                    - padding_config.right
                                    - child.dimensions.width;
                            }
                            HorizontalAlignment::Center => {
                                let width_offset = (parent.dimensions.width
                                    - child.dimensions.width
                                    - padding_config.left
                                    - padding_config.right)
                                    / 2.;

                                child.positions.x = parent.positions.x + width_offset;
                            }
                        }
                    }
                }
            }

            match layout_direction {
                LayoutDirection::LeftToRight => {
                    child_bounding_box.width += parent.childs.len() as f32 * child_gap;

                    let mut offset = 0.;
                    let start_x = match horizontal_alignment {
                        HorizontalAlignment::Left => parent.positions.x + padding_config.left,
                        HorizontalAlignment::Center => {
                            parent.positions.x
                                + (parent.dimensions.width
                                    - child_bounding_box.width
                                    - padding_config.left
                                    - padding_config.right)
                                    / 2.
                        }
                        HorizontalAlignment::Right => {
                            parent.positions.x + parent.dimensions.width
                                - child_bounding_box.width
                                - padding_config.right
                        }
                    };

                    for child in &parent.childs {
                        let mut child = child.borrow_mut();

                        child.positions.x = start_x + offset;
                        offset += child.dimensions.width + child_gap;
                    }
                }
                LayoutDirection::TopToBottom => {
                    child_bounding_box.height += parent.childs.len() as f32 * child_gap;

                    let mut offset = 0.;
                    let start_y = match vertical_alignment {
                        VerticalAlignment::Top => parent.positions.y + padding_config.top,
                        VerticalAlignment::Center => {
                            parent.positions.y
                                + (parent.dimensions.height
                                    - child_bounding_box.height
                                    - padding_config.top
                                    - padding_config.bottom)
                                    / 2.
                        }
                        VerticalAlignment::Bottom => {
                            parent.positions.y + parent.dimensions.height
                                - child_bounding_box.height
                                - padding_config.bottom
                        }
                    };

                    for child in &parent.childs {
                        let mut child = child.borrow_mut();

                        child.positions.y = start_y + offset;
                        offset += child.dimensions.height + child_gap;
                    }
                }
            }
        }
    }

    fn percent_sizing(&mut self, x_axis: bool) {
        for element in (self.element_chain_bottomup).iter().rev() {
            let parent = element.borrow();

            for child in &parent.childs {
                let mut child = child.borrow_mut();

                if x_axis {
                    if matches!(
                        parent.element_config.borrow().width.sizing_type,
                        SizingType::Grow
                    ) {
                        child.grow_on_percent_mark = true;
                        continue;
                    }

                    if matches!(
                        child.element_config.borrow().width.sizing_type,
                        SizingType::Percent
                    ) {
                        let percentage = child.element_config.borrow().width.percent;

                        child.dimensions.width = parent.dimensions.width * percentage;
                    }
                } else {
                    if matches!(
                        parent.element_config.borrow().height.sizing_type,
                        SizingType::Grow
                    ) {
                        child.grow_on_percent_mark = true;
                        continue;
                    }

                    if matches!(
                        child.element_config.borrow().height.sizing_type,
                        SizingType::Percent
                    ) {
                        let percentage = child.element_config.borrow().height.percent;

                        child.dimensions.height = parent.dimensions.height * percentage;
                    }
                }
            }
        }
    }

    fn grow_sizing(&mut self, x_axis: bool) {
        for element in (self.element_chain_bottomup).iter().rev() {
            let parent = element.borrow_mut();
            let parent_config = parent.element_config.borrow();

            /*
               If this is a grow element, then at this stage this element must have had a concrete value
               (since it has already passed the grow constraint from the grandparent), thus any children
               that is marked for awaiting a concrete grow value can (and must) be solved here.
            */

            if x_axis {
                if matches!(parent_config.width.sizing_type, SizingType::Grow) {
                    for child in &parent.childs {
                        let mut child = child.borrow_mut();

                        if child.grow_on_percent_mark {
                            let percentage_value = child.element_config.borrow().width.percent;
                            child.dimensions.width = parent.dimensions.width * percentage_value;
                            child.grow_on_percent_mark = false;
                        }
                    }
                }
            } else if matches!(parent_config.height.sizing_type, SizingType::Grow) {
                for child in &parent.childs {
                    let mut child = child.borrow_mut();

                    if child.grow_on_percent_mark {
                        let percentage_value = child.element_config.borrow().height.percent;
                        child.dimensions.height = parent.dimensions.height * percentage_value;
                        child.grow_on_percent_mark = false;
                    }
                }
            }

            let mut grow_child_vec: Vec<Rc<RefCell<Element>>> = Vec::new();
            let mut remaining_dimensions: f32;

            /*
               Growing algorithm also depend on the layout direction of the element.
               If it is growing alongside the layout direction, then sharing of the
               grow value is necessary.
               Otherwise, just give it the parent element value, surely this wont
               bite. Heh.
            */

            if x_axis {
                remaining_dimensions = parent.dimensions.width
                    - parent_config.padding.left
                    - parent_config.padding.right;

                if matches!(parent_config.layout_direction, LayoutDirection::LeftToRight) && parent.childs.len() > 1 {
                    remaining_dimensions -= parent_config.gap * (parent.childs.len() - 1) as f32;
                }
            } else {
                remaining_dimensions = parent.dimensions.height
                    - parent_config.padding.top
                    - parent_config.padding.bottom;

                if matches!(parent_config.layout_direction, LayoutDirection::TopToBottom) && parent.childs.len() > 1 {
                    remaining_dimensions -= parent_config.gap * (parent.childs.len() - 1) as f32;
                }
            }

            for child_ref in &parent.childs {
                let child = child_ref.borrow();
                let child_config = child.element_config.borrow();

                if x_axis {
                    if matches!(child_config.width.sizing_type, SizingType::Grow) {
                        grow_child_vec.push(Rc::clone(child_ref));
                        remaining_dimensions -= child.dimensions.width;
                        continue;
                    }

                    remaining_dimensions -= child.dimensions.width;
                } else {
                    if matches!(child_config.height.sizing_type, SizingType::Grow) {
                        grow_child_vec.push(Rc::clone(child_ref));
                        remaining_dimensions -= child.dimensions.height;
                        continue;
                    }

                    remaining_dimensions -= child.dimensions.height;
                }
            }

            // Sort all children that need to solve grow for by their current size.
            // We grow these child until they have the same size, then distribute the rest evenly.
            grow_child_vec.sort_by(|a, b| -> Ordering {
                let a = a.borrow();
                let b = b.borrow();

                if x_axis {
                    if a.dimensions.width > b.dimensions.width {
                        return Ordering::Greater;
                    }

                    if a.dimensions.width < b.dimensions.width {
                        return Ordering::Less;
                    }

                    Ordering::Equal
                } else {
                    if a.dimensions.height > b.dimensions.height {
                        return Ordering::Greater;
                    }

                    if a.dimensions.height < b.dimensions.height {
                        return Ordering::Less;
                    }

                    Ordering::Equal
                }
            });

            let mut min_sizing: f32 = 0.;
            let mut index = 0;

            // grow all childs to the biggest child.
            while index < grow_child_vec.len() {
                if x_axis {
                    if matches!(parent_config.layout_direction, LayoutDirection::TopToBottom) {
                        {
                            let mut element = grow_child_vec[index].borrow_mut();
                            element.dimensions.width += remaining_dimensions;
                        }
                        grow_child_vec.remove(index);
                        continue;
                    }

                    let element_size = grow_child_vec[index].borrow().dimensions.width;

                    if element_size > min_sizing {
                        min_sizing = element_size;

                        if (remaining_dimensions / (index + 1) as f32) < min_sizing {
                            min_sizing = remaining_dimensions / (index + 1) as f32;

                            // grow each element toward the final min_sizing
                            let mut id = 0;
                            while id < index {
                                let mut element = grow_child_vec[id].borrow_mut();
                                let element_max_val = element.element_config.borrow().width.max_val;

                                // clamp the grow to the max_value, if applicable
                                if element_max_val != 0. {
                                    let delta = element_max_val - element.dimensions.width;
                                    remaining_dimensions -= delta;
                                    element.dimensions.width = element_max_val;

                                    drop(element);

                                    grow_child_vec.remove(id);
                                    index -= 1;
                                    continue;
                                }

                                element.dimensions.width = min_sizing;
                                id += 1;
                            }

                            break;
                        }

                        // grow each element fairly to the min_sizing
                        let mut id = 0;
                        while id < index {
                            let mut element = grow_child_vec[id].borrow_mut();
                            let element_max_val = element.element_config.borrow().width.max_val;

                            // clamp the grow to the max_value, if applicable
                            if element_max_val != 0. {
                                let delta = element_max_val - element.dimensions.width;
                                remaining_dimensions -= delta;
                                element.dimensions.width = element_max_val;

                                drop(element);

                                grow_child_vec.remove(id);
                                index -= 1;
                                id += 1;
                                continue;
                            }

                            let delta = min_sizing - element.dimensions.width;

                            element.dimensions.width = min_sizing;
                            remaining_dimensions -= delta;
                            id += 1;
                        }
                    }
                } else {
                    if matches!(parent_config.layout_direction, LayoutDirection::LeftToRight) {
                        {
                            let mut element = grow_child_vec[index].borrow_mut();
                            element.dimensions.height += remaining_dimensions;
                        }
                        grow_child_vec.remove(index);
                        continue;
                    }

                    let element_size = grow_child_vec[index].borrow().dimensions.height;

                    if element_size > min_sizing {
                        min_sizing = element_size;

                        if (remaining_dimensions / (index + 1) as f32) < min_sizing {
                            min_sizing = remaining_dimensions / (index + 1) as f32;

                            // grow each element toward the final min_sizing
                            let mut id = 0;
                            while id < index {
                                let mut element = grow_child_vec[id].borrow_mut();
                                let element_max_val =
                                    element.element_config.borrow().height.max_val;

                                // clamp the grow to the max_value, if applicable
                                if element_max_val != 0. {
                                    let delta = element_max_val - element.dimensions.height;
                                    remaining_dimensions -= delta;
                                    element.dimensions.height = element_max_val;

                                    drop(element);

                                    grow_child_vec.remove(id);
                                    index -= 1;
                                    continue;
                                }

                                element.dimensions.height = min_sizing;
                                id += 1;
                            }

                            break;
                        }

                        // grow each element fairly to the min_sizing
                        let mut id = 0;
                        while id < index {
                            let mut element = grow_child_vec[id].borrow_mut();
                            let element_max_val = element.element_config.borrow().height.max_val;

                            // clamp the grow to the max_value, if applicable
                            if element_max_val != 0. {
                                let delta = element_max_val - element.dimensions.height;
                                remaining_dimensions -= delta;
                                element.dimensions.height = element_max_val;

                                drop(element);

                                grow_child_vec.remove(id);
                                index -= 1;
                                id += 1;
                                continue;
                            }

                            let delta = min_sizing - element.dimensions.height;

                            element.dimensions.height = min_sizing;
                            remaining_dimensions -= delta;
                            id += 1;
                        }
                    }
                }

                index += 1;
            }

            // distribute remaining value equally
            remaining_dimensions /= grow_child_vec.len() as f32;

            for element in &grow_child_vec {
                let mut element = element.borrow_mut();

                if x_axis {
                    element.dimensions.width += remaining_dimensions;
                } else {
                    element.dimensions.height += remaining_dimensions;
                }
            }
        }
    }

    pub fn end_layout(&mut self) -> Vec<RenderCommand> {
        let mut root_element = self.element_stack.pop_back().expect("Root element must always be there");

        root_element.dimensions.width = root_element.element_config.borrow().width.max_val;
        root_element.dimensions.height = root_element.element_config.borrow().height.max_val;

        let root_element = Rc::new(RefCell::new(root_element));

        self.element_chain_bottomup.push(root_element);

        // process the configuration
        // Step 1: Fit Sizing Width
        self.fit_sizing(true);

        // Step 2: Percentage Width
        self.percent_sizing(true);

        // Step 3: Grow Width
        self.grow_sizing(true);

        // Step 4: Wrap Text

        // Step 5: Fit Sizing Height
        self.fit_sizing(false);

        // Step 6: Percentage Height
        self.percent_sizing(false);

        // Step 7: Grow Height
        self.grow_sizing(false);

        // Step 8: Positions
        self.position_element();

        let mut render_commands: Vec<RenderCommand> = Vec::new();

        self.element_chain_bottomup.pop();

        for element in (self.element_chain_bottomup).iter().rev() {
            let element = element.borrow();

            render_commands.push(RenderCommand {
                dimension: element.dimensions,
                position: element.positions,
                color: element.element_config.borrow().color,
            });
        }

        render_commands
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

            match element_config.width.sizing_type {
                SizingType::Fixed => {
                    current_element.dimensions.width = element_config.width.max_val;
                }
                SizingType::Grow => {
                    current_element.dimensions.width = element_config.width.min_val;
                }
                _ => {}
            }

            match element_config.height.sizing_type {
                SizingType::Fixed => {
                    current_element.dimensions.height = element_config.height.max_val;
                }
                SizingType::Grow => {
                    current_element.dimensions.height = element_config.height.min_val;
                }
                _ => {}
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
