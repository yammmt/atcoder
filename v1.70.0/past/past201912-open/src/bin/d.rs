use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [Usize1; n],
    }

    let mut cnt = vec![0; n];
    for a in an {
        cnt[a] += 1;
    }

    if cnt.iter().all(|&c| c == 1) {
        println!("Correct");
    } else {
        let mut written_from = 0;
        let mut written_to = 0;
        for (i, &c) in cnt.iter().enumerate() {
            if c == 0 {
                written_from = i + 1;
            } else if c == 2 {
                written_to = i + 1;
            }
        }
        println!("{written_to} {written_from}");
    }
}
