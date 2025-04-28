use std::{cell::RefCell, rc::Rc};

use ui_library::{ElementConfig, LayoutContext, PaddingConfig, SizingConfig};

struct ExampleObj {
    pub config: Rc<RefCell<ElementConfig>>,
}

pub fn main() {
    let mut layout_context = LayoutContext::create_context(1280., 720.);

    let obj: ExampleObj = ExampleObj {
        config: ElementConfig::new(ElementConfig {
            width: SizingConfig::fit(),
            height: SizingConfig::fit(),
            ..Default::default()
        }),
    };

    layout_context.begin_layout();
    layout_context.add_element(
        Rc::clone(&(obj.config)),
        |layout_context| {
            layout_context.add_element(
                ElementConfig::new(ElementConfig {
                    width: SizingConfig::fixed(100.),
                    height: SizingConfig::fixed(50.),
                    ..Default::default()
                }),
                |_|{},
            );

            layout_context.add_element(
                ElementConfig::new(ElementConfig {
                    width: SizingConfig::fixed(150.),
                    height: SizingConfig::fixed(100.),
                    ..Default::default()
                }),
                |_|{},
            );
        },
    );
    layout_context.end_layout();
}
