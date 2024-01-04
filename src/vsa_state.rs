use crate::synth::vsa::*;
use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::egui::{self, Id, Context, Area, Rect};

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
        let area = Area::new("vsa").id(id).default_pos(vec2pos(pos));

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
                self.area.show(egui_ctx, |ui| {
                    ui.label("Unlearned");
                    ui.label(format!("{} -> {}", start, goal));
                });
            }
        }
    }

    pub fn rect(&self, egui_ctx: &Context) -> Option<Rect> {
        egui_ctx.memory(|mem| {
            mem.area_rect(self.id())
        })
    }

    pub fn subtree_rect(&self, egui_ctx: &Context) -> Option<Rect> {
        self.children.iter().fold(self.rect(egui_ctx), |rect, child| {
            let child_rect = child.subtree_rect(egui_ctx);
            rect.zip(child_rect).map(|(rect, child_rect)| rect.union(child_rect))
        })
    }

    pub fn repel_children(&mut self, egui_ctx: &Context) {
        if self.subtree_rect(egui_ctx).is_none() {
            return;
        }

        for i in 0..self.children.len() {
            self.children[i].repel_children(egui_ctx);

            // possibly only look at adjacent children
            let mut x_force = 0.0;
            let i_rect = self.children[i].subtree_rect(egui_ctx).unwrap();
            for j in (i+1)..self.children.len() {
                let j_rect = self.children[j].subtree_rect(egui_ctx).unwrap();
                let x_dist = i_rect.center().x - j_rect.center().x;
                if i_rect.expand(10.0).intersects(j_rect) {
                    // repel
                    x_force += x_dist.signum() * 0.1;
                }
            }

            let old_pos = i_rect.left_top() + egui::Vec2::new(10.0 * i as f32, 0.0);
            let new_area = self.children[i].area.current_pos(egui::Pos2::new(old_pos.x + x_force, old_pos.y));
            self.children[i].area = new_area;
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
