use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i32; n],
    }

    let mut ans = i32::MAX;
    for a in an {
        let mut cur = 0;
        let mut aa = a;
        while aa % 2 == 0 {
            aa /= 2;
            cur += 1;
        }
        ans = ans.min(cur);
    }

    println!("{ans}");
}
