// BinaryHeap では範囲内最大値のものを高速に更新できない

use proconio::input;

// v[i] > k を満たす最小の i を返す (or k より小さい v の要素数)
fn upper_bound(v: &[usize], k: usize) -> usize {
    let mut low: isize = -1;
    let mut high = v.len() as isize;

    while high - low > 1 {
        let mid = (low + high) / 2;
        if v[mid as usize] > k {
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
        mut an: [usize; n],
    }

    an.reverse();
    let mut ans = vec![an[0]];
    for a in an.iter().skip(1) {
        if ans[ans.len() - 1] <= *a {
            ans.push(*a);
        } else {
            let n = upper_bound(&ans, *a);
            ans[n] = *a;
        }
    }

    println!("{}", ans.len());
}
