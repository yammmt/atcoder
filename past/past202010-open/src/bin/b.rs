use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
    }
    if y == 0 {
        println!("ERROR");
        return;
    }

    print!("{}.", x / y);
    let mut cur_x = x % y;
    for _ in 0..2 {
        cur_x *= 10;
        print!("{}", cur_x / y);
        cur_x %= y;
    }

    println!();
}
