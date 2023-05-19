use crate::synth::vsa::*;

trait ToHtml {
    fn to_html(&self) -> String;
}

impl<L, F> ToHtml for VSA<L, F>
where
    L: Clone + Eq + std::hash::Hash + std::fmt::Debug + InputLit + pyo3::ToPyObject,
    F: Language<L> + std::hash::Hash + std::fmt::Debug + Copy,
    AST<L, F>: std::fmt::Display,
{
    fn to_html(&self) -> String {
        match self {
            VSA::Leaf(set) => {
                let mut s = String::new();
                s.push_str("<div class=\"leaf\">");
                for l in set {
                    s.push_str(&format!("<span class=\"lit\">{}</span>", l.clone()));
                }
                s.push_str("</div>");
                s
            }
            VSA::Union(vsas) => {
                let mut s = String::new();
                s.push_str("<div class=\"union\">");
                for vsa in vsas {
                    s.push_str(&vsa.to_html());
                }
                s.push_str("</div>");
                s
            }
            VSA::Join { op, children } => {
                let mut s = String::new();
                s.push_str("<div class=\"join\">");
                s.push_str("<span class=\"op\">");
                s.push_str(&format!("{:?}", op));
                s.push_str("</span>");
                for child in children {
                    s.push_str(&child.to_html());
                }
                s.push_str("</div>");
                s
            }
            VSA::Unlearned { goal } => {
                let mut s = String::new();
                s.push_str("<div class=\"unlearned\">");
                s.push_str(&format!("{:?}", goal));
                s.push_str("</div>");
                s
            }
        }
    }
}
