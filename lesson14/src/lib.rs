// src/lib.rs
pub mod garden;

pub fn eat_at_restaurant() {
    println!("Eating at restaurant");
    garden::eat_at_garden();
}

// src/garden.rs
pub mod vegetables;

pub fn eat_at_garden() {
    println!("Eating in the garden");
    vegetables::eat_vegetable();
}

// src/garden/vegetables.rs
pub fn eat_vegetable() {
    println!("Eating a vegetable");
}