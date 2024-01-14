use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n],
    }
    let mut an_sorted = an.clone();
    an_sorted.sort_unstable();

    let mut v = vec![vec![]; k];
    for (i, a) in an.iter().enumerate() {
        v[i % k].push(*a);
    }

    for vv in &mut v {
        vv.sort_unstable();
    }
    let mut vcur = vec![];
    let mut vi = 0;
    for i in 0..n {
        vcur.push(v[i % k][vi]);
        if i % k == k - 1 {
            vi += 1;
        }
    }

    for i in 0..n {
        if vcur[i] != an_sorted[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
