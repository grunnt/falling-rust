use crate::{element::Element, sandbox::SandBox};
use bevy::prelude::Resource;
use rand::Rng;
use rand_xoshiro::{rand_core::SeedableRng, Xoshiro256Plus};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tool {
    Pixel,
    FillCircle,
    FillSquare,
    SprayCircle,
}

#[derive(Resource)]
pub struct ToolBox {
    pub tool: Tool,
    pub element: Element,
    pub tool_size: usize,
    random: Xoshiro256Plus,
}

impl ToolBox {
    pub fn apply(&mut self, level: &mut SandBox, x: usize, y: usize) {
        let half_size = self.tool_size / 2;
        let remainder = if half_size == 0 {
            1
        } else {
            self.tool_size % half_size
        };
        let x1 = if x > half_size { x - half_size } else { 1 };
        let x2 = if x + half_size + remainder < level.width() {
            x + half_size + remainder
        } else {
            level.width()
        };
        let y1 = if y > half_size { y - half_size } else { 1 };
        let y2 = if y + half_size + remainder < level.height() {
            y + half_size + remainder
        } else {
            level.height()
        };
        match self.tool {
            Tool::Pixel => {
                level.set_element(x, y, self.element);
            }
            Tool::FillCircle => {
                let radius_sq = (half_size * half_size) as isize;
                for cy in y1..y2 {
                    for cx in x1..x2 {
                        let dx = (cx as isize - x as isize).abs();
                        let dy = (cy as isize - y as isize).abs();
                        if dx * dx + dy * dy <= radius_sq {
                            level.set_element(cx, cy, self.element);
                        }
                    }
                }
            }
            Tool::FillSquare => {
                for cy in y1..y2 {
                    for cx in x1..x2 {
                        level.set_element(cx, cy, self.element);
                    }
                }
            }
            Tool::SprayCircle => {
                let radius_sq = (half_size * half_size) as isize;
                let count = if half_size > 3 { half_size / 3 } else { 1 };
                for _ in 0..count {
                    let cx = self.random.gen_range(x1..=x2 - 1);
                    let cy = self.random.gen_range(y1..=y2 - 1);
                    let dx = (cx as isize - x as isize).abs();
                    let dy = (cy as isize - y as isize).abs();
                    if dx * dx + dy * dy <= radius_sq {
                        level.set_element(cx, cy, self.element);
                    }
                }
            }
        }
    }
}

impl Default for ToolBox {
    fn default() -> Self {
        Self {
            tool: Tool::FillCircle,
            element: Element::Sand,
            tool_size: 8,
            random: Xoshiro256Plus::from_entropy(),
        }
    }
}
