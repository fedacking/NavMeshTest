use macroquad::math::Vec2;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub struct Edge(pub Vec2, pub Vec2);

impl Edge {
    pub fn new(v1: Vec2, v2: Vec2) -> Edge {
        if v1.x < v2.x || (v1.x == v2.x && v1.y < v2.y) {
            return Edge(v1, v2);
        }
        Edge(v2, v1)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0.x.to_bits());
        state.write_u32(self.0.y.to_bits());
        state.write_u32(self.1.x.to_bits());
        state.write_u32(self.1.y.to_bits());
    }
}

impl Edge {
    fn on_segment(&self, point: Vec2) -> bool {
         (self.1.x > point.x) && (point.x > self.0.x) &&
        ((self.1.y > point.y) && (point.y > self.0.y) ||
         (self.1.y < point.y) && (point.y < self.0.y))
    }

    pub fn get_intersection_point(&self, other: &Edge) -> Option<Vec2> {
        let a1 = self.1.y - self.0.y;
        let b1 = self.0.x - self.1.x;
        let c1 = a1 * self.1.x + b1 * self.1.y;

        let a2 = other.1.y - other.0.y;
        let b2 = other.0.x - other.1.x;
        let c2 = a2 * other.0.x + b2 * other.0.y;

        let determinant = a1 * b2 - a2 * b1;

        if determinant.abs() < f32::EPSILON {
            return None;
        }

        let x = (b2 * c1 - b1 * c2) / determinant;
        let y = (a1 * c2 - a2 * c1) / determinant;

        let point = Vec2::new(x, y);
        if self.on_segment(point) && other.on_segment(point) {
            Some(point)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NavTriangle {
    pub coordinates: [Vec2; 3],
}

impl NavTriangle {
    pub fn from_coordinates(mut coordinates: [Vec2; 3]) -> NavTriangle {
        let det = coordinates[0].x * (coordinates[1].y - coordinates[2].y)
            + coordinates[1].x * (coordinates[2].y - coordinates[0].y)
            + coordinates[2].x * (coordinates[0].y - coordinates[1].y);
        if det > 0.0 {
            (coordinates[1], coordinates[0]) = (coordinates[0], coordinates[1]);
        }
        NavTriangle { coordinates }
    }
}

impl PartialEq for NavTriangle {
    fn eq(&self, other: &Self) -> bool {
        let a = &self.coordinates;
        let b = other.coordinates;
        (a[0] == b[0] && a[1] == b[1] && a[2] == b[2])
            || (a[0] == b[1] && a[1] == b[2] && a[2] == b[0])
            || (a[0] == b[0] && a[1] == b[1] && a[2] == b[2])
    }
}

impl NavTriangle {
    pub fn point_in_triangle(&self, point: Vec2) -> bool {
        let v0 = self.coordinates[1] - self.coordinates[0];
        let v1 = self.coordinates[2] - self.coordinates[0];
        let v2 = point - self.coordinates[0];

        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);

        let denom = d00 * d11 - d01 * d01;
        if denom == 0.0 {
            return false; // Degenerate triangle
        }

        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;

        u >= 0.0 && v >= 0.0 && w >= 0.0
    }

    pub fn point_in_circumcircle(&self, point: Vec2) -> bool {
        let [a, b, c] = self.coordinates;

        let ax = a.x - point.x;
        let ay = a.y - point.y;
        let bx = b.x - point.x;
        let by = b.y - point.y;
        let cx = c.x - point.x;
        let cy = c.y - point.y;

        let det = (ax * ax + ay * ay) * (bx * cy - by * cx)
            - (bx * bx + by * by) * (ax * cy - ay * cx)
            + (cx * cx + cy * cy) * (ax * by - ay * bx);

        det < 0.0
    }

    pub fn triangle_share_point(&self, other: &NavTriangle) -> bool {
        self.coordinates[0] == other.coordinates[0]
            || self.coordinates[0] == other.coordinates[1]
            || self.coordinates[0] == other.coordinates[2]
            || self.coordinates[1] == other.coordinates[0]
            || self.coordinates[1] == other.coordinates[1]
            || self.coordinates[1] == other.coordinates[2]
            || self.coordinates[2] == other.coordinates[0]
            || self.coordinates[2] == other.coordinates[1]
            || self.coordinates[2] == other.coordinates[2]
    }
}
