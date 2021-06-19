use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut pass = n;
    let mut fail = 0;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        if mid * (mid + 1) / 2 >= n {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{}", pass);
}
