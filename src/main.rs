use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;
use ui_library::{ElementConfig, LayoutContext, PaddingConfig, SizingConfig};

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
            ..Default::default()
        }),
    };

    loop {
        clear_background(BLACK);

        layout_context.begin_layout();
        layout_context.add_element(Rc::clone(&(obj.config)), |layout_context| {
            layout_context.add_element(
                ElementConfig::new(ElementConfig {
                    // bright cyan
                    width: SizingConfig::fixed(100.),
                    height: SizingConfig::fixed(50.),
                    color: ui_library::Color {
                        r: 19,
                        g: 235,
                        b: 247,
                        a: 255,
                    },
                    ..Default::default()
                }),
                |_| {},
            );

            layout_context.add_element(
                ElementConfig::new(ElementConfig {
                    // bright red
                    width: SizingConfig::grow(),
                    height: SizingConfig::fixed(50.),
                    color: ui_library::Color {
                        r: 247,
                        g: 38,
                        b: 19,
                        a: 255,
                    },
                    gap: 5.,
                    padding: PaddingConfig::same_padding(5.),
                    ..Default::default()
                }), |_| {});

            layout_context.add_element(
                ElementConfig::new(ElementConfig {
                    // yellow
                    width: SizingConfig::fixed(150.),
                    height: SizingConfig::fixed(100.),
                    color: ui_library::Color {
                        r: 247,
                        g: 224,
                        b: 19,
                        a: 255,
                    },
                    ..Default::default()
                }),
                |_| {},
            );
        });

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