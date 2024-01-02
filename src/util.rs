use egui_macroquad::egui::{self, Id};
use egui_macroquad::macroquad::prelude::*;

use std::rc::Rc;

// potentially changes?
pub fn rc_to_id<T>(rc: Rc<T>) -> Id {
    let ptr = Rc::into_raw(rc);
    Id::new(ptr as usize)
}

#[inline(always)]
pub fn vec2pos(v: Vec2) -> egui::Pos2 {
    egui::Pos2::new(v.x, v.y)
}
