use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        an: [Usize1; n],
    }

    let mut members = vec![0; k];
    for a in an {
        members[a] += 1;
    }
    members.sort_unstable();
    members.reverse();
    let largest = *members.first().unwrap();
    let mut ans = 0;
    for m in members {
        if m >= largest - 1 {
            ans += 1;
        } else {
            break;
        }
    }

    println!("{ans}");
}
