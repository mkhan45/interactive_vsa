use crate::synth::vsa::*;
use egui_macroquad::egui::{self, Area, Context, Id, InnerResponse, Rect};
use egui_macroquad::macroquad::prelude::*;

use crate::util::{rc_to_id, vec2pos};

use std::rc::Rc;

const ARROW_ORDER: egui::layers::Order = egui::layers::Order::Middle;

#[derive(Clone, Debug)]
pub struct RichVSA {
    pub vsa: Rc<VSA<Lit, Fun>>,
    pub input: Lit,
    pub other_inputs: Vec<(Lit, Option<Lit>)>,
    pub goal: Lit,
    pub area: egui::Area,
    pub last_move: Vec2,
    pub collapsed: bool,
    pub children: Vec<RichVSA>,
    pub drag: Option<Vec2>,
    pub editable: bool,
}

impl RichVSA {
    pub fn new(
        vsa: Rc<VSA<Lit, Fun>>,
        input: Lit,
        goal: Lit,
        pos: Vec2,
        other_inps: Vec<(Lit, Option<Lit>)>,
    ) -> Self {
        let x_offs = 120.0;
        let y_offs = 120.0 + 30.0 * other_inps.len() as f32;

        let children = match vsa.as_ref() {
            VSA::Leaf(_) | VSA::Unlearned { .. } => Vec::new(),
            VSA::Union(vsas) => {
                vsas.into_iter()
                    .enumerate()
                    .map(|(i, vsa)| {
                        // TODO: choose good pos
                        RichVSA::new(
                            vsa.clone(),
                            input.clone(),
                            goal.clone(),
                            pos + Vec2::new(x_offs * i as f32, y_offs),
                            other_inps.clone(),
                        )
                    })
                    .collect()
            }
            VSA::Join {
                children,
                children_goals,
                ..
            } => children
                .iter()
                .zip(children_goals.iter())
                .enumerate()
                .map(|(i, (vsa, goal))| {
                    RichVSA::new(
                        vsa.clone(),
                        input.clone(),
                        goal.clone(),
                        pos + Vec2::new(x_offs * i as f32, y_offs),
                        other_inps.clone(),
                    )
                })
                .collect(),
        };

        let id = rc_to_id(vsa.clone());
        let area = Area::new("vsa").id(id).default_pos(vec2pos(pos));

        Self {
            vsa,
            input,
            other_inputs: other_inps,
            goal,
            area,
            last_move: Vec2::ZERO,
            collapsed: false,
            children,
            drag: None,
            editable: false,
        }
    }

    pub fn editable(self) -> Self {
        Self {
            editable: true,
            ..self
        }
    }

    #[inline(always)]
    pub fn id(&self) -> Id {
        rc_to_id(self.vsa.clone())
    }

    pub fn set_vsa_style(ui: &mut egui::Ui) {
        // doesnt do anything
        let style = ui.style_mut();
        style.spacing.item_spacing = egui::Vec2::new(10.0, 10.0);
        style.spacing.window_margin = egui::style::Margin::same(10.0);
    }

    pub fn draw(
        &mut self,
        labels: bool,
        learn_depth: usize,
        search_depth: usize,
        egui_ctx: &Context,
    ) {
        let learn_pos = self.rect(egui_ctx).map(|r| {
            let egui::Pos2 { x, y } = r.left_top();
            vec2(x, y)
        });
        match self.vsa.as_ref() {
            VSA::Leaf(asts) => {
                let sorted_asts = {
                    let mut asts = asts.iter().collect::<Vec<_>>();
                    asts.sort_by_key(|ast| ast.size());
                    asts
                };
                self.area.show(egui_ctx, |ui| {
                    Self::set_vsa_style(ui);
                    if labels {
                        ui.label("Leaf");
                        ui.label(format!("{} → {}", self.input, self.goal));
                    }
                    let selected_ast = sorted_asts.iter().find(|ast| {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}", ast));
                            asts.len() > 1 && ui.button("Select").clicked()
                        })
                        .inner
                    });
                    if let Some(ast) = selected_ast {
                        let unwrapped_ast = ast.as_ref().clone();
                        let new_vsa = VSA::singleton(unwrapped_ast);
                        let self_mut = Rc::as_ptr(&self.vsa) as *mut _;
                        // Safety: probably
                        unsafe { std::ptr::write(self_mut, new_vsa) };
                        self.children.clear();
                    }
                });
            }
            VSA::Union(_) => {
                let InnerResponse { response, .. } = self.area.show(egui_ctx, |ui| {
                    Self::set_vsa_style(ui);
                    if labels {
                        ui.label("Union");
                    }
                    ui.label(format!("{} → {}", self.input, self.goal));
                    Self::draw_other_inps(&mut self.other_inputs, ui);
                });
                let edrag = response
                    .dragged_by(egui::PointerButton::Primary)
                    .then(|| response.drag_delta());
                self.drag = edrag.map(|drag| Vec2::new(drag.x, drag.y));
                let id = self.id();
                for vsa in &mut self.children {
                    vsa.draw(labels, learn_depth, search_depth, egui_ctx);
                    draw_area_arrows(id, vsa.id(), egui_ctx);
                }
            }
            VSA::Join {
                op, children_goals, ..
            } => {
                let InnerResponse { response, .. } = self.area.show(egui_ctx, |ui| {
                    Self::set_vsa_style(ui);
                    if labels {
                        ui.label(format!("{:?} Join", op));
                        ui.label(format!("{} → {}", self.input, self.goal));
                    }

                    let args = children_goals
                        .iter()
                        .map(|goal| format!("{}", goal))
                        .collect::<Vec<_>>()
                        .join(", ");
                    ui.label(format!("{:?}({})", op, args));
                    // Self::draw_other_inps(&mut self.other_inputs, ui);
                });
                let edrag = response
                    .dragged_by(egui::PointerButton::Primary)
                    .then(|| response.drag_delta());
                self.drag = edrag.map(|drag| Vec2::new(drag.x, drag.y));
                let id = self.id();
                for vsa in self.children.iter_mut() {
                    vsa.draw(labels, learn_depth, search_depth, egui_ctx);
                    draw_area_arrows(id, vsa.id(), egui_ctx);
                }
            }
            VSA::Unlearned { start, goal } => {
                self.area.show(egui_ctx, |ui| {
                    Self::set_vsa_style(ui);
                    ui.label("Unlearned");
                    if self.editable {
                        ui.horizontal(|ui| {
                            let mut inp_str = match &self.input {
                                Lit::StringConst(s) => s.clone(),
                                _ => "".to_string(),
                            };
                            let mut goal_str = match &self.goal {
                                Lit::StringConst(s) => s.clone(),
                                _ => "".to_string(),
                            };
                            ui.text_edit_singleline(&mut inp_str);
                            ui.label("→");
                            ui.text_edit_singleline(&mut goal_str);
                            self.input = Lit::StringConst(inp_str);
                            self.goal = Lit::StringConst(goal_str);
                            let new_vsa = VSA::Unlearned {
                                start: self.input.clone(),
                                goal: self.goal.clone(),
                            };
                            let self_mut = Rc::as_ptr(&self.vsa) as *mut VSA<Lit, Fun>;
                            // Safety: probably
                            unsafe { std::ptr::write(self_mut, new_vsa) };
                        });
                    } else {
                        ui.label(format!("{} → {}", start, goal));
                    }
                    Self::draw_other_inps(&mut self.other_inputs, ui);
                    if ui.button("Learn").clicked() {
                        self.editable = false;

                        let complete_other_inps = self.other_inputs.iter().enumerate().filter_map(|(i, (inp, out))| {
                            out.clone().map(|out| (i+1, (inp.clone(), out)))
                        });

                        let complete_examples = std::iter::once((0, (start.clone(), goal.clone())))
                            .chain(complete_other_inps)
                            .collect::<Vec<_>>(); 

                        let mut char_sets = complete_examples.iter().map(|(_, (inp, out))| match (inp, out) {
                            (Lit::StringConst(inp), Lit::StringConst(out)) => inp
                                .chars()
                                .chain(out.chars())
                                .filter(|c| !c.is_alphanumeric())
                                .map(|c| match c {
                                    '.' => Lit::StringConst("\\.".to_string()),
                                    '{' => Lit::StringConst("\\{".to_string()),
                                    '}' => Lit::StringConst("\\{".to_string()),
                                    _ => Lit::StringConst(c.to_string()),
                                })
                            .collect::<std::collections::HashSet<_>>(),
                            _ => std::collections::HashSet::new(),
                        });

                        let chars = char_sets
                            .next()
                            .map(|s1| {
                                s1.iter()
                                    .filter(|c| char_sets.clone().all(|s2| s2.contains(c)))
                                    .cloned()
                                    .collect::<Vec<_>>()
                            })
                        .unwrap_or_default();

                        use std::collections::HashMap;
                        let mut all_cache = HashMap::new();
                        let mut bank = crate::synth::bank::Bank::new();
                        let mut regex_bank = crate::synth::bank::Bank::new();

                        let num_examples = 1 + self.other_inputs.len();

                        // TODO: need to pass in to children
                        // dbg!(&chars);
                        for prim in [
                            Lit::Input,
                            Lit::StringConst("".to_string()),
                            Lit::StringConst(" ".to_string()),
                            Lit::StringConst(".".to_string()),
                            Lit::LocConst(0),
                            Lit::LocConst(1),
                            Lit::LocEnd,
                        ]
                            .into_iter()
                            .chain(chars.clone().into_iter())
                            {
                                bank.size_mut(1).push(AST::Lit(prim.clone()));
                                all_cache.insert(
                                    std::iter::repeat(prim.clone()).take(num_examples).collect(),
                                    Rc::new(VSA::singleton(AST::Lit(prim.clone()))),
                                    );
                            }

                        for prim in [
                            Lit::StringConst("\\d".to_string()),
                            Lit::StringConst("\\b".to_string()),
                            Lit::StringConst("[a-z]".to_string()),
                            Lit::StringConst("[A-Z]".to_string()),
                        ]
                            .into_iter()
                            .chain(chars.into_iter())
                            {
                                regex_bank.size_mut(1).push(AST::Lit(prim.clone()));
                            }

                        let bottom_up_inps = std::iter::once(start.clone())
                            .chain(self.other_inputs.iter().map(|(inp, _)| inp.clone()))
                            .collect::<Vec<_>>();
                        for i in 1..=search_depth {
                            crate::synth::bottom_up(
                                bottom_up_inps.iter(),
                                // std::iter::once(&start.clone()),
                                i,
                                &mut all_cache,
                                &mut bank,
                                &mut regex_bank,
                                false,
                                );
                        }
                        // dbg!(&bank);

                        let mut ex_vsas = complete_examples.iter().map(|(i, (inp, out))| {
                            let mut cache: HashMap<Lit, Rc<VSA<Lit, Fun>>> = HashMap::new();
                            for (outs, vsa) in all_cache.iter() {
                                if let Some(v) = cache.get_mut(&outs[*i]) {
                                    *v = Rc::new(VSA::unify(vsa.clone(), v.clone()));
                                } else {
                                    cache.insert(outs[*i].clone(), vsa.clone());
                                }
                            }

                            crate::synth::learn_to_depth(
                                inp, 
                                out, 
                                &mut cache, 
                                &bank,
                                learn_depth,
                                )
                        });

                        let mut res = RichVSA::new(
                            ex_vsas.next().unwrap(),
                            start.clone(),
                            goal.clone(),
                            Vec2::new(0.0, 0.0),
                            self.other_inputs.clone(),
                        );

                        // let mut res: Rc<VSA<_, _>> = ex_vsas.next().unwrap();
                        // dbg!(&res);
                        for vsa in ex_vsas {
                            if let Some(prog) = res.vsa.clone().pick_best(|ast| ast.cost()) {
                                if complete_examples.iter().all(|(_, (inp, out))| prog.eval(inp) == *out) {
                                    break;
                                };
                            }

                            let rich_vsa = RichVSA::new(
                                vsa.clone(),
                                start.clone(),
                                goal.clone(),
                                Vec2::new(0.0, 0.0),
                                self.other_inputs.clone(),
                            );
                            res = rich_vsa.intersect(&res);
                            // dbg!(&vsa);
                            // res = VSA::flatten(Rc::new(res.intersect(vsa.as_ref())));
                        }

                        // TODO: somehow convert union of unlearned to richvsa with multiexample

                        let new_vsa = res.vsa.as_ref().clone();
                        let self_mut = Rc::as_ptr(&self.vsa) as *mut _;
                        // Safety: probably
                        unsafe { std::ptr::write(self_mut, new_vsa) };
                        // let rich_vsa = RichVSA::new(
                        //     self.vsa.clone(),
                        //     self.input.clone(),
                        //     self.goal.clone(),
                        //     learn_pos.unwrap(),
                        //     self.other_inputs.iter().map(|(inp, _)| (inp.clone(), None)).collect(),
                        //     );
                        self.children = res.children;
                    }
                });
            }
        }

        if let Some(rect) = self.rect(egui_ctx) {
            let painter =
                egui_ctx.layer_painter(egui::layers::LayerId::new(ARROW_ORDER, self.id()));
            painter.rect_stroke(
                rect,
                egui::Rounding::ZERO,
                egui::Stroke::new(1.0, egui::Color32::BLACK),
                );
        }

        // if let Some(rect) = self.subtree_rect(egui_ctx) {
        //     let painter = egui_ctx.layer_painter(egui::layers::LayerId::new(ARROW_ORDER, self.id()));
        //     painter.rect_stroke(rect, egui::Rounding::ZERO, egui::Stroke::new(1.0, egui::Color32::RED));
        // }
    }

    pub fn rect(&self, egui_ctx: &Context) -> Option<Rect> {
        egui_ctx.memory(|mem| mem.area_rect(self.id()).map(|r| r.expand(10.0)))
    }

    pub fn subtree_rect(&self, egui_ctx: &Context) -> Option<Rect> {
        self.children
            .iter()
            .fold(self.rect(egui_ctx), |rect, child| {
                let child_rect = child.subtree_rect(egui_ctx);
                rect.zip(child_rect)
                    .map(|(rect, child_rect)| rect.union(child_rect))
            })
    }

    pub fn updated_rect(&self, egui_ctx: &Context) -> Option<Rect> {
        egui_ctx.memory(|mem| {
            let evec = egui::Vec2::new(self.last_move.x, self.last_move.y);
            mem.area_rect(self.id()).map(|mem| mem.translate(evec))
        })
    }

    pub fn updated_subtree_rect(&self, egui_ctx: &Context) -> Option<Rect> {
        self.children
            .iter()
            .fold(self.updated_rect(egui_ctx), |rect, child| {
                let child_rect = child.updated_subtree_rect(egui_ctx);
                rect.zip(child_rect)
                    .map(|(rect, child_rect)| rect.union(child_rect))
            })
    }

    pub fn drag_subtrees(&mut self) {
        if let Some(drag) = self.drag {
            for child in &mut self.children {
                child.move_subtree(drag);
            }
            self.drag = None;
        } else {
            for child in &mut self.children {
                child.drag_subtrees();
            }
        }
    }

    pub fn any_children_dragged(&self) -> bool {
        self.drag.is_some()
            || self
            .children
            .iter()
            .any(|child| child.any_children_dragged())
    }

    pub fn repel_children(&mut self, egui_ctx: &Context) {
        if self.updated_subtree_rect(egui_ctx).is_none() {
            return;
        }

        for i in 0..self.children.len() {
            self.children[i].repel_children(egui_ctx);

            if self.children[i].any_children_dragged() {
                continue;
            }

            // possibly only look at adjacent children
            let mut x_force = 0.0;
            let i_rect = self.children[i].updated_subtree_rect(egui_ctx).unwrap();
            for j in 0..self.children.len() {
                if i == j {
                    continue;
                }
                let j_rect = self.children[j].updated_subtree_rect(egui_ctx).unwrap();
                let x_dist = i_rect.center().x - j_rect.center().x;
                if i_rect.expand(15.0).intersects(j_rect.expand(15.0)) {
                    // repel
                    x_force += x_dist.signum() * 5.0;

                    // let painter = egui_ctx.layer_painter(egui::layers::LayerId::new(ARROW_ORDER, self.id()));
                    // painter.rect_stroke(i_rect, egui::Rounding::ZERO, egui::Stroke::new(1.0, egui::Color32::YELLOW));
                    // painter.rect_stroke(j_rect, egui::Rounding::ZERO, egui::Stroke::new(1.0, egui::Color32::YELLOW));
                }
            }

            // dbg!(x_force);
            // TODO:
            // improved algo:
            //  - find highest child that intersects another subtree, and repel only it
            self.children[i].move_subtree(Vec2::new(x_force, 0.0));
        }
    }

    pub fn zero_last_move(&mut self) {
        self.last_move = Vec2::ZERO;
        for child in &mut self.children {
            child.zero_last_move();
        }
    }

    pub fn move_subtree(&mut self, delta: Vec2) {
        self.last_move += delta;
        for child in &mut self.children {
            child.move_subtree(delta);
        }
    }

    pub fn update_subtree(&mut self, egui_ctx: &Context) {
        if let Some(updated_rect) = self.updated_rect(egui_ctx) {
            let updated_pos = updated_rect.left_top();
            self.area = self
                .area
                .current_pos(egui::Pos2::new(updated_pos.x, updated_pos.y));
            for child in &mut self.children {
                child.update_subtree(egui_ctx);
            }
        }
    }

    pub fn find_clicked_node(
        &mut self,
        pos: egui::Pos2,
        egui_ctx: &Context,
        ) -> Option<&mut RichVSA> {
        if let Some(rect) = self.rect(egui_ctx) {
            if rect.contains(egui::Pos2::new(pos.x, pos.y)) {
                return Some(self);
            }
        }
        self.children
            .iter_mut()
            .find_map(|child| child.find_clicked_node(pos, egui_ctx))
    }

    pub fn find_parent_of_vsa(&mut self, vsa: &Rc<VSA<Lit, Fun>>) -> Option<&mut RichVSA> {
        if Rc::ptr_eq(&self.vsa, vsa) {
            return None;
        } else if self
            .children
                .iter()
                .any(|child| Rc::ptr_eq(&child.vsa, vsa))
                {
                    return Some(self);
                }

        self.children
            .iter_mut()
            .find_map(|child| child.find_parent_of_vsa(vsa))
    }

    pub fn draw_other_inps(other_inps: &mut Vec<(Lit, Option<Lit>)>, ui: &mut egui::Ui) {
        let mut kill_inps = std::collections::HashSet::new();
        for (other_inp, other_out) in other_inps.iter_mut() {
            let mut other_inp_str = match other_inp {
                Lit::StringConst(s) => s.clone(),
                _ => todo!(),
            };
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut other_inp_str);
                ui.label("→");
                if let Some(Lit::StringConst(other_out_str)) = other_out {
                    ui.text_edit_singleline(other_out_str);
                    if ui.button("X").clicked() {
                        *other_out = None;
                    }
                } else {
                    if ui.button("Add Output").clicked() {
                        *other_out = Some(Lit::StringConst("".to_string()));
                    }
                    if ui.button("X").clicked() {
                        kill_inps.insert(other_inp.clone());
                    }
                }

                *other_inp = Lit::StringConst(other_inp_str);
            });
        }

        other_inps.retain(|(inp, _)| match inp {
            Lit::StringConst(s) => !kill_inps.contains(&Lit::StringConst(s.clone())),
            _ => todo!(),
        });

        if ui.button("Add Example").clicked() {
            other_inps.push((Lit::StringConst("".to_string()), None));
        }
    }

    pub fn intersect(&self, other: &RichVSA) -> RichVSA {
        match (self.vsa.as_ref(), other.vsa.as_ref()) {
            (vsa, VSA::Union(union)) | (VSA::Union(union), vsa) => {
                let rich_union = if self.vsa.as_ref() == vsa {
                    self
                } else {
                    other
                };

                let ncs: Vec<_> = 
                    rich_union.children.iter().map(|child| child.intersect(other)).collect();

                let nvsa_children: Vec<_> = 
                    union.iter().map(|n1| Rc::new(n1.clone().intersect(vsa))).collect();
                let new_vsa = VSA::Union(nvsa_children);

                RichVSA {
                    vsa: Rc::new(new_vsa),
                    input: self.input.clone(),
                    other_inputs: self.other_inputs.clone(),
                    goal: self.goal.clone(),
                    children: ncs,
                    ..*self
                }
            }

            #[rustfmt::skip]
            (VSA::Join { op: l_op, .. }, VSA::Join { op: r_op, .. }) if l_op != r_op => {
                RichVSA {
                    vsa: Rc::new(VSA::empty()),
                    input: self.input.clone(),
                    other_inputs: self.other_inputs.clone(),
                    goal: self.goal.clone(),
                    children: Vec::new(),
                    ..*self
                }
            }

            #[rustfmt::skip]
            (VSA::Join { op, children: l_children, children_goals }, VSA::Join { op: _, children: r_children, .. })
                => {
                    let ncs: Vec<_> = self.children.iter().zip(other.children.iter()).map(|(l, r)| {
                        let real_r = RichVSA { 
                            other_inputs: r.other_inputs.iter().map(|(inp, _)| (inp.clone(), None)).collect(),
                            ..r.clone()
                        };
                        l.intersect(&real_r)
                    }).collect();

                    let nvsa_children: Vec<_> = 
                        l_children.iter().zip(r_children).map(|(l, r)| Rc::new(l.intersect(r))).collect();

                    let new_vsa = VSA::Join {
                        op: op.clone(),
                        children: nvsa_children,
                        children_goals: children_goals.clone(),
                    };

                    RichVSA {
                        vsa: Rc::new(new_vsa),
                        input: self.input.clone(),
                        other_inputs: self.other_inputs.clone(),
                        goal: self.goal.clone(),
                        children: ncs,
                        ..*self
                    }
                }

            #[rustfmt::skip]
            (VSA::Join { op, children, .. }, VSA::Leaf(s)) | (VSA::Leaf(s), VSA::Join { op, children, .. })
                => {
                    let new_vsa = VSA::Leaf(s.iter().filter(|pj| {
                        match pj.as_ref() {
                            AST::App { fun, args } if fun == op =>
                                args.iter().zip(children).all(|(arg, vsa)| vsa.contains(arg)),
                            _ => false
                        }
                    }).cloned().collect());

                    RichVSA {
                        vsa: Rc::new(new_vsa),
                        input: self.input.clone(),
                        other_inputs: self.other_inputs.clone(),
                        goal: self.goal.clone(),
                        children: Vec::new(),
                        ..*self
                    }
                }

            (VSA::Leaf(_), VSA::Leaf(_)) => {
                RichVSA {
                    vsa: Rc::new(self.vsa.intersect(other.vsa.as_ref())),
                    input: self.input.clone(),
                    other_inputs: self.other_inputs.clone(),
                    goal: self.goal.clone(),
                    children: Vec::new(),
                    ..*self
                }
            }

            (VSA::Unlearned { .. }, VSA::Unlearned { start: r_start, goal: r_goal }) => {
                let other_inputs = 
                    self.other_inputs.iter()
                    .filter(|(inp, _)| inp != r_start)
                    .cloned()
                    .chain(std::iter::once((r_start.clone(), Some(r_goal.clone()))))
                    .collect();

                RichVSA {
                    vsa: self.vsa.clone(),
                    input: self.input.clone(),
                    goal: self.goal.clone(),
                    children: Vec::new(),
                    other_inputs,
                    // other_inputs: self.other_inputs.clone().m
                    ..*self
                }
            }

            _ => todo!() // intersection of unlearned with others, just unionize
        }
    }

    // pub fn reintersect(&mut self) {
    //     match self.vsa.as_ref() {
    //         VSA::Leaf(_) | VSA::Unlearned { .. } => {}
    //         VSA::Union(vsas) => {
    //             // if we have unlearned, we convert it to other_inp
    //             let unlearneds = vsas.iter().filter_map(|vsa| match vsa.as_ref() {
    //                 VSA::Unlearned { .. } => Some(vsa.clone()),
    //                 _ => None,
    //             });
    //             let cumulative_unlearned = unlearneds.reduce(|a, b| {
    //                 todo!()
    //             });
    //             let others = vsas.iter().filter_map(|vsa| match vsa.as_ref() {
    //                 VSA::Unlearned { .. } => None,
    //                 _ => Some(vsa.clone()),
    //             });
    //             let set = if let Some(cumulative_unlearned) = cumulative_unlearned {
    //                 std::iter::once(cumulative_unlearned).chain(others).collect()
    //             } else {
    //                 others.collect()
    //             };
    //             let new_vsa = VSA::Union(set);
    //             let self_mut = Rc::as_ptr(&self.vsa) as *mut _;
    //             // Safety: probably
    //             unsafe { std::ptr::write(self_mut, new_vsa) };
    //             // TODO: get children
    //         }
    //         VSA::Join { .. } => {
    //             for child in self.children.iter_mut() {
    //                 child.reintersect();
    //             }
    //         }
    //     }
    // }
}

impl PartialEq for RichVSA {
    fn eq(&self, other: &Self) -> bool {
        // probably not strictly parial eq
        self.vsa == other.vsa
    }
}

fn draw_area_arrows(start_id: Id, end_id: Id, egui_ctx: &Context) {
    // TODO: use .rect()
    let positions = egui_ctx.memory(|mem| {
        let start_rect = mem.area_rect(start_id);
        let end_rect = mem.area_rect(end_id);
        start_rect.zip(end_rect).map(|(start_rect, end_rect)| {
            (
                start_rect.expand(10.0).center_bottom(),
                end_rect.expand(10.0).center_top(),
                )
        })
    });

    if let Some((sp, ep)) = positions {
        draw_arrow(start_id, sp, ep, egui_ctx);
    }
}

fn draw_arrow(id: Id, sp: egui::Pos2, ep: egui::Pos2, egui_ctx: &Context) {
    let painter = egui_ctx.layer_painter(egui::layers::LayerId::new(ARROW_ORDER, id));
    painter.line_segment([sp, ep], egui::Stroke::new(1.0, egui::Color32::BLACK));
    // painter.arrow(sp, vec, egui::Stroke::new(1.0, egui::Color32::WHITE));
}
