use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    // push/pop は最大で 2N 回しか発生しないが streak 判断が長くなると辛そう
    // 現在の streak で枝刈りできそう 20 が 3 個続いて次に 30 が 2 個、次に... という感じ
    // 中間が抜き去られることはない

    let mut tsutsu = vec![];
    let mut ball_len = 0;
    for a in &an {
        if ball_len == 0 {
            tsutsu.push((*a, 1));
            ball_len += 1;
        } else if let Some(last_ball) = tsutsu.pop() {
            if *a == last_ball.0 {
                if *a == last_ball.1 + 1 {
                    ball_len -= *a - 1;
                } else {
                    tsutsu.push((*a, last_ball.1 + 1));
                    ball_len += 1;
                }
            } else {
                tsutsu.push(last_ball);
                tsutsu.push((*a, 1));
                ball_len += 1;
            }
        } else {
            unreachable!();
        }

        println!("{ball_len}");
    }
}
