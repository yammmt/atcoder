use proconio::fastout;
use proconio::input;
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
        e: u32,
        f: u32,
        x: u32,
    }

    let score = |walk_time, speed, rest_time| {
        let cycle_time: u32 = walk_time + rest_time;
        walk_time * speed * (x / cycle_time) + (x % cycle_time).min(walk_time) * speed
    };

    let tkhs = score(a, b, c);
    let aoki = score(d, e, f);
    println!(
        "{}",
        match tkhs.cmp(&aoki) {
            Ordering::Less => "Aoki",
            Ordering::Equal => "Draw",
            Ordering::Greater => "Takahashi",
        }
    );
}
