use super::super::common::HitRecord;
use super::super::math::{
    transform_point3, transform_vec3, unwrap_matrix4d_config_to_matrix4d, Matrix4D, Matrix4DConfig,
    Point3D, Ray, Vec3D,
};
use super::super::sampler::Sampler;
use super::quadrilateral::{
    quadrilateral_area, quadrilateral_intersect, quadrilateral_normal, quadrilateral_sample,
};
use super::shape::{SampleResult, Shape};
use super::triangle::{triangle_area, triangle_intersect, triangle_normal, triangle_sample};
use super::utils::load_mesh;
use cgmath::InnerSpace;
use serde::Deserialize;
use std::sync::Arc;

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

impl Shape for Mesh {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
                    let normal = triangle_normal(
                        self.vertices[indices[0]],
                        self.vertices[indices[1]],
                        self.vertices[indices[2]],
                    );
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
                    let normal = quadrilateral_normal(
                        self.vertices[indices[0]],
                        self.vertices[indices[1]],
                        self.vertices[indices[2]],
                        self.vertices[indices[3]],
                    );
                    (t, p, normal)
                }
                _ => panic!("Mesh with non-triangle or non-quadrilateral face is not supported"),
            };

            closest_so_far = t;
            hit_record = Some(HitRecord {
                t: t,
                p: p,
                normal: normal,
                shape: Some(self as &dyn Shape),
                object: None,
            });
        }

        hit_record
    }

    fn transform(&self, transform: &Matrix4D) -> Arc<dyn Shape> {
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
        Arc::new(mesh)
    }

    fn sample(&self, sampler: &mut dyn Sampler) -> SampleResult {
        // get all the areas of the faces
        let mut areas = Vec::new();
        for indices in &self.indices {
            let area = match indices.len() {
                3 => triangle_area(
                    self.vertices[indices[0]],
                    self.vertices[indices[1]],
                    self.vertices[indices[2]],
                ),
                4 => quadrilateral_area(
                    self.vertices[indices[0]],
                    self.vertices[indices[1]],
                    self.vertices[indices[2]],
                    self.vertices[indices[3]],
                ),
                _ => panic!("Mesh with non-triangle or non-quadrilateral face is not supported"),
            };
            areas.push(area);
        }

        // area weighted random selection
        let total_area: f64 = areas.iter().sum();
        let mut area = sampler.get_1d() * total_area;
        let mut face_index = 0;
        for (i, a) in areas.iter().enumerate() {
            area -= a;
            if area <= 0.0 {
                face_index = i;
                break;
            }
        }

        // sample the selected face
        let (u, v) = sampler.get_2d();
        let p = match self.indices[face_index].len() {
            3 => triangle_sample(
                self.vertices[self.indices[face_index][0]],
                self.vertices[self.indices[face_index][1]],
                self.vertices[self.indices[face_index][2]],
                u,
                v,
            ),
            4 => quadrilateral_sample(
                self.vertices[self.indices[face_index][0]],
                self.vertices[self.indices[face_index][1]],
                self.vertices[self.indices[face_index][2]],
                self.vertices[self.indices[face_index][3]],
                u,
                v,
                sampler.get_1d(),
            ),
            _ => panic!("Mesh with non-triangle or non-quadrilateral face is not supported"),
        };

        let normal = match self.indices[face_index].len() {
            3 => triangle_normal(
                self.vertices[self.indices[face_index][0]],
                self.vertices[self.indices[face_index][1]],
                self.vertices[self.indices[face_index][2]],
            ),
            4 => quadrilateral_normal(
                self.vertices[self.indices[face_index][0]],
                self.vertices[self.indices[face_index][1]],
                self.vertices[self.indices[face_index][2]],
                self.vertices[self.indices[face_index][3]],
            ),
            _ => panic!("Mesh with non-triangle or non-quadrilateral face is not supported"),
        };

        SampleResult {
            p: p,
            normal: normal,
            pdf: 1.0 / total_area,
        }
    }
}

impl MeshConfig {
    pub fn to_shape(&self) -> Arc<dyn Shape> {
        load_mesh(&self.file)
            .unwrap()
            .transform(&unwrap_matrix4d_config_to_matrix4d(self.transform.as_ref()))
    }
}
