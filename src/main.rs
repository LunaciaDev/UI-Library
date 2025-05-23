use std::rc::Rc;

use macroquad::prelude::*;
use ui_library::{
    ElementConfig, LayoutContext, SizingConfig, TextConfig, TextMeasurement,
};

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

fn measurement_fn(text: &str, font_id: u32, font_size: u16) -> TextMeasurement {
    let _ = font_id;
    let measure_res = measure_text(text, None, font_size, 1.);
    TextMeasurement {
        width: measure_res.width,
        height: measure_res.height,
        x_offset: 0.,
        y_offset: measure_res.offset_y,
    }
}

#[macroquad::main(window_config)]
pub async fn main() {
    let mut layout_context = LayoutContext::create_context(1280., 720.);
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque rhoncus nunc semper, porttitor quam vel, accumsan nunc. Nullam justo nisi, rutrum ac metus tincidunt, fringilla accumsan tortor. Vestibulum fringilla nisl ex, et pharetra erat feugiat eget. Maecenas eget porttitor neque. In eu purus consequat, pellentesque mauris vitae, consequat dui. Maecenas quis vulputate mi, non consequat turpis. Vivamus sed velit sit amet neque pellentesque auctor a eget lacus. Sed ultricies, erat eu efficitur fermentum, ex dolor blandit nibh, vitae mattis ligula enim ac tortor. Donec semper auctor commodo. Duis ac sem gravida, pharetra nibh at, laoreet justo. Fusce quis aliquet nisl, sed elementum ante.

Aliquam gravida mi a neque pretium, et maximus lacus aliquet. Nunc turpis nisi, volutpat in finibus nec, ullamcorper id quam. Nulla vel sem non odio porta consequat non vitae mi. Donec in eleifend arcu, a gravida nisl. Nam ut lorem eu neque auctor convallis. Integer pulvinar ex nec finibus mollis. Nam fringilla mollis elit nec semper. Nullam a dolor non nisl eleifend viverra in in dui. Nam convallis a orci eget mollis. Nam erat magna, fringilla ut ultrices vitae, dictum vitae massa. Nam vitae metus at ipsum porta suscipit non eget enim. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi consectetur nisl sodales molestie facilisis. Donec id enim commodo, condimentum ex at, pellentesque purus. Sed ipsum tortor, egestas non cursus sed, congue sit amet magna.

Duis vel vehicula ante, vitae scelerisque nunc. In semper, sem vel dignissim aliquam, massa purus egestas metus, vitae tincidunt libero quam ac massa. Praesent commodo, purus a elementum iaculis, elit massa auctor dui, ut interdum ipsum urna eget sapien. Aliquam at enim vel nibh egestas vestibulum. Etiam vel feugiat urna, ac semper nulla. Sed ac metus dapibus, aliquet augue vitae, varius nisl. Quisque facilisis rutrum dolor quis dapibus. Morbi sed dapibus odio, in hendrerit erat. Nulla eleifend scelerisque varius. Mauris sit amet lobortis neque, sed aliquam nulla. Suspendisse interdum porta odio, quis finibus eros condimentum eu. Proin laoreet sit amet arcu at consequat. Etiam ut pharetra nulla. Nulla ac sapien eget dui pellentesque placerat.";
    layout_context.set_measurement_fn(measurement_fn);

    let mut width = 50.;
    let mut delta = 1.;

    loop {
        // act
        width += delta;

        if !(50.0..=800.0).contains(&width) {
            delta *= -1.;
        }

        // layout
        clear_background(WHITE);

        layout_context.begin_layout();
        layout_context.add_text(
            text,
            None,
            Rc::new(TextConfig {
                font_id: 0,
                font_size: 16,
                color: ui_library::Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                },
                break_word: true,
            }),
        );
        layout_context.add_element(
            ElementConfig::new(ElementConfig {
                width: SizingConfig::fixed(width),
                height: SizingConfig::grow(),
                color: ui_library::Color {
                    r: 254,
                    g: 0,
                    b: 0,
                    a: 255,
                },
                ..Default::default()
            }),
            |_| {},
        );

        let render_commands = layout_context.end_layout();

        // draw
        for render_command in render_commands {
            if render_command.text_config.is_some() {
                let text_config = render_command.text_config.expect("text_config");
                let text = render_command.text.expect("text");

                draw_text(
                    &text,
                    render_command.position.x,
                    render_command.position.y,
                    text_config.font_size as f32,
                    Color::from_rgba(
                        text_config.color.r,
                        text_config.color.g,
                        text_config.color.b,
                        text_config.color.a,
                    ),
                );
            }

            draw_rectangle(
                render_command.position.x,
                render_command.position.y,
                render_command.dimension.width,
                render_command.dimension.height,
                Color::from_rgba(
                    render_command.color.r,
                    render_command.color.g,
                    render_command.color.b,
                    render_command.color.a,
                ),
            );
        }

        next_frame().await
    }
}

// multiline printing: The base position is y pos - offset, then add font-size for each line
