// :fu: 21-05 一歩目はともかく以後の条件分岐ややこしく案の定ペナ率が高い
// K が奇数かつ A_n がすべて負数である場合のみ剰余前の値が負となる
//   A_n に余っている正数があるならそれを入れてやれば負数の個数の偶奇を反転できるため
// WA: どうして？正の数の個数を減らしていっても通るのでは？

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        an: [i64; n],
    }
    let d = 1_000_000_007;

    if n == k {
        let mut ans = 1;
        for a in &an {
            ans = (ans * a.abs()) % d;
        }
        println!(
            "{}",
            if an.iter().filter(|&a| *a < 0).count() % 2 == 0 {
                ans % d
            } else {
                d - ans
            }
        );
        return;
    }

    let mut org_plus = vec![];
    let mut org_minus = vec![];
    for a in &an {
        if *a < 0 {
            org_minus.push(*a);
        } else {
            org_plus.push(*a);
        }
    }
    org_plus.sort();
    org_minus.sort();

    if k % 2 == 1 && an.iter().all(|&a| a < 0) {
        // どうしても負数になるので絶対値昇順に掛ける
        let mut ans = 1;
        org_minus.reverse();
        for m in org_minus.iter().take(k) {
            ans = (ans * m.abs()) % d;
        }
        println!("{}", d - ans);
        return;
    }

    // Editional 2: 偶奇の辻褄合わない気がするがどうして
    org_plus.reverse();
    while org_plus.len() < k {
        // 番兵
        org_plus.push(0);
    }
    let mut ans = 1;
    let mut cur_k = 0;
    let mut p_i = 0;
    let mut m_i = 0;
    while cur_k < k {
        if cur_k == k - 1 {
            ans = (ans * org_plus[p_i]) % d;
            cur_k += 1;
        } else if m_i + 1 >= org_minus.len() {
            // 負数はもう使えない
            while cur_k < k {
                ans = (ans * org_plus[p_i]) % d;
                p_i += 1;
                cur_k += 1;
            }
        } else {
            let pp = org_plus[p_i] * org_plus[p_i + 1];
            let mm = org_minus[m_i] * org_minus[m_i + 1];
            if pp > mm {
                ans = (ans * (org_plus[p_i] % d)) % d;
                p_i += 1;
                cur_k += 1;
            } else {
                ans = (ans * (mm % d)) % d;
                m_i += 2;
                cur_k += 2;
            }
        }
    }

    println!("{}", ans);
}
