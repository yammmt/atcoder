use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        lrn: [(isize, isize); n],
    }

    // 最{大,小}値の総和を取ると区間内の数は全部作れる, って名前ついていたっけか

    let mut lsum = 0;
    let mut rsum = 0;
    for &(l, r) in &lrn {
        lsum += l;
        rsum += r;
    }

    if lsum > 0 || rsum < 0 {
        println!("No");
        return;
    }

    let mut ans = Vec::with_capacity(n);
    let mut ans_sum = 0;
    for &(l, _r) in &lrn {
        ans.push(l);
        ans_sum += l;
    }
    for (i, &(l, r)) in lrn.iter().enumerate() {
        // ans_sum <= 0 が常に成り立つ
        let added = (0 - ans_sum).min(r - l);
        ans[i] += added;
        ans_sum += added;
    }

    println!("Yes");
    for (i, v) in ans.iter().enumerate() {
        print!("{v}");
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
