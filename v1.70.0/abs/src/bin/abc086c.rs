use proconio::input;

fn main() {
    input! {
        n: usize,
        txyn: [(i32, i32, i32); n],
    }

    let mut cur = (0, 0, 0);
    for txy in &txyn {
        let move_amount = (txy.1 - cur.1).abs() + (txy.2 - cur.2).abs();
        let move_time = txy.0 - cur.0;
        if !(move_amount <= move_time && (move_time - move_amount) % 2 == 0) {
            println!("No");
            return;
        }

        cur = *txy;
    }

    println!("Yes");
}
