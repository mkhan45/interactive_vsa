use crate::synth::vsa::*;
use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::egui;

use crate::vsa_state::*;

const HELP_ORDER: egui::layers::Order = egui::layers::Order::Middle;

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
}

pub struct MainState {
    pub vsas: Vec<RichVSA>,
    pub camera: Camera,
    pub frames_since_last_drag: Option<usize>,
    pub vsa_labels: bool,
}

impl MainState {
    pub fn new(vsas: Vec<RichVSA>) -> Self {
        Self {
            vsas,
            camera: Camera {
                pos: vec2(0.0, 0.0),
                zoom: 1.0,
            },
            frames_since_last_drag: None,
            vsa_labels: false,
        }
    }

    pub fn update(&mut self) {
        egui_macroquad::cfg(|egui_ctx| {
            for vsa in &mut self.vsas {
                vsa.zero_last_move();
            }
            for vsa in &mut self.vsas {
                // TODO: figure out how to disable when dragging
                vsa.repel_children(egui_ctx);

                if egui_ctx.input(|inp| inp.modifiers.command_only()) {
                    vsa.drag_subtrees();
                }
            }
            egui_ctx.input(|inp| {
                if inp.pointer.middle_down() || inp.modifiers.shift {
                    let delta = inp.pointer.delta();
                    for vsa in &mut self.vsas {
                        vsa.move_subtree(vec2(delta.x, delta.y));
                    }
                }
            });

            for vsa in &mut self.vsas {
                vsa.update_subtree(egui_ctx);
            }

            // let scroll = egui_ctx.input(|inp| inp.scroll_delta);
            // if scroll.y != 0.0 {
            //     bad
            //     self.camera.zoom *= 1.0 + scroll.y / 10.0;
            //     egui_ctx.style_mut(|style| {
            //         style.override_font_id = Some(egui::FontId::monospace(16.0 * self.camera.zoom));
            //     });
            // }

            // // let scroll = egui_ctx.input(|inp| inp.scroll_delta);
            // if scroll.y != 0.0 {
            //     let old_zoom = egui_ctx.zoom_factor();
            //     let new_zoom = old_zoom * (1.0 + scroll.y / 10.0);
            //     // crashes
            //     egui_ctx.set_zoom_factor(new_zoom);
            // }
        });
    }

    pub fn draw(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            clear_background(WHITE);

            // egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
            //     egui::gui_zoom::zoom_menu_buttons(ui);
            // });

            for vsa in &mut self.vsas {
                vsa.draw(self.vsa_labels, egui_ctx);
                // draw_vsa(vsa.vsa.clone(), Vec2::new(100.0, 100.0), &vsa.input, None, egui_ctx);
            }

            egui::TopBottomPanel::top("top bar").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    let vsa_label_text = egui::RichText::new("VSA Labels").size(18.0);
                    ui.checkbox(&mut self.vsa_labels, vsa_label_text);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            "Hold middle mouse or shift to drag, hold control click to move a whole subtree",
                        )
                    });
                });
            });
        });
        egui_macroquad::draw();
    }
}
