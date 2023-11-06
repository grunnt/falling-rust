use bevy::prelude::*;
use bevy::utils::Instant;

use crate::element::*;
use crate::pseudo_random::PseudoRandom;
use crate::sandbox::*;

#[derive(Clone, Resource)]
pub struct Simulation {
    pub running: bool,
    pub step: bool,
    pub frame_time_ms: u128,
    pub random: PseudoRandom,
}

impl Default for Simulation {
    fn default() -> Self {
        Simulation::new()
    }
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            running: true,
            step: false,
            frame_time_ms: 0,
            random: PseudoRandom::new(),
        }
    }
}

// System used to simulate the world a single step each frame
pub fn simulation_system(mut sandbox: Query<&mut SandBox>, mut simulation: ResMut<Simulation>) {
    match sandbox.get_single_mut() {
        Ok(mut sandbox) => {
            simulation_step(simulation.as_mut(), sandbox.as_mut());
        }
        Err(_) => {
            return;
        }
    }
}

pub fn simulation_step(simulation: &mut Simulation, sandbox: &mut SandBox) {
    let start = Instant::now();
    if simulation.running || simulation.step {
        simulation.step = false;
        let visited = sandbox.toggle_visited_state();
        let (width, height) = (sandbox.width() - 1, sandbox.height() - 1);
        for y in (1..height).rev() {
            // Switch X order every frame to avoid simulation artifacts
            if visited {
                for x in 1..width {
                    update_cell(x, y, sandbox, simulation.random.next());
                }
            } else {
                for x in (1..width).rev() {
                    update_cell(x, y, sandbox, simulation.random.next());
                }
            }
        }
    }
    let duration = Instant::now() - start;
    simulation.frame_time_ms = duration.as_millis();
}

fn update_cell(x: usize, y: usize, sandbox: &mut SandBox, random: u32) {
    // Step 1: handle interactions with surrounding cells
    let cell = sandbox.get(x, y).clone();
    if cell.visited == sandbox.is_visited_state() {
        // Visited this one already
        return;
    }
    let cell_type = element_type(cell.element);

    // Generic element effects
    if cell_type.has_flag(FLAG_IGNITES) {
        handle_igniting_cell(x, y, sandbox, random);
    }

    if cell_type.has_flag(FLAG_ACIDIC) {
        handle_acidic_cell(x, y, sandbox, random);
    }

    if cell_type.has_flag(FLAG_IS_SOURCE) {
        handle_source_cell(x, y, sandbox, cell_type, random);
    }

    // Element-specific handling
    let mut marked_as_visited = match cell.element {
        Element::Air => update_air(x, y, sandbox, random),
        Element::Water => update_water(x, y, sandbox, random),
        Element::Drain => update_drain(x, y, sandbox, random),
        Element::Fire => update_fire(x, y, sandbox, random),
        Element::Ash => update_ash(x, y, sandbox, random),
        Element::Lava => update_lava(x, y, sandbox, random),
        Element::Smoke => update_smoke(x, y, sandbox, random),
        Element::Life => update_life(x, y, sandbox, random),
        Element::Iron => update_iron(x, y, sandbox, random),
        Element::Plant => update_plant(x, y, sandbox, random),
        Element::Seed => update_seed(x, y, sandbox, random),
        Element::TNT => update_tnt(x, y, sandbox, random),
        Element::Explosion => update_explosion(x, y, sandbox, random),
        _ => false,
    };

    // Element form handling (movement)
    match cell_type.form {
        ElementForm::Solid => {}
        ElementForm::Powder => {
            marked_as_visited = handle_powder_form(sandbox, x, y, random);
        }
        ElementForm::Liquid => {
            marked_as_visited = handle_liquid_form(sandbox, x, y, random);
        }
        ElementForm::Gas => {
            marked_as_visited = handle_gas_form(sandbox, x, y, random);
        }
    }

    if !marked_as_visited {
        sandbox.set_visited(x, y);
    }
}

fn handle_igniting_cell(x: usize, y: usize, sandbox: &mut SandBox, random: u32) {
    for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        let neighbor_cell = sandbox.get(nx, ny);
        let neighbor_type = element_type(neighbor_cell.element);
        if neighbor_cell.element == Element::TNT {
            sandbox.set_element_with_strength(
                nx,
                ny,
                Element::Explosion,
                neighbor_cell.strength,
                random,
            );
        } else if neighbor_type.has_flag(FLAG_BURNS) {
            if neighbor_type.has_flag(FLAG_TURNS_INTO_ASH) && once_per(random, 3) {
                sandbox.get_mut(nx, ny).dissolve_to(Element::Ash);
            } else {
                sandbox.get_mut(nx, ny).dissolve_to(Element::Fire);
            }
        }
    }
}

fn handle_acidic_cell(x: usize, y: usize, sandbox: &mut SandBox, random: u32) {
    for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        let neighbor_cell = sandbox.get_mut(nx, ny);
        let neighbor_type = element_type(neighbor_cell.element);
        if neighbor_type.has_flag(FLAG_DISSOLVES_IN_ACID)
            && once_per(random, (neighbor_cell.strength / 2).max(2) as u32)
        {
            if sandbox.get_mut(nx, ny).dissolve_to(Element::Air) {
                if once_per(random, 2) {
                    sandbox.set_element(x, y, Element::Smoke, random);
                } else {
                    sandbox.clear_cell(x, y);
                }
            }
        }
    }
}

fn handle_powder_form(sandbox: &mut SandBox, x: usize, y: usize, random: u32) -> bool {
    // Can we fall straignt down?
    let below_element = sandbox.get(x, y + 1).element;
    let below_element_type = element_type(below_element);
    if below_element_type.form == ElementForm::Liquid || below_element_type.form == ElementForm::Gas
    {
        sandbox.swap(x, y, x, y + 1);
        return true;
    }
    // Can we slide off diagonally?
    let neighbor_x = random_neighbor_x(x, random);
    let neighbor_element = sandbox.get(neighbor_x, y + 1).element;
    let neighbor_type = element_type(neighbor_element);
    if neighbor_type.form == ElementForm::Liquid || neighbor_type.form == ElementForm::Gas {
        sandbox.swap(x, y, neighbor_x, y + 1);
        return true;
    }
    // Can we slide of diagonally the other way?
    let neighbor_x = random_other_neighbor_x(x, random);
    let neighbor_element = sandbox.get(neighbor_x, y + 1).element;
    let neighbor_type = element_type(neighbor_element);
    if neighbor_type.form == ElementForm::Liquid || neighbor_type.form == ElementForm::Gas {
        sandbox.swap(x, y, neighbor_x, y + 1);
        return true;
    }
    false
}

fn handle_liquid_form(sandbox: &mut SandBox, x: usize, y: usize, random: u32) -> bool {
    let cell = sandbox.get(x, y).clone();
    let cell_element_type = element_type(cell.element);

    let random_60 = random % 60;
    let check_x = if random_60 < 58 {
        x
    } else if random_60 == 58 {
        x - 1
    } else {
        x + 1
    };

    // Liquid falls down in gas or when heavier than the element below
    let below_element = sandbox.get(check_x, y + 1).element;
    let below_element_type = element_type(below_element);
    if below_element_type.form == ElementForm::Gas
        || (below_element_type.form == ElementForm::Liquid
            && below_element != cell.element
            && below_element_type.weight < cell_element_type.weight
            && once_per(random, 3))
    {
        sandbox.swap(x, y, check_x, y + 1);
        return true;
    }

    // Liquid flows sideways. Strength of the cell indicates the speed of sideways flow.
    let check_left = once_per(random, 2);
    for n in 1..cell.strength as usize {
        let check_x_opt = if check_left {
            if x > n {
                Some(x - n)
            } else {
                None
            }
        } else {
            if x + n < sandbox.width() - 1 {
                Some(x + n)
            } else {
                None
            }
        };
        if let Some(check_x) = check_x_opt {
            let neighbor = sandbox.get(check_x, y);
            let neighbor_element_type = element_type(neighbor.element);
            if neighbor_element_type.form == ElementForm::Gas
                || (neighbor_element_type.form == ElementForm::Liquid
                    && neighbor.element != cell.element
                    && neighbor_element_type.weight < cell_element_type.weight
                    && once_per(random, 3))
            {
                // Slide sideways
                sandbox.swap(x, y, check_x, y);
                return true;
            }
            if neighbor.element != cell.element {
                break;
            }
        } else {
            break;
        }
    }

    true
}

fn handle_gas_form(sandbox: &mut SandBox, x: usize, y: usize, random: u32) -> bool {
    let cell = sandbox.get(x, y).clone();
    let cell_element_type = element_type(cell.element);

    // Move in a random direction, with a tendency upwards
    let (nx, ny) = match random % 4 {
        0 => (x + 1, y),
        1 => (x - 1, y),
        _ => (x, y - 1),
    };
    let neighbor_element = sandbox.get(nx, ny).element;
    let neighbor_element_type = element_type(neighbor_element);
    if neighbor_element_type.form == ElementForm::Gas
        && cell.element != neighbor_element
        && neighbor_element_type.weight > cell_element_type.weight
        && (cell.element == Element::Air || once_per(random, 2))
    {
        sandbox.swap(x, y, nx, ny);
        return true;
    }
    false
}

fn handle_source_cell(
    x: usize,
    y: usize,
    sandbox: &mut SandBox,
    cell_type: &ElementType,
    random: u32,
) {
    for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        if sandbox.get(nx, ny).element == Element::Air {
            sandbox.set_element(nx, ny, cell_type.source_element, random);
        }
    }
}

fn update_water(x: usize, y: usize, sandbox: &mut SandBox, random: u32) -> bool {
    let (nx, ny) = match random % 4 {
        0 => (x - 1, y),
        1 => (x + 1, y),
        2 => (x, y - 1),
        _ => (x, y + 1),
    };
    let neighbor_element = sandbox.get(nx, ny).element;
    match neighbor_element {
        Element::Acid => {
            sandbox.get_mut(nx, ny).dissolve_to(Element::Water);
            return false;
        }
        Element::Lava => {
            if sandbox.get_mut(nx, ny).dissolve_to(Element::Rock) {
                sandbox.clear_cell(x, y);
            }
            return false;
        }
        Element::Fire => {
            sandbox.clear_cell(x, y);
            sandbox.set_element(nx, ny, Element::Water, random);
            return true;
        }
        _ => {}
    }
    false
}

fn update_drain(x: usize, y: usize, sandbox: &mut SandBox, _random: u32) -> bool {
    // Remove any liquid on top, left or right of this cell
    let element_form = element_type(sandbox.get(x, y - 1).element).form;
    if element_form == ElementForm::Liquid {
        sandbox.clear_cell(x, y - 1);
        return true;
    }
    let element_form = element_type(sandbox.get(x - 1, y).element).form;
    if element_form == ElementForm::Liquid {
        sandbox.clear_cell(x - 1, y);
        return true;
    }
    let element_form = element_type(sandbox.get(x + 1, y).element).form;
    if element_form == ElementForm::Liquid {
        sandbox.clear_cell(x + 1, y);
        return true;
    }
    false
}

fn update_fire(x: usize, y: usize, sandbox: &mut SandBox, random: u32) -> bool {
    // Reduce fire strength over time
    if once_per(random, 2) && sandbox.get_mut(x, y).dissolve_to(Element::Air) {
        sandbox.set_element(x, y, Element::Smoke, random);
        return true;
    }
    false
}

fn update_ash(x: usize, y: usize, sandbox: &mut SandBox, random: u32) -> bool {
    if once_per(random, 100) && sandbox.get_mut(x, y).dissolve_to(Element::Air) {
        return true;
    }
    false
}

fn update_lava(x: usize, y: usize, sandbox: &mut SandBox, random: u32) -> bool {
    let cell = sandbox.get_mut(x, y);
    // Cool down when no longer at max hotness
    if once_per(random, 2) && cell.strength < element_type(Element::Lava).strength {
        if sandbox.get_mut(x, y).dissolve_to(Element::Rock) {
            return true;
        }
    }
    // Give off sparks
    if once_per(random, 100) && sandbox.get(x, y - 1).element == Element::Air {
        sandbox.set_element(x, y - 1, Element::Fire, random);
    }
    false
}

fn update_smoke(x: usize, y: usize, sandbox: &mut SandBox, random: u32) -> bool {
    if once_per(random, 2) && sandbox.get_mut(x, y).dissolve_to(Element::Air) {
        sandbox.clear_cell(x, y);
        return true;
    }
    false
}

fn update_iron(x: usize, y: usize, sandbox: &mut SandBox, random: u32) -> bool {
    let mut rusty_neighbor = false;
    for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        let element = sandbox.get(nx, ny).element;
        if element_type(element).has_flag(FLAG_CAUSES_RUST) {
            rusty_neighbor = true;
            break;
        }
    }
    if rusty_neighbor {
        // Rust iron by reducing its strength somewhat randomly
        if once_per(random, 3) && !sandbox.reduce_strength(x, y, 1) {
            // Turn into rust
            sandbox.set_element(x, y, Element::Rust, random);
            return true;
        }
    }
    false
}

fn update_seed(x: usize, y: usize, sandbox: &mut SandBox, random: u32) -> bool {
    // Check if we have water and nutrition
    let mut nutrition = false;
    let mut water = false;
    for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        let neighbor_element = sandbox.get(nx, ny).element;
        if !nutrition && element_type(neighbor_element).has_flag(FLAG_NUTRITIOUS) {
            nutrition = true;
        }
        if !water && element_type(neighbor_element).has_flag(FLAG_WET) {
            water = true;
        }
    }

    if nutrition && water {
        // Convert to a new plant
        sandbox.set_element_with_strength(
            x,
            y,
            Element::Plant,
            element_type(Element::Seed).strength,
            random,
        );
        sandbox.get_mut(x, y).variant = element_type(Element::Seed).strength;
        true
    } else {
        false
    }
}

fn update_plant(x: usize, y: usize, sandbox: &mut SandBox, random: u32) -> bool {
    let (cell_strength, cell_variant) = {
        let cell = sandbox.get(x, y);
        (cell.strength, cell.variant)
    };
    if cell_variant <= 1 {
        // Sometimes turns into seed
        if once_per(random, 5) {
            sandbox.set_element(x, y, Element::Seed, random);
        }
    }

    // Are we still attached to the plant?
    let mut attached = false;
    if cell_variant == element_type(Element::Seed).strength {
        // Root cell
        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            let neighbor = sandbox.get(nx, ny);
            if neighbor.element != Element::Plant
                && element_type(neighbor.element).has_flag(FLAG_NUTRITIOUS)
            {
                attached = true;
                break;
            }
        }
    } else {
        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            let neighbor = sandbox.get(nx, ny);
            if neighbor.element == Element::Plant && neighbor.variant > cell_variant {
                attached = true;
                break;
            }
        }
    }
    if !attached {
        // Not attached, so die
        sandbox.set_element(x, y, Element::Ash, random);
        return true;
    }
    if cell_strength <= 1 {
        // Not growing anymore
        return false;
    }
    // Plant is still growing
    let (nx, ny) = match random % 1000 {
        0 | 1 => (x - 1, y),
        2 | 3 => (x + 1, y),
        4..=100 => (x, y - 1),
        _ => return false,
    };
    let other_element = sandbox.get(nx, ny).element;
    let new_cell_strength = cell_strength - 1;
    if element_type(other_element).has_flag(FLAG_ALLOW_PLANT) {
        sandbox.set_element_with_strength(nx, ny, Element::Plant, new_cell_strength, random);
        sandbox.get_mut(nx, ny).variant = cell_variant - 1;
        sandbox.reduce_strength(x, y, new_cell_strength);
    }

    false
}

fn update_tnt(x: usize, y: usize, sandbox: &mut SandBox, random: u32) -> bool {
    let strength = sandbox.get(x, y).strength;
    if strength == element_type(Element::TNT).strength {
        return false;
    }
    sandbox.set_element_with_strength(x, y, Element::Explosion, strength, random);
    true
}

fn update_explosion(x: usize, y: usize, sandbox: &mut SandBox, random: u32) -> bool {
    if !sandbox.reduce_strength(x, y, 1) {
        sandbox.set_element(x, y, Element::Fire, random);
        return true;
    }
    // Spread explosion
    let strength = sandbox.get(x, y).strength;
    let neighbors = match random % 2 {
        0 => [(x - 1, y), (x + 1, y)],
        _ => [(x, y - 1), (x, y + 1)],
    };
    for (nx, ny) in neighbors {
        let neighbor = sandbox.get_mut(nx, ny);
        if neighbor.element == Element::TNT {
            let explosion_strength = if neighbor.strength + strength < 255 {
                neighbor.strength + strength
            } else {
                neighbor.strength
            };
            sandbox.set_element_with_strength(
                nx,
                ny,
                Element::Explosion,
                explosion_strength,
                random,
            );
        } else if neighbor.element != Element::Explosion {
            let neighbor_type = element_type(neighbor.element);
            let neighbor_strength = if neighbor_type.has_flag(FLAG_BLAST_RESISTANT) {
                neighbor.strength
            } else {
                0
            };
            if neighbor_strength < strength {
                sandbox.set_element_with_strength(
                    nx,
                    ny,
                    Element::Explosion,
                    strength - neighbor_strength,
                    random,
                );
            }
        }
    }
    true
}

fn update_air(x: usize, y: usize, sandbox: &mut SandBox, random: u32) -> bool {
    let mut living_neighbors = 0;
    if sandbox.get(x - 1, y - 1).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x, y - 1).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x + 1, y - 1).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x - 1, y).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x + 1, y).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x - 1, y + 1).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x, y + 1).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x + 1, y + 1).element == Element::Life {
        living_neighbors += 1;
    }
    if living_neighbors == 3 {
        sandbox.set_element(x, y, Element::Life, random);
        return true;
    }
    false
}

fn update_life(x: usize, y: usize, sandbox: &mut SandBox, random: u32) -> bool {
    let mut living_neighbors = 0;
    if sandbox.get(x - 1, y - 1).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x, y - 1).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x + 1, y - 1).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x - 1, y).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x + 1, y).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x - 1, y + 1).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x, y + 1).element == Element::Life {
        living_neighbors += 1;
    }
    if sandbox.get(x + 1, y + 1).element == Element::Life {
        living_neighbors += 1;
    }
    if living_neighbors < 2 || living_neighbors > 3 {
        sandbox.set_element(x, y, Element::Air, random);
        return true;
    }
    // Keep on living
    false
}

pub fn random_neighbor_x(x: usize, random: u32) -> usize {
    if random % 2 == 0 {
        x + 1
    } else {
        x - 1
    }
}

pub fn random_other_neighbor_x(x: usize, random: u32) -> usize {
    if random % 2 == 0 {
        x - 1
    } else {
        x + 1
    }
}

fn once_per(random: u32, count: u32) -> bool {
    random % count == 0
}
