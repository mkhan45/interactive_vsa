use crate::synth::vsa::*;
use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::egui;

use crate::vsa_state::*;

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
}

pub struct MainState {
    pub vsas: Vec<RichVSA>,
    pub camera: Camera,
    pub frames_since_last_drag: Option<usize>,
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

                if egui_ctx.input(|inp| inp.key_down(egui::Key::Z)) {
                    vsa.drag_subtrees();
                }
            }
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
            clear_background(BLACK);

            // egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
            //     egui::gui_zoom::zoom_menu_buttons(ui);
            // });

            for vsa in &mut self.vsas {
                vsa.draw(egui_ctx);
                // draw_vsa(vsa.vsa.clone(), Vec2::new(100.0, 100.0), &vsa.input, None, egui_ctx);
            }
        });
        egui_macroquad::draw();
    }
}
