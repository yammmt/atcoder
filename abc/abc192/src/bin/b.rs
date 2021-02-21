use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let vc = s.chars().collect::<Vec<char>>();

    for i in 0..vc.len() {
        if (i % 2 == 0 && vc[i].is_ascii_uppercase())
            || (i % 2 == 1 && vc[i].is_ascii_lowercase())
        {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
