use super::super::math::{
    transform_point3, transform_vec3, unwrap_matrix4d_config_to_matrix4d, Matrix4D, Matrix4DConfig,
    Point3D, Ray, Vec3D,
};
use super::super::object::HitRecord;
use super::quadrilateral::quadrilateral_intersect;
use super::triangle::triangle_intersect;
use super::utils::load_mesh;
use cgmath::InnerSpace;
use serde::Deserialize;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Point3D>,
    pub normals: Vec<Vec3D>,
    pub indices: Vec<Vec<usize>>,
}

#[derive(Deserialize)]
pub struct MeshConfig {
    file: String,
    transform: Option<Matrix4DConfig>,
}

impl Mesh {
    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for indices in &self.indices {
            let (t, p, normal) = match indices.len() {
                3 => {
                    // triangle
                    let (t, _u, _v) = match triangle_intersect(
                        self.vertices[indices[0]],
                        self.vertices[indices[1]],
                        self.vertices[indices[2]],
                        ray,
                        t_min,
                        closest_so_far,
                    ) {
                        Some((t, u, v)) => (t, u, v),
                        None => continue,
                    };

                    let p = ray.at(t);
                    let normal = (self.vertices[indices[1]] - self.vertices[indices[0]])
                        .cross(self.vertices[indices[2]] - self.vertices[indices[0]])
                        .normalize();
                    (t, p, normal)
                }
                4 => {
                    // quadrilateral
                    let (t, _u, _v, _w) = match quadrilateral_intersect(
                        self.vertices[indices[0]],
                        self.vertices[indices[1]],
                        self.vertices[indices[2]],
                        self.vertices[indices[3]],
                        ray,
                        t_min,
                        closest_so_far,
                    ) {
                        Some((t, u, v, w)) => (t, u, v, w),
                        None => continue,
                    };

                    let p = ray.at(t);
                    let normal = (self.vertices[indices[1]] - self.vertices[indices[0]])
                        .cross(self.vertices[indices[2]] - self.vertices[indices[0]])
                        .normalize();
                    (t, p, normal)
                }
                _ => panic!("Mesh with non-triangle or non-quadrilateral face is not supported"),
            };

            closest_so_far = t;
            hit_record = Some(HitRecord {
                t: t,
                p: p,
                normal: normal,
                material: None,
            });
        }

        hit_record
    }

    pub fn transform(&mut self, transform: &Matrix4D) -> Self {
        let mesh = Mesh {
            vertices: self
                .vertices
                .iter()
                .map(|v| transform_point3(*transform, *v))
                .collect(),
            normals: self
                .normals
                .iter()
                .map(|n| transform_vec3(*transform, *n).normalize())
                .collect(),
            indices: self.indices.clone(),
        };
        mesh
    }
}

impl MeshConfig {
    pub fn to_instance(&self) -> Mesh {
        load_mesh(&self.file)
            .unwrap()
            .transform(&unwrap_matrix4d_config_to_matrix4d(self.transform.as_ref()))
    }
}
