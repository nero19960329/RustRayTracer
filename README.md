# RustRayTracer

[![codecov](https://codecov.io/gh/nero19960329/RustRayTracer/graph/badge.svg?token=D2BBB05QHQ)](https://codecov.io/gh/nero19960329/RustRayTracer)
[![CI](https://github.com/nero19960329/RustRayTracer/actions/workflows/ci.yml/badge.svg)](https://github.com/nero19960329/RustRayTracer/actions/workflows/ci.yml)

A soft ray tracer implemented by Rust.

# Features

- Cameras
  - [x] Perspective Camera
  - [ ] Depth of Field
  - ...
- Materials
  - [x] Lambertian
  - [x] Phong Specular
  - [x] Ideal Reflector
  - [x] Ideal Dielectric
  - [ ] Microfacet
  - ...
- Objects
  - [x] Sphere
  - [x] Plane
  - [ ] Triangle
  - ...
- Rendering
  - [x] Monte-Carlo Path Tracing
  - [ ] Bidirectional Path Tracing
  - [ ] Metropolis Light Transport
  - ...
- Scene
  - [x] smallpt
  - [ ] Cornell Box
  - [ ] Veach MIS
  - [ ] ...
- Aggregation
  - [ ] BVH
  - [ ] Kd-Tree
  - ...
- Post Processing
  - [x] Tone Mapping
  - [x] Gamma Correction
  - [x] White Balance
  - ...

# Example Scenes

Cornell Box MCPT 320x240 16384spp

![Cornell Box MCPT 320x240 16384spp](https://i.imgur.com/b27Enof.png)

# Gallery

https://storage.cloud.google.com/rust-ray-tracer/gallery/v0.1.1/report.html
