use std::io::Write;

mod html;
mod synth;

use html::ToHtml;

use askama::Template;

#[derive(Template)]
#[template(path = "test.html")]
struct VSATemplate {
    vsa_html: String,
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
        (
            Lit::StringConst("Hello".to_string()),
            Lit::StringConst("Hello World".to_string()),
        ),
        (
            Lit::StringConst("Goodbye".to_string()),
            Lit::StringConst("Goodbye World".to_string()),
        ),
    ];

    let vsa = crate::synth::VSA::Unlearned { 
        start: Lit::StringConst("First Last".to_string()),
        goal: Lit::StringConst("First".to_string()),
    };
    // let (vsa, ast) = synth::top_down(&examples);
    // let flat_vsa = crate::synth::vsa::VSA::flatten(std::rc::Rc::new(vsa));
    // println!("{}", ast.unwrap());
    // println!("{:?}", flat_vsa);

    let template = VSATemplate {
        vsa_html: std::rc::Rc::new(vsa).to_html(&Lit::StringConst("Hello".to_string())),
    };
    let file = std::fs::File::create("docs/vsa.html")?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(template.render().unwrap().as_bytes())?;

    Ok(())
}
