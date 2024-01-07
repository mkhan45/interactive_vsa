use crate::synth::vsa::*;
use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::egui::{self, Id, Context, Area, Rect, InnerResponse};

use crate::util::{rc_to_id, vec2pos};

use std::rc::Rc;

const ARROW_ORDER: egui::layers::Order = egui::layers::Order::Middle;

pub struct RichVSA {
    pub vsa: Rc<VSA<Lit, Fun>>,
    pub input: Lit,
    pub goal: Lit,
    pub area: egui::Area,
    pub last_move: Vec2,
    pub collapsed: bool,
    pub children: Vec<RichVSA>,
    pub drag: Option<Vec2>,
}

impl RichVSA {
    pub fn new(vsa: Rc<VSA<Lit, Fun>>, input: Lit, goal: Lit, pos: Vec2) -> Self {
        let y_offs = 120.0;
        let x_offs = 100.0;
        let children = match vsa.as_ref() {
            VSA::Leaf(_) | VSA::Unlearned { .. } => Vec::new(),
            VSA::Union(vsas) => {
                vsas.into_iter().enumerate().map(|(i, vsa)| {
                    // TODO: choose good pos
                    RichVSA::new(vsa.clone(), input.clone(), goal.clone(), pos + Vec2::new(x_offs * i as f32, y_offs))
                }).collect()
            }
            VSA::Join { children, children_goals, .. } => {
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
            last_move: Vec2::ZERO,
            collapsed: false,
            children,
            drag: None,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> Id {
        rc_to_id(self.vsa.clone())
    }

    pub fn set_vsa_style(ui: &mut egui::Ui) {
        // doesnt do anything
        let style = ui.style_mut();
        style.spacing.item_spacing = egui::Vec2::new(10.0, 10.0);
        style.spacing.window_margin = egui::style::Margin::same(10.0);
    }

    pub fn draw(&mut self, egui_ctx: &Context) {
        let learn_pos = self.rect(egui_ctx).map(|r| {
            let egui::Pos2 { x, y } = r.left_top();
            vec2(x, y)
        });
        match self.vsa.as_ref() {
            VSA::Leaf(asts) => {
                self.area.show(egui_ctx, |ui| {
                    Self::set_vsa_style(ui);
                    ui.label(format!("{} -> {}", self.input, self.goal));
                    for ast in asts {
                        ui.label(format!("{}", ast));
                    }
                });
            }
            VSA::Union(_) => {
                let InnerResponse { response, .. } = self.area.show(egui_ctx, |ui| {
                    Self::set_vsa_style(ui);
                    ui.label(format!("{} -> {}", self.input, self.goal));
                });
                let edrag = 
                    response.dragged_by(egui::PointerButton::Primary).then(|| response.drag_delta());
                self.drag = edrag.map(|drag| Vec2::new(drag.x, drag.y));
                let id = self.id();
                for vsa in &mut self.children {
                    vsa.draw(egui_ctx);
                    draw_area_arrows(id, vsa.id(), egui_ctx);
                }
            }
            VSA::Join { op, children_goals, .. } => {
                let InnerResponse { response, .. } = self.area.show(egui_ctx, |ui| {
                    Self::set_vsa_style(ui);
                    ui.label(format!("{} -> {}", self.input, self.goal));

                    let args = children_goals.iter().map(|goal| {
                        format!("{}", goal)
                    }).collect::<Vec<_>>().join(", ");
                    ui.label(format!("{:?}({})", op, args));
                });
                let edrag = 
                    response.dragged_by(egui::PointerButton::Primary).then(|| response.drag_delta());
                self.drag = edrag.map(|drag| Vec2::new(drag.x, drag.y));
                let id = self.id();
                for vsa in self.children.iter_mut() {
                    vsa.draw(egui_ctx);
                    draw_area_arrows(id, vsa.id(), egui_ctx);
                }
            }
            VSA::Unlearned { start, goal } => {
                self.area.show(egui_ctx, |ui| {
                    Self::set_vsa_style(ui);
                    ui.label("Unlearned");
                    ui.label(format!("{} -> {}", start, goal));
                    if ui.button("Learn").clicked() {
                        use std::collections::HashMap;
                        let mut all_cache = HashMap::new();
                        let mut bank = crate::synth::bank::Bank::new();
                        let mut regex_bank = crate::synth::bank::Bank::new();
                        crate::synth::bottom_up(
                            std::iter::once(start),
                            5,
                            &mut all_cache,
                            &mut bank,
                            &mut regex_bank,
                            false
                        );
                        let mut cache = // idr what this does
                            all_cache.iter().map(|(results, ast)| (results[0].clone(), ast.clone())).collect();
                        let new_vsa_rc = crate::synth::learn_to_depth(start, goal, &mut cache, &bank, 1);
                        let new_vsa = Rc::into_inner(new_vsa_rc);
                        let self_mut = Rc::as_ptr(&self.vsa) as *mut _;
                        unsafe { std::ptr::write(self_mut, new_vsa) };
                        let rich_vsa = 
                            RichVSA::new(
                                self.vsa.clone(), self.input.clone(), self.goal.clone(), learn_pos.unwrap()
                            );
                        self.children = rich_vsa.children;
                        // TODO: send a signal and learn to depth
                    }
                });
            }
        }
     
        if let Some(rect) = self.rect(egui_ctx) {
            let painter = egui_ctx.layer_painter(egui::layers::LayerId::new(ARROW_ORDER, self.id()));
            painter.rect_stroke(rect, egui::Rounding::ZERO, egui::Stroke::new(1.0, egui::Color32::BLACK));
        }

        // if let Some(rect) = self.subtree_rect(egui_ctx) {
        //     let painter = egui_ctx.layer_painter(egui::layers::LayerId::new(ARROW_ORDER, self.id()));
        //     painter.rect_stroke(rect, egui::Rounding::ZERO, egui::Stroke::new(1.0, egui::Color32::RED));
        // }
    }

    pub fn rect(&self, egui_ctx: &Context) -> Option<Rect> {
        egui_ctx.memory(|mem| {
            mem.area_rect(self.id()).map(|r| r.expand(10.0))
        })
    }

    pub fn subtree_rect(&self, egui_ctx: &Context) -> Option<Rect> {
        self.children.iter().fold(self.rect(egui_ctx), |rect, child| {
            let child_rect = child.subtree_rect(egui_ctx);
            rect.zip(child_rect).map(|(rect, child_rect)| rect.union(child_rect))
        })
    }

    pub fn updated_rect(&self, egui_ctx: &Context) -> Option<Rect> {
        egui_ctx.memory(|mem| {
            let evec = egui::Vec2::new(self.last_move.x, self.last_move.y);
            mem.area_rect(self.id()).map(|mem| mem.translate(evec))
        })
    }

    pub fn updated_subtree_rect(&self, egui_ctx: &Context) -> Option<Rect> {
        self.children.iter().fold(self.updated_rect(egui_ctx), |rect, child| {
            let child_rect = child.updated_subtree_rect(egui_ctx);
            rect.zip(child_rect).map(|(rect, child_rect)| rect.union(child_rect))
        })
    }

    pub fn drag_subtrees(&mut self) {
        if let Some(drag) = self.drag {
            for child in &mut self.children {
                child.move_subtree(drag);
            }
            self.drag = None;
        } else {
            for child in &mut self.children {
                child.drag_subtrees();
            }
        }
    }

    pub fn repel_children(&mut self, egui_ctx: &Context) {
        if self.updated_subtree_rect(egui_ctx).is_none() {
            return;
        }

        for i in 0..self.children.len() {
            self.children[i].repel_children(egui_ctx);

            // possibly only look at adjacent children
            let mut x_force = 0.0;
            let i_rect = self.children[i].updated_subtree_rect(egui_ctx).unwrap();
            for j in 0..self.children.len() {
                if i == j {
                    continue;
                }
                let j_rect = self.children[j].updated_subtree_rect(egui_ctx).unwrap();
                let x_dist = i_rect.center().x - j_rect.center().x;
                if i_rect.expand(15.0).intersects(j_rect.expand(15.0)) {
                    // repel
                    x_force += x_dist.signum() * 5.0;

                    // let painter = egui_ctx.layer_painter(egui::layers::LayerId::new(ARROW_ORDER, self.id()));
                    // painter.rect_stroke(i_rect, egui::Rounding::ZERO, egui::Stroke::new(1.0, egui::Color32::YELLOW));
                    // painter.rect_stroke(j_rect, egui::Rounding::ZERO, egui::Stroke::new(1.0, egui::Color32::YELLOW));
                }
            }

            // dbg!(x_force);
            // TODO:
            // improved algo:
            //  - find highest child that intersects another subtree, and repel only it
            self.children[i].move_subtree(Vec2::new(x_force, 0.0));
        }
    }

    pub fn zero_last_move(&mut self) {
        self.last_move = Vec2::ZERO;
        for child in &mut self.children {
            child.zero_last_move();
        }
    }

    pub fn move_subtree(&mut self, delta: Vec2) {
        self.last_move += delta;
        for child in &mut self.children {
            child.move_subtree(delta);
        }
    }

    pub fn update_subtree(&mut self, egui_ctx: &Context) {
        if let Some(updated_rect) = self.updated_rect(egui_ctx) {
            let updated_pos = updated_rect.left_top();
            self.area = self.area.current_pos(egui::Pos2::new(updated_pos.x, updated_pos.y));
            for child in &mut self.children {
                child.update_subtree(egui_ctx);
            }
        }
    }
}

fn draw_area_arrows(start_id: Id, end_id: Id, egui_ctx: &Context) {
    // TODO: use .rect()
    let positions = egui_ctx.memory(|mem| {
        let start_rect = mem.area_rect(start_id);
        let end_rect = mem.area_rect(end_id);
        start_rect.zip(end_rect).map(|(start_rect, end_rect)| {
            (start_rect.expand(10.0).center_bottom(), end_rect.expand(10.0).center_top())
        })
    });

    if let Some((sp, ep)) = positions {
        draw_arrow(start_id, sp, ep, egui_ctx);
    }
}

fn draw_arrow(id: Id, sp: egui::Pos2, ep: egui::Pos2, egui_ctx: &Context) {
    let painter = egui_ctx.layer_painter(egui::layers::LayerId::new(ARROW_ORDER, id));
    painter.line_segment([sp, ep], egui::Stroke::new(1.0, egui::Color32::BLACK));
    // painter.arrow(sp, vec, egui::Stroke::new(1.0, egui::Color32::WHITE));
}
