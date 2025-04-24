use ui_library::{ElementConfig, LayoutContext, SizingConfig};

pub fn main() {
    let mut layout_context = LayoutContext::create_context();

    layout_context.begin_layout();
    layout_context.add_element(
        ElementConfig {
            width: SizingConfig::fit(),
            height: SizingConfig::fit(),
            ..Default::default()
        },
        Some(|layout_context| {
            layout_context.add_element(ElementConfig {
                width: SizingConfig::fixed(100.),
                height: SizingConfig::fixed(50.),
                ..Default::default()
            }, None);

            layout_context.add_element(ElementConfig {
                width: SizingConfig::fixed(150.),
                height: SizingConfig::fixed(100.),
                ..Default::default()
            }, None);
        }),
    );
    layout_context.end_layout();
}
