use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    for i in (1..=1_000_000).rev() {
        let iii = i * i * i;

        if iii > n {
            continue;
        }

        let mut vkai = vec![];
        let mut iii_r = iii;
        while iii_r > 0 {
            vkai.push(iii_r % 10);
            iii_r /= 10;
        }
        let mut kaibun = 0;
        for k in vkai {
            kaibun *= 10;
            kaibun += k;
        }

        if kaibun == iii {
            println!("{iii}");
            return;
        }
    }
}
