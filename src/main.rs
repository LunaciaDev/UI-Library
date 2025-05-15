use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;
use ui_library::{AlignmentConfig, ElementConfig, HorizontalAlignment, LayoutContext, LayoutDirection, PaddingConfig, SizingConfig, VerticalAlignment};

struct ExampleObj {
    pub config: Rc<RefCell<ElementConfig>>,
}

fn window_config() -> Conf {
    Conf {
        window_title: "Test Window".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
pub async fn main() {
    let mut layout_context = LayoutContext::create_context(1280., 720.);

    let obj: ExampleObj = ExampleObj {
        config: ElementConfig::new(ElementConfig {
            // black bg
            width: SizingConfig::percent(0.5),
            height: SizingConfig::fit(),
            color: ui_library::Color {
                r: 2,
                g: 10,
                b: 7,
                a: 255,
            },
            gap: 5.,
            padding: PaddingConfig::same_padding(5.),
            layout_direction: LayoutDirection::TopToBottom,
            ..Default::default()
        }),
    };

    loop {
        clear_background(WHITE);

        layout_context.begin_layout();
        layout_context.add_element(
            ElementConfig::new(ElementConfig {
                width: SizingConfig::grow(),
                height: SizingConfig::grow(),
                child_alignment: AlignmentConfig {
                    align_y: VerticalAlignment::Center,
                    align_x: HorizontalAlignment::Center,
                },
                color: ui_library::Color { r: 0, g: 0, b: 0, a: 255 },
                ..Default::default()
            }),
            |layout_context| {
                layout_context.add_element(
                    ElementConfig::new(ElementConfig {
                        width: SizingConfig::fixed(150.),
                        height: SizingConfig::fixed(150.),
                        color: ui_library::Color { r: 254, g: 1, b: 1, a: 255 },
                        ..Default::default()
                    }),
                    |_| {},
                );
                layout_context.add_element(
                    ElementConfig::new(ElementConfig {
                        width: SizingConfig::fixed(200.),
                        height: SizingConfig::fixed(200.),
                        color: ui_library::Color { r: 1, g: 254, b: 1, a: 255 },
                        ..Default::default()
                    }),
                    |_| {},
                );
            },
        );

        let render_commands = layout_context.end_layout();

        for render_command in render_commands {
            draw_rectangle(
                render_command.position.x,
                render_command.position.y,
                render_command.dimension.width,
                render_command.dimension.height,
                Color::from_rgba(render_command.color.r, render_command.color.g, render_command.color.b, render_command.color.a),
            );
        }

        next_frame().await
    }
}

// multiline printing: The base position is y pos - offset, then add font-size for each line