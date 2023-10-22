use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    if n == 2 {
        println!("1");
        return;
    }

    // 入力時点でのグラフは上に凸のような形状だったりギザギザしたり
    // 直感的には平均に均すじゃだめ？ WA になる
    // 唯一の極値をもてばそれで終わり, ではない

    // そもそも Ai の合計も 100 でしかないし貪欲ではだめ？だめそう
    let a_sum = an.iter().sum::<i64>();
    let mut bn = vec![a_sum / (n as i64 - 2); n];
    bn[0] = 0;
    bn[n - 1] = 0;
    let mut b_sum = bn.iter().sum::<i64>();
    while b_sum < a_sum {
        let mut updated = 0;
        let mut smallest_pts = i64::MAX / 3;
        for i in 1..n - 1 {
            let p_before = (bn[i] - bn[i - 1]).pow(2) + (bn[i + 1] - bn[i]).pow(2);
            let p_after = (bn[i] + 1 - bn[i - 1]).pow(2) + (bn[i + 1] - (bn[i] + 1)).pow(2);
            let pts = p_after - p_before;
            if pts < smallest_pts {
                updated = i;
                smallest_pts = pts;
            }
        }
        bn[updated] += 1;
        b_sum += 1;
        // println!("{:?}", bn);
    }

    let mut ans = 0.0f64;
    // sample2
    // let bn = [0, 3, 3, 3, 3, 3, 0];
    // println!("{:?}", bn);
    for i in 1..n {
        ans += (((bn[i] - bn[i - 1]) as f64).powf(2.0) + 1.0).sqrt();
    }

    println!("{ans}");
}
