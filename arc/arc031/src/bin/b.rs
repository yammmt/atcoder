use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        axy: [Chars; 10],
    }

    let dir = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for x in 0..10 {
        for y in 0..10 {
            let mut curmass = axy.clone();
            curmass[x][y] = 'o';
            let mut visited = vec![vec![false; 10]; 10];
            let mut vdq = VecDeque::new();
            vdq.push_back((x, y));
            visited[x][y] = true;
            while let Some(cur) = vdq.pop_front() {
                for d in &dir {
                    let next_x = cur.0 as isize + d.0;
                    let next_y = cur.1 as isize + d.1;
                    if !(0..=9).contains(&next_x) || !(0..=9).contains(&next_y) {
                        continue;
                    }

                    let unext_x = next_x as usize;
                    let unext_y = next_y as usize;
                    if !visited[unext_x][unext_y] && curmass[unext_x][unext_y] == 'o' {
                        vdq.push_back((unext_x, unext_y));
                        visited[unext_x][unext_y] = true;
                    }
                }
            }
            // println!("{:?}", visited);

            'outer: for i in 0..10 {
                for j in 0..10 {
                    if curmass[i][j] == 'o' && !visited[i][j] {
                        break 'outer;
                    }
                }
                if i == 9 {
                    // println!("{}, {}", x, y);
                    println!("YES");
                    return;
                }
            }
        }
    }

    println!("NO");
}
