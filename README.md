
# Ray Tracing In One Week in Rust

this is my implementation of [Peter Shirley's Ray Tracing in One Week](https://raytracing.github.io/books/RayTracingInOneWeekend.html) with Rust language.

In this repo, I'll try my best to make full use of Rust's features to fulfill this mission. I expect the whole period to be within one month.

## Installation and Execution

### Clone the source code

```bash
git clone git@github.com:Hyiker/ray_tracing_in_one_week_rust.git
```

### Render a sample image

```bash
cd ./ray_tracing_in_one_week_rust
```

```bash
cargo build && cargo run > output/sample.ppm
```

or, run this code with release build mode(could be way much faster)

```bash
cargo build --release && cargo run --release > output/sample.ppm
```

## Chapters to finish

- [x] 2. Output an Image
- [x] 3. The vec3 Class
- [x] 4. Rays, a Simple Camera, and Background
- [x] 5. Adding a Sphere
- [x] 6. Surface Normals and Multiple Objects
- [x] 7. Antialiasing
- [x] 8. Diffuse Materials
- [x] 9. Metal
- [x] 10. Dielectrics
- [ ] 11. Positionable Camera
- [ ] 12. Defocus Blur
- [ ] 13. Where Next?
