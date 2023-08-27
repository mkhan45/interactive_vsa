use crate::synth::vsa::*;
use std::rc::Rc;

pub trait ToHtml<L, F>
where
    L: Clone + Eq + std::hash::Hash + std::fmt::Debug + InputLit,
    F: Language<L> + std::hash::Hash + std::fmt::Debug + Copy + std::cmp::Eq,
    AST<L, F>: std::fmt::Display,
{
    fn to_html(&self, input: &L) -> String;
}

impl<L, F> ToHtml<L, F> for Rc<VSA<L, F>>
where
    L: Clone + Eq + std::hash::Hash + std::fmt::Debug + InputLit,
    F: Language<L> + std::hash::Hash + std::fmt::Debug + Copy + std::cmp::Eq,
    AST<L, F>: std::fmt::Display,
{
    fn to_html(&self, input: &L) -> String {
        fn to_ptr<T>(t: Rc<T>) -> *const Rc<T> {
            let b = Box::new(t);
            Box::into_raw(b)
        }

        match self.as_ref() {
            _ if self.is_empty() => "".to_string(),
            VSA::Leaf(set) => {
                let mut s = String::new();
                s.push_str(
                    format!(
                        "<div class=\"leaf box\" id='{}'>",
                        to_ptr(self.clone()) as usize
                    )
                    .as_str(),
                );
                for l in set {
                    s.push_str(&format!("<span class=\"lit\">{}</span>", l.clone()));
                }
                s.push_str("</div>");
                s
            }
            VSA::Union(vsas) => {
                let child_html = vsas
                    .iter()
                    .map(|c| c.to_html(input))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!(
                    "
                <div class=\"union\">
                    <div class=\"box\">
                        <span class=\"op\">∪</span>
                        <div class=\"union-label\">{:?} → {:?}</div>
                    </div>
                    <div class=\"join-children\">
                        {}
                    </div>
                </div>",
                    input,
                    self.eval(input),
                    child_html
                )
            }
            VSA::Join { op, children } => {
                let mut s = String::new();
                let child_html = children
                    .iter()
                    .map(|c| c.to_html(input))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!(
                    "
                <div class=\"join\">
                    <div class=\"box\">
                        <span class=\"op\">{:?}</span>
                        <div class=\"join-label\">{:?} → {:?}</div>
                    </div>
                    <div class=\"join-children\">
                        {}
                    </div>
                </div>
                ",
                    op,
                    input,
                    self.eval(input),
                    child_html
                )
            }
            VSA::Unlearned { start, goal } => {
                let id = to_ptr(self.clone()) as usize;
                let mut s = String::new();
                s.push_str(&format!("<div class=\"unlearned box\" id='{}'>", to_ptr(self.clone()) as usize));
                s.push_str("<div class='unlearned-label'> Unlearned </div>");
                s.push_str(&format!("{:?} → {:?}", start, goal));
                s.push_str(&format!("<br/><button class='unlearned-btn' onclick='learn(this, {})'> Learn </button>", id));
                s.push_str("</div>");
                s
            }
        }
    }
}
