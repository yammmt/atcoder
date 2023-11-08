use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        an: [usize; n],
    }

    // かかる時間の総和
    let mut front_half = vec![];
    for i in 0..2u64.pow((n / 2) as u32) {
        let mut cur_time = 0;
        let mut ii = i;
        for a in an.iter().take(n / 2) {
            if ii % 2 == 0 {
                cur_time += a;
            }
            ii /= 2;
        }
        front_half.push(cur_time);
    }
    front_half.sort_unstable();

    let mut ans = 0;
    let rear_half_len = if n % 2 == 0 { n - n / 2 } else { n - n / 2 + 1 };
    for i in 0..2u64.pow(rear_half_len as u32) {
        let mut cur_time = 0;
        let mut ii = i;
        for a in an.iter().take(n).skip(n / 2) {
            if ii % 2 == 0 {
                cur_time += a;
            }
            ii /= 2;
        }
        if cur_time > t {
            continue;
        }

        let rest_time = t - cur_time;
        let mut pass = 0;
        let mut fail = front_half.len();
        while fail - pass > 1 {
            let mid = (pass + fail) / 2;
            if front_half[mid] <= rest_time {
                pass = mid;
            } else {
                fail = mid;
            }
        }

        ans = ans.max(cur_time + front_half[pass]);
    }

    println!("{ans}");
}
