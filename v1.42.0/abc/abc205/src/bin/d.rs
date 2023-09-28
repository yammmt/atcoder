use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        an: [usize; n],
        kq: [usize; q],
    }

    // 正整数 (no 0)
    for k in &kq {
        // an[0] 未満
        if *k < an[0] {
            println!("{}", k);
            continue;
        }

        let mut pass = 0;
        let mut fail = n;

        while fail - pass > 1 {
            let mid = (pass + fail) / 2;
            // an[mid] 時点で小さい方から an[mid] - mid - 1 個の数字は既出
            if an[mid] - mid - 1 < *k {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        // println!("{} {}", pass, fail);

        println!("{}", an[pass] + *k - (an[pass] - pass - 1));
    }
}
