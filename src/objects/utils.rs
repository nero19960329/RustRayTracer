use super::super::material::Material;
use super::super::math::{Point3D, Vec3D};
use super::mesh::Mesh;
use cgmath::InnerSpace;
use log::info;
use ply_rs::parser::Parser;
use ply_rs::ply::DefaultElement;
use std::fs::File;
use std::sync::Arc;

pub trait MeshLoader {
    fn load(&self, path: &str, material: Arc<dyn Material>) -> Result<Mesh, String>;
}

pub struct PlyMeshLoader {}

impl MeshLoader for PlyMeshLoader {
    fn load(&self, path: &str, material: Arc<dyn Material>) -> Result<Mesh, String> {
        info!("Loading mesh from {}", path);
        let mut file = File::open(path).unwrap();
        let p = Parser::<DefaultElement>::new();
        let ply = p.read_ply(&mut file).unwrap();
        let payload = ply.payload;

        let vertex_element = &payload["vertex"];
        let mut vertices: Vec<Point3D> = Vec::new();
        let mut normals: Vec<Vec3D> = Vec::new();
        for vertex in vertex_element {
            let x = match vertex["x"] {
                ply_rs::ply::Property::Float(x) => x as f64,
                _ => panic!("x's type unrecognized"),
            };
            let y = match vertex["y"] {
                ply_rs::ply::Property::Float(y) => y as f64,
                _ => panic!("y's type unrecognized"),
            };
            let z = match vertex["z"] {
                ply_rs::ply::Property::Float(z) => z as f64,
                _ => panic!("z's type unrecognized"),
            };
            vertices.push(Point3D::new(x, y, z));

            let nx = match vertex["nx"] {
                ply_rs::ply::Property::Float(nx) => nx as f64,
                _ => panic!("nx's type unrecognized"),
            };
            let ny = match vertex["ny"] {
                ply_rs::ply::Property::Float(ny) => ny as f64,
                _ => panic!("ny's type unrecognized"),
            };
            let nz = match vertex["nz"] {
                ply_rs::ply::Property::Float(nz) => nz as f64,
                _ => panic!("nz's type unrecognized"),
            };
            normals.push(Vec3D::new(nx, ny, nz).normalize());
        }

        let face_element = &payload["face"];
        let mut indices: Vec<Vec<usize>> = Vec::new();
        for face in face_element {
            let mut face_indices: Vec<usize> = Vec::new();
            let vertex_indices = match &face["vertex_indices"] {
                ply_rs::ply::Property::ListUInt(vertex_indices) => vertex_indices,
                _ => panic!("vertex_indices's type unrecognized"),
            };
            for vertex_index in vertex_indices {
                face_indices.push(*vertex_index as usize);
            }
            indices.push(face_indices);
        }

        info!(
            "Loaded mesh with {} vertices and {} faces",
            vertices.len(),
            indices.len()
        );
        Ok(Mesh {
            vertices,
            normals,
            indices,
            material,
        })
    }
}

pub fn load_mesh(path: &str, material: Arc<dyn Material>) -> Result<Mesh, String> {
    match path.split('.').last() {
        Some("ply") => {
            let loader = PlyMeshLoader {};
            loader.load(path, material)
        }
        _ => Err(format!("Unsupported mesh format: {}", path)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::material::MockMaterial;

    #[test]
    fn test_load_mesh_from_ply() {
        let ply_name = "assets/test.ply";
        let loader = PlyMeshLoader {};
        let mesh = loader.load(ply_name, Arc::new(MockMaterial {}));
        assert!(!mesh.is_err());
        let mesh = mesh.unwrap();
        assert_eq!(mesh.vertices.len(), 24);
        assert_eq!(mesh.normals.len(), 24);
        assert_eq!(mesh.indices.len(), 6);
    }
}
