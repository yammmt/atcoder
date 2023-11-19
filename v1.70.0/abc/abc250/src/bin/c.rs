use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        xq: [usize; q],
    }

    let mut balls = (0..n).collect::<Vec<usize>>();
    let mut i_pos = (0..n).collect::<Vec<usize>>();
    for x in xq {
        let left_ball = x - 1;
        let left_pos = i_pos[left_ball];
        let right_ball = balls[if left_pos == n - 1 {
            left_pos - 1
        } else {
            left_pos + 1
        }];
        let right_pos = i_pos[right_ball];
        // println!("lb: {left_ball}, lp: {left_pos}");
        // println!("rb: {right_ball}, rp: {right_pos}");

        i_pos[left_ball] = right_pos;
        i_pos[right_ball] = left_pos;
        balls.swap(left_pos, right_pos);
        // println!("{:?}", balls);
    }

    for (i, b) in balls.iter().enumerate() {
        print!("{}", b + 1);
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
