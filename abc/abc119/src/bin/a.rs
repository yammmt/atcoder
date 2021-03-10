use proconio::input;

fn main() {
    input! {
        s: String,
    }
    println!(
        "{}",
        if &*s <= "2019/04/30" {
            "Heisei"
        } else {
            "TBD"
        }
    );
}
