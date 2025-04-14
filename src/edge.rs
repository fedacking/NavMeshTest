use std::hash::Hash;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Edge (
    pub geo::Point,
    pub geo::Point,
);

impl Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0.x().to_bits());
        state.write_u64(self.0.y().to_bits());
        state.write_u64(self.1.x().to_bits());
        state.write_u64(self.1.y().to_bits());
    }
}

impl Eq for Edge {}

impl Edge {
    pub fn new<'a>(mut p1: &'a geo::Point, mut p2: &'a geo::Point) -> Edge {
        if p1.x() > p2.x() || (p1.x() == p2.x() && p1.y() > p2.y()) {
            (p1, p2) = (p2, p1);
        }
        Edge(p1.clone(), p2.clone())
    }
}