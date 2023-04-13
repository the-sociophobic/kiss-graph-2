// data.rs

use bevy::prelude::*;

pub struct CustomPoint {
    pub name: String,
    pub pos: Vec3,
    pub color: Color,
}

impl CustomPoint {
    pub fn new(name: &str, pos: Vec3, color: Color) -> Self {
        CustomPoint {
            name: name.to_string(),
            pos,
            color,
        }
    }
}

pub fn data() -> Vec<CustomPoint> {
    vec![
        CustomPoint::new("Lef", Vec3::new(0.0, 0.0, 0.0), Color::VIOLET),
        CustomPoint::new("ADHD", Vec3::new(0.0, 5.0, 0.0), Color::GREEN),
        CustomPoint::new("VFX", Vec3::new(0.0, 0.0, 5.0), Color::RED),
    ]
}
