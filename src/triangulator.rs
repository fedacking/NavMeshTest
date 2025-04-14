use approx::AbsDiffEq;
use std::collections::HashSet;
use delaunator::{triangulate, Triangulation};
use geo::{Contains, Coord, Intersects, Line, LineIntersection, LineString, Point, Triangle};
use geo::line_intersection::line_intersection;
use pathfinding::num_traits::real::Real;
use crate::edge::Edge;

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

fn indexes_to_triangles(points: &Vec<delaunator::Point>, triangles: &Vec<usize>) -> Vec<Triangle> {
    let mut index = 0;
    let mut result = Vec::new();
    while index + 2 < triangles.len() {
        let i0 = convert_from_p_del(&points[triangles[index + 0]]);
        let i1 = convert_from_p_del(&points[triangles[index + 1]]);
        let i2 = convert_from_p_del(&points[triangles[index + 2]]);
        let array: [geo::Point; 3] = [i0, i1, i2];
        result.push(Triangle::from(array));
        index += 3;
    }
    result
}

fn indexes_to_edges(points: &Vec<delaunator::Point>, triangles: &Vec<usize>) -> HashSet<Edge> {
    let mut index = 0;
    let mut result = HashSet::new();
    while index + 2 < triangles.len() {
        let i0 = convert_from_p_del(&points[triangles[index + 0]]);
        let i1 = convert_from_p_del(&points[triangles[index + 1]]);
        let i2 = convert_from_p_del(&points[triangles[index + 2]]);
        let array: [geo::Point; 3] = [i0, i1, i2];
        result.insert(Edge::new(&i0, &i1));
        result.insert(Edge::new(&i1, &i2));
        result.insert(Edge::new(&i2, &i0));
        index += 3;
    }
    result
}

fn edges_to_lines(edges: &HashSet<Edge>) -> Vec<Line> {
    let mut lines = Vec::new();
    for edge in edges {
        lines.push(Line::new(edge.0, edge.1));
    }
    lines
}

fn lines_intersect(l1: &Line, l2: &Line) -> Option<geo::Point> {
    if l1 == l2 { return None; }
    if l1.start == l2.start { return None; }
    if l1.end == l2.start { return None; }
    if l1.start == l2.end { return None; }
    if l1.end == l2.end { return None; }

    match line_intersection(*l1, *l2) {
        None => None,
        Some(line_inter) => {match line_inter {
            LineIntersection::SinglePoint { intersection, is_proper } => {
                if is_proper {
                    Some(Point::from(intersection))
                } else {
                    None
                }
            }
            LineIntersection::Collinear { .. } => None
        }}
    }
}

fn check_point_close(point: &geo::Point, dps: &Vec<delaunator::Point>) -> bool {
    for p in dps {
        let other = convert_from_p_del(p);
        if point.abs_diff_eq(&other, delaunator::EPSILON) {
            return true;
        }
    }
    false
}

fn check_triangle_in_polygons(triangle: &Triangle, polygons: &Vec<LineString>) -> bool {
    for polygon in polygons {
        if polygon.contains(&Coord::from(triangle.0)) &&  polygon.contains(&Coord::from(triangle.1))  &&
            polygon.contains(&Coord::from(triangle.2))  {
            return true;
        }
    }
    false
}

/// This function returns a conforming Delaunay triangulation with a series of constraints.
/// The method is adding points where the constraints happen to intersect with the edges of a triangle
pub fn from_npp(mut points: &mut Vec<geo::Point>, mut constraints: &mut Vec<LineString>) -> Vec<Triangle> {
    let mut dps: Vec<delaunator::Point> = points.iter().map(convert_to_p_del).collect();
    let mut triangulation = triangulate(&*dps);

    let mut has_added = true;
    while has_added {
        has_added = false;

        let edges = indexes_to_edges(&dps, &triangulation.triangles);
        let lines = edges_to_lines(&edges);
        let mut new_constraints: Vec<LineString> = Vec::new();

        for constraint in &mut *constraints {
            let mut new_poly_coords: Vec<geo::Coord> = Vec::new();

            for line in constraint.lines() {
                new_poly_coords.push(Coord::from(line.start));
                for other in &lines {
                    match lines_intersect(&line, other) {
                        Some(intersection) => {
                            if !check_point_close(&intersection, &dps) {
                                points.push(intersection);
                                dps.push(convert_to_p_del(&intersection));
                                new_poly_coords.push(Coord::from(intersection));
                                has_added = true;
                            }
                        }
                        None => ()
                    }
                }
            }

            let mut new_poly = LineString(new_poly_coords);
            new_poly.close();
            new_constraints.push(new_poly);

            triangulation = triangulate(&*dps);
        }

        constraints.clear();
        for new_constraint in new_constraints {
            constraints.push(new_constraint);
        }
    }

    let mut triangles = indexes_to_triangles(&dps, &triangulation.triangles);

    triangles.retain(|t| !check_triangle_in_polygons(t, constraints));

    triangles
}