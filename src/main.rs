mod triangulator;
mod nav_mesh;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use macroquad::prelude::*;
use geo::Point;

fn read_file(filename: &str) -> io::Result<Vec<Vec<Point>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut vec = vec![];

    let mut lines = reader.lines().into_iter();

    let first_line = lines.next().unwrap();
    dbg!(&first_line);

    for line in lines {
        let line = line?; // Read the line
        let segments: Vec<&str> = line.split('-').collect();
        let mut points: Vec<Point> = vec![];
        dbg!(&line);

        for segment in segments {
            let coordinates: Vec<&str> = segment.split(';').collect();
            if coordinates.len() == 2 {
                let x: f64 =
                    coordinates[0].parse().unwrap_or(0.0);
                let y: f64 =
                    coordinates[1].parse().unwrap_or(0.0);
                points.push(Point::new(x, y ));
            }
        }
        vec.push(points);
    }
    Ok(vec)
}

#[macroquad::main("Navmesh Visualizer")]
async fn main() {
    let vec = read_file("test_input/test2.fnav").unwrap();
    request_new_screen_size(1000.0, 1000.0); // horrible hardcoded stuff
    loop {
        clear_background(BLACK);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}
