use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    println!(
        "{}",
        match a == 2 || b == 2 {
            true => "No",
            false => "Yes",
        }
    );
}
