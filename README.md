# Fraya

An OK software path-tracer.

Development start: 16th of April 2026

# Current state

Renders a monkey at a relatively solid ~12 FPS(depends on distance) with 15,744 triangles.

BVH depth is 16, 24 depth gives slightly better results.

![Demo](screenshots/monkey_bvh-2026-05-26_22.22.26.png)

# TODO

- [x] Trace a single triangle
    - [x] Surfaces for rendering on
    - [x] SDL window surface
    - [x] Render module with high level primitives
    - [x] Möller–Trumbore intersection POC
- [x] A camera that can move and look
- [x] Multi-triangle rendering
    - [x] The concept of a `Mesh`.
    - [x] Finding a way to 
    - [x] Rendering a `Mesh`.
    - [x] For now via iterating one by one.
- [ ] BVH(May move)
    - [x] Cleanup some the APIs particularly `Triangle`.
    - [x] SAH-based AABB implemntation.
    - [x] Very naive traversal over the BVH.
    - [ ] T-pruning in traversal.
    - [ ] First visit the closer AABB.
- [ ] Point light
