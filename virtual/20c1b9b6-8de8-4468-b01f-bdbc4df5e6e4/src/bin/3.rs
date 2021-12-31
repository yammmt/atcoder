use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        hh: usize,
        ww: usize,
    }
    // 行を全消ししてから列を消せるだけとして
    println!("{}", h * w - (hh * w + ww * (h - hh)));
}
