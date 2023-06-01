# Octree
Rust library for Octree, optimised for multi-threading, capable of tracking highly dynamic environment.

<div align="center">

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/WenqingZong/Octree/rust_ci.yml?style=for-the-badge)
[![Documentation](https://img.shields.io/badge/Doc-Read-blue?style=for-the-badge)](https://wenqingzong.github.io/Octree/octree/index.html)
![Codecov](https://img.shields.io/codecov/c/github/WenqingZong/Octree?style=for-the-badge)

</div>

## Todo
This library is under development, currently it's only a single threaded Octree implementation, but already supports dynamic object tracking.

Things to do in the near future:
 - Extend to multi-thread, can use [adriankrupa](https://github.com/adriankrupa/Octree)'s `C++` implementation as a reference.
 - Benchmarking. Currently the fastest `rust` Octree is [this](https://crates.io/crates/octree) crate, but it's implemented in single thread as well. It's expected that a multi-thread implementation performs faster.
 - A demo. Could use this library to reimplement my [N Body Simulator](https://github.com/WenqingZong/N-Body-Simulator). My current simulator was a coursework for `Computer Graphics` course at The University of Manchester, it used `python` with an $O(n^2)$ algorithm to calculate acceleration, its `FPS` is horrible.