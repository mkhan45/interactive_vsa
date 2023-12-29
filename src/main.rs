use std::io::Write;

use egui_macroquad::macroquad;
use macroquad::prelude::*;

mod synth;
mod draw;
mod main_state;

#[macroquad::main("Cloth")]
async fn main() -> Result<(), std::io::Error> {
    // let code = include_str!("../combo_parser.py");
    // Python::with_gil(|py| -> PyResult<()> {
    //     let combo_parser = PyModule::from_code(py, code, "combo_parser.py", "combo_parser")?;
    //     dbg!(combo_parser);
    //     Ok(())
    // })?;

    use crate::synth::vsa::Lit;
    let examples = vec![
        (
            Lit::StringConst("First Last".to_string()),
            Lit::StringConst("F L".to_string()),
        ),
        (
            Lit::StringConst("Another Name".to_string()),
            Lit::StringConst("A N".to_string()),
        ),
    ];

    let (vsa, ast) = synth::top_down(&examples);
    let flat_vsa = crate::synth::vsa::VSA::flatten(std::rc::Rc::new(vsa));
    println!("{}", ast.unwrap());
    println!("{:?}", flat_vsa);

    loop {
        next_frame().await;
    }
}
