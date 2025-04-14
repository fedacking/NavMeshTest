mod triangulator;
mod nav_mesh;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use macroquad::prelude::*;
use geo::{Coord, LineString, Point};

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
    dbg!(file_data);
    request_new_screen_size(1000.0, 1000.0); // horrible hardcoded stuff
    loop {
        clear_background(BLACK);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}
