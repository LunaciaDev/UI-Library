use macroquad::prelude::*;
use ui_library::{
    ElementConfig, FixedDimensionConfig, GrowDimensionConfig, LayoutContext, TextConfig,
    TextMeasurement,
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
        clear_background(BLACK);

        layout_context.begin_layout();
        layout_context.add_text(
            text,
            TextConfig {
                width: ui_library::DimensionConfig::Grow(GrowDimensionConfig {
                    min_size: 0.,
                    max_size: 0.,
                }),
                height: ui_library::DimensionConfig::Grow(GrowDimensionConfig {
                    min_size: 0.,
                    max_size: 0.,
                }),
                font_id: 0,
                font_size: 16,
                font_color: ui_library::Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
                break_word: true,
            },
        );
        layout_context.add_element(
            ElementConfig::new(ElementConfig {
                width: ui_library::DimensionConfig::Fixed(FixedDimensionConfig { size: width }),
                height: ui_library::DimensionConfig::Grow(GrowDimensionConfig {
                    min_size: 0.,
                    max_size: 0.,
                }),
                color: ui_library::Color {
                    r: 254,
                    g: 0,
                    b: 0,
                    a: 255,
                },
                ..Default::default()
            }),
            |_| {}
        );

        let render_commands = layout_context.end_layout();

        // draw
        for render_command in render_commands {
            match render_command.render_data {
                ui_library::RenderData::Text(text_render_data) => {
                    draw_text(
                        &text_render_data.text,
                        render_command.position.x,
                        render_command.position.y,
                        text_render_data.font_size as f32,
                        Color::from_rgba(
                            text_render_data.font_color.r,
                            text_render_data.font_color.g,
                            text_render_data.font_color.b,
                            text_render_data.font_color.a,
                        ),
                    );
                }
                ui_library::RenderData::Rectangle(rectangle_render_data) => {
                    draw_rectangle(
                        render_command.position.x,
                        render_command.position.y,
                        rectangle_render_data.dimenions.width,
                        rectangle_render_data.dimenions.height,
                        Color::from_rgba(
                            rectangle_render_data.color.r,
                            rectangle_render_data.color.g,
                            rectangle_render_data.color.b,
                            rectangle_render_data.color.a,
                        ),
                    );
                }
                _ => {}
            }
        }

        next_frame().await
    }
}

// multiline printing: The base position is y pos - offset, then add font-size for each line
