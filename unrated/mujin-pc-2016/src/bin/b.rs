// :fu: 超数問

use proconio::input;

fn main() {
    input! {
        mut edges: [i32; 3],
    }
    edges.sort_unstable();

    let radius_outer = edges.iter().sum::<i32>() as f64;
    let radius_inner = if edges[0] + edges[1] < edges[2] {
        (edges[2] - edges[1] - edges[0]) as f64
    } else {
        0.0
    };

    println!(
        "{}",
        radius_outer.powf(2.0) * std::f64::consts::PI
            - radius_inner.powf(2.0) * std::f64::consts::PI
    );
}
