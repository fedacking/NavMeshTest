use std::hash::{Hash, Hasher};
use macroquad::math::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Edge (pub Vec2, pub Vec2);

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for Edge {

}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0.x < self.1.x {
            state.write_u32(self.0.x.to_bits());
            state.write_u32(self.0.y.to_bits());
            state.write_u32(self.1.x.to_bits());
            state.write_u32(self.1.y.to_bits());
        } else {
            state.write_u32(self.1.x.to_bits());
            state.write_u32(self.1.y.to_bits());
            state.write_u32(self.0.x.to_bits());
            state.write_u32(self.0.y.to_bits());
        }
    }
}
