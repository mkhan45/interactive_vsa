use egui_macroquad::macroquad;
use egui_macroquad::egui;
use macroquad::prelude::*;

mod synth;
mod main_state;
mod vsa_state;
mod util;

use synth::vsa::{VSA, Lit, Fun, AST};
use vsa_state::RichVSA;

use std::rc::Rc;

#[macroquad::main("Cloth")]
async fn main() -> Result<(), std::io::Error> {
    // use crate::synth::vsa::Lit;
    let examples = vec![
        (
            Lit::StringConst("I have 17 cookies".to_string()),
            Lit::StringConst("17".to_string()),
        ),
        // (
        //     Lit::StringConst("Give me at least 3 cookies".to_string()),
        //     Lit::StringConst("3".to_string()),
        // ),
        // (
        //     Lit::StringConst("This number is 489".to_string()),
        //     Lit::StringConst("489".to_string()),
        // ),
    ];

    let (vsa, ast) = synth::top_down(&examples);
    let flat_vsa = crate::synth::vsa::VSA::flatten(std::rc::Rc::new(vsa));
    println!("{}", ast.unwrap());
    println!("{:?}", flat_vsa);

    egui_macroquad::cfg(|egui_ctx| {
        egui_ctx.style_mut(|style| {
            style.override_font_id = Some(egui::FontId::monospace(24.0));
            style.wrap = Some(false);
            style.visuals = egui::style::Visuals::light();
        });
    });

    // let vsa = Rc::new({
    //     let mut set = std::collections::HashSet::new();
    //     set.insert(Rc::new(AST::Lit(Lit::StringConst("First Last".to_string()))));
    //     // VSA::Leaf(set)
    //     VSA::Union(vec![
    //                Rc::new(VSA::Leaf(set.clone())),
    //                Rc::new(VSA::Leaf(set))
    //     ])
    // });
    let unlearned_vsa = Rc::new(VSA::<_, Fun>::Unlearned {
        start: Lit::StringConst("First Last".to_string()),
        goal: Lit::StringConst("F.L.".to_string()),
    });
    let mut vsas = Vec::new();
    vsas.push(RichVSA::new(
            unlearned_vsa, 
            Lit::StringConst("First Last".to_string()),
            Lit::StringConst("F.L.".to_string()),
            Vec2::new(screen_width() / 2.0, 100.0),
    ).editable());
    let mut main_state = main_state::MainState::new(vsas);
    loop {
        next_frame().await;
        main_state.draw();
        main_state.update();
    }
}
