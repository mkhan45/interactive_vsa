use egui_macroquad::macroquad;
use macroquad::prelude::*;

mod synth;
mod draw;
mod main_state;

use main_state::VSAState;
use synth::vsa::{VSA, Lit, Fun, AST};
use std::rc::Rc;

#[macroquad::main("Cloth")]
async fn main() -> Result<(), std::io::Error> {
    // use crate::synth::vsa::Lit;
    // let examples = vec![
    //     (
    //         Lit::StringConst("First Last".to_string()),
    //         Lit::StringConst("F L".to_string()),
    //     ),
    //     (
    //         Lit::StringConst("Another Name".to_string()),
    //         Lit::StringConst("A N".to_string()),
    //     ),
    // ];

    // let (vsa, ast) = synth::top_down(&examples);
    // let flat_vsa = crate::synth::vsa::VSA::flatten(std::rc::Rc::new(vsa));
    // println!("{}", ast.unwrap());
    // println!("{:?}", flat_vsa);

    let vsa = {
        let mut set = std::collections::HashSet::new();
        set.insert(Rc::new(AST::Lit(Lit::StringConst("First Last".to_string()))));
        VSA::Union(vec!(Rc::new(VSA::Leaf(set))))
    };
    let mut vsas = Vec::new();
    vsas.push(VSAState {
        vsa: Rc::new(vsa),
        pos: vec2(0.0, 0.0),
        collapsed_nodes: std::collections::HashSet::new(),
    });
    let main_state = main_state::MainState::new(vsas);
    loop {
        next_frame().await;
        main_state.draw();
    }
}
