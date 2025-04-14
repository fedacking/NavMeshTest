use std::collections::HashSet;
use delaunator::{triangulate, Triangulation};
use geo::{Line, LineString, Triangle};

fn convert_to_p_del(point: &geo::Point) -> delaunator::Point {
    delaunator::Point {
        x: point.x(),
        y: point.y(),
    }
}

fn convert_from_p_del(point: &delaunator::Point) -> geo::Point {
    geo::Point::new(
        point.x,
        point.y,
    )
}

fn get_edges(points: &Vec<geo::Point>, triangulation: Triangulation) -> HashSet<Line> {
    let mut edges = HashSet::new();
    edges
}

fn indexes_to_triangles(points: &Vec<geo::Point>, triangles: &Vec<usize>) -> Vec<Triangle> {
    let mut index = 0;
    let mut result = Vec::new();
    while index + 2 < triangles.len() {
        let i0 = points[triangles[index + 0]];
        let i1 = points[triangles[index + 1]];
        let i2 = points[triangles[index + 2]];
        let array: [geo::Point; 3] = [i0, i1, i2];
        result.push(Triangle::from(array));
        index += 3;
    }
    result
}

/// This function returns a conforming Delaunay triangulation with a series of constraints.
/// The method is adding points where the constraints happen to intersect with the edges of a triangle
pub fn from_npp(points: &Vec<geo::Point>, constraints: &Vec<LineString>) -> Vec<Triangle> {
    let dps: Vec<delaunator::Point> = points.iter().map(convert_to_p_del).collect();
    let triangulations = triangulate(&*dps);


    vec![]
}