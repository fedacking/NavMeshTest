mod shape_math;

use crate::shape_math::{Edge, NavTriangle};
use macroquad::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::path::Path;
use delaunator::{Point, triangulate};
extern crate geo;
extern crate line_intersection;


const INTERFACE_MULT: f64 = 10.0;
const INTERFACE_OFFSET: f64 = 0.0;

#[derive(Debug, Clone)]
struct Map {
    triangles: Vec<NavTriangle>,
    non_passable_polygons: Vec<Vec<Point>>,
}

fn check_triangle_in_polygons(t: &NavTriangle, polygons: &Vec<Vec<Point>>) -> bool {
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

fn indexes_to_triangles(points: &Vec<Point>, triangles: &Vec<usize>) -> Vec<NavTriangle> {
    let mut index = 0;
    let mut result = Vec::new();
    while index + 2 < triangles.len() {
        result.push(NavTriangle::from_coordinates([
            points[triangles[index]].clone(),
            points[triangles[index +1]].clone(),
            points[triangles[index + 2]].clone(),
        ]));
        index += 3;
    }
    result
}

fn indexes_to_edges(points: &Vec<Point>, triangles: &Vec<usize>) -> HashSet<Edge> {
    let mut index = 0;
    let mut result = HashSet::new();
    while index + 2 < triangles.len() {
        result.insert(Edge::new(
            points[triangles[index]].clone().into(),
            points[triangles[index +1]].clone().into(),
        ));
        result.insert(Edge::new(
            points[triangles[index]].clone().into(),
            points[triangles[index +2]].clone().into(),
        ));
        result.insert(Edge::new(
            points[triangles[index+1]].clone().into(),
            points[triangles[index +2]].clone().into(),
        ));
        index += 3;
    }
    result
}

impl Map {
    pub fn from_npp(non_passable_polygons: &Vec<Vec<Point>>) -> Self {
        let mut clone_polygons = non_passable_polygons.clone();
        let mut combined_points: Vec<Point> = non_passable_polygons
            .clone()
            .into_iter()
            .flatten()
            .collect();
        combined_points.push(Point { x: 0.0, y: 60.0 });
        combined_points.push(Point { x: 1000.0, y: 60.0 });
        combined_points.push(Point { x: 0.0, y: 1000.0 });
        combined_points.push(Point {
            x: 1000.0,
            y: 1000.0,
        });
        let mut triangulation = triangulate(&*combined_points);
        let mut triangles = indexes_to_triangles(&combined_points, &triangulation.triangles);
        let mut hash_edges: HashSet<Edge> = indexes_to_edges(&combined_points, &triangulation.triangles);
        //println!("{:?}", triangles);

        let mut flag = true;
        while flag {
            flag = false;

            for (polygon_index, polygon) in clone_polygons.clone().into_iter().enumerate() {
                for (i, point) in polygon.iter().enumerate() {
                    let other_index = (i + 1) % polygon.len();
                    let edge = Edge::new(point.clone(), (&polygon[other_index]).clone());
                    let extra_points = edge.get_intersection_points(&hash_edges);
                    for (offset, extra_point) in extra_points.iter().enumerate() {
                        clone_polygons[polygon_index].insert(other_index + offset, extra_point.clone());
                        combined_points.push(extra_point.clone());
                        flag = true;
                    }
                }
            }
            let mut triangulation = triangulate(&*combined_points);
            triangles = indexes_to_triangles(&combined_points, &triangulation.triangles);
            hash_edges = indexes_to_edges(&combined_points, &triangulation.triangles);
        }
        // while flag > 0{
        //     flag -= 1;
        //     for (polygon_index, polygon) in ref_polygons.clone().into_iter().enumerate() {
        //         for (i, point) in polygon.iter().enumerate() {
        //             let other_index = (i + 1) % polygon.len();
        //             let edge = Edge::new(*point, polygon[other_index]);
        //             let extra_points = triangulation.check_edge(&edge);
        //             for (offset, extra_point) in extra_points.iter().enumerate() {
        //                 &new_non_passable_polygons[polygon_index].insert(other_index + offset, *extra_point);
        //                 combined_points.push(*extra_point);
        //                 print!("{extra_point:?}")
        //             }
        //         }
        //     }
        //     println!();
        //     ref_polygons = &new_non_passable_polygons;
        //     triangulation.triangulate(&combined_points);
        // }
        // triangles = triangulation.triangles.clone();

        //triangles.retain(|t| !check_triangle_in_polygons(t, &non_passable_polygons));
        Map {
            triangles,
            non_passable_polygons: clone_polygons,
        }
    }
}

fn draw_nav_triangle(nav_triangle: &NavTriangle) {
    let v1 = Vec2 {
        x: nav_triangle.coordinates[0].x as f32,
        y: nav_triangle.coordinates[0].y as f32,
    };
    let v2 = Vec2 {
        x: nav_triangle.coordinates[1].x as f32,
        y: nav_triangle.coordinates[1].y as f32,
    };
    let v3 = Vec2 {
        x: nav_triangle.coordinates[2].x as f32,
        y: nav_triangle.coordinates[2].y as f32,
    };
    draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, BLACK);
    draw_line(v1.x, v1.y, v3.x, v3.y, 2.0, BLACK);
    draw_line(v3.x, v3.y, v2.x, v2.y, 2.0, BLACK);
    draw_triangle(v1, v2, v3, GREEN);
}

fn read_file(filename: &str) -> io::Result<Vec<Vec<Point>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut vec = vec![];

    for line in reader.lines() {
        let line = line?; // Read the line
        let segments: Vec<&str> = line.split('-').collect();
        let mut points: Vec<Point> = vec![];

        for segment in segments {
            let coordinates: Vec<&str> = segment.split(';').collect();
            if coordinates.len() == 2 {
                let x: f64 =
                    coordinates[0].parse().unwrap_or(0.0) * INTERFACE_MULT + INTERFACE_OFFSET;
                let y: f64 =
                    coordinates[1].parse().unwrap_or(0.0) * INTERFACE_MULT + INTERFACE_OFFSET;
                points.push(Point { x, y });
            }
        }
        vec.push(points);
    }
    Ok(vec)
}

#[macroquad::main("Navmesh Visualizer")]
async fn main() {
    let vec = read_file("test_input/test2.fnav").unwrap();
    let mut mesh = Map::from_npp(&vec);
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
