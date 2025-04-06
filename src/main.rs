mod shape_math;

use crate::shape_math::{Edge, NavTriangle};
use macroquad::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::path::Path;

const INTERFACE_MULT: f32 = 10.0;
const INTERFACE_OFFSET: f32 = 0.0;

#[derive(Debug, Clone)]
struct DelaunayTriangulation {
    triangles: Vec<NavTriangle>,
    edges: HashSet<Edge>,
}

impl DelaunayTriangulation {
    fn new() -> Self {
        Self {
            triangles: Vec::new(),
            edges: HashSet::new(),
        }
    }

    fn triangulate(&mut self, points: &Vec<Vec2>) {
        self.triangles.clear();
        self.edges.clear();
        let bounding_triangle = NavTriangle::from_coordinates([
            Vec2 {
                x: -10000.0,
                y: 10000.0,
            }, //TODO: make it so that it creates a triangle based on the points
            Vec2 {
                x: 10000.0,
                y: 10000.0,
            },
            Vec2 {
                x: 0.0,
                y: -10000.0,
            },
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
                self.triangles
                    .push(NavTriangle::from_coordinates([edge.0, edge.1, *point]));
            }
        }

        self.triangles
            .retain(|t| !t.triangle_share_point(&bounding_triangle));

        for triangle in &self.triangles {
            self.edges.insert(Edge::new(triangle.coordinates[0], triangle.coordinates[1]));
            self.edges.insert(Edge::new(triangle.coordinates[1], triangle.coordinates[2]));
            self.edges.insert(Edge::new(triangle.coordinates[2], triangle.coordinates[0]));
        }
    }

    pub fn check_edge(&self, edge: &Edge) -> Vec<Vec2> {
        let mut vec = Vec::new();
        for other in &self.edges {
            match edge.get_intersection_point(&other) {
                Some(point) => { vec.push(point); },
                None => {}
            }
        }
        vec
    }
}

#[derive(Debug, Clone)]
struct Map {
    triangles: Vec<NavTriangle>,
    non_passable_polygons: Vec<Vec<Vec2>>,
}

fn check_triangle_in_polygons(t: &NavTriangle, polygons: &Vec<Vec<Vec2>>) -> bool {
    for polygon in polygons {
        if polygon.contains(&t.coordinates[0])
            && polygon.contains(&t.coordinates[1])
            && polygon.contains(&t.coordinates[2])
        {
            return true;
        }
    }
    false
}

impl Map {
    pub fn from_npp(mut non_passable_polygons: Vec<Vec<Vec2>>) -> Self {
        let mut ref_polygons = &non_passable_polygons;
        let mut combined_points: Vec<Vec2> = non_passable_polygons
            .clone()
            .into_iter()
            .flatten()
            .collect();
        combined_points.push(Vec2 { x: 0.0, y: 60.0 });
        combined_points.push(Vec2 { x: 1000.0, y: 60.0 });
        combined_points.push(Vec2 { x: 0.0, y: 1000.0 });
        combined_points.push(Vec2 {
            x: 1000.0,
            y: 1000.0,
        });
        let mut triangulation = DelaunayTriangulation::new();
        triangulation.triangulate(&combined_points);
        let mut triangles = triangulation.triangles.clone();

        let mut new_non_passable_polygons = non_passable_polygons.clone();

        let mut flag = 2;
        while flag > 0{
            flag -= 1;
            for (polygon_index, polygon) in ref_polygons.clone().into_iter().enumerate() {
                for (i, point) in polygon.iter().enumerate() {
                    let other_index = (i + 1) % polygon.len();
                    let edge = Edge::new(*point, polygon[other_index]);
                    let extra_points = triangulation.check_edge(&edge);
                    for (offset, extra_point) in extra_points.iter().enumerate() {
                        &new_non_passable_polygons[polygon_index].insert(other_index + offset, *extra_point);
                        combined_points.push(*extra_point);
                        print!("{extra_point:?}")
                    }
                }
            }
            println!();
            ref_polygons = &new_non_passable_polygons;
            triangulation.triangulate(&combined_points);
        }
        triangles = triangulation.triangles.clone();

        //triangles.retain(|t| !check_triangle_in_polygons(t, &non_passable_polygons));
        Map {
            triangles,
            non_passable_polygons,
        }
    }
}

fn draw_nav_triangle(nav_triangle: &NavTriangle) {
    let v1 = Vec2 {
        x: nav_triangle.coordinates[0][0],
        y: nav_triangle.coordinates[0][1],
    };
    let v2 = Vec2 {
        x: nav_triangle.coordinates[1][0],
        y: nav_triangle.coordinates[1][1],
    };
    let v3 = Vec2 {
        x: nav_triangle.coordinates[2][0],
        y: nav_triangle.coordinates[2][1],
    };
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
                let x: f32 =
                    coordinates[0].parse().unwrap_or(0.0) * INTERFACE_MULT + INTERFACE_OFFSET;
                let y: f32 =
                    coordinates[1].parse().unwrap_or(0.0) * INTERFACE_MULT + INTERFACE_OFFSET;
                points.push(Vec2 { x, y });
            }
        }
        vec.push(points);
    }
    Ok(vec)
}

#[macroquad::main("Navmesh Visualizer")]
async fn main() {
    let mut vec = read_file("test_input/test2.fnav").unwrap();
    let mut mesh = Map::from_npp(vec);
    request_new_screen_size(1000.0, 1000.0); // horrible hardcoded stuff
    loop {
        clear_background(BLACK);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, WHITE);
        for triangle in &mesh.triangles {
            draw_nav_triangle(triangle);
        }
        next_frame().await
    }
}
