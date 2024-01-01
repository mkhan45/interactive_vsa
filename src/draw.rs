use crate::synth::vsa::*;

use egui_macroquad::egui::{self, Ui, Painter, Window, Id, Context};
use egui_macroquad::macroquad::math::{Vec2, vec2};

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
                draw_ast( ast, pos, parent_id, ui);
            }
        }
        VSA::Union(vsas) => {
            draw_union_root(id, pos, parent_id, ui);
            let y_offs = 60.0;
            if vsas.len() > 1 {
                let x_offs = {
                    let single_offs = &20.0; // can't copy f32 otherwise???
                    let n = vsas.len() as i32;
                    ((-n/2)..(n/2)).map(|i| (i as f32) * *single_offs)
                };
                for (vsa, x) in vsas.iter().zip(x_offs) {
                    draw_vsa(vsa.clone(), pos + vec2(x, y_offs), Some(id), ui);
                }
            } else /* vsas.len() == 1 */ {
                let vsa = &vsas[0];
                draw_vsa(vsa.clone(), pos + vec2(0.0, y_offs), Some(id), ui);
            }
        }
        VSA::Join { op, children } => {
            // TODO: need input to eval children args
            draw_join_root(id, op.clone(), pos, parent_id, ui);
            todo!()
        }
        VSA::Unlearned { start, goal } => {
            
        }
    }
}

#[inline(always)]
pub fn vec2pos(v: Vec2) -> egui::Pos2 {
    egui::Pos2::new(v.x, v.y)
}

pub fn floating_window(title: &str, id: Id, start_pos: Vec2) -> Window {
    Window::new(title)
        .id(id)
        .default_pos(vec2pos(start_pos))
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
}

pub fn draw_union_root(id: Id, start_pos: Vec2, parent_id: Option<Id>, ui: &Context) {
    floating_window("Union", id, start_pos).show(ui, |ui| ui.label("Union"));
    draw_arrow_opt(id, parent_id, ui);
}

pub fn draw_join_root<L, F>(id: Id, op: &F, start_pos: Vec2, parent_id: Option<Id>, ui: &Context) 
where
    L: Clone + Eq + std::hash::Hash + std::fmt::Debug + InputLit,
    F: Language<L> + std::hash::Hash + std::fmt::Debug + Eq,
{
    floating_window("Join", id, start_pos).show(ui, |ui| ui.label(format!("{:?}", op)));
    draw_arrow_opt(id, parent_id, ui);
}

pub fn draw_ast<L, F>(ast: &AST<L, F>, start_pos: Vec2, parent_id: Option<Id>, ui: &Context)
where
    L: Clone + Eq + std::hash::Hash + std::fmt::Debug + InputLit,
    F: Language<L> + std::hash::Hash + std::fmt::Debug + Eq,
{
    let id_seed = format!("{:?}{:?}", parent_id, ast);
    let ast_id = Id::new(id_seed);

    floating_window("AST", ast_id, start_pos).show(ui, |ui| ui.label(format!("{:?}", ast)));
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
