fn backtracking(
    i: usize,
    stack: &mut Vec<i64>,
    nums: &[u8],
    express: &mut Vec<String>,
    target: i64,
    result: &mut Vec<String>,
) {
    if i == nums.len() {
        let mut sum = 0;
        for &num in stack.iter() {
            sum += num;
        }
        if sum == target {
            result.push(express.iter().cloned().collect::<_>());
        }

        return;
    }

    let mut res = 0;
    let mut flag = false;
    for i in i..nums.len() {
        if flag {
            return;
        }

        let num = (nums[i] - b'0') as i64;
        if res == 0 && num == 0 {
            flag = true;
        }
        res = res * 10 + (nums[i] - b'0') as i64;

        if stack.is_empty() {
            stack.push(res);
            express.push(res.to_string());
            backtracking(i+1, stack, nums, express, target, result);
            stack.pop();
            express.pop();
        } else {
            // multi
            let prev = stack.pop().unwrap();
            stack.push(prev * res);
            express.push('*'.to_string());
            express.push(res.to_string());
            backtracking(i+1, stack, nums, express, target, result);
            stack.pop();
            express.pop();
            express.pop();
            stack.push(prev);

            // minus
            stack.push(-res);
            express.push('-'.to_string());
            express.push(res.to_string());
            backtracking(i+1, stack, nums, express, target, result);
            stack.pop();
            express.pop();
            express.pop();

            // plus
            stack.push(res);
            express.push('+'.to_string());
            express.push(res.to_string());
            backtracking(i+1, stack, nums, express, target, result);
            stack.pop();
            express.pop();
            express.pop();
        }
    }
}

fn add_operators(num: String, target: i32) -> Vec<String> {
    let num = num.as_bytes();
    let mut result = vec![];
    backtracking(0, &mut vec![], num, &mut vec![], target as i64, &mut result);

    result
}

pub fn main() {
    let num = "105".to_string();
    let target = 5;
    println!("{:?}", add_operators(num, target));
}
