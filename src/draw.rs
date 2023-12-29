use crate::synth::vsa::*;

fn draw_vsa<L, F>(vsa: &VSA<L, F>)
where
    L: Clone + Eq + std::hash::Hash + std::fmt::Debug + InputLit,
    F: Language<L> + std::hash::Hash + std::fmt::Debug + Eq,
{
    match vsa {
        VSA::Leaf(asts) => todo!(),
        VSA::Union(vsas) => todo!(),
        VSA::Join { op, children } => todo!(),
        VSA::Unlearned { goal } => todo!(),
    }
}

fn draw_ast<L, F>(ast: &AST<L, F>)
where
    L: Clone + Eq + std::hash::Hash + std::fmt::Debug + InputLit,
    F: Language<L> + std::hash::Hash + std::fmt::Debug + Eq,
{
    match ast {
        AST::Lit(lit) => todo!(),
        AST::App { fun, args } => todo!(),
        AST::JS { code, input, typ } => todo!(),
    }
}
