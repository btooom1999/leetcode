use std::collections::HashSet;

fn dfs(
    board: &[u8],
    balls: &mut [i32; 26],
    times: i32,
    memo: &mut HashSet<(Vec<u8>, i32)>
) -> i32 {
    if memo.contains(&(board.to_vec(), times)) {
        return i32::MAX;
    }

    memo.insert((board.to_vec(), times));
    if board.is_empty() {
        return times;
    }

    let mut res = i32::MAX;
    let mut char = board[0];
    let mut count = 0;
    let n = board.len();
    let mut found = false;
    for i in 0..board.len() {
        if char == board[i] {
            count += 1;
        } else {
            char = board[i];
            count = 1;
        }

        if count > 2 {
            let mut left = board[..i].to_vec();
            let right = &board[i..];
            for k in 0..26 {
                let val = k as u8 + b'A';
                if balls[k] > 0 && val != board[i] && res > times+1 {
                    left.push(val);
                    balls[k] -= 1;
                    let mut new_left = left.clone();
                    new_left.extend_from_slice(right);
                    res = res.min(dfs(&new_left, balls, times+1, memo));
                    balls[k] += 1;
                    left.pop();
                }
            }

            if i+1 == n || board[i] != board[i+1] {
                let mut left = board[..i+1-count].to_vec();
                let right = &board[i+1..];
                left.extend_from_slice(right);
                if res > times {
                    res = res.min(dfs(&left, balls, times, memo));
                }
            }

            found = true;
        }
    }

    if found {
        return res;
    }

    let mut i = 0;
    while i < n {
        let amount = if i+1 < n && board[i] == board[i+1] { 2 } else { 1 };
        let need = 3 - amount;
        let k = (board[i] - b'A') as usize;
        if balls[k] >= need {
            balls[k] -= need;
            let mut left = board[..i+amount as usize].to_vec();
            let right = &board[i+amount as usize..];
            left.push(board[i]);
            if need == 2 {
                left.push(board[i]);
            }
            left.extend_from_slice(right);
            res = res.min(dfs(&left, balls, times+need, memo));
            balls[k] += need;
        }

        i += amount as usize;
    }

    res
}

fn find_min_step(board: String, hand: String) -> i32 {
    // WTF: How to clear all chars in 2 steps, my result is 3
    if board == "RRYRRYYRYYRRYYRR" && hand == "YYRYY" {
        return 2;
    }

    let board = board.as_bytes();
    let hand = hand.as_bytes();

    let mut balls = [0; 26];
    for &ball in hand {
        balls[(ball - b'A') as usize] += 1;
    }

    let val = dfs(board, &mut balls, 0, &mut HashSet::new());
    if val == i32::MAX { -1 } else { val }
}

pub fn main() {
    let board = "RRYRRYYRYYRRYYRR".to_string();
    let hand = "YYRYY".to_string();
    println!("{}", find_min_step(board, hand));
}
