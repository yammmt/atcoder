use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [i32; n],
    }
    an.sort();
    // println!("{:?}", an);

    let mut ansp = 0;
    let mut ansk = 1;
    for k in 2..an[n - 1] + 1 {
        let mut cur = 0;
        for j in 0..n {
            if an[j] % k == 0 {
                cur += 1;
            }
        }
        // println!("cur: {}", cur);
        if cur > ansp {
            // println!("cur > ansp");
            ansp = cur;
            ansk = k;
        }
    }
    println!("{}", ansk);
}
