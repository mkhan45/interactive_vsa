use pyo3::prelude::*;
use std::io::Write;

mod synth;
mod html;

use html::ToHtml;

use askama::Template;

#[derive(Template)]
#[template(path = "test.html")]
struct VSATemplate {
    vsa_html: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let code = include_str!("../combo_parser.py");
    // Python::with_gil(|py| -> PyResult<()> {
    //     let combo_parser = PyModule::from_code(py, code, "combo_parser.py", "combo_parser")?;
    //     dbg!(combo_parser);
    //     Ok(())
    // })?;

    use crate::synth::vsa::Lit;
    let examples = vec![
        (Lit::StringConst("First Last".to_string()), Lit::StringConst("FL".to_string())),
        (Lit::StringConst("Another Name".to_string()), Lit::StringConst("AN".to_string())),
    ];

    let (vsa, ast) = synth::top_down(&examples);
    println!("{}", ast.unwrap());
    println!("{}", vsa.to_html());

    let template = VSATemplate { vsa_html: vsa.to_html() };
    let file = std::fs::File::create("vsa.html")?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(template.render().unwrap().as_bytes())?;

    Ok(())
}
