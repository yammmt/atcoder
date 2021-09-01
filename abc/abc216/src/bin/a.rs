use proconio::input;

fn main() {
    input! {
        xy: f64,
    }
    let x = xy.floor() as u32;
    let y = (xy * 10.0) as u32 % 10;
    if y <= 2 {
        println!("{}-", x);
    } else if y <= 6 {
        println!("{}", x);
    } else {
        println!("{}+", x);
    }
}
