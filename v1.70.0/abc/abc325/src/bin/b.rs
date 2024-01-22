use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        wxn: [(usize, usize); n],
    }

    let mut participants = vec![0; 24];
    for (w, x) in wxn {
        let x_begin = (x + 9) % 24;
        let x_end = x_begin + 8;
        for i in x_begin..=x_end.min(23) {
            participants[i] += w;
        }
        let x_end = x_end % 24;
        if x_end < x_begin {
            for i in 0..=x_end {
                participants[i] += w;
            }
        }
    }

    println!("{}", participants.iter().max().unwrap());
}
