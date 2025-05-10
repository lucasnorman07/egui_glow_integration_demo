use std::time::{Duration, Instant};

use super::Viewport;
use egui::{Align, CornerRadius, Layout};
use winit::window::Window;

#[derive(PartialEq)]
enum Choice {
    First,
    Second,
}

pub struct GuiExample {
    choice: Choice,
    translate: (f32, f32, f32),
    rotate: (f32, f32, f32),
    scale: (f32, f32, f32),
    viewport: Option<Viewport>,
    frame_count: u32,
    accumulator: Duration,
    last_frame_time: Instant,
    fps: u32,
}

impl GuiExample {
    pub fn new() -> Self {
        Self {
            choice: Choice::First,
            translate: (0.0, 0.0, -5.0),
            rotate: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            viewport: None,
            frame_count: 0,
            accumulator: Duration::ZERO,
            last_frame_time: Instant::now(),
            fps: 0,
        }
    }

    pub fn get_viewport(&self, window: &Window) -> Option<Viewport> {
        if let Some(viewport) = &self.viewport {
            let window_height = window.inner_size().height;
            Some(Viewport::new(
                viewport.x,
                // Reverse the y since OpenGL uses a different origin
                window_height as i32 - viewport.y - viewport.height,
                viewport.width,
                viewport.height,
            ))
        } else {
            None
        }
    }

    pub fn get_translate(&self) -> (f32, f32, f32) {
        self.translate
    }

    pub fn get_rotate(&self) -> (f32, f32, f32) {
        self.rotate
    }

    pub fn get_scale(&self) -> (f32, f32, f32) {
        self.scale
    }

    pub fn update(&mut self, raw_input: egui::RawInput, ctx: &egui::Context) -> egui::FullOutput {
        // Calculate the delta time
        let now = Instant::now();
        let dt = now - self.last_frame_time;
        self.last_frame_time = now;

        // Update the time accumulator and frame count
        self.accumulator += dt;
        self.frame_count += 1;

        // If 0.1 seconds have passed then update the fps indicator
        if self.accumulator >= Duration::from_secs_f32(0.1) {
            self.fps = (self.frame_count as f32 / self.accumulator.as_secs_f32()) as u32;
            self.accumulator = Duration::ZERO;
            self.frame_count = 0;
        }

        ctx.run(raw_input, |ctx| {
            egui::SidePanel::left("Hierarchy")
                .min_width(150.0)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.collapsing("Main scene", |ui| {
                        ui.label("Cube");
                        ui.collapsing("Game Object 1", |ui| {
                            ui.label("Sphere");
                        });
                        ui.collapsing("Game Object 2", |ui| {
                            ui.label("Point Light");
                            ui.label("Cylinder");
                        });
                    });
                });

            egui::TopBottomPanel::bottom("Bottom panel")
                .min_height(105.0)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.visuals_mut().widgets.inactive.corner_radius = CornerRadius::same(0);
                        ui.visuals_mut().widgets.hovered.corner_radius = CornerRadius::same(5);
                        ui.visuals_mut().widgets.active.corner_radius = CornerRadius::same(5);
                        ui.selectable_value(&mut self.choice, Choice::First, "Console");
                        ui.selectable_value(&mut self.choice, Choice::Second, "Content Browser");
                    });

                    ui.separator();

                    if self.choice == Choice::First {
                        ui.label("Lorem ipsum dolor sit amet consectetur adipisicing elit.");
                        ui.label("Perspiciatis maxime nostrum fuga dolorem vel ipsa ut debitis");
                        ui.label("est delectus asperiores enim earum dignissimos dicta distinctio");
                        ui.label("ratione, culpa nesciunt consequatur quasi.");
                    } else {
                        ui.heading("Content Browser");
                    }

                    // To allow for resizing
                    ui.allocate_space(ui.available_size());
                });

            egui::SidePanel::right("Properties")
                .min_width(220.0)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.heading("Transform");
                    ui.horizontal(|ui| {
                        ui.label("Translate");
                        // Adds space between the text and inputs
                        ui.allocate_ui_with_layout(
                            ui.available_size(),
                            Layout::right_to_left(Align::Center),
                            |ui| {
                                // The inputs are in the reverse order
                                ui.add(egui::DragValue::new(&mut self.translate.2).speed(0.05));
                                ui.add(egui::DragValue::new(&mut self.translate.1).speed(0.05));
                                ui.add(egui::DragValue::new(&mut self.translate.0).speed(0.05));
                            },
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Rotate");
                        // Adds space between the text and inputs
                        ui.allocate_ui_with_layout(
                            ui.available_size(),
                            Layout::right_to_left(Align::Center),
                            |ui| {
                                // The inputs are in the reverse order
                                ui.add(egui::DragValue::new(&mut self.rotate.2).speed(1.0));
                                ui.add(egui::DragValue::new(&mut self.rotate.1).speed(1.0));
                                ui.add(egui::DragValue::new(&mut self.rotate.0).speed(1.0));
                            },
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Scale");
                        // Adds space between the text and inputs
                        ui.allocate_ui_with_layout(
                            ui.available_size(),
                            Layout::right_to_left(Align::Center),
                            |ui| {
                                // The inputs are in the reverse order
                                ui.add(egui::DragValue::new(&mut self.scale.2).speed(0.01));
                                ui.add(egui::DragValue::new(&mut self.scale.1).speed(0.01));
                                ui.add(egui::DragValue::new(&mut self.scale.0).speed(0.01));
                            },
                        );
                    });
                });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Egui Demo Application");
                ui.label(format!("FPS: {}", self.fps));

                let rect = ui.max_rect();
                let (x, y) = rect.min.into();
                let (width, height) = rect.size().into();

                let pixels_per_point = ctx.pixels_per_point();

                // Set the viewport which the custom graphics will render in
                self.viewport = Some(Viewport::new(
                    (x * pixels_per_point) as i32,
                    (y * pixels_per_point) as i32,
                    (width * pixels_per_point) as i32,
                    (height * pixels_per_point) as i32,
                ));
            });
        })
    }
}
