use crate::synth::vsa::*;
use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::egui::{self, Id, Context, Area};

use crate::util::{rc_to_id, vec2pos};

use std::rc::Rc;

const ARROW_ORDER: egui::layers::Order = egui::layers::Order::Middle;

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
                let y_offs = 100.0;
                let x_offs = 100.0;
                vsas.into_iter().enumerate().map(|(i, vsa)| {
                    // TODO: choose good pos
                    RichVSA::new(vsa.clone(), input.clone(), goal.clone(), pos + Vec2::new(x_offs * i as f32, y_offs))
                }).collect()
            }
            VSA::Join { children, children_goals, .. } => {
                let y_offs = 100.0;
                let x_offs = 100.0;
                children.iter().zip(children_goals.iter()).enumerate().map(|(i, (vsa, goal))| {
                    RichVSA::new(vsa.clone(), input.clone(), goal.clone(), pos + Vec2::new(x_offs * i as f32, y_offs))
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
            children,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> Id {
        rc_to_id(self.vsa.clone())
    }

    pub fn draw(&self, egui_ctx: &Context) {
        // TODO:
        // 1. replace pos with area
        // 2. replace inp with input
        match self.vsa.as_ref() {
            VSA::Leaf(asts) => {
                self.area.show(egui_ctx, |ui| {
                    ui.label(format!("{} -> {}", self.input, self.goal));
                    for ast in asts {
                        ui.label(format!("{}", ast));
                    }
                });
            }
            VSA::Union(_) => {
                self.area.show(egui_ctx, |ui| {
                    ui.label("Union");
                    ui.label(format!("{} -> {}", self.input, self.goal));
                });
                for vsa in &self.children {
                    vsa.draw(egui_ctx);
                    draw_area_arrows(self.id(), vsa.id(), egui_ctx);
                }
            }
            VSA::Join { op, children_goals, .. } => {
                self.area.show(egui_ctx, |ui| {
                    ui.label("Join");
                    ui.label(format!("{} -> {}", self.input, self.goal));

                    let args = children_goals.iter().map(|goal| {
                        format!("{}", goal)
                    }).collect::<Vec<_>>().join(", ");
                    ui.label(format!("{:?}({})", op, args));
                });
                for vsa in self.children.iter() {
                    vsa.draw(egui_ctx);
                    draw_area_arrows(self.id(), vsa.id(), egui_ctx);
                }
            }
            VSA::Unlearned { start, goal } => {

            }
        }
    }
}

fn draw_area_arrows(start_id: Id, end_id: Id, egui_ctx: &Context) {
    let positions = egui_ctx.memory(|mem| {
        let start_rect = mem.area_rect(start_id);
        let end_rect = mem.area_rect(end_id);
        start_rect.zip(end_rect).map(|(start_rect, end_rect)| {
            (start_rect.center_bottom(), end_rect.center_top())
        })
    });

    if let Some((sp, ep)) = positions {
        draw_arrow(start_id, sp, ep, egui_ctx);
    }
}

fn draw_arrow(id: Id, sp: egui::Pos2, ep: egui::Pos2, egui_ctx: &Context) {
    let painter = egui_ctx.layer_painter(egui::layers::LayerId::new(ARROW_ORDER, id));
    painter.line_segment([sp, ep], egui::Stroke::new(1.0, egui::Color32::WHITE));
    // painter.arrow(sp, vec, egui::Stroke::new(1.0, egui::Color32::WHITE));
}
