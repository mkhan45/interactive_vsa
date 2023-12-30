use crate::synth::vsa::*;
use egui_macroquad::macroquad::prelude::*;
use std::rc::Rc;

use std::collections::HashSet;

use crate::draw::draw_vsa;

pub struct VSAState {
    pub vsa: Rc<VSA<Lit, Fun>>,
    pub pos: Vec2,
    pub collapsed_nodes: HashSet<Rc<VSA<Lit, Fun>>>,
}

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
}

pub struct MainState {
    pub vsas: Vec<VSAState>,
    pub camera: Camera,
}

impl MainState {
    pub fn new(vsas: Vec<VSAState>) -> Self {
        Self {
            vsas,
            camera: Camera {
                pos: vec2(0.0, 0.0),
                zoom: 1.0,
            },
        }
    }

    pub fn draw(&self) {
        egui_macroquad::ui(|egui_ctx| {
            clear_background(BLACK);
            for vsa in &self.vsas {
                draw_vsa(&vsa.vsa, Some(vsa.pos), egui_ctx);
            }
        });
        egui_macroquad::draw();
    }
}
