use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use crate::data_type::*;

/**
 * TODO: REMOVE ALL POSSIBLE PANIC CODE WITH RESULT EQUIVALENT.
 * TODO: ALSO REMOVE AS MANY CLONE ON THE TEXT MESS AS POSSIBLE.
 */
impl ElementConfig {
    pub fn new(config: ElementConfig) -> Rc<ElementConfig> {
        Rc::new(config)
    }

    pub fn new_from(config: Rc<ElementConfig>) -> Rc<ElementConfig> {
        Rc::new(*config)
    }
}

impl DimensionConfig {
    pub fn fit() -> DimensionConfig {
        DimensionConfig::Fit(FitSizingConfig { min_size: 0. })
    }

    pub fn fit_clamped(min_size: f32) -> DimensionConfig {
        DimensionConfig::Fit(FitSizingConfig { min_size })
    }

    pub fn grow() -> DimensionConfig {
        DimensionConfig::Grow(GrowDimensionConfig {
            min_size: 0.,
            max_size: 0.,
        })
    }

    pub fn grow_clamped(min_size: f32, max_size: f32) -> DimensionConfig {
        DimensionConfig::Grow(GrowDimensionConfig { min_size, max_size })
    }

    pub fn fixed(size: f32) -> DimensionConfig {
        DimensionConfig::Fixed(FixedDimensionConfig { size })
    }

    pub fn percent(percent: f32) -> DimensionConfig {
        DimensionConfig::Percent(PercentDimenionConfig { percent })
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

    pub fn no_padding() -> PaddingConfig {
        PaddingConfig {
            left: 0.,
            right: 0.,
            top: 0.,
            bottom: 0.,
        }
    }
}

impl AlignmentConfig {
    pub fn new(align_x: HorizontalAlignment, align_y: VerticalAlignment) -> AlignmentConfig {
        AlignmentConfig { align_y, align_x }
    }
}

impl Element {
    pub fn new(id: u64, element_config: TypeConfig) -> Element {
        Element {
            dimensions: Dimensions::default(),
            id,
            position: Position::default(),
            child_elements: Vec::new(),
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
            element_tree_post_order: Vec::new(),
            measure_text_fn: Box::new(|_, _, _| -> TextMeasurement {
                panic!("No text measurement function was provided!")
            }),
            measure_text_cache: HashMap::new(),
        }
    }

    pub fn set_measurement_fn(
        &mut self,
        function: impl Fn(&str, u32, u16) -> TextMeasurement + 'static,
    ) {
        self.measure_text_fn = Box::new(function);
    }

    pub fn begin_layout(&mut self) {
        self.element_stack.clear();
        self.element_tree_post_order.clear();
        self.top_id = 1;
        self.element_stack.push_back(Element::new(
            0,
            TypeConfig::Rectangle(Rc::new(ElementConfig {
                width: DimensionConfig::fixed(self.root_dimensions.width),
                height: DimensionConfig::fixed(self.root_dimensions.height),
                ..Default::default()
            })),
        ));
    }

    fn fit_sizing(&mut self, x_axis: bool) {
        for element in &self.element_tree_post_order {
            let mut element = element.borrow_mut();

            match element.element_config.clone() {
                TypeConfig::Rectangle(element_config) => {
                    let layout_direction = element_config.child_layout_direction;

                    if x_axis {
                        if let DimensionConfig::Fit(fit_config) = element_config.width {
                            match layout_direction {
                                LayoutDirection::LeftToRight => {
                                    let mut width_accumulator = 0.;

                                    for child in &element.child_elements {
                                        width_accumulator += child.borrow().dimensions.width;
                                    }

                                    element.dimensions.width = width_accumulator;

                                    let gaps = element_config.child_gap;
                                    element.dimensions.width +=
                                        (element.child_elements.len() - 1) as f32 * gaps;

                                    // if the element is too small, clamp it to min_size
                                    element.dimensions.width =
                                        fit_config.min_size.max(element.dimensions.width);
                                }
                                LayoutDirection::TopToBottom => {
                                    let mut max_width: f32 = 0.;

                                    for child in &element.child_elements {
                                        max_width = max_width.max(child.borrow().dimensions.width);
                                    }

                                    element.dimensions.width = max_width.max(fit_config.min_size);
                                }
                            }

                            let padding_width =
                                element_config.padding.left + element_config.padding.right;
                            element.dimensions.width += padding_width;
                        }
                    } else if let DimensionConfig::Fit(fit_config) = element_config.height {
                        match layout_direction {
                            LayoutDirection::LeftToRight => {
                                let mut max_height: f32 = 0.;

                                for child in &element.child_elements {
                                    max_height = max_height.max(child.borrow().dimensions.height);
                                }

                                element.dimensions.height = max_height.max(fit_config.min_size);
                            }
                            LayoutDirection::TopToBottom => {
                                let mut height_accumulator = 0.;

                                for child in &element.child_elements {
                                    height_accumulator += child.borrow().dimensions.height;
                                }

                                element.dimensions.height = height_accumulator;

                                let gap = element_config.child_gap;

                                element.dimensions.height +=
                                    (element.child_elements.len() - 1) as f32 * gap;

                                element.dimensions.height =
                                    fit_config.min_size.max(element.dimensions.height);
                            }
                        }

                        let padding_height =
                            element_config.padding.top + element_config.padding.bottom;
                        element.dimensions.height += padding_height;
                    }
                }

                TypeConfig::Text(text_config) => {
                    if x_axis {
                        if let DimensionConfig::Fit(fit_config) = text_config.width {
                            if fit_config.min_size != 0. {
                                element.dimensions.width = fit_config.min_size
                            }
                        }
                    }

                    // this is disabled as we do not have text overflow yet.
                    /*
                    else if let DimensionConfig::Fit(fit_config) = text_config.height {
                        if fit_config.min_size != 0. {
                            element.dimensions.height = fit_config.min_size;
                        }
                    }
                    */
                }
            }
        }
    }

    fn position_element(&mut self) {
        for element in (self.element_tree_post_order).iter().rev() {
            let parent = element.borrow_mut();

            let padding_config;
            let child_gap;
            let layout_direction;
            let horizontal_alignment;
            let vertical_alignment;

            match &parent.element_config {
                TypeConfig::Rectangle(element_config) => {
                    padding_config = element_config.padding;
                    child_gap = element_config.child_gap;
                    layout_direction = element_config.child_layout_direction;
                    horizontal_alignment = element_config.child_alignment.align_x;
                    vertical_alignment = element_config.child_alignment.align_y;
                }
                TypeConfig::Text(_) => {
                    continue;
                }
            }

            /*
               On Alignments:

               When we are aligning along the layout direction, all element act as one singular large element.
               When we are aligning against the layout direction however, each element align individually.
            */

            let mut childs_boundingbox = Dimensions::default();

            for child in &parent.child_elements {
                let mut child = child.borrow_mut();
                let child_dimensions = child.dimensions;

                match layout_direction {
                    LayoutDirection::LeftToRight => {
                        childs_boundingbox.width += child_dimensions.width;

                        match vertical_alignment {
                            VerticalAlignment::Top => {
                                child.position.y = parent.position.y + padding_config.top;
                            }
                            VerticalAlignment::Bottom => {
                                child.position.y = parent.position.y + parent.dimensions.height
                                    - padding_config.bottom
                                    - child.dimensions.height;
                            }
                            VerticalAlignment::Center => {
                                let height_offset = (parent.dimensions.height
                                    - child.dimensions.height
                                    - padding_config.top
                                    - padding_config.bottom)
                                    / 2.;
                                child.position.y = parent.position.y + height_offset;
                            }
                        }
                    }
                    LayoutDirection::TopToBottom => {
                        childs_boundingbox.height += child_dimensions.height;

                        match horizontal_alignment {
                            HorizontalAlignment::Left => {
                                child.position.x = parent.position.x + padding_config.left;
                            }
                            HorizontalAlignment::Right => {
                                child.position.x = parent.position.x + parent.dimensions.width
                                    - padding_config.right
                                    - child.dimensions.width;
                            }
                            HorizontalAlignment::Center => {
                                let width_offset = (parent.dimensions.width
                                    - child.dimensions.width
                                    - padding_config.left
                                    - padding_config.right)
                                    / 2.;

                                child.position.x = parent.position.x + width_offset;
                            }
                        }
                    }
                }
            }

            match layout_direction {
                LayoutDirection::LeftToRight => {
                    childs_boundingbox.width += parent.child_elements.len() as f32 * child_gap;

                    let mut offset = 0.;
                    let start_x = match horizontal_alignment {
                        HorizontalAlignment::Left => parent.position.x + padding_config.left,
                        HorizontalAlignment::Center => {
                            parent.position.x
                                + (parent.dimensions.width
                                    - childs_boundingbox.width
                                    - padding_config.left
                                    - padding_config.right)
                                    / 2.
                        }
                        HorizontalAlignment::Right => {
                            parent.position.x + parent.dimensions.width
                                - childs_boundingbox.width
                                - padding_config.right
                        }
                    };

                    for child in &parent.child_elements {
                        let mut child = child.borrow_mut();

                        child.position.x = start_x + offset;
                        offset += child.dimensions.width + child_gap;
                    }
                }
                LayoutDirection::TopToBottom => {
                    childs_boundingbox.height += parent.child_elements.len() as f32 * child_gap;

                    let mut offset = 0.;
                    let start_y = match vertical_alignment {
                        VerticalAlignment::Top => parent.position.y + padding_config.top,
                        VerticalAlignment::Center => {
                            parent.position.y
                                + (parent.dimensions.height
                                    - childs_boundingbox.height
                                    - padding_config.top
                                    - padding_config.bottom)
                                    / 2.
                        }
                        VerticalAlignment::Bottom => {
                            parent.position.y + parent.dimensions.height
                                - childs_boundingbox.height
                                - padding_config.bottom
                        }
                    };

                    for child in &parent.child_elements {
                        let mut child = child.borrow_mut();

                        child.position.y = start_y + offset;
                        offset += child.dimensions.height + child_gap;
                    }
                }
            }
        }
    }

    fn percent_sizing(&mut self, x_axis: bool) {
        for element in (self.element_tree_post_order).iter().rev() {
            let parent = element.borrow();

            if parent.child_elements.is_empty() {
                return;
            }

            if let TypeConfig::Rectangle(parent_config) = &parent.element_config {
                let parent_undefined_size = matches!(parent_config.width, DimensionConfig::Grow(_));
                let child_count = parent.child_elements.len();

                for child_index in 0..child_count {
                    let mut child = parent.child_elements[child_index].borrow_mut();

                    if parent_undefined_size {
                        child.grow_on_percent_mark = true;
                        continue;
                    }

                    if x_axis {
                        let width_config = match &child.element_config {
                            TypeConfig::Rectangle(child_config) => child_config.width,
                            TypeConfig::Text(child_text_config) => child_text_config.width,
                        };

                        if let DimensionConfig::Percent(percent_config) = width_config {
                            child.dimensions.width =
                                parent.dimensions.width * percent_config.percent;

                            if child_index == 0 {
                                child.dimensions.width -= parent_config.padding.left;
                            }

                            if child_index == child_count - 1 {
                                child.dimensions.width -= parent_config.padding.right;
                            }

                            if child_count > 1
                                && matches!(
                                    parent_config.child_layout_direction,
                                    LayoutDirection::LeftToRight
                                )
                            {
                                child.dimensions.width -= parent_config.child_gap
                                    / if child_index == 0 || child_index == child_count - 1 {
                                        2.
                                    } else {
                                        1.
                                    };
                            }
                        }
                    } else {
                        let height_config = match &child.element_config {
                            TypeConfig::Rectangle(child_config) => child_config.height,
                            TypeConfig::Text(child_text_config) => child_text_config.height,
                        };

                        if let DimensionConfig::Percent(percent_config) = height_config {
                            child.dimensions.height =
                                parent.dimensions.height * percent_config.percent;

                            if child_index == 0 {
                                child.dimensions.height -= parent_config.padding.top;
                            }

                            if child_index == child_count - 1 {
                                child.dimensions.height -= parent_config.padding.bottom;
                            }

                            if child_count > 1
                                && matches!(
                                    parent_config.child_layout_direction,
                                    LayoutDirection::TopToBottom
                                )
                            {
                                child.dimensions.height -= parent_config.child_gap
                                    / if child_index == 0 || child_index == child_count - 1 {
                                        2.
                                    } else {
                                        1.
                                    };
                            }
                        }
                    }
                }
            }
        }
    }

    fn grow_sizing(&mut self, x_axis: bool) {
        /*
            Why no shrinking? Actually, it is not possible with the current
            configuration that an overflow may happen with grow elements being responsible.

            Grow element has no "preferred size". Only min and max, and it'd take any value.
            So if other childs already overflowed the parent, nothing can be done since
            grow elements are already at min_value.

            If later we introduce "preferred size" for grow element, then this become a bug.
            But personally, I think preferred size is no-go. It makes reasoning with sizing
            more complicated - does the Grow element try to size itself between min-max,
            streching to max starting from min, or do they try to get as close as possible
            to preferred size?
        */

        for element in (self.element_tree_post_order).iter().rev() {
            let parent = element.borrow_mut();

            let parent_config = match &parent.element_config {
                TypeConfig::Rectangle(rect_conf) => rect_conf,

                // text config cannot have children sizing
                TypeConfig::Text(_) => continue,
            };

            /*
               If this is a grow element, then at this stage this element must have
               had a concrete value (since it has already passed the grow
               constraint from the grandparent), thus any children that is marked
               for awaiting a concrete grow value can (and must) be solved here.
            */

            let child_count = parent.child_elements.len();
            let half_gap = parent_config.child_gap / 2.;

            if x_axis {
                if let DimensionConfig::Grow(_) = parent_config.width {
                    for child_index in 0..child_count {
                        let mut child = parent.child_elements[child_index].borrow_mut();
                        if !child.grow_on_percent_mark {
                            continue;
                        }

                        let width_config = match &child.element_config {
                            TypeConfig::Rectangle(rect_conf) => rect_conf.width,
                            TypeConfig::Text(text_conf) => text_conf.width,
                        };

                        let width_percentage = match width_config {
                            DimensionConfig::Percent(percentage_conf) => percentage_conf.percent,
                            _ => {
                                panic!("There is no other sizing type that depends on parent here")
                            }
                        };

                        child.dimensions.width = parent.dimensions.width * width_percentage;

                        if child_index == 0 {
                            child.dimensions.width -= parent_config.padding.left;
                        }

                        if child_index == child_count - 1 {
                            child.dimensions.width -= parent_config.padding.right;
                        }

                        if child_count > 1
                            && matches!(
                                parent_config.child_layout_direction,
                                LayoutDirection::LeftToRight
                            )
                        {
                            if child_index == 0 || child_index == child_count - 1 {
                                child.dimensions.width -= half_gap;
                            } else {
                                child.dimensions.width -= 2. * half_gap;
                            }
                        }
                    }
                }
            } else if let DimensionConfig::Grow(_) = parent_config.height {
                for child_index in 0..child_count {
                    let mut child = parent.child_elements[child_index].borrow_mut();
                    if !child.grow_on_percent_mark {
                        continue;
                    }

                    let height_config = match &child.element_config {
                        TypeConfig::Rectangle(rect_conf) => rect_conf.height,
                        TypeConfig::Text(text_conf) => text_conf.height,
                    };

                    let height_percentage = match height_config {
                        DimensionConfig::Percent(percentage_conf) => percentage_conf.percent,
                        _ => {
                            panic!("There is no other sizing type that depends on parent here")
                        }
                    };

                    child.dimensions.height = parent.dimensions.height * height_percentage;

                    if child_index == 0 {
                        child.dimensions.height -= parent_config.padding.top;
                    }

                    if child_index == child_count - 1 {
                        child.dimensions.height -= parent_config.padding.bottom;
                    }

                    if child_count > 1
                        && matches!(
                            parent_config.child_layout_direction,
                            LayoutDirection::TopToBottom
                        )
                    {
                        if child_index == 0 || child_index == child_count - 1 {
                            child.dimensions.height -= half_gap;
                        } else {
                            child.dimensions.height -= 2. * half_gap;
                        }
                    }
                }
            }

            let mut grow_child_vec: Vec<Rc<RefCell<Element>>> = Vec::new();
            let mut remaining_dimensions: f32;

            /*
               Growing algorithm also depend on the layout direction of the element.
               If it is growing alongside the layout direction, then sharing of the
               grow value is necessary. Otherwise, just give it the parent element
               value, surely this wont bite. Heh.
            */

            if x_axis {
                remaining_dimensions = parent.dimensions.width
                    - parent_config.padding.left
                    - parent_config.padding.right;

                if matches!(
                    parent_config.child_layout_direction,
                    LayoutDirection::LeftToRight
                ) && parent.child_elements.len() > 1
                {
                    remaining_dimensions -=
                        parent_config.child_gap * (parent.child_elements.len() - 1) as f32;
                }
            } else {
                remaining_dimensions = parent.dimensions.height
                    - parent_config.padding.top
                    - parent_config.padding.bottom;

                if matches!(
                    parent_config.child_layout_direction,
                    LayoutDirection::TopToBottom
                ) && parent.child_elements.len() > 1
                {
                    remaining_dimensions -=
                        parent_config.child_gap * (parent.child_elements.len() - 1) as f32;
                }
            }

            for child_ref in &parent.child_elements {
                let mut child = child_ref.borrow_mut();

                if x_axis {
                    let width_config = match &child.element_config {
                        TypeConfig::Rectangle(rect_conf) => rect_conf.width,
                        TypeConfig::Text(text_conf) => text_conf.width,
                    };

                    if let DimensionConfig::Grow(_) = width_config {
                        if matches!(
                            parent_config.child_layout_direction,
                            LayoutDirection::TopToBottom
                        ) {
                            child.dimensions.width = remaining_dimensions;
                            continue;
                        }

                        grow_child_vec.push(Rc::clone(child_ref));
                        remaining_dimensions -= child.dimensions.width;
                        continue;
                    }

                    if matches!(
                        parent_config.child_layout_direction,
                        LayoutDirection::LeftToRight
                    ) {
                        remaining_dimensions -= child.dimensions.width;
                    }
                } else {
                    let height_config = match &child.element_config {
                        TypeConfig::Rectangle(rect_conf) => rect_conf.height,
                        TypeConfig::Text(text_conf) => text_conf.height,
                    };

                    if let DimensionConfig::Grow(_) = height_config {
                        if matches!(
                            parent_config.child_layout_direction,
                            LayoutDirection::LeftToRight
                        ) {
                            child.dimensions.height = remaining_dimensions;
                            continue;
                        }

                        grow_child_vec.push(Rc::clone(child_ref));
                        remaining_dimensions -= child.dimensions.height;
                        continue;
                    }

                    if matches!(
                        parent_config.child_layout_direction,
                        LayoutDirection::TopToBottom
                    ) {
                        remaining_dimensions -= child.dimensions.height;
                    }
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
                    if matches!(
                        parent_config.child_layout_direction,
                        LayoutDirection::TopToBottom
                    ) {
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
                                let width_config = match &element.element_config {
                                    TypeConfig::Rectangle(rect_conf) => rect_conf.width,
                                    TypeConfig::Text(text_conf) => text_conf.width,
                                };
                                if let DimensionConfig::Grow(width_config) = width_config {
                                    let max_val = width_config.max_size;

                                    // clamp the grow to the max_value, if applicable
                                    if max_val != 0. {
                                        let delta = max_val - element.dimensions.width;
                                        remaining_dimensions -= delta;
                                        element.dimensions.width = max_val;

                                        drop(element);

                                        grow_child_vec.remove(id);
                                        index -= 1;
                                        continue;
                                    }

                                    element.dimensions.width = min_sizing;
                                    id += 1;
                                }
                            }
                            break;
                        }

                        // grow each element fairly to the min_sizing
                        let mut id = 0;
                        while id < index {
                            let mut element = grow_child_vec[id].borrow_mut();
                            let width_config = match &element.element_config {
                                TypeConfig::Rectangle(rect_conf) => rect_conf.width,
                                TypeConfig::Text(text_conf) => text_conf.width,
                            };
                            if let DimensionConfig::Grow(width_config) = width_config {
                                let max_size = width_config.max_size;
                                // clamp the grow to the max_value, if applicable
                                if max_size != 0. {
                                    let delta = max_size - element.dimensions.width;
                                    remaining_dimensions -= delta;
                                    element.dimensions.width = max_size;

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
                    }
                } else {
                    if matches!(
                        parent_config.child_layout_direction,
                        LayoutDirection::LeftToRight
                    ) {
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
                                let height_config = match &element.element_config {
                                    TypeConfig::Rectangle(rect_conf) => rect_conf.height,
                                    TypeConfig::Text(text_conf) => text_conf.height,
                                };
                                if let DimensionConfig::Grow(height_config) = height_config {
                                    let max_size = height_config.max_size;

                                    // clamp the grow to the max_value, if applicable
                                    if max_size != 0. {
                                        let delta = max_size - element.dimensions.height;
                                        remaining_dimensions -= delta;
                                        element.dimensions.height = max_size;

                                        drop(element);

                                        grow_child_vec.remove(id);
                                        index -= 1;
                                        continue;
                                    }

                                    element.dimensions.height = min_sizing;
                                    id += 1;
                                }
                            }

                            break;
                        }

                        // grow each element fairly to the min_sizing
                        let mut id = 0;
                        while id < index {
                            let mut element = grow_child_vec[id].borrow_mut();
                            let height_config = match &element.element_config {
                                TypeConfig::Rectangle(rect_conf) => rect_conf.height,
                                TypeConfig::Text(text_conf) => text_conf.height,
                            };
                            if let DimensionConfig::Grow(height_config) = height_config {
                                let max_val = height_config.max_size;

                                // clamp the grow to the max_value, if applicable
                                if max_val != 0. {
                                    let delta = max_val - element.dimensions.height;
                                    remaining_dimensions -= delta;
                                    element.dimensions.height = max_val;

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

    fn wrap_text(&mut self) {
        for element in &self.element_tree_post_order {
            let mut element = element.borrow_mut();
            let text_config = match &element.element_config {
                TypeConfig::Text(text_config) => text_config,
                _ => continue,
            };

            let mut text_lines: Vec<Rc<Element>> = Vec::new();

            if !text_config.break_word {
                continue;
            }

            // Since this is an immediate mode layout, a simple greedy text-breaking will suffice.
            let font_id = text_config.font_id;
            let font_size = text_config.font_size;
            let space_measurement = get_measurement(
                &mut self.measure_text_cache,
                &self.measure_text_fn,
                " ",
                font_id,
                font_size,
            );
            let text = text_config.text.clone();
            let word_list: Vec<&str> = text.split(" ").collect();

            let mut run_width: f32 = 0.;
            let mut run_height: f32 = 0.;
            let mut height_offset = 0.;
            let mut run_y_offset = 0.;
            let mut run_str: String = String::new();

            for word in word_list {
                let word_size = get_measurement(
                    &mut self.measure_text_cache,
                    &self.measure_text_fn,
                    word,
                    font_id,
                    font_size,
                );

                // edge case: a single word is larger than the container's width
                // equality here can happen as 0. is assigned as base value.
                if run_width == 0. && word_size.width > element.dimensions.width {
                    let mut text_element = create_text_element(element.id, word);
                    text_element.position.y = height_offset;
                    height_offset += word_size.height;
                    text_lines.push(Rc::new(text_element));
                }

                // if adding this word cause the current run to overflow
                if run_width + word_size.width + space_measurement.width > element.dimensions.width
                {
                    let mut text_element = create_text_element(element.id, &run_str);
                    text_element.position.y = height_offset + run_y_offset;
                    height_offset += run_height;
                    run_str.clear();
                    run_width = 0.;
                    run_height = 0.;
                    run_y_offset = 0.;
                    text_lines.push(Rc::new(text_element));
                }

                if run_width != 0. {
                    run_width += space_measurement.width;
                    run_str += " ";
                }
                run_height = run_height.max(word_size.height);
                run_y_offset = run_y_offset.max(word_size.y_offset);
                run_width += word_size.width;
                run_str += word;
            }

            // push the remaining texts
            let mut text_element = create_text_element(element.id, &run_str);
            text_element.position.y = height_offset + run_y_offset;
            height_offset += run_height;
            text_lines.push(Rc::new(text_element));
            element.dimensions.height = height_offset;

            if let TypeConfig::Text(text_config) = &mut element.element_config {
                text_config.text_lines = text_lines;
            }
        }
    }

    pub fn end_layout(&mut self) -> Vec<RenderCommand> {
        let mut root_element = self
            .element_stack
            .pop_back()
            .expect("Root element must always be there");

        root_element.dimensions.width = self.root_dimensions.width;
        root_element.dimensions.height = self.root_dimensions.height;

        let root_element = Rc::new(RefCell::new(root_element));

        self.element_tree_post_order.push(root_element);

        // process the configuration
        // Step 1: Fit Sizing Width
        self.fit_sizing(true);

        // Step 2: Percentage Width
        self.percent_sizing(true);

        // Step 3: Grow Width
        self.grow_sizing(true);

        // Step 4: Wrap Text
        self.wrap_text();

        // Step 5: Fit Sizing Height
        self.fit_sizing(false);

        // Step 6: Percentage Height
        self.percent_sizing(false);

        // Step 7: Grow Height
        self.grow_sizing(false);

        // Step 8: Positions
        self.position_element();

        let mut render_commands: Vec<RenderCommand> = Vec::new();

        // remove the implicit root element (TODO: think about exposing this root to public for use?)
        self.element_tree_post_order.pop();

        /*
            A consequence to using the stack is that element at the same level in the tree
            will be drawn in reverse order of insertion. That is, if A and B is inserted
            in that order and at the same level, this will create render command for B
            before A.
        */

        for element in (self.element_tree_post_order).iter().rev() {
            let element = element.borrow();

            match &element.element_config {
                TypeConfig::Rectangle(element_config) => {
                    render_commands.push(RenderCommand {
                        position: element.position,
                        render_data: RenderData::Rectangle(RectangleRenderData {
                            dimenions: element.dimensions,
                            color: element_config.color,
                        }),
                    });
                }
                TypeConfig::Text(element_config) => {
                    for text_element in &element_config.text_lines {
                        render_commands.push(RenderCommand {
                            position: Position {
                                x: element.position.x,
                                y: element.position.y + text_element.position.y,
                            },
                            render_data: RenderData::Text(TextRenderData {
                                font_id: element_config.font_id,
                                text: match &text_element.element_config {
                                    TypeConfig::Text(text_config) => text_config.text.clone(),
                                    _ => panic!("This has to be text type config"),
                                },
                                font_size: element_config.font_size,
                                font_color: element_config.font_color,
                            }),
                        });
                    }
                }
            }
        }

        render_commands
    }

    fn open_element(&mut self, element_config: Rc<ElementConfig>) {
        self.element_stack.push_back(Element::new(
            self.top_id,
            TypeConfig::Rectangle(element_config),
        ));
        self.top_id += 1;
    }

    fn close_element(&mut self) {
        let mut current_element = self
            .element_stack
            .pop_back()
            .expect("The element stack cannot be empty");

        let mut parent_element = self
            .element_stack
            .pop_back()
            .expect("Any element must have a parent element.");

        // Configure constant values
        {
            let width_config = match current_element.element_config {
                TypeConfig::Rectangle(ref rect_conf) => rect_conf.width,
                _ => panic!("Only rect config here."),
            };

            match width_config {
                DimensionConfig::Fixed(conf) => {
                    current_element.dimensions.width = conf.size;
                }
                DimensionConfig::Grow(conf) => {
                    current_element.dimensions.width = conf.min_size;
                }
                _ => {}
            }

            let height_config = match current_element.element_config {
                TypeConfig::Rectangle(ref rect_conf) => rect_conf.height,
                _ => panic!("Only rect config here."),
            };

            match height_config {
                DimensionConfig::Fixed(conf) => {
                    current_element.dimensions.height = conf.size;
                }
                DimensionConfig::Grow(conf) => {
                    current_element.dimensions.height = conf.min_size;
                }
                _ => {}
            };
        }

        let current_element = Rc::new(RefCell::new(current_element));

        parent_element
            .child_elements
            .push(Rc::clone(&current_element));
        self.element_tree_post_order.push(current_element);
        self.element_stack.push_back(parent_element);
    }

    pub fn add_element<F: FnOnce(&mut LayoutContext)>(
        &mut self,
        element_config: Rc<ElementConfig>,
        inner_layout: F,
    ) {
        self.open_element(element_config);
        inner_layout(self);
        self.close_element();
    }

    /**
     * Fit Sizing will cause the text to collapse
     * TODO make better docs
     */
    pub fn add_text(&mut self, text: &str, text_config: TextConfig) {
        // text element cannot have children, so we implement custom logic instead of reusing.

        let mut element_starting_width = 0.;

        let mut parent_element = self
            .element_stack
            .pop_back()
            .expect("Any element must have a parent element.");

        match text_config.width {
            DimensionConfig::Grow(_) => {
                let text_dimension =
                    (self.measure_text_fn)(text, text_config.font_id, text_config.font_size);
                element_starting_width = text_dimension.width;
            }
            DimensionConfig::Fixed(fixed_config) => {
                element_starting_width = fixed_config.size;
            }
            _ => {}
        }

        let text_config = InternalTextConfig::new_from(text, text_config);

        let mut current_element = Element::new(self.top_id, TypeConfig::Text(text_config));
        current_element.dimensions.width = element_starting_width;

        let current_element = Rc::new(RefCell::new(current_element));

        parent_element
            .child_elements
            .push(Rc::clone(&current_element));
        self.element_tree_post_order.push(current_element);
        self.element_stack.push_back(parent_element);
    }
}

impl InternalTextConfig {
    pub fn new_from(text: &str, text_config: TextConfig) -> InternalTextConfig {
        InternalTextConfig {
            width: text_config.width,
            height: text_config.height,
            font_id: text_config.font_id,
            break_word: text_config.break_word,
            text: Rc::from(text),
            font_size: text_config.font_size,
            font_color: text_config.font_color,
            text_lines: Vec::new(),
        }
    }
}

fn construct_key(word: &str, font_id: u32, font_size: u16) -> String {
    font_id.to_string() + &font_size.to_string() + word
}

fn get_measurement(
    measurement_cache: &mut HashMap<String, TextMeasurement>,
    measure_text: impl Fn(&str, u32, u16) -> TextMeasurement,
    text: &str,
    font_id: u32,
    font_size: u16,
) -> TextMeasurement {
    let key = construct_key(text, font_id, font_size);

    match measurement_cache.get(&key) {
        Some(val) => *val,
        None => {
            let text_measurement = (measure_text)(text, font_id, font_size);
            measurement_cache.insert(key, text_measurement);
            text_measurement
        }
    }
}

fn create_text_element(element_id: u64, text: &str) -> Element {
    let text_config = InternalTextConfig {
        width: DimensionConfig::Fixed(FixedDimensionConfig { size: 0. }),
        height: DimensionConfig::Fixed(FixedDimensionConfig { size: 0. }),
        font_id: 0,
        break_word: false,
        text: Rc::from(text),
        font_size: 0,
        font_color: Color::default(),
        text_lines: Vec::new(),
    };

    Element::new(element_id, TypeConfig::Text(text_config))
}
