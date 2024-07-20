use ordered_float::OrderedFloat;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abn: [(i64, i64); n],
        cdm: [(i64, i64); m],
    }

    let mut cnt = 9;
    let mut pass = 0.0;
    let mut fail = 100_000.0 * 5.0 + 1.0;
    while fail - pass > 1.0e-8 && cnt < 1000 {
        let mid = (pass + fail) / 2.0;

        let mut normals = vec![];
        for &(a, b) in &abn {
            normals.push((OrderedFloat(b as f64 - mid * a as f64), (a, b)));
        }
        normals.sort_unstable();
        normals.reverse();

        let mut helpers = vec![];
        for &(c, d) in &cdm {
            helpers.push((OrderedFloat(d as f64 - mid * c as f64), (c, d)));
        }
        helpers.sort_unstable();
        helpers.reverse();

        let mut magic = 0;
        let mut mass = 0;
        for monster in normals.iter().take(4) {
            magic += monster.1 .1;
            mass += monster.1 .0;
        }
        if helpers[0].0 > normals[4].0 {
            magic += helpers[0].1 .1;
            mass += helpers[0].1 .0;
        } else {
            magic += normals[4].1 .1;
            mass += normals[4].1 .0;
        }

        if magic as f64 / mass as f64 >= mid {
            pass = mid;
        } else {
            fail = mid;
        }

        cnt += 1;
    }

    println!("{pass}");
}
