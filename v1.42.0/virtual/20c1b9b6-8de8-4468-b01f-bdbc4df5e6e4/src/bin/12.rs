use proconio::input;

fn main() {
    input! {
        n: usize,
        dn2: [(usize, usize); n],
    }

    let mut streaks = 0;
    for d in dn2 {
        streaks = if d.0 == d.1 { streaks + 1 } else { 0 };

        if streaks == 3 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
