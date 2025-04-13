use std::collections::HashSet;
use macroquad::math::Vec2;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div};
use delaunator::Point;
use geo::{Intersects, Line, LineIntersection};
use geo::Coord;
use geo::line_intersection::line_intersection;

#[derive(Debug, Clone)]
pub struct Edge(pub Point, pub Point);

impl Edge {
    pub fn new(v1: Point, v2: Point) -> Edge {
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
        state.write_u64(self.0.x.to_bits());
        state.write_u64(self.0.y.to_bits());
        state.write_u64(self.1.x.to_bits());
        state.write_u64(self.1.y.to_bits());
    }
}

impl Edge {
    pub fn get_intersection_points(&self, others: &HashSet<Edge>) -> Vec<Point> {
        let mut vec = vec![];
        for other in others {
            if (f64::abs(other.0.x - self.0.x) < 0.01 && f64::abs(other.0.y - self.0.y) < 0.01) ||
                (f64::abs(other.1.x - self.0.x) < 0.01 && f64::abs(other.1.y - self.0.y) < 0.01) ||
                (f64::abs(other.0.x - self.1.x) < 0.01 && f64::abs(other.0.y - self.1.y) < 0.01) ||
                (f64::abs(other.1.x - self.1.x) < 0.01 && f64::abs(other.1.y - self.1.y) < 0.01){
                continue;
            }

            let segment = Line{
                start:Coord{x: self.0.x, y: self.0.y },
                end:Coord{x: self.1.x, y: self.1.y }
            };

            let other_segment = Line{
                start:Coord{x: other.0.x, y: other.0.y },
                end:Coord{x: other.1.x, y: other.1.y }
            };

            match line_intersection(
                segment,
                other_segment,
            ) {
                Some(inter) => match inter {
                    LineIntersection::SinglePoint{ intersection, is_proper} => {
                        if is_proper {
                            vec.push(Point{x: intersection.x, y: intersection.y});
                        }
                    },
                    _ => {}
                }
                None => (),
            }

        }
        vec
    }
}

#[derive(Debug, Clone)]
pub struct NavTriangle {
    pub coordinates: [Point; 3],
}

impl NavTriangle {
    pub fn from_coordinates(mut coordinates: [Point; 3]) -> NavTriangle {
        let det = coordinates[0].x * (coordinates[1].y - coordinates[2].y)
            + coordinates[1].x * (coordinates[2].y - coordinates[0].y)
            + coordinates[2].x * (coordinates[0].y - coordinates[1].y);
        if det > 0.0 {
            NavTriangle { coordinates: [coordinates[1].clone(), coordinates[0].clone(), coordinates[2].clone()] }
        } else {
            NavTriangle { coordinates }
        }
    }

    pub fn center(&self) -> geo::Point {
        let v1 = geo::Point::new(self.coordinates[0].x, self.coordinates[0].y);
        let v2 = geo::Point::new(self.coordinates[1].x, self.coordinates[1].y);
        let v3 = geo::Point::new(self.coordinates[2].x, self.coordinates[2].y);
        v1.add(v2).add(v3).div(3.0)
    }
}

impl PartialEq for NavTriangle {
    fn eq(&self, other: &Self) -> bool {
        let a = &self.coordinates;
        let b = &other.coordinates;
        (a[0] == b[0] && a[1] == b[1] && a[2] == b[2])
            || (a[0] == b[1] && a[1] == b[2] && a[2] == b[0])
            || (a[0] == b[0] && a[1] == b[1] && a[2] == b[2])
    }
}

 impl NavTriangle {
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
