use crate::element::*;

#[derive(Clone, Debug)]
pub struct Cell {
    pub element: Element,
    pub variant: u8,
    pub strength: u8,
    pub visited: bool,
}

impl Cell {
    pub fn dissolve_to(&mut self, element: Element) -> bool {
        if self.strength > 0 {
            self.strength -= 1;
            false
        } else {
            self.element = element;
            self.strength = element_type(element).strength;
            true
        }
    }

    pub fn dissolve_to_with_speed(&mut self, element: Element, speed: u8) -> bool {
        if self.strength > speed {
            self.strength -= speed;
            false
        } else {
            self.element = element;
            self.strength = element_type(element).strength;
            true
        }
    }

    pub fn clear(&mut self) {
        self.element = Element::Air;
        self.strength = element_type(Element::Air).strength;
        self.variant = 0;
    }
}
