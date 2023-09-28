use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut nn = n;
    let mut fnn = 0;
    while nn > 0 {
        fnn += nn % 10;
        nn /= 10;
    }
    println!(
        "{}",
        match n % fnn {
            0 => "Yes",
            _ => "No",
        }
    );
}
