// 桁 DP　(editorial は違う) 手動デバッグが難しいので祈る

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: u64,
        b: u64,
    }

    let mut nn = n;
    let mut vn = vec![];
    while nn > 0 {
        vn.push(nn % 10);
        nn /= 10;
    }
    vn.reverse();
    let vn = vn;

    // 桁 DP して f(m) をただ数えるだけでは m の値が保持できない
    // m の値を愚直に保持すると 10^11 個の値を管理することになり TLE
    // f(m) の値に対して m は一通りなので, あり得る f(m) を列挙して
    // 最後に残った f(m) すべてに対し m を算出して判定しよう
    let mut available_fm = HashSet::new();
    for nn in &vn {
        let mut cur_fm = HashSet::new();
        // 今の桁が最上位桁
        for j in 1..*nn {
            cur_fm.insert(j);
        }

        // n 未満であることが確定
        for afm in &available_fm {
            for j in 0..10 {
                cur_fm.insert(*afm * j);
            }
        }

        available_fm = cur_fm;
    }

    // m - f(m) = B => m = f(m) + B
    let mut ans = HashSet::new();
    for fm in &available_fm {
        // println!("{}", fm);
        let cur_m = b + *fm;
        if cur_m <= n {
            // println!("  {}", cur_m);
            let mut cur_fm = 1;
            let mut mm = cur_m;
            while mm > 0 {
                cur_fm *= mm % 10;
                mm /= 10;
            }

            if cur_m == b + cur_fm {
                ans.insert(*fm);
            }
        }
    }

    // n と一致する場合の積を見ていないのでここで見る
    // fnn は既出である場合がある (というか桁並び替えるだけなので大抵のケースでは既出になる？)
    let mut fnn = 1;
    for a in &vn {
        fnn *= *a;
    }
    if n == b + fnn {
        ans.insert(fnn);
    }

    println!("{}", ans.len());
}
