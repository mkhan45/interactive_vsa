use crate::synth::vsa::*;

use egui_macroquad::egui::{self, Ui, Painter, Window, Id, Context};
use egui_macroquad::macroquad::math::Vec2;

use std::rc::Rc;

const ARROW_ORDER: egui::layers::Order = egui::layers::Order::Middle;

// potentially changes?
pub fn rc_to_id<T>(rc: Rc<T>) -> Id {
    let ptr = Rc::into_raw(rc);
    Id::new(ptr as usize)
}

pub fn draw_vsa<L, F>(vsa: Rc<VSA<L, F>>, pos: Vec2, parent_id: Option<Id>, ui: &Context)
where
    L: Clone + Eq + std::hash::Hash + std::fmt::Debug + InputLit,
    F: Language<L> + std::hash::Hash + std::fmt::Debug + Eq,
{
    let id = rc_to_id(vsa.clone());
    match vsa.as_ref() {
        VSA::Leaf(asts) => {
            for ast in asts {
                draw_ast(id, ast, pos, parent_id, ui);
            }
        }
        VSA::Union(vsas) => {
            draw_union_root(id, pos, parent_id, ui);
            for vsa in vsas {
                draw_vsa(vsa.clone(), pos, Some(id), ui);
            }
        }
        VSA::Join { op, children } => todo!(),
        VSA::Unlearned { goal } => todo!(),
    }
}

#[inline(always)]
pub fn vec2pos(v: Vec2) -> egui::Pos2 {
    egui::Pos2::new(v.x, v.y)
}

pub fn draw_union_root(id: Id, start_pos: Vec2, parent_id: Option<Id>, ui: &Context) {
    Window::new("AST")
        .id(id)
        .default_pos(vec2pos(start_pos))
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .show(ui, |ui| {
            ui.label(format!("Union"));
        });

    draw_arrow_opt(id, parent_id, ui);
}

pub fn draw_ast<L, F>(id: Id, ast: &AST<L, F>, start_pos: Vec2, parent_id: Option<Id>, ui: &Context)
where
    L: Clone + Eq + std::hash::Hash + std::fmt::Debug + InputLit,
    F: Language<L> + std::hash::Hash + std::fmt::Debug + Eq,
{
    let id_seed = format!("{:?}{:?}", id, ast);
    let ast_id = Id::new(id_seed);

    Window::new("AST")
        .id(ast_id)
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .default_pos(vec2pos(start_pos))
        .show(ui, |ui| {
            ui.label(format!("{:?}", ast));
        });

    draw_arrow_opt(ast_id, parent_id, ui);
}

pub fn draw_arrow_opt(id: Id, end_id: Option<Id>, ui: &Context) {
    if let Some(end_id) = end_id {
        draw_arrow(id, end_id, ui);
    }
}

pub fn draw_arrow(id: Id, end_id: Id, ui: &Context) {
    let painter = ui.layer_painter(egui::layers::LayerId::new(ARROW_ORDER, id));
    let start_pos = id_to_top_pos(id, ui);
    let end_pos = id_to_bot_pos(end_id, ui);
    if let Some((sp, ep)) = start_pos.zip(end_pos) {
        let arrow = egui::epaint::Shape::line_segment(
            [vec2pos(sp), vec2pos(ep)],
            (1.0, egui::Color32::WHITE),
        );
        painter.add(arrow);
    }
}

pub fn id_to_top_pos(id: Id, ui: &Context) -> Option<Vec2> {
    ui.memory(|mem| mem.area_rect(id).map(|area| {
        let half_size = area.width() / 2.0;
        Vec2::new(area.left() + half_size - 0.5, area.top())
    }))
}

pub fn id_to_bot_pos(id: Id, ui: &Context) -> Option<Vec2> {
    ui.memory(|mem| mem.area_rect(id).map(|area| {
        let half_size = area.width() / 2.0;
        Vec2::new(area.left() + half_size - 0.5, area.bottom())
    }))
}
