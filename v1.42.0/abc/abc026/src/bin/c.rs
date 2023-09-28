// 17.5 min

use proconio::input;

fn main() {
    input! {
        n: usize,
        bn: [usize; n - 1],
    }

    let mut vbuka = vec![vec![]; n + 1];
    for (i, b) in bn.iter().enumerate() {
        vbuka[*b].push(i + 2);
    }
    // println!("{:?}", vbuka);

    let mut vsalary = vec![1; n + 1];
    for i in (1..n + 1).rev() {
        if vbuka[i].is_empty() {
            continue;
        }

        let mut smax = 0;
        let mut smin = std::usize::MAX;
        for j in &vbuka[i] {
            smax = smax.max(vsalary[*j]);
            smin = smin.min(vsalary[*j]);
        }
        vsalary[i] = smax + smin + 1;
    }
    // println!("{:?}", vsalary);

    println!("{}", vsalary[1]);
}
