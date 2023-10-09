use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: usize,
        mut f_n: [usize; n],
    }


    f_n.sort();
    f_n.reverse();
    let mut normal_fee_sum: usize = f_n.iter().sum();
    let mut normal_fee_idx_min = 0;
    let mut tour_purchased = 0;
    let mut ans = usize::MAX;
    // (tour_purchased - 1) * d <= n
    while tour_purchased * d <= n + d {
        let tour_date = tour_purchased * d;
        for i in normal_fee_idx_min..tour_date.min(n) {
            normal_fee_sum -= f_n[i];
        }
        normal_fee_idx_min = tour_date;

        let cur = tour_purchased * p + normal_fee_sum;
        ans = ans.min(cur);

        tour_purchased += 1;
    }

    println!("{ans}");
}
