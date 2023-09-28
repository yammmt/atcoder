// WA: 察し

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    println!(
        "{}",
        if h > 1 && w > 1 {
            ((w + 1) / 2) * ((h + 1) / 2)
        } else if h == 1 {
            w
        } else if w == 1 {
            h
        } else {
            // h = w = 1
            1
        }
    );
}
