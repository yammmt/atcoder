use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    // XOR 取った値と和が同じ = bit の出現箇所が重複しない
    // 尺取りでのびていく

    let mut ans = 0;
    let mut a_sum = 0;
    let mut a_xor = 0;
    let mut right = 0;
    for left in 0..n {
        while right < n && a_sum + an[right] == a_xor ^ an[right] {
            a_sum += an[right];
            a_xor ^= an[right];
            right += 1;
        }
        ans += right - left;

        a_sum -= an[left];
        a_xor ^= an[left];
    }

    println!("{ans}");
}
