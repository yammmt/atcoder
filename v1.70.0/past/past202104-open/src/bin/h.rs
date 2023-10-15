use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        q: i64,
        ptn: [(i64, i64); n],
    }

    let mut w_cankiri = vec![];
    let mut wo_cankiri = vec![];
    for pt in &ptn {
        if pt.1 == 1 {
            w_cankiri.push(pt.0);
        } else {
            wo_cankiri.push(pt.0);
        }
    }
    w_cankiri.sort_unstable();
    wo_cankiri.sort_unstable();

    let mut ans = i64::MAX;
    // 缶切りつきを i 個, 缶切りなしを j 個買う
    let mut j = m.min(wo_cankiri.len());
    let mut i = m - j;
    let mut w_cankiri_sum = w_cankiri.iter().take(i).sum::<i64>();
    let mut wo_cankiri_sum = wo_cankiri.iter().take(j).sum::<i64>();
    loop {
        let ii = i as i64;
        let cur = (ii + k - 1) / k * q + w_cankiri_sum + wo_cankiri_sum;
        ans = ans.min(cur);

        if i >= w_cankiri.len() || j == 0 {
            break;
        }

        w_cankiri_sum += w_cankiri[i];
        wo_cankiri_sum -= wo_cankiri[j - 1];

        i += 1;
        j -= 1;
    }

    println!("{ans}");
}
