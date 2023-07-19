#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{
    egui,
    egui::{InnerResponse, Painter, Vec2},
    emath::Align2,
    epaint::{Color32, Pos2, Rect, Rounding},
};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(520.0, 440.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    position: Vec2,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("My egui Application");
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Click each year").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));

            let inner_response = egui::Area::new("my_area")
                // .fixed_pos(egui::pos2(32.0, 32.0))
                .show(ctx, |ui| {
                    // let rect = Rect {
                    //     min: Pos2::ZERO,
                    //     max: Pos2::new(100.0, 150.0),
                    // };
                    // ui.painter().rect_filled(
                    //     rect.translate(self.dragged_delta),
                    //     Rounding::none(),
                    //     Color32::LIGHT_GRAY,
                    // );

                    draw_window_ui(ui, &mut self.position, |ui| {
                        ui.label("Floating text suhidhqihsdhiudhi!")
                    });
                });

            // self.dragged_delta += dbg!(inner_response.response.drag_delta());
        });
    }
}

pub fn draw_window_ui<R>(
    ui: &mut egui::Ui,
    position: &mut Vec2,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    // Widget code can be broken up in four steps:
    //  1. Decide a size for the widget
    //  2. Allocate space for it
    //  3. Handle interactions with the widget (if any)
    //  4. Paint the widget

    // 1. Deciding widget size:
    // You can query the `ui` how much space is available,
    // but in this example we have a fixed size widget based on the height of a standard button:
    let desired_size = ui.spacing().interact_size.y * egui::vec2(20.0, 25.0);

    // 2. Allocating space:
    // This is where we get a region of the screen assigned.
    // We also tell the Ui to sense clicks in the allocated region.
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::drag());

    if response.dragged() {
        *position += response.drag_delta();
    }

    if response.drag_released() {
        let Vec2 { x, y } = *position;
        *position = Vec2 {
            x: round_every(x, 25.0),
            y: round_every(y, 25.0),
        };
    }

    // 4. Paint!
    // Make sure we need to paint:
    if ui.is_rect_visible(rect) {
        // We will follow the current style by asking
        // "how should something that is being interacted with be painted?".
        // This will, for instance, give us different colors when the widget is hovered or clicked.
        let visuals = ui.style().noninteractive();
        // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        let rect = rect.expand(visuals.expansion);
        ui.painter().rect(
            rect.translate(*position),
            Rounding::none(),
            visuals.bg_fill,
            visuals.bg_stroke,
        );
        // // Paint the circle, animating it from left to right with `how_on`:
        // let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        // let center = egui::pos2(circle_x, rect.center().y);
        // ui.painter()
        //     .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    let inner = add_contents(ui);
    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    InnerResponse { inner, response }
}

fn round_every(val: f32, round: f32) -> f32 {
    let top = val + round;
    let bot = (val - round).abs();
    if top < bot {
        val + round
    } else {
        val - round
    }
}
