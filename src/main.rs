use std::collections::HashSet;
use macroquad::prelude::*;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::path::Path;

const INTERFACE_MULT: f32 = 10.0;
const INTERFACE_OFFSET: f32 = 0.0;

#[derive(Debug, Clone, Copy)]
struct NavTriangle {
    coordinates: [Vec2; 3]
}

impl NavTriangle {
    pub fn from_coordinates(mut coordinates: [Vec2; 3]) -> NavTriangle {
        let det = coordinates[0].x * (coordinates[1].y - coordinates[2].y) +
            coordinates[1].x * (coordinates[2].y - coordinates[0].y) +
            coordinates[2].x * (coordinates[0].y - coordinates[1].y);
        if det > 0.0 {
            (coordinates[1], coordinates[0]) = (coordinates[0], coordinates[1]);
        }
        NavTriangle { coordinates }
    }
}


impl PartialEq for NavTriangle {
    fn eq(&self, other: &Self) -> bool {
        let a = &self.coordinates; let b = other.coordinates;
        (a[0] == b[0] && a[1] == b[1] && a[2] == b[2]) ||
        (a[0] == b[1] && a[1] == b[2] && a[2] == b[0]) ||
        (a[0] == b[0] && a[1] == b[1] && a[2] == b[2])
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge (Vec2, Vec2);

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for Edge {

}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0.x < self.1.x {
            state.write_u32(self.0.x.to_bits());
            state.write_u32(self.0.y.to_bits());
            state.write_u32(self.1.x.to_bits());
            state.write_u32(self.1.y.to_bits());
        } else {
            state.write_u32(self.1.x.to_bits());
            state.write_u32(self.1.y.to_bits());
            state.write_u32(self.0.x.to_bits());
            state.write_u32(self.0.y.to_bits());
        }
    }
}

fn point_in_triangle(point: Vec2, nav_triangle: &NavTriangle) -> bool {
    let v0 = nav_triangle.coordinates[1] - nav_triangle.coordinates[0];
    let v1 = nav_triangle.coordinates[2] - nav_triangle.coordinates[0];
    let v2 = point - nav_triangle.coordinates[0];

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

pub fn point_in_circumcircle(point: Vec2, triangle: &NavTriangle) -> bool {
    let [a, b, c] = triangle.coordinates;

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

pub fn triangle_share_point(a: &NavTriangle, b: &NavTriangle) -> bool {
    a.coordinates[0] == b.coordinates[0] || a.coordinates[0] == b.coordinates[1] || a.coordinates[0] == b.coordinates[2] ||
    a.coordinates[1] == b.coordinates[0] || a.coordinates[1] == b.coordinates[1] || a.coordinates[1] == b.coordinates[2] ||
    a.coordinates[2] == b.coordinates[0] || a.coordinates[2] == b.coordinates[1] || a.coordinates[2] == b.coordinates[2]
}



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
                if point_in_circumcircle(*point, &triangle) {
                    bad_triangles.push(*triangle);
                } else {
                    println!("Bad triangle: {:?}", triangle);
                }
            }

            let mut polygon = HashSet::new();
            for triangle in &bad_triangles {
                let edges = vec![
                    Edge(triangle.coordinates[0], triangle.coordinates[1]),
                    Edge(triangle.coordinates[1], triangle.coordinates[2]),
                    Edge(triangle.coordinates[2], triangle.coordinates[0]),
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

        self.triangles.retain(|t| !triangle_share_point(t, &bounding_triangle));
    }
}

/*
fn triangulate(points: Vec<Vec2>) -> Vec<NavTriangle> {
    let bounding_triangle = NavTriangle{ coordinates: [
        Vec2{ x: f32::MIN, y: f32::MAX },
        Vec2{ x: f32::MAX, y: f32::MAX },
        Vec2{ x: 0.0, y: f32::MIN },
    ]};
    let mut all_triangles: Vec<&NavTriangle> = vec![&bounding_triangle];

    for point in points {
        let mut bounding_triangles: Vec<&NavTriangle> = vec![];
        for triangle in &all_triangles {
            if point_in_circumcircle(point, &triangle) {
                bounding_triangles.push(&triangle);
            }
        }

        let mut polygon = HashSet::new();
        for triangle in &bounding_triangles {
            let edges = vec![
                Edge(triangle.coordinates[0], triangle.coordinates[1]),
                Edge(triangle.coordinates[1], triangle.coordinates[2]),
                Edge(triangle.coordinates[2], triangle.coordinates[0])
            ];

            for edge in edges {
                if !polygon.insert(edge) {
                    polygon.remove(&edge);
                }
            }
        }

        all_triangles.retain(|t| bounding_triangles.contains(&t));

        for edge in polygon {
            all_triangles.push(&NavTriangle{ coordinates:[point, edge.0, edge.1]})
        }
    }

    all_triangles.iter().map(|t| (**t).clone()).collect()
}
*/



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