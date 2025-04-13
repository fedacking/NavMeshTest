use geo::{Point, Polygon, Triangle};

pub struct NavMesh {
    pub triangles: Vec<Triangle>,
}

pub fn vec(points: &Vec<Point>, polygons: &Vec<Polygon>) -> NavMesh {
    NavMesh {
        triangles: vec![]
    }
}