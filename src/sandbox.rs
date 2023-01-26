use crate::{cell::*, element::*};
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

#[derive(Component)]
pub struct SandBox {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    visited_state: bool,
    pub render_time_ms: u128,
}

impl SandBox {
    pub fn new(width: usize, height: usize) -> Self {
        let mut sandbox = SandBox::empty(width, height);
        // Set indestructible pixels at the border to ease computations
        for x in 0..sandbox.width() {
            sandbox.set_element(x, 0, Element::Indestructible, 0);
            sandbox.set_element(x, sandbox.height() - 1, Element::Indestructible, 0);
        }
        for y in 0..sandbox.height() {
            sandbox.set_element(0, y, Element::Indestructible, 0);
            sandbox.set_element(sandbox.width() - 1, y, Element::Indestructible, 0);
        }
        sandbox
    }

    fn empty(width: usize, height: usize) -> Self {
        SandBox {
            width,
            height,
            cells: vec![
                Cell {
                    element: Element::Air,
                    variant: 0,
                    strength: 0,
                    visited: false,
                };
                width * height
            ],
            visited_state: false,
            render_time_ms: 0,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        let index = self.index(x, y);
        &self.cells[index]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        let index = self.index(x, y);
        &mut self.cells[index]
    }

    pub fn reduce_strength(&mut self, x: usize, y: usize, amount: u8) -> bool {
        let index = self.index(x, y);
        let cell = &mut self.cells[index];
        if cell.strength > 0 {
            cell.strength = if cell.strength > amount {
                cell.strength - amount
            } else {
                0
            };
            true
        } else {
            false
        }
    }

    pub fn clear_cell(&mut self, x: usize, y: usize) {
        self.set_element(x, y, Element::Air, 0);
    }

    pub fn set_element_with_strength(
        &mut self,
        x: usize,
        y: usize,
        element: Element,
        strength: u8,
        random: u32,
    ) {
        let index = self.index(x, y);
        let mut cell = &mut self.cells[index];
        if cell.element == Element::Indestructible {
            // Cannot edit these blocks
            return;
        }
        cell.element = element;
        cell.visited = self.visited_state;
        cell.strength = strength;
        let render = element_type(element).render;
        if render == RenderMethod::VariantLinear || render == RenderMethod::Flicker {
            cell.variant = random as u8;
        }
    }

    pub fn set_element(&mut self, x: usize, y: usize, element: Element, random: u32) {
        self.set_element_with_strength(x, y, element, element_type(element).strength, random);
    }

    pub fn swap(&mut self, x: usize, y: usize, x2: usize, y2: usize) {
        let index1 = self.index(x, y);
        let index2 = self.index(x2, y2);
        let mut cell = self.cells[index1].clone();
        let mut cell2 = self.cells[index2].clone();
        if cell.element == Element::Indestructible || cell2.element == Element::Indestructible {
            // Cannot edit these blocks
            return;
        }
        // cell is moved to the place of cell 2, so becomes the second cell
        cell.visited = self.visited_state;
        cell2.visited = self.visited_state;
        self.cells[index1] = cell2;
        self.cells[index2] = cell;
    }

    pub fn set_visited(&mut self, x: usize, y: usize) {
        let index = self.index(x, y);
        self.cells[index].visited = self.visited_state;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn toggle_visited_state(&mut self) -> bool {
        self.visited_state = !self.visited_state;
        self.visited_state
    }

    pub fn is_visited_state(&self) -> bool {
        self.visited_state
    }

    pub fn clear(&mut self) {
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let index = self.index(x, y);
                let mut cell = &mut self.cells[index];
                cell.element = Element::Air;
                cell.visited = self.visited_state;
            }
        }
    }

    #[inline(always)]
    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }
}

pub fn spawn_sandbox(mut commands: Commands, images: &mut Assets<Image>, width: u32, height: u32) {
    let image_handle = {
        let image = Image::new_fill(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[255, 0, 0, 255],
            TextureFormat::Rgba8UnormSrgb,
        );
        images.add(image)
    };
    commands
        .spawn(SandBox::new(width as usize, height as usize))
        .insert(SpriteBundle {
            texture: image_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });
}
