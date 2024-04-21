use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
    }

    let mut two_pow = vec![1usize];
    while two_pow.len() < 63 {
        two_pow.push(*two_pow.last().unwrap() * 2);
    }
    let two_pow = two_pow;

    let two_pow_max = |n| {
        let mut lb = 0;
        let mut ub = two_pow.len();
        while ub - lb > 1 {
            let mid = (ub + lb) / 2;
            if n < two_pow[mid] {
                ub = mid;
            } else {
                lb = mid;
            }
        }

        lb
    };

    let mut l_cur = l;
    let mut ans = vec![];
    if l_cur == 0 {
        // l=0 の場合は r より小さい (以下？) 最大の 2 べきまで引っ張れる
        let p = two_pow_max(r);
        ans.push((0, two_pow[p]));
        l_cur = two_pow[p];
    }

    while l_cur < r {
        let mut p = two_pow_max(l_cur);
        let mut a = l_cur / two_pow[p];
        let mut b = two_pow[p] * (a + 1);
        while p != 0 && (l_cur % two_pow[p] != 0 || b > r) {
            p -= 1;
            a = l_cur / two_pow[p];
            b = two_pow[p] * (a + 1);
        }

        ans.push((l_cur, b));
        l_cur = b;
    }

    println!("{}", ans.len());
    for a in ans {
        println!("{} {}", a.0, a.1);
    }
}
