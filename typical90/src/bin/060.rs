// LIS 最長増加部分列

use proconio::input;

const INF: usize = std::usize::MAX / 3;

fn lower_bound(v: &[usize], k: usize) -> usize {
    let mut low: isize = -1;
    let mut high = v.len() as isize;

    while high - low > 1 {
        let mid = (low + high) / 2;
        if v[mid as usize] >= k {
            high = mid;
        } else {
            low = mid;
        }
    }
    high as usize
}

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut dp_up = vec![INF; n];
    let mut len_up = vec![];
    for a in &an {
        let i = lower_bound(&dp_up, *a);
        dp_up[i] = *a;
        len_up.push(i);
    }
    // println!("{:?}", dp_up);
    for i in 1..n {
        len_up[i] = len_up[i].max(len_up[i - 1]);
    }
    // println!("{:?}",len_up);

    let mut dp_down = vec![INF; n];
    let mut len_down = vec![];
    for a in an.iter().rev() {
        let i = lower_bound(&dp_down, *a);
        dp_down[i] = *a;
        len_down.push(i);
    }
    // dp_down.reverse();
    // println!("{:?}", dp_down);
    for i in 1..n {
        len_down[i] = len_down[i].max(len_down[i - 1]);
    }
    len_down.reverse();
    // println!("{:?}",len_down);

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(len_up[i] + len_down[i] + 1);
    }

    println!("{}", ans);
}
