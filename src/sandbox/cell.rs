use crate::sandbox::*;

// A cell that contains the state of a single pixel in the sand box.
#[derive(Clone, Debug)]
pub struct Cell {
    // Element in this cell
    pub element: Element,
    // Generic data fields, usage depends on element
    pub variant: u8,
    pub strength: u8,
    // Toggles each simulation step, to avoid duplicate simulation
    pub visited: bool,
}

impl Cell {
    // Reduce strength and turn into the given element of strength is zero
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
}
