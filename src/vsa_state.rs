use crate::synth::vsa::*;
use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::egui::{self, Id, Context, Area};

use crate::util::{rc_to_id, vec2pos};

use std::rc::Rc;

pub struct RichVSA {
    pub vsa: Rc<VSA<Lit, Fun>>,
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

        let id = rc_to_id(vsa.clone());
        let area = Area::new(id).default_pos(vec2pos(pos));

        Self {
            vsa,
            input,
            goal,
            area,
            collapsed: false,
            children: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn id(&self) -> Id {
        rc_to_id(self.vsa.clone())
    }

    pub fn draw(&self, parent_id: Option<Id>, egui_ctx: &Context) {
        // TODO:
        // 1. replace pos with area
        // 2. replace inp with input
        let id = self.id();
        match self.vsa.as_ref() {
            VSA::Leaf(asts) => {
                // TODO: put all ASTS in one window
                // for ast in asts {
                //     draw_ast(ast, pos, parent_id, ui);
                // }
            }
            VSA::Union(vsas) => {
                todo!()
            }
            VSA::Join { op, children, children_goals } => {
                todo!()
            }
            VSA::Unlearned { start, goal } => {

            }
        }
    }
}
