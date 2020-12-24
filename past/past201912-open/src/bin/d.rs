use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i32; n],
    }
    let mut cnt = vec![0; n + 1];
    for a in &an {
        cnt[*a as usize] += 1;
    }
    let mut ooi = 0;
    let mut sukunai = 0;
    for (i, c) in cnt.iter().enumerate().skip(1) {
        if *c == 2 {
            ooi = i;
        } else if *c == 0 {
            sukunai = i;
        }
    }
    if ooi == 0 {
        println!("Correct");
    } else {
        println!("{} {}", ooi, sukunai);
    }
}
