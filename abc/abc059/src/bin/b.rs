use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    }

    if a.len() > b.len() {
        println!("GREATER");
    } else if a.len() < b.len() {
        println!("LESS");
    } else {
        if a > b {
            println!("GREATER");
        } else if a < b {
            println!("LESS");
        } else {
            println!("EQUAL");
        }
    }
}
