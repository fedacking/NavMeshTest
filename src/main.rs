mod triangulator;
mod nav_mesh;
mod edge;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use macroquad::prelude::*;
use geo::{coord, Coord, LineString, Point, Triangle};
use crate::nav_mesh::NavMesh;
use crate::triangulator::from_npp;

fn draw_nav_triangle(triangle: &Triangle, color: Color) {
    let [l1, l2, l3] = triangle.to_array();
    let v1 = Vec2 { x: l1.x as f32, y: l1.y as f32 };
    let v2 = Vec2 { x: l2.x as f32, y: l2.y as f32 };
    let v3 = Vec2 { x: l3.x as f32, y: l3.y as f32 };
    draw_line(v1.x, v1.y, v2.x, v2.y, 2.0, BLACK);
    draw_line(v1.x, v1.y, v3.x, v3.y, 2.0, BLACK);
    draw_line(v3.x, v3.y, v2.x, v2.y, 2.0, BLACK);
    draw_triangle(v1, v2, v3, color);
}

fn draw_path(triangle: &Vec<Triangle>, path: &Vec<usize>) {
    for point in path {
        draw_nav_triangle(&triangle[*point], BLUE);
    }
}

fn read_file(filename: &str) -> io::Result<(Vec<Point>, Vec<LineString>)> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines().into_iter();

    let first_line = lines.next().unwrap()?;
    let mut points: Vec<Point> = first_line.split("-").map(|v| {
        let split = v.split(";").collect::<Vec<&str>>();
        Point::new(split[0].parse().unwrap_or(0.0), split[1].parse().unwrap_or(0.0))
    }).collect();

    let mut polygons: Vec<LineString> = Vec::new();

    for line in lines {
        let line = line?; // Read the line
        let segments: Vec<&str> = line.split('-').collect();
        let mut poly_coordinates: Vec<Coord> = vec![];

        for segment in segments {
            let coordinates: Vec<&str> = segment.split(';').collect();
            if coordinates.len() == 2 {
                let x: f64 =
                    coordinates[0].parse().unwrap_or(0.0);
                let y: f64 =
                    coordinates[1].parse().unwrap_or(0.0);
                let point = Point::new(x, y);
                poly_coordinates.push(Coord::from(point));
                points.push(point);
            }
        }

        let mut ls = LineString(poly_coordinates);
        ls.close();
        polygons.push(ls);
    }
    Ok((points, polygons))
}

#[macroquad::main("Navmesh Visualizer")]
async fn main() {
    let file_data = read_file("test_input/test2.fnav").unwrap();
    let mut points = file_data.0;
    let mut constraints = file_data.1;
    let navmesh = NavMesh::new(&mut points, &mut constraints);
    let path = navmesh.path(&coord! {x: 1.0, y: 1.0}, &coord! {x: 999.0, y: 999.0}).unwrap();
    request_new_screen_size(1000.0, 1000.0); // horrible hardcoded stuff
    loop {
        clear_background(BLACK);
        for triangle in &navmesh.triangles {
            draw_nav_triangle(triangle, GREEN);
        }
        draw_path(&navmesh.triangles, &path.0);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}
