use crate::synth::vsa::*;
use egui_macroquad::macroquad::prelude::*;
use egui_macroquad::egui;

use crate::vsa_state::*;

const HELP_ORDER: egui::layers::Order = egui::layers::Order::Middle;

pub struct Camera {
    pub pos: Vec2,
    pub zoom: f32,
}

pub struct MainState {
    pub vsas: Vec<RichVSA>,
    pub camera: Camera,
    pub frames_since_last_drag: Option<usize>,
    pub vsa_labels: bool,
    pub current_tool: Tool,
    pub learn_depth: usize,
    pub search_depth: usize,
    pub messages: Vec<Message>,
    pub show_help: bool,
}

pub struct Message {
    pub text: egui::RichText,
    pub remaining_frames: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tool {
    Drag,
    Select,
    Prune,
    Extract,
}

impl MainState {
    pub fn new(vsas: Vec<RichVSA>) -> Self {
        Self {
            vsas,
            camera: Camera {
                pos: vec2(0.0, 0.0),
                zoom: 1.0,
            },
            frames_since_last_drag: None,
            vsa_labels: false,
            current_tool: Tool::Drag,
            learn_depth: 1,
            search_depth: 4,
            messages: vec![],
            show_help: true,
        }
    }

    pub fn update(&mut self) {
        egui_macroquad::cfg(|egui_ctx| {
            for vsa in &mut self.vsas {
                vsa.zero_last_move();
            }
            for vsa in &mut self.vsas {
                // TODO: figure out how to disable when dragging
                vsa.repel_children(egui_ctx);

                if self.current_tool == Tool::Drag && egui_ctx.input(|inp| inp.modifiers.command_only()) {
                    vsa.drag_subtrees();
                }
            }

            egui_ctx.input(|inp| {
                if inp.pointer.middle_down() || inp.modifiers.shift {
                    let delta = inp.pointer.delta();
                    for vsa in &mut self.vsas {
                        vsa.move_subtree(vec2(delta.x, delta.y));
                    }
                }
            });

            let (clicked, pos_opt) = egui_ctx.input(|inp| {
                (inp.pointer.primary_down(), inp.pointer.interact_pos())
            });

            let tool_change = egui_ctx.input(|inp| {
                if inp.key_down(egui::Key::D) {
                    Some(Tool::Drag)
                } else if inp.key_down(egui::Key::S) {
                    Some(Tool::Select)
                } else if inp.key_down(egui::Key::F) {
                    Some(Tool::Prune)
                } else if inp.key_down(egui::Key::E) {
                    Some(Tool::Extract)
                } else {
                    None
                }
            });
            if let Some(tool) = tool_change {
                self.current_tool = tool;
            }

            if [Tool::Select, Tool::Prune].contains(&self.current_tool) && clicked {
                let pos = pos_opt.unwrap();
                let clicked_node = 
                    self.vsas.iter_mut().find_map(|vsa| vsa.find_clicked_node(pos, egui_ctx)).map(|n| n.vsa.clone());
                let parent = clicked_node.clone().map(|child| {
                    self.vsas.iter_mut().find_map(|vsa| vsa.find_parent_of_vsa(&child))
                }).flatten();
                if let Some((parent, child)) = parent.zip(clicked_node) {
                    if matches!(parent.vsa.as_ref(), VSA::Union(_)) {
                        let mut kill_vsas = vec![];
                        if self.current_tool == Tool::Select {
                            for child in parent.children.iter().filter(|c| !std::rc::Rc::ptr_eq(&c.vsa, &child)) {
                                kill_vsas.push(child.vsa.clone());
                            }
                            parent.children.retain(|c| std::rc::Rc::ptr_eq(&c.vsa, &child));
                        } else /* if self.current_tool == Tool::Prune */{
                            parent.children.retain(|c| !std::rc::Rc::ptr_eq(&c.vsa, &child));
                            kill_vsas.push(child);
                        }

                        let ctrl_clicked = egui_ctx.input(|inp| inp.modifiers.command_only());
                        if !ctrl_clicked {
                            self.current_tool = Tool::Drag;
                        }

                        for vsa in kill_vsas {
                            let new_vsa = VSA::empty();
                            let vsa_rc_mut = std::rc::Rc::as_ptr(&vsa) as *mut VSA<Lit, Fun>;
                            // safety: probably
                            unsafe { std::ptr::write(vsa_rc_mut, new_vsa); }
                        }
                    }
                }
            }

            if self.current_tool == Tool::Extract && clicked {
                let pos = pos_opt.unwrap();
                let clicked_node = 
                    self.vsas.iter_mut().find_map(|vsa| vsa.find_clicked_node(pos, egui_ctx));
                if let Some(clicked_node) = clicked_node {
                    let ast = clicked_node.vsa.pick_best(|ast| ast.cost());
                    if let Some(ast) = ast {
                        let new_vsa = VSA::singleton(ast);
                        let vsa_rc_mut = std::rc::Rc::as_ptr(&clicked_node.vsa) as *mut VSA<Lit, Fun>;
                        // safety: probably
                        unsafe { std::ptr::write(vsa_rc_mut, new_vsa); }
                        clicked_node.children.clear();
                        self.current_tool = Tool::Drag;
                    } else {
                        self.messages.push(Message {
                            text: egui::RichText::new("No AST could be extracted").size(24.0).color(egui::Color32::RED),
                            remaining_frames: 600,
                        });
                        self.current_tool = Tool::Drag;
                    }
                }
            }

            for vsa in &mut self.vsas {
                vsa.update_subtree(egui_ctx);
            }

            // let scroll = egui_ctx.input(|inp| inp.scroll_delta);
            // if scroll.y != 0.0 {
            //     bad
            //     self.camera.zoom *= 1.0 + scroll.y / 10.0;
            //     egui_ctx.style_mut(|style| {
            //         style.override_font_id = Some(egui::FontId::monospace(16.0 * self.camera.zoom));
            //     });
            // }

            // // let scroll = egui_ctx.input(|inp| inp.scroll_delta);
            // if scroll.y != 0.0 {
            //     let old_zoom = egui_ctx.zoom_factor();
            //     let new_zoom = old_zoom * (1.0 + scroll.y / 10.0);
            //     // crashes
            //     egui_ctx.set_zoom_factor(new_zoom);
            // }
        });
    }

    pub fn draw(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            clear_background(WHITE);

            // egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
            //     egui::gui_zoom::zoom_menu_buttons(ui);
            // });

            for vsa in &mut self.vsas {
                vsa.draw(self.vsa_labels, self.learn_depth, self.search_depth, egui_ctx);
                // draw_vsa(vsa.vsa.clone(), Vec2::new(100.0, 100.0), &vsa.input, None, egui_ctx);
            }

            egui::TopBottomPanel::top("top bar").show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    let vsa_label_text = egui::RichText::new("VSA Labels").size(24.0);
                    ui.checkbox(&mut self.vsa_labels, vsa_label_text);

                    let tool_str = format!("Current Tool: {:?}", self.current_tool);
                    let tools_text = egui::RichText::new(tool_str).size(24.0);
                    ui.menu_button(tools_text, |ui| {
                        ui.selectable_value(&mut self.current_tool, Tool::Drag, "Drag");
                        ui.selectable_value(&mut self.current_tool, Tool::Select, "Select");
                        ui.selectable_value(&mut self.current_tool, Tool::Prune, "Prune");
                        ui.selectable_value(&mut self.current_tool, Tool::Extract, "Extract");
                    });

                    ui.add(egui::widgets::Slider::new(&mut self.learn_depth, 1..=10).text("Learn Depth"));
                    ui.add(egui::widgets::Slider::new(&mut self.search_depth, 1..=9).text("Search Depth"));
                });
            });

            egui::Window::new("help")
                .open(&mut self.show_help)
                .default_pos(egui::Pos2::new(screen_width() - 410.0, 0.0))
                .show(egui_ctx, |ui| {
                    ui.set_max_width(400.0);
                    ui.label(concat!(
                    "Middle mouse or shift drag to pan\n",
                    "Click/hold to drag nodes\n",
                    "Control/command click to drag a whole subtree\n\n",
                    "D: Drag tool\n",
                    "S: Select tool\n",
                    "F: Prune tool\n",
                    "E: Extract tool",
                    ));
                });

            for (i, message) in self.messages.iter_mut().enumerate() {
                let pos = vec2(0.0, 50.0 + 50.0 * i as f32);
                egui::Window::new(format!("error{}", i))
                    .fixed_pos(crate::util::vec2pos(pos))
                    .show(egui_ctx, |ui| {
                        ui.label(message.text.clone());
                    });
                message.remaining_frames -= 1;
            }

            self.messages.retain(|m| m.remaining_frames > 0);
        });
        egui_macroquad::draw();
    }
}
