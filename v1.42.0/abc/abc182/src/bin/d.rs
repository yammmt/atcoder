use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut move_max = Vec::with_capacity(n + 1);
    // move_max.push(0);
    let mut cur_max = 0;
    let mut cur = 0;
    for i in 0..n {
        cur_max = cur_max.max(cur + an[i]);
        move_max.push(cur_max);
        cur += an[i];
    }
    // println!("{:?}", move_max);

    let mut ans = 0;
    let mut prev = 0;
    let mut cur = 0;
    for i in 0..n {
        // println!("cur: {}", cur);
        ans = ans.max(cur + move_max[i]);
        cur += prev + an[i];
        prev += an[i];
    }
    println!("{}", ans);
}
