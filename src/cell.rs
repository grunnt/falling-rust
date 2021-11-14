use crate::element::Element;

#[derive(Clone, Debug)]
pub struct Cell {
    pub element: Element,
    pub variant: u8,
    pub strength: u8,
    pub visited: bool,
    pub source: bool,
}

impl Cell {
    pub fn dissolve_to(&mut self, element: Element) -> bool {
        if self.strength > 0 {
            self.strength -= 1;
            false
        } else {
            self.element = element;
            self.strength = element.strength();
            true
        }
    }
}
