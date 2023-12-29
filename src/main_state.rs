use crate::synth::vsa::*;
use egui_macroquad::macroquad::prelude::*;
use std::rc::Rc;

use std::collections::HashSet;

pub struct VSAState {
    vsa: Rc<VSA<Lit, Fun>>,
    pos: Vec2,
    collapsed_nodes: HashSet<Rc<VSA<Lit, Fun>>>,
}

pub struct Camera {
    pos: Vec2,
    zoom: f32,
}

pub struct MainState {
    vsas: Vec<VSAState>,
    camera: Camera,
}
