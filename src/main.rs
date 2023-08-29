use std::io::Write;

mod html;
mod synth;

use std::rc::Rc;

use html::ToHtml;

use askama::Template;

use wasm_bindgen::prelude::*;

#[derive(Template)]
#[template(path = "test.html")]
struct VSATemplate {
    vsa_html: String,
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn learn(id: usize) -> String {
    let (start, goal) = unsafe { 
        let node = *Box::from_raw(id as *mut Rc<synth::VSA>);
        match node.as_ref() {
            synth::VSA::Unlearned { start, goal } => (start.clone(), goal.clone()),
            _ => panic!("Tried to learn a learned node"),
        }
    };


    // build bank
    let mut bank = synth::bank::Bank::new();
    let mut all_cache = std::collections::HashMap::new();
    let mut regex_bank = synth::bank::Bank::new();
    synth::bottom_up(std::iter::once(&start), 5, &mut all_cache, &mut bank, &mut regex_bank, false);
    let mut cache = all_cache.iter().map(|(results, ast)| (results[0].clone(), ast.clone())).collect();
    let new_vsa = synth::learn_to_depth(&start, &goal, &mut cache, &bank, 1);

    web_sys::console::log_1(&format!("vsa: {:?}", new_vsa).into());

    new_vsa.to_html(&start)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let code = include_str!("../combo_parser.py");
    // Python::with_gil(|py| -> PyResult<()> {
    //     let combo_parser = PyModule::from_code(py, code, "combo_parser.py", "combo_parser")?;
    //     dbg!(combo_parser);
    //     Ok(())
    // })?;
    //
    println!("test");

    use crate::synth::vsa::Lit;
    // let examples = vec![
    //     (
    //         Lit::StringConst("Hello".to_string()),
    //         Lit::StringConst("Hello World".to_string()),
    //     ),
    //     (
    //         Lit::StringConst("Goodbye".to_string()),
    //         Lit::StringConst("Goodbye World".to_string()),
    //     ),
    // ];

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let vsa = crate::synth::VSA::Unlearned { 
        start: Lit::StringConst("First Last".to_string()),
        goal: Lit::StringConst("F.L.".to_string()),
    };
    // let (vsa, ast) = synth::top_down(&examples);
    // let flat_vsa = crate::synth::vsa::VSA::flatten(std::rc::Rc::new(vsa));
    // println!("{}", ast.unwrap());
    // println!("{:?}", flat_vsa);
    // let template = VSATemplate {
    //     vsa_html: std::rc::Rc::new(vsa).to_html(&Lit::StringConst("Hello".to_string())),
    // };
    // let file = std::fs::File::create("docs/vsa.html")?;
    // let mut writer = std::io::BufWriter::new(file);
    // writer.write_all(template.render().unwrap().as_bytes())?;

    // let root = document.get_element_by_id("root").unwrap();
    let root = document.get_element_by_id("root").unwrap();
    root.set_inner_html(&Rc::new(vsa).to_html(&Lit::StringConst("First Last".to_string())));

    Ok(())
}
