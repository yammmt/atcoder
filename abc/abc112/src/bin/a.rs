use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    match n {
        1 => println!("Hello World"),
        2 => {
            input! {
                a: i32,
                b: i32,
            }
            println!("{}", a + b);
        }
        _ => unreachable!(),
    }
}
