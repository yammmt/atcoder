use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let s_l = s.to_lowercase();
    let t_l = t.to_lowercase();
    println!(
        "{}",
        if s == t {
            "same"
        } else if s_l == t_l {
            "case-insensitive"
        } else {
            "different"
        }
    );
}
