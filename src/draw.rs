use crate::synth::vsa::*;

use egui_macroquad::egui::{Ui, Painter, Window, Id, Context};
use egui_macroquad::macroquad::math::Vec2;

pub fn draw_vsa<L, F>(vsa: &VSA<L, F>, parent_pos: Option<Vec2>, ui: &Context)
where
    L: Clone + Eq + std::hash::Hash + std::fmt::Debug + InputLit,
    F: Language<L> + std::hash::Hash + std::fmt::Debug + Eq,
{
    match vsa {
        VSA::Leaf(asts) => {
            for ast in asts {
                draw_ast(ast, parent_pos, ui);
            }
        }
        VSA::Union(vsas) => todo!(),
        VSA::Join { op, children } => todo!(),
        VSA::Unlearned { goal } => todo!(),
    }
}

pub fn draw_ast<L, F>(ast: &AST<L, F>, parent_pos: Option<Vec2>, ui: &Context)
where
    L: Clone + Eq + std::hash::Hash + std::fmt::Debug + InputLit,
    F: Language<L> + std::hash::Hash + std::fmt::Debug + Eq,
{
    let id_seed = format!("{:?}{:?}", ast, parent_pos);
    Window::new("AST")
        .id(Id::new(id_seed))
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .show(ui, |ui| {
            ui.label(format!("{:?}", ast));
        });
    // match ast {
    //     AST::Lit(lit) => todo!(),
    //     AST::App { fun, args } => todo!(),
    //     AST::JS { code, input, typ } => todo!(),
    // }
}
