use std::ops::{Add, Div};
use geo::{Contains, Coord, EuclideanDistance, LineString, Point, Triangle};
use pathfinding::prelude::astar;
use crate::triangulator::from_npp;

pub fn check_edge_in_triangle(o1: Coord, o2: Coord, triangle: &Triangle) -> bool {
    (o1 == triangle.0 || o1 == triangle.1 || o1 == triangle.2) &&
    (o2 == triangle.0 || o2 == triangle.1 || o2 == triangle.2)
}

fn make_graph(triangles: &Vec<Triangle>) -> Vec<Vec<(usize, u64)>> {
    let mut vec = Vec::new();

    for triangle in triangles {
        let mut inner_vec: Vec<(usize, u64)> = Vec::new();
        let [v1, v2, v3] = [triangle.0, triangle.1, triangle.2];
        let center = v1.add(v2).add(v3).div(3.0);
        for (i, other_triangle) in triangles.iter().enumerate() {
            if other_triangle == triangle { continue }
            if check_edge_in_triangle(v1, v2, other_triangle) ||
                check_edge_in_triangle(v2, v3, other_triangle) ||
                check_edge_in_triangle(v3, v1, other_triangle) {
                let [o1, o2, o3] = [other_triangle.0, other_triangle.1, other_triangle.2];
                let o_center = o1.add(o2).add(o3).div(3.0);

                let distance = center.euclidean_distance(&o_center);

                inner_vec.push((i, distance as u64));
            }
        }
        vec.push(inner_vec);
    };
    vec
}

pub struct NavMesh {
    pub triangles: Vec<Triangle>,
    pub graph: Vec<Vec<(usize, u64)>>,
}

impl NavMesh {
    pub fn new(mut points: &mut Vec<geo::Point>, mut constraints: &mut Vec<LineString>) -> NavMesh {
        let triangles = from_npp(points, constraints);
        let graph = make_graph(&triangles);

        NavMesh {
            triangles,
            graph,
        }
    }

    fn find_triangle(&self, coord: &Coord) -> Option<usize> {
        for (i, triangle) in self.triangles.iter().enumerate() {
            if triangle.contains(coord) {
                return Some(i);
            }
        }
        None
    }

    fn find_successors(&self, index: &usize) -> Vec<(usize, u64)> {
        self.graph[*index].clone()
    }

    fn heuristic(&self, index_1: &usize, index_2: &usize) -> u64 {
        let triangle_1 = &self.triangles[*index_1];
        let triangle_2 = &self.triangles[*index_2];

        let [v1, v2, v3] = [triangle_1.0, triangle_1.1, triangle_1.2];
        let center = v1.add(v2).add(v3).div(3.0);
        let [o1, o2, o3] = [triangle_2.0, triangle_2.1, triangle_2.2];
        let o_center = v1.add(v2).add(v3).div(3.0);
        center.euclidean_distance(&o_center) as u64
    }

    pub fn path(&self, coord1: &Coord, coord2: &Coord) -> Option<(Vec<usize>, u64)> {
        let index_1 = self.find_triangle(coord1).unwrap();
        let index_2 = self.find_triangle(coord2).unwrap();

        let path = astar(&index_1, |i| { self.find_successors(i) },
                         |i| { self.heuristic(i, &index_2) },
                         | succ | { index_2 == *succ });
        path
    }
}