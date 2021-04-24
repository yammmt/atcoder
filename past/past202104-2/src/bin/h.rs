use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        q: usize,
        ptn: [(usize, usize); n],
    }

    let mut wo_kiri = vec![];
    let mut w_kiri = vec![];
    for pt in &ptn {
        if pt.1 == 0 {
            wo_kiri.push(pt.0);
        } else {
            w_kiri.push(pt.0);
        }
    }
    wo_kiri.sort();
    w_kiri.sort();
    // println!("{:?}", wo_kiri);
    // println!("{:?}", w_kiri);

    let mut ans = std::usize::MAX / 2;
    // 缶切り不要を i 個, 缶切り要を j 個
    let mut wo_kiri_sum = 0;
    let w_kiri_init_num = m.min(w_kiri.len());
    let mut w_kiri_sum = w_kiri.iter().take(w_kiri_init_num).sum::<usize>();
    for i in 0..wo_kiri.len() + 1 {
        // println!("wo: {}, w: {}", i, m - i);
        if i > wo_kiri.len() {
            break;
        } else if i + w_kiri_init_num < m {
            wo_kiri_sum += wo_kiri[i];
            continue;
        }

        let j = m - i;
        // println!("    wo: {}", wo_kiri_sum);
        // println!("    w: {}", w_kiri_sum);
        let cur = wo_kiri_sum + w_kiri_sum + q * ((j + k - 1) / k);
        // println!("  {}", cur);
        ans = ans.min(cur);
        if j == 0 {
            break;
        }

        if i != wo_kiri.len() {
            wo_kiri_sum += wo_kiri[i];
            w_kiri_sum -= w_kiri[j - 1];
        }
    }

    println!("{}", ans);
}
