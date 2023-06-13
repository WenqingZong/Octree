#[macro_use]
extern crate timeit;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;

use gnuplot::{AxesCommon, Caption, Figure};

use octree::point::Point3D;
use octree::Octree;

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
        results.insert(i, timeit_loops!(10, {
            Octree::new(points.iter().take(i).collect());
        }));
    }
    results
}

#[cfg_attr(tarpaulin, skip)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the text file
    let file = File::open("./data/points.txt")?;
    let points = read_points(file);
    let results = bench_test(&points);
    let output_path = "./data/bench.png";

    let mut figure = Figure::new();
    let num_points: Vec<usize> = results.keys().cloned().collect();
    let durations: Vec<f64> = results.values().cloned().collect();

    figure.axes2d()
    .lines_points(&num_points, &durations, &[Caption("My Octree, Single Thread")])
    .set_title("Octree Building Benchmark", &[])
    .set_x_label("Number of Points", &[])
    .set_y_label("Durations (sec)", &[]);

    figure.save_to_png(output_path, 800, 600).expect("Failed to save figure");

    Ok(())
}
