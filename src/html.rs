use crate::synth::vsa::*;

use std::rc::Rc;

pub trait ToHtml<L, F>
where L: Clone + Eq + std::hash::Hash + std::fmt::Debug + InputLit + pyo3::ToPyObject,
      F: Language<L> + std::hash::Hash + std::fmt::Debug + Copy + std::cmp::Eq,
      AST<L, F>: std::fmt::Display
{
    fn to_html(&self, input: &L) -> String;
}

impl<L, F> ToHtml<L, F> for Rc<VSA<L, F>>
where
    L: Clone + Eq + std::hash::Hash + std::fmt::Debug + InputLit + pyo3::ToPyObject,
    F: Language<L> + std::hash::Hash + std::fmt::Debug + Copy + std::cmp::Eq,
    AST<L, F>: std::fmt::Display,
{
    fn to_html(&self, input: &L) -> String {
        fn to_ptr<T>(t: Rc<T>) -> *const Rc<T> {
            let b = Box::new(t);
            Box::into_raw(b)
        }

        match self.as_ref() {
            _ if self.is_empty() => {
                "".to_string()
            }
            VSA::Leaf(set) => {
                let mut s = String::new();
                s.push_str(format!("<div class=\"leaf box\" id='{}'>", to_ptr(self.clone()) as usize).as_str());
                for l in set {
                    s.push_str(&format!("<span class=\"lit\">{}</span>", l.clone()));
                }
                s.push_str("</div>");
                s
            }
            VSA::Union(vsas) => {
                let mut s = String::new();
                s.push_str("<div class=\"union\">");
                s.push_str("<div class=\"box goal-label\">");
                s.push_str(format!("{:?} → {:?}", input, self.eval(input)).as_str());
                s.push_str("</div>");
                s.push_str("<div class=\"join-children\">");
                for vsa in vsas {
                    s.push_str(&vsa.to_html(input));
                }
                s.pop();
                s.push_str("</div>");
                s.push_str("</div>");
                s
            }
            VSA::Join { op, children } => {
                let mut s = String::new();
                s.push_str("<div class=\"join\">");
                s.push_str("<div class=\"box\">");
                s.push_str("<span class=\"op\">");
                s.push_str(&format!("{:?}", op));
                s.push_str("</span>");
                s.push_str("<div class=\"join-label\">");
                s.push_str(format!("{:?} → {:?}", input, self.eval(input)).as_str());
                s.push_str("</div>");
                s.push_str("</div>");
                s.push_str("<div class=\"join-children\">");
                for child in children {
                    s.push_str(&child.to_html(input));
                }
                s.push_str("</div>");
                s.push_str("</div>");
                s
            }
            VSA::Unlearned { goal } => {
                let mut s = String::new();
                s.push_str("<div class=\"unlearned box\">");
                s.push_str(&format!("{:?}", goal));
                s.push_str("</div>");
                s
            }
        }
    }
}
