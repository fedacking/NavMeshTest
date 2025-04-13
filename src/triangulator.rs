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

/// This function returns a conforming delauney triangulation with a series of constraints.
/// The method is adding points where the constraints happen to intersect with the edges of a triangle
pub fn from_npp(points: &Vec<geo::Point>, constraints: &Vec<LineString>) -> Vec<Triangle> {
    let dps: Vec<delaunator::Point> = points.iter().map(convert_to_p_del).collect();
    let triangulations = triangulate(&*dps);

    vec![]
}