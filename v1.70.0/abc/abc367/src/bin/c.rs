use proconio::fastout;
use proconio::input;

const R_MAX: usize = 5;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        rn: [usize; n],
    }

    // 最長パターンは [1, 5] の整数を 8 つ
    // 八重ループを書く…は現実味がなく, bit 全探索的な剰余場合分けをする

    let mut ans = vec![];
    for mut bit in 0..R_MAX.pow(n as u32) {
        let mut r_pass = true;
        let mut cur = Vec::with_capacity(n);
        for i in 0..n {
            let candidate = bit % R_MAX + 1;
            if candidate > rn[i] {
                r_pass = false;
                break;
            }

            cur.push(candidate);
            bit /= R_MAX;
        }

        if !r_pass {
            continue;
        }

        if cur.iter().sum::<usize>() % k == 0 {
            ans.push(cur);
        }
    }

    // 要る？列挙順で保証できるのでは
    ans.sort_unstable();
    for a in ans {
        for (i, aa) in a.iter().enumerate() {
            print!("{aa}");
            if i == a.len() - 1 {
                println!();
            } else {
                print!(" ");
            }
        }
    }
}
