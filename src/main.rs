mod shape_math;

use std::collections::HashSet;
use macroquad::prelude::*;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::path::Path;
use crate::shape_math::{Edge, NavTriangle};

const INTERFACE_MULT: f32 = 10.0;
const INTERFACE_OFFSET: f32 = 0.0;

#[derive(Debug)]
struct DelaunayTriangulation {
    triangles: Vec<NavTriangle>,
}

impl DelaunayTriangulation {
    fn new() -> Self {
        Self { triangles: Vec::new() }
    }

    fn triangulate(&mut self, points: &Vec<Vec2>) {
        let bounding_triangle = NavTriangle::from_coordinates([
            Vec2{ x: -10000.0, y: 10000.0 }, //TODO: make it so that it creates a triangle based on the points
            Vec2{ x: 10000.0 , y: 10000.0 },
            Vec2{ x: 0.0, y: -10000.0 },
        ]);

        self.triangles.push(bounding_triangle);

        for point in points {
            let mut bad_triangles = Vec::new();

            for triangle in &self.triangles {
                if triangle.point_in_circumcircle(*point) {
                    bad_triangles.push(*triangle);
                }
            }

            let mut polygon = HashSet::new();
            for triangle in &bad_triangles {
                let edges = vec![
                    Edge::new(triangle.coordinates[0], triangle.coordinates[1]),
                    Edge::new(triangle.coordinates[1], triangle.coordinates[2]),
                    Edge::new(triangle.coordinates[2], triangle.coordinates[0]),
                ];

                for edge in edges {
                    if !polygon.insert(edge) {
                        polygon.remove(&edge);
                    }
                }
            }

            self.triangles.retain(|t| !bad_triangles.contains(t));

            for edge in polygon {
                self.triangles.push(NavTriangle::from_coordinates([edge.0, edge.1, *point]));
            }
        }

        self.triangles.retain(|t| !t.triangle_share_point(&bounding_triangle));
    }
}


#[derive(Debug, Clone)]
struct Map {
    triangles: Vec<NavTriangle>,
    non_passable_polygons: Vec<Vec<Vec2>>,
}

fn check_triangle_in_polygons(t: &NavTriangle, polygons: &Vec<Vec<Vec2>>) -> bool {
    for polygon in polygons {
        if polygon.contains(&t.coordinates[0]) && polygon.contains(&t.coordinates[1]) && polygon.contains(&t.coordinates[2]) {
            return true;
        }
    }
    false
}

impl Map {
    pub fn from_npp(non_passable_polygons: Vec<Vec<Vec2>>) -> Self {
        let mut combined_points: Vec<Vec2> = non_passable_polygons.clone().into_iter().flatten().collect();
        combined_points.push(Vec2{ x: 0.0, y: 60.0 });
        combined_points.push(Vec2{ x: 1000.0, y: 60.0 });
        combined_points.push(Vec2{ x: 0.0, y: 1000.0 });
        combined_points.push(Vec2{ x: 1000.0, y: 1000.0 });
        let mut triangulation = DelaunayTriangulation::new();
        triangulation.triangulate(&combined_points);
        let mut triangles = triangulation.triangles;
        triangles.retain(|t| {
            !check_triangle_in_polygons(t, &non_passable_polygons)
        });
        Map{triangles, non_passable_polygons}
    }
}




fn draw_nav_triangle(nav_triangle: &NavTriangle) {
    let v1 = Vec2{ x: nav_triangle.coordinates[0][0], y: nav_triangle.coordinates[0][1]};
    let v2 = Vec2{ x: nav_triangle.coordinates[1][0], y: nav_triangle.coordinates[1][1]};
    let v3 = Vec2{ x: nav_triangle.coordinates[2][0], y: nav_triangle.coordinates[2][1]};
    draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, BLACK);
    draw_line(v1.x, v1.y, v3.x, v3.y, 2.0, BLACK);
    draw_line(v3.x, v3.y, v2.x, v2.y, 2.0, BLACK);
    draw_triangle(v1, v2, v3, GREEN);
}

fn read_file(filename: &str) -> io::Result<Vec<Vec<Vec2>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut vec = vec![];

    for line in reader.lines() {
        let line = line?; // Read the line
        let segments: Vec<&str> = line.split('-').collect();
        let mut points: Vec<Vec2> = vec![];

        for segment in segments {
            let coordinates: Vec<&str> = segment.split(';').collect();
            if coordinates.len() == 2 {
                let x: f32 = coordinates[0].parse().unwrap_or(0.0) * INTERFACE_MULT + INTERFACE_OFFSET;
                let y: f32 = coordinates[1].parse().unwrap_or(0.0) * INTERFACE_MULT + INTERFACE_OFFSET;
                points.push(Vec2 { x, y });
            }
        }
        vec.push(points);
    }
    Ok(vec)
}


#[macroquad::main("Navmesh Visualizer")]
async fn main() {
    let mut vec = read_file("test_input/test1.fnav").unwrap();
    println!("{:?}", vec);
    let mut mesh = Map::from_npp(vec);
    println!("{:?}", mesh.triangles);
    loop {
        clear_background(BLACK);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, WHITE);
        for triangle in &mesh.triangles {
            draw_nav_triangle(triangle);
        }
        next_frame().await
    }
}