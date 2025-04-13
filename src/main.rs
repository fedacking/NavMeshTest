mod shape_math;
mod triangle_stuff;

use crate::shape_math::{Edge, NavTriangle};
use macroquad::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::path::Path;
use delaunator::{Point, triangulate};
use crate::triangle_stuff::{check_edge_in_triangle, Map};

extern crate geo;


const INTERFACE_MULT: f64 = 10.0;
const INTERFACE_OFFSET: f64 = 0.0;

pub fn draw_graph(map: &Map){
    for (i, nav_triangle) in map.triangles.iter().enumerate() {
        let center = nav_triangle.center();
        draw_circle(center.x() as f32, center.y() as f32, 4.0, BLUE);
        let graph = map.graph.clone();
        for (j, _) in &graph[i] {
            let o_center = map.triangles[*j].center();
            draw_line(center.x() as f32, center.y() as f32, o_center.x() as f32, o_center.y() as f32, 2.0, BLUE);
        }
    }
    println!();
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
        draw_graph(&mesh);
        next_frame().await
    }
}
