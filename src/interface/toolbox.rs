use std::fmt;

use bevy::prelude::Resource;

use crate::{pseudo_random::PseudoRandom, sandbox::*};

// Tools for editing the world
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tool {
    Pixel,
    Circle,
    Square,
    Spray,
    Fill,
}

#[derive(Resource)]
pub struct ToolBox {
    pub tool: Tool,
    pub element: Element,
    pub tool_size: usize,
    pub random: PseudoRandom,
}

impl ToolBox {
    pub fn apply(&mut self, sandbox: &mut SandBox, x: usize, y: usize) {
        let half_size = self.tool_size / 2;
        let remainder = if half_size == 0 {
            1
        } else {
            self.tool_size % half_size
        };
        let x1 = if x > half_size { x - half_size } else { 1 };
        let x2 = if x + half_size + remainder < sandbox.width() {
            x + half_size + remainder
        } else {
            sandbox.width()
        };
        let y1 = if y > half_size { y - half_size } else { 1 };
        let y2 = if y + half_size + remainder < sandbox.height() {
            y + half_size + remainder
        } else {
            sandbox.height()
        };
        match self.tool {
            Tool::Pixel => {
                sandbox.set_element(x, y, self.element);
            }
            Tool::Circle => {
                let radius_sq = (half_size * half_size) as isize;
                for cy in y1..y2 {
                    for cx in x1..x2 {
                        let dx = (cx as isize - x as isize).abs();
                        let dy = (cy as isize - y as isize).abs();
                        if dx * dx + dy * dy <= radius_sq {
                            sandbox.set_element(cx, cy, self.element);
                        }
                    }
                }
            }
            Tool::Square => {
                for cy in y1..y2 {
                    for cx in x1..x2 {
                        sandbox.set_element(cx, cy, self.element);
                    }
                }
            }
            Tool::Spray => {
                let radius_sq = (half_size * half_size) as isize;
                let count = if half_size > 3 { half_size / 3 } else { 1 };
                for _ in 0..count {
                    let cx = x1 + self.random.next() as usize % (x2 - x1);
                    let cy = y1 + self.random.next() as usize % (y2 - y1);
                    let dx = (cx as isize - x as isize).abs();
                    let dy = (cy as isize - y as isize).abs();
                    if dx * dx + dy * dy <= radius_sq {
                        sandbox.set_element(cx, cy, self.element);
                    }
                }
            }
            Tool::Fill => {
                let mut checklist = Vec::new();
                let element_to_replace = sandbox.get(x, y).element;
                if element_to_replace == self.element
                    || element_to_replace == Element::Indestructible
                {
                    return;
                }
                sandbox.set_element(x, y, self.element);
                checklist.push((x, y));
                while !checklist.is_empty() {
                    let (x, y) = checklist.pop().unwrap();
                    for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                        let neighbor_element = sandbox.get(nx, ny).element;
                        if neighbor_element == element_to_replace
                            && neighbor_element != Element::Indestructible
                        {
                            sandbox.set_element(nx, ny, self.element);
                            checklist.push((nx, ny));
                        }
                    }
                }
            }
        }
    }
}

impl Default for ToolBox {
    fn default() -> Self {
        Self {
            tool: Tool::Circle,
            element: Element::Sand,
            tool_size: 8,
            random: PseudoRandom::new(),
        }
    }
}

impl fmt::Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
