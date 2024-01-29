# RustRayTracer

[![codecov](https://codecov.io/gh/nero19960329/RustRayTracer/graph/badge.svg?token=D2BBB05QHQ)](https://codecov.io/gh/nero19960329/RustRayTracer)
[![CI](https://github.com/nero19960329/RustRayTracer/actions/workflows/ci.yml/badge.svg)](https://github.com/nero19960329/RustRayTracer/actions/workflows/ci.yml)

A soft ray tracer implemented by Rust.

# Features

- Cameras
  - [x] Perspective Camera
  - [ ] Depth of Field
  - [ ] ...
- Materials
  - [x] Lambertian
  - [x] Phong Specular
  - [x] Ideal Reflector
  - [x] Ideal Dielectric
  - [ ] Microfacet
  - [ ] ...
- Objects
  - [x] Sphere
  - [x] Plane
  - [x] Triangle
  - [x] Quadrilateral
  - [x] Mesh
  - [ ] ...
- Sampler
  - [x] Random
  - [x] Stratified
  - [ ] Halton
  - [ ] Sobol
  - [ ] ...
- Rendering
  - [x] Monte-Carlo Path Tracing
  - [ ] Bidirectional Path Tracing
  - [ ] Metropolis Light Transport
  - [ ] ...
- Scene
  - [x] smallpt
  - [x] Cornell Box
  - [ ] Veach MIS
  - [ ] ...
- Aggregation
  - [ ] BVH
  - [ ] Kd-Tree
  - [ ] ...
- Post Processing
  - [x] Tone Mapping
  - [x] Gamma Correction
  - [x] White Balance
  - [ ] ...

# Example Scenes

smallpt scene MCPT 1024x768 16384spp

![smallpt scene MCPT 1024x768 16384spp](https://i.imgur.com/DuFLUKm.png)

Cornell Box scene MCPT 500x500 16384spp

![Cornell Box scene MCPT 500x500 16384spp](https://i.imgur.com/e4lEBgj.png)

# Gallery

https://storage.cloud.google.com/rust-ray-tracer/gallery/v0.2.1/report.html
