fn find(map: &mut Vec<usize>, x: usize) -> usize {
    if map[x] != x {
        map[x] = find(map, map[x]);
    }

    map[x]
}

fn union(map: &mut Vec<usize>, a: usize, b: usize) {
    let root_a = find(map, a);
    let root_b = find(map, b);

    if root_a <= root_b {
        map[root_b] = root_a;
    } else {
        map[root_a] = root_b;
    }
}

fn equations_possible(equations: Vec<String>) -> bool {
    let mut map = (0..26).collect::<Vec<usize>>();
    for equation in &equations {
        let equation = equation.as_bytes();
        if equation[1] == b'=' {
            union(&mut map, (equation[0] - b'a') as usize, (equation[3] - b'a') as usize);
        }
    }

    for equation in equations {
        let equation = equation.as_bytes();
        if equation[1] == b'!' {
            let a = find(&mut map, (equation[0] - b'a') as usize);
            let b = find(&mut map, (equation[3] - b'a') as usize);
            if a == b { return false; }
        }
    }

    true
}

pub fn main() {
    let equations = ["a!=b","b!=c","c!=a"].into_iter().map(String::from).collect();
    println!("{}", equations_possible(equations));
}
