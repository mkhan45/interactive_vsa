use pyo3::prelude::*;

mod synth;
mod html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let code = include_str!("../combo_parser.py");
    // Python::with_gil(|py| -> PyResult<()> {
    //     let combo_parser = PyModule::from_code(py, code, "combo_parser.py", "combo_parser")?;
    //     dbg!(combo_parser);
    //     Ok(())
    // })?;

    use crate::synth::vsa::Lit;
    let examples = vec![
        (Lit::StringConst("hello".to_string()), Lit::StringConst("HELLO WORLD".to_string())),
        (Lit::StringConst("abcdef".to_string()), Lit::StringConst("ABCDEF WORLD".to_string())),
    ];

    let res = synth::top_down(&examples).unwrap();
    println!("{}", res);

    Ok(())
}
