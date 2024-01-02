use crate::synth::vsa::*;
use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::egui;

use std::rc::Rc;

pub struct RichVSA {
    pub vsa: VSA<Lit, Fun>,
    pub input: Lit,
    pub goal: Lit,
    pub area: egui::Area,
    pub collapsed: bool,
    pub children: Vec<RichVSA>,
}

impl RichVSA {
    pub fn new(vsa: Rc<VSA<Lit, Fun>>, input: Lit, goal: Lit, pos: Vec2) -> Self {
        let children = match vsa.as_ref() {
            VSA::Leaf(_) | VSA::Unlearned { .. } => Vec::new(),
            VSA::Union(vsas) => {
                vsas.into_iter().map(|vsa| {
                    // TODO: choose good pos
                    RichVSA::new(vsa.clone(), input.clone(), goal.clone(), pos)
                }).collect()
            }
            VSA::Join { op, children, children_goals } => {
                children.iter().zip(children_goals.iter()).map(|(vsa, goal)| {
                    RichVSA::new(vsa.clone(), input.clone(), goal.clone(), pos)
                }).collect()
            }
        };

        Self {
            vsa,
            input,
            area,
            collapsed: false,
            children: Vec::new(),
        }
    }
}
