use crate::synth::vsa::*;
use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::egui;
use std::rc::Rc;

use crate::draw::draw_vsa;

pub struct VSAState {
    pub vsa: Rc<VSA<Lit, Fun>>,
    pub collapsed: bool,
    pub input: Lit,
    pub area: egui::Area,
}

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
}

pub struct MainState {
    pub vsas: Vec<VSAState>,
    pub camera: Camera,
    pub egui_ctx: egui::Context,
}

impl MainState {
    pub fn new(vsas: Vec<VSAState>) -> Self {
        Self {
            vsas,
            camera: Camera {
                pos: vec2(0.0, 0.0),
                zoom: 1.0,
            },
            egui_ctx: egui::Context::default(),
        }
    }

    pub fn update(&mut self) {
    }

    pub fn draw(&self) {
        egui_macroquad::ui(|egui_ctx| {
            clear_background(BLACK);
            for vsa in &self.vsas {
                draw_vsa(vsa.vsa.clone(), Vec2::new(100.0, 100.0), &vsa.input, None, egui_ctx);
            }
        });
        egui_macroquad::draw();
    }
}
