use crate::element::{Element, ElementForm};
use crate::sandbox::*;
use bevy::prelude::*;
use bevy::utils::Instant;

#[derive(Clone, Debug, Resource)]
pub struct Simulation {
    pub running: bool,
    pub step: bool,
    pub frame_time_ms: u128,
}

impl Default for Simulation {
    fn default() -> Self {
        Self {
            running: true,
            step: false,
            frame_time_ms: 0,
        }
    }
}

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

pub fn simulation_step(mut simulation: &mut Simulation, sandbox: &mut SandBox) {
    let start = Instant::now();
    if simulation.running || simulation.step {
        simulation.step = false;
        let visited = sandbox.toggle_visited_state();
        let (width, height) = (sandbox.width() - 1, sandbox.height() - 1);
        for y in (1..height).rev() {
            // Switch X order every frame to avoid simulation artifacts
            if visited {
                for x in 1..width {
                    update_cell(x, y, sandbox);
                }
            } else {
                for x in (1..width).rev() {
                    update_cell(x, y, sandbox);
                }
            }
        }
    }
    let duration = Instant::now() - start;
    simulation.frame_time_ms = duration.as_millis();
}

fn update_cell(x: usize, y: usize, sandbox: &mut SandBox) {
    let cell = sandbox.get(x, y);
    if cell.visited == sandbox.is_visited_state() {
        // Visited this one already
        return;
    }
    let marked_as_visited = match cell.element {
        Element::Air => update_air(x, y, sandbox),
        Element::Sand => update_sand(x, y, sandbox),
        Element::Water => update_water(x, y, sandbox),
        Element::Acid => update_acid(x, y, sandbox),
        Element::Oil => update_oil(x, y, sandbox),
        Element::Drain => update_drain(x, y, sandbox),
        Element::Fire => update_fire(x, y, sandbox),
        Element::Ash => update_ash(x, y, sandbox),
        Element::Lava => update_lava(x, y, sandbox),
        Element::Smoke => update_smoke(x, y, sandbox),
        Element::Life => update_life(x, y, sandbox),
        Element::Iron => update_iron(x, y, sandbox),
        Element::Rust => update_sand(x, y, sandbox),
        Element::Plant => update_plant(x, y, sandbox),
        Element::Seed => update_seed(x, y, sandbox),
        Element::TNT => update_tnt(x, y, sandbox),
        Element::Explosion => update_explosion(x, y, sandbox),
        Element::Fuse => update_fuse(x, y, sandbox),
        Element::Wood => false,
        Element::Rock => false,
        Element::Indestructible => false,
        Element::WaterSource => update_source(x, y, Element::Water, sandbox),
        Element::AcidSource => update_source(x, y, Element::Acid, sandbox),
        Element::OilSource => update_source(x, y, Element::Oil, sandbox),
        Element::LavaSource => update_source(x, y, Element::Lava, sandbox),
        Element::FireSource => update_source(x, y, Element::Fire, sandbox),
    };
    if !marked_as_visited {
        sandbox.set_visited(x, y);
    }
}

fn update_sand(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    let element_below = sandbox.get(x, y + 1).element;
    if element_below == Element::Air
        || element_below == Element::Water
        || element_below == Element::Fire
        || element_below == Element::Oil
    {
        // Fall down
        sandbox.swap(x, y, x, y + 1);
        return true;
    }
    if element_below == Element::Acid {
        // Dissolve into the acid below
        if sandbox.get_mut(x, y).dissolve_to(Element::Air) {
            sandbox.clear_cell(x, y + 1);
            return false;
        } else {
            sandbox.swap(x, y, x, y + 1);
            return true;
        }
    }
    let neighbour_x = sandbox.random_neighbour_x(x);
    let neighbour_element = sandbox.get(neighbour_x, y + 1).element;
    if neighbour_element == Element::Air
        || neighbour_element == Element::Water
        || neighbour_element == Element::Fire
        || neighbour_element == Element::Oil
    {
        // Slide to random neighbour diagonally
        sandbox.swap(x, y, neighbour_x, y + 1);
        return true;
    }
    if neighbour_element == Element::Acid {
        // Dissolve in acid diagonally
        if sandbox
            .get_mut(neighbour_x, y + 1)
            .dissolve_to(Element::Air)
        {
            sandbox.clear_cell(x, y + 1);
            return false;
        } else {
            sandbox.swap(x, y, neighbour_x, y + 1);
            return true;
        }
    }
    false
}

fn update_water(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    let random = sandbox.random(60);
    let check_x = if random < 58 {
        x
    } else if random == 58 {
        x - 1
    } else {
        x + 1
    };
    // First see what hapens if we touch the below neighbour
    if let Some(value) = touch_water(sandbox, x, y, check_x, y + 1, random) {
        return value;
    }
    // Water flows sideways
    for n in 1..16 {
        let check_x_opt = if random < 30 {
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
            let neighbour = sandbox.get(check_x, y);
            let neighbour_element = neighbour.element;
            if let Some(value) = touch_water(sandbox, x, y, check_x, y, random) {
                return value;
            }
            if neighbour_element != Element::Water {
                break;
            }
        }
    }
    return false;
}

fn touch_water(
    sandbox: &mut SandBox,
    water_x: usize,
    water_y: usize,
    other_x: usize,
    other_y: usize,
    random: usize,
) -> Option<bool> {
    let other_element = sandbox.get(other_x, other_y).element;
    if other_element == Element::Air || other_element == Element::Oil {
        sandbox.swap(water_x, water_y, other_x, other_y);
        return Some(true);
    }
    if other_element == Element::Acid {
        sandbox
            .get_mut(other_x, other_y)
            .dissolve_to(Element::Water);
        if water_y < other_y && random % 2 == 0 {
            sandbox.swap(water_x, water_y, other_x, other_y)
        }
        return Some(false);
    }
    if other_element == Element::Lava {
        // Lava cools down
        if sandbox.get_mut(other_x, other_y).dissolve_to(Element::Rock) {
            sandbox.clear_cell(water_x, water_y);
        }
        return Some(false);
    }
    if other_element == Element::Fire {
        sandbox.clear_cell(water_x, water_y);
        sandbox.set_element(other_x, other_y, Element::Water);
        return Some(true);
    }
    None
}

fn update_acid(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    let random = sandbox.random(60);
    let check_x = if random < 50 {
        x
    } else if random < 55 {
        x - 1
    } else {
        x + 1
    };
    let element_below = sandbox.get(check_x, y + 1).element;
    if element_below == Element::Air || element_below == Element::Fire {
        // Acid falls down in air and fire
        sandbox.swap(x, y, check_x, y + 1);
        return true;
    }
    if element_below == Element::Water {
        // Acid turns to water when in contact
        sandbox.get_mut(x, y).dissolve_to(Element::Water);
        return false;
    }
    if element_below.dissolves_in_acid() {
        if sandbox.get_mut(check_x, y + 1).dissolve_to(Element::Air) {
            sandbox.clear_cell(x, y);
            return true;
        }
        return false;
    }
    // Acid flows sideways in air (somewhat more slowly than water)
    for n in 1..8 {
        let check_x_opt = if random < 30 {
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
            let neighbour = sandbox.get(check_x, y);
            let neighbour_element = neighbour.element;
            if neighbour_element == Element::Air {
                sandbox.swap(x, y, check_x, y);
                return true;
            }
            if neighbour_element.dissolves_in_acid() {
                if sandbox.get_mut(check_x, y).dissolve_to(Element::Air) {
                    sandbox.clear_cell(x, y);
                    return true;
                }
                return true;
            }
            if neighbour_element != Element::Acid {
                break;
            }
        }
    }
    false
}

fn update_oil(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    let random = sandbox.random(500);
    let check_x = if random > 50 {
        x
    } else if random > 25 {
        x - 1
    } else {
        x + 1
    };
    let element_below = sandbox.get(check_x, y + 1).element;
    if element_below == Element::Air || element_below == Element::Acid {
        // Oil falls down in air and acid
        sandbox.swap(x, y, check_x, y + 1);
        return true;
    }
    // Oil flows sideways in air and water (somewhat more slowly than water)
    for n in 1..8 {
        let check_x_opt = if random < 250 {
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
            let neighbour = sandbox.get(check_x, y);
            let neighbour_element = neighbour.element;
            if neighbour_element == Element::Air || (n == 1 && neighbour_element == Element::Acid) {
                sandbox.swap(x, y, check_x, y);
                return true;
            }
            if neighbour_element != Element::Oil {
                break;
            }
        }
    }
    false
}

fn update_drain(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    // Remove any liquid on top, left or right of this cell
    let element_form = sandbox.get(x, y - 1).element.form();
    if element_form == ElementForm::Liquid {
        sandbox.clear_cell(x, y - 1);
        return true;
    }
    let element_form = sandbox.get(x - 1, y).element.form();
    if element_form == ElementForm::Liquid {
        sandbox.clear_cell(x - 1, y);
        return true;
    }
    let element_form = sandbox.get(x + 1, y).element.form();
    if element_form == ElementForm::Liquid {
        sandbox.clear_cell(x + 1, y);
        return true;
    }
    false
}

fn update_fire(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    let random = sandbox.random(8);
    // Reduce fire strength over time
    if random > 4 && !sandbox.reduce_strength(x, y, 1) {
        sandbox.set_element(x, y, Element::Smoke);
        return true;
    }
    // Make fire flicker
    let cell = sandbox.get_mut(x, y);
    cell.variant = (cell.variant + random as u8 * 10) % 255;
    // Move in a random direction, with a tendency upwards
    let (nx, ny) = match random {
        0 => (x, y + 1),
        1 => (x + 1, y),
        2 => (x - 1, y),
        3 => (x, y - 1),
        _ => return false,
    };
    let element = sandbox.get(nx, ny).element;
    if element == Element::Air {
        sandbox.swap(x, y, nx, ny);
        return true;
    }
    if element == Element::TNT || element == Element::Fuse {
        sandbox.get_mut(nx, ny).strength -= 1;
        return false;
    }
    if element.burns() {
        if element.form() == ElementForm::Solid && random > 3 {
            // Sometimes burnable elements turn into ash
            sandbox.get_mut(nx, ny).dissolve_to(Element::Ash);
        } else {
            sandbox.get_mut(nx, ny).dissolve_to(Element::Fire);
        }
        return false;
    }
    false
}

fn update_ash(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    update_sand(x, y, sandbox)
}

fn update_lava(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    let random = sandbox.random(500);
    let cell = sandbox.get_mut(x, y);
    // Cool down when no longer at max hotness
    if random < 250 && cell.strength < 64 {
        if sandbox.get_mut(x, y).dissolve_to(Element::Rock) {
            return true;
        }
    }
    // Give off sparks
    if random == 0 && sandbox.get(x, y - 1).element == Element::Air {
        sandbox.set_element(x, y - 1, Element::Fire);
    }
    // Fall down
    if let Some(visited) = touch_lava(sandbox, x, y, x, y + 1) {
        return visited;
    }
    // Slide down diagonally
    let neighbour_x = sandbox.random_neighbour_x(x);
    if let Some(visited) = touch_lava(sandbox, x, y, neighbour_x, y + 1) {
        return visited;
    }
    // Slide horizontally
    if let Some(visited) = touch_lava(sandbox, x, y, neighbour_x, y) {
        return visited;
    }
    false
}

fn touch_lava(
    sandbox: &mut SandBox,
    lava_x: usize,
    lava_y: usize,
    other_x: usize,
    other_y: usize,
) -> Option<bool> {
    let element = sandbox.get(other_x, other_y).element;
    if element == Element::Air
        || element == Element::Acid
        || element == Element::Water
        || element == Element::Fire
    {
        sandbox.swap(lava_x, lava_y, other_x, other_y);
        return Some(true);
    }
    if element.burns() {
        sandbox.get_mut(other_x, other_y).dissolve_to(Element::Fire);
        return Some(false);
    }
    None
}

fn update_smoke(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    let random = sandbox.random(5);
    // Reduce fire strength over time
    if random > 2 && !sandbox.reduce_strength(x, y, 1) {
        sandbox.clear_cell(x, y);
        return true;
    }
    // Move in a random direction, with a tendency upwards
    let (nx, ny) = match random {
        0 => (x + 1, y),
        1 => (x - 1, y),
        _ => (x, y - 1),
    };
    let neighbour_element = sandbox.get(nx, ny).element;
    if neighbour_element == Element::Air {
        sandbox.swap(x, y, nx, ny);
        return true;
    }
    if neighbour_element == Element::Fire || neighbour_element.form() == ElementForm::Liquid {
        sandbox.clear_cell(x, y);
        return true;
    }
    false
}

fn update_iron(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    let rusty_neighbour = sandbox.get(x - 1, y).element.causes_rust()
        || sandbox.get(x + 1, y).element.causes_rust()
        || sandbox.get(x, y - 1).element.causes_rust()
        || sandbox.get(x, y + 1).element.causes_rust();

    if rusty_neighbour {
        // Rust iron by reducing its strength somewhat randomly
        let random = sandbox.random(5);
        if random > 2 && !sandbox.reduce_strength(x, y, 1) {
            // Turn into rust
            sandbox.set_element(x, y, Element::Rust);
            return true;
        }
    }
    false
}

fn update_seed(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    // See if we need to fall down or slide diagonally
    let element_below = sandbox.get(x, y + 1).element;
    if element_below == Element::Air
        || element_below == Element::Water
        || element_below == Element::Fire
        || element_below == Element::Oil
    {
        // Fall down
        sandbox.swap(x, y, x, y + 1);
        return true;
    }

    let neighbour_x = sandbox.random_neighbour_x(x);
    let neighbour_element = sandbox.get(neighbour_x, y + 1).element;
    if neighbour_element == Element::Air
        || neighbour_element == Element::Water
        || neighbour_element == Element::Fire
        || neighbour_element == Element::Oil
    {
        // Slide to random neighbour diagonally
        sandbox.swap(x, y, neighbour_x, y + 1);
        return true;
    }

    // Check if we have water and nutrition
    let mut nutrition = false;
    let mut water = false;
    for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        let neighbour_element = sandbox.get(nx, ny).element;
        if !nutrition && neighbour_element.plant_nutrition() {
            nutrition = true;
        }
        if !water && neighbour_element.plant_watering() {
            water = true;
        }
    }

    if nutrition && water {
        // Convert to a new plant
        sandbox.set_element_with_strength(x, y, Element::Plant, Element::Seed.strength());
        sandbox.get_mut(x, y).variant = Element::Seed.strength();
        true
    } else {
        false
    }
}

fn update_plant(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    let random = sandbox.random(1000);
    let (cell_strength, cell_variant) = {
        let cell = sandbox.get(x, y);
        (cell.strength, cell.variant)
    };
    if cell_variant <= 1 {
        // Sometimes turns into seed
        if random > 990 {
            sandbox.set_element(x, y, Element::Seed);
        }
    }

    // Are we still attached to the plant?
    let mut attached = false;
    if cell_variant == Element::Seed.strength() {
        // Root cell
        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            let neighbour = sandbox.get(nx, ny);
            if neighbour.element != Element::Plant && neighbour.element.plant_nutrition() {
                attached = true;
                break;
            }
        }
    } else {
        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            let neighbour = sandbox.get(nx, ny);
            if neighbour.element == Element::Plant && neighbour.variant > cell_variant {
                attached = true;
                break;
            }
        }
    }
    if !attached {
        // Not attached, so die
        sandbox.set_element(x, y, Element::Ash);
        return true;
    }
    if cell_strength <= 1 {
        // Not growing anymore
        return false;
    }
    // Plant is still growing
    if random > 970 {
        let random = random - 980;
        let (nx, ny) = match random {
            0 | 1 => (x - 1, y),
            2 | 3 => (x + 1, y),
            _ => (x, y - 1),
        };
        let other_element = sandbox.get(nx, ny).element;
        let new_cell_strength = cell_strength - 1;
        if other_element.allows_plant_growth() {
            sandbox.set_element_with_strength(nx, ny, Element::Plant, new_cell_strength);
            sandbox.get_mut(nx, ny).variant = cell_variant - 1;
            sandbox.reduce_strength(x, y, new_cell_strength);
        }
    }
    false
}

fn update_tnt(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    let strength = sandbox.get(x, y).strength;
    if strength == Element::TNT.strength() {
        return false;
    }
    sandbox.set_element_with_strength(x, y, Element::Explosion, strength);
    true
}

fn update_explosion(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    if !sandbox.reduce_strength(x, y, 1) {
        sandbox.set_element(x, y, Element::Fire);
        return true;
    }
    let random = sandbox.random(1000);
    // Make explosion flicker
    let cell = sandbox.get_mut(x, y);
    cell.variant = (cell.variant + random as u8 * 20) % 255;
    // Spread explosion
    let strength = sandbox.get(x, y).strength;
    // TODO optimize
    let neighbours = match random % 2 {
        0 => [(x - 1, y), (x + 1, y)],
        _ => [(x, y - 1), (x, y + 1)],
    };
    for (nx, ny) in neighbours {
        let neighbour = sandbox.get_mut(nx, ny);
        if neighbour.element == Element::TNT {
            let explosion_strength = if neighbour.strength + strength < 255 {
                neighbour.strength + strength
            } else {
                neighbour.strength
            };
            sandbox.set_element_with_strength(nx, ny, Element::Explosion, explosion_strength);
        } else if neighbour.element != Element::Explosion {
            let blast_resistance = neighbour.element.blast_resistance();
            if blast_resistance < strength {
                sandbox.set_element_with_strength(
                    nx,
                    ny,
                    Element::Explosion,
                    strength - blast_resistance,
                );
            }
        }
    }
    true
}

fn update_fuse(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    if sandbox.get(x, y).strength == Element::Fuse.strength() {
        return false;
    }
    if sandbox.get_mut(x, y).dissolve_to(Element::Fire) {
        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            let neighbour = sandbox.get_mut(nx, ny);
            if neighbour.element == Element::Fuse
                && neighbour.element.strength() == Element::Fuse.strength()
            {
                neighbour.strength -= 1;
            } else if neighbour.element == Element::TNT
                && neighbour.element.strength() == Element::TNT.strength()
            {
                neighbour.strength -= 1;
            }
        }
        return true;
    }
    false
}

fn update_source(x: usize, y: usize, element: Element, sandbox: &mut SandBox) -> bool {
    for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
        if sandbox.get(nx, ny).element == Element::Air {
            sandbox.set_element(nx, ny, element);
            return true;
        }
    }
    false
}

fn update_air(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    let mut living_neighbours = 0;
    if sandbox.get(x - 1, y - 1).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x, y - 1).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x + 1, y - 1).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x - 1, y).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x + 1, y).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x - 1, y + 1).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x, y + 1).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x + 1, y + 1).element == Element::Life {
        living_neighbours += 1;
    }
    if living_neighbours == 3 {
        sandbox.set_element(x, y, Element::Life);
        return true;
    }
    false
}

fn update_life(x: usize, y: usize, sandbox: &mut SandBox) -> bool {
    let mut living_neighbours = 0;
    if sandbox.get(x - 1, y - 1).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x, y - 1).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x + 1, y - 1).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x - 1, y).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x + 1, y).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x - 1, y + 1).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x, y + 1).element == Element::Life {
        living_neighbours += 1;
    }
    if sandbox.get(x + 1, y + 1).element == Element::Life {
        living_neighbours += 1;
    }
    if living_neighbours < 2 || living_neighbours > 3 {
        sandbox.set_element(x, y, Element::Air);
        return true;
    }
    // Keep on living
    false
}
