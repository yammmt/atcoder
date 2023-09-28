// 12min (エスパー)

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
    }

    let mut vconnected = vec![0; n];
    for ab in &abm {
        vconnected[ab.0 - 1] += 1;
        vconnected[ab.1 - 1] += 1;
    }
    println!(
        "{}",
        match vconnected.iter().any(|a| a % 2 == 1) {
            true => "NO",
            false => "YES",
        }
    );
}
