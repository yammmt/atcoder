use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n],
    }

    let mut head_gap_max = 0;
    let mut head_gap_max_i = 0;
    for (i, (a, b)) in abn.iter().enumerate() {
        let head_gap = b - a;
        if head_gap > head_gap_max {
            head_gap_max = head_gap;
            head_gap_max_i = i;
        }
    }

    let shoulder_sum = abn.iter().map(|ab| ab.0).sum::<usize>();
    let ans = shoulder_sum - abn[head_gap_max_i].0 + abn[head_gap_max_i].1;
    println!("{ans}");
}
