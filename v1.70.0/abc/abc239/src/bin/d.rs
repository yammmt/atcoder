use proconio::input;

fn is_prime(n: i64) -> bool {
    if n == 1 {
        return false;
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }
    true
}

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    // 高橋くんが出す手を決めると青木くんに勝ち目がなくなるなら高橋くん
    // そうでなければ青木くんが勝てる
    for i in a..=b {
        let mut takahashi_win = true;
        for j in c..=d {
            if is_prime(i + j) {
                takahashi_win = false;
                break;
            }
        }

        if takahashi_win {
            println!("Takahashi");
            return;
        }
    }

    println!("Aoki");
}
