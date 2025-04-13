use std::collections::HashSet;
use std::ops::{Add, Div};
use delaunator::{triangulate, Point};
use geo::EuclideanDistance;
use crate::shape_math::{Edge, NavTriangle};

#[derive(Debug, Clone)]
pub struct Map {
    pub triangles: Vec<NavTriangle>,
    pub non_passable_polygons: Vec<Vec<Point>>,
    pub graph: Vec<Vec<(usize, f64)>>
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
            points[triangles[index]].clone(),
            points[triangles[index + 1]].clone(),
        ));
        result.insert(Edge::new(
            points[triangles[index]].clone(),
            points[triangles[index + 2]].clone(),
        ));
        result.insert(Edge::new(
            points[triangles[index + 1]].clone(),
            points[triangles[index + 2]].clone(),
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
                        if not_too_close(extra_point, &combined_points) {
                            clone_polygons[polygon_index].insert(other_index + offset, extra_point.clone());
                            combined_points.push(extra_point.clone());
                            flag = true;
                        }
                    }
                }
            }
            let mut triangulation = triangulate(&*combined_points);
            triangles = indexes_to_triangles(&combined_points, &triangulation.triangles);
            hash_edges = indexes_to_edges(&combined_points, &triangulation.triangles);
        }
        triangles.retain(|t| !check_triangle_in_polygons(t, &clone_polygons));

        let graph = Self::make_graph(&triangles);

        Map {
            triangles,
            non_passable_polygons: clone_polygons,
            graph,
        }
    }

    fn make_graph(nav_triangles: &Vec<NavTriangle>) -> Vec<Vec<(usize, f64)>> {
        let mut vec = Vec::new();

        for nav_triangle in nav_triangles {
            let mut inner_vec: Vec<(usize, f64)> = Vec::new();
            let v1 = geo::Point::new(nav_triangle.coordinates[0].x, nav_triangle.coordinates[0].y);
            let v2 = geo::Point::new(nav_triangle.coordinates[1].x, nav_triangle.coordinates[1].y);
            let v3 = geo::Point::new(nav_triangle.coordinates[2].x, nav_triangle.coordinates[2].y);
            let center = v1.add(v2).add(v3).div(3.0);
            for (i, other_triangle) in nav_triangles.iter().enumerate() {
                if other_triangle == nav_triangle { continue }
                if check_edge_in_triangle(v1, v2, other_triangle) ||
                    check_edge_in_triangle(v2, v3, other_triangle) ||
                    check_edge_in_triangle(v3, v1, other_triangle) {
                    let o_center = other_triangle.center();

                    let distance = center.euclidean_distance(&o_center);

                    inner_vec.push((i, distance));
                }
            }
            vec.push(inner_vec);
        };
        vec
    }
}

fn not_too_close(point: &Point, points: &Vec<Point>) -> bool {
    for other in points {
        if f64::abs(other.x - point.x).abs() < 0.01 && f64::abs(other.y - point.y).abs() < 0.01 {
            return false;
        }
    }
    true
}

pub fn check_edge_in_triangle(o1: geo::Point, o2: geo::Point, nav_triangle: &NavTriangle) -> bool {
    let v1 = geo::Point::new(nav_triangle.coordinates[0].x, nav_triangle.coordinates[0].y);
    let v2 = geo::Point::new(nav_triangle.coordinates[1].x, nav_triangle.coordinates[1].y);
    let v3 = geo::Point::new(nav_triangle.coordinates[2].x, nav_triangle.coordinates[2].y);

    (o1 == v1 || o1 == v2 || o1 == v3) && (o2 == v1 || o2 == v2 || o2 == v3)
}