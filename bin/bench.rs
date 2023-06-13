#[macro_use]
extern crate timeit;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

use gnuplot::{AxesCommon, Caption, Figure};
use other_octree::Octree as OtherOctree;

use octree::point::Point3D;
use octree::Octree;

// To satisfy the other crate's requirement.
struct Point3DIterator {
    points: Vec<Point3D>,
}

struct Wapper(Vec<Point3D>);

impl IntoIterator for Wapper {
    type Item = [f64; 3];
    type IntoIter = Point3DIterator;

    fn into_iter(self) -> Self::IntoIter {
        Point3DIterator { points: self.0 }
    }
}

impl Iterator for Point3DIterator {
    type Item = [f64; 3];

    fn next(&mut self) -> Option<Self::Item> {
        self.points
            .pop()
            .map(|p| [p.x as f64, p.y as f64, p.z as f64])
    }
}

fn read_points(file: File) -> Vec<Point3D> {
    let reader = BufReader::new(file);
    let mut points = Vec::new();

    for line in reader.lines() {
        let numbers: Vec<f32> = line
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        points.push(Point3D::new(numbers[0], numbers[1], numbers[2]));
    }
    points
}

fn bench_test(points: &Vec<Point3D>) -> BTreeMap<usize, f64> {
    let mut results = BTreeMap::new();
    for i in (0..=points.len()).step_by(1000) {
        results.insert(
            i,
            timeit_loops!(10, {
                Octree::new(points.iter().take(i).collect());
            }),
        );
    }
    results
}

fn bench_test_baseline(points: &Vec<Point3D>) -> BTreeMap<usize, f64> {
    let mut results = BTreeMap::new();
    for i in (0..points.len()).step_by(1000) {
        results.insert(
            i,
            timeit_loops!(10, {
                let mut tree = OtherOctree::new(Wapper(points.clone()));
                tree.build(8);
            }),
        );
    }
    results
}

#[cfg_attr(tarpaulin, skip)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        eprintln!("\x1b[93mRunning benchmark in debug mode is meaningless. add '--release' option!\x1b[0m");
    }

    // Open the text file
    let file = File::open("./data/points.txt")?;
    let points = read_points(file);
    let results = bench_test(&points);
    let baseline_results = bench_test_baseline(&points);
    let output_path = "./data/bench.png";

    let mut figure = Figure::new();
    let num_points: Vec<usize> = results.keys().cloned().collect();
    let durations: Vec<f64> = results.values().cloned().collect();
    let durations_baseline: Vec<f64> = baseline_results.values().cloned().collect();

    figure
        .axes2d()
        .lines_points(
            &num_points,
            &durations,
            &[Caption("My Octree, Single Thread")],
        )
        .lines_points(&num_points, &durations_baseline, &[Caption("Baseline")])
        .set_title("Octree Building Benchmark", &[])
        .set_x_label("Number of Points", &[])
        .set_y_label("Durations (sec)", &[]);

    figure
        .save_to_png(output_path, 800, 600)
        .expect("Failed to save figure");

    println!("Benchmark result is saved to {}", output_path);
    Ok(())
}
