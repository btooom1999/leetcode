use std::collections::{HashMap, HashSet};

fn find(map: &mut Vec<usize>, x: usize) -> usize {
    if map[x] != x {
        map[x] = find(map, map[x]);
    }

    map[x]
}

fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let n = accounts.len();
    let mut graph = (0..n).collect::<Vec<_>>();
    let mut hashmap = HashMap::new();
    for i in 0..n {
        let emails = &accounts[i][1..];
        let mut root_a = i;
        for email in emails {
            if let Some(&j) = hashmap.get(email) {
                let root_b = find(&mut graph, j);
                if root_a > root_b {
                    graph[root_a] = root_b;
                    root_a = root_b;
                } else {
                    graph[root_b] = root_a;
                }
            }
        }

        for email in emails {
            hashmap.insert(email.to_owned(), root_a);
        }
    }

    let mut res = HashMap::<_, HashSet<String>>::new();
    for i in 0..n {
        let parent = graph[i];
        let root = find(&mut graph, parent);
        let accounts = &accounts[i][1..];
        let entry = res.entry(root).or_default();
        for account in accounts {
            entry.insert(account.to_owned());
        }
    }

    res
        .into_iter()
        .map(|(i, rest)| {
            let mut res = vec![accounts[i][0].clone()];
            let mut rest = rest.into_iter().collect::<Vec<_>>();
            rest.sort();
            res.extend(rest);

            res
        })
        .collect()
}

pub fn main() {
    let accounts = [
        ["John","johnsmith@mail.com","john_newyork@mail.com"].to_vec(),
        ["John","johnsmith@mail.com","john00@mail.com"].to_vec(),
        ["Mary","mary@mail.com"].to_vec(),
        ["John","johnnybravo@mail.com"].to_vec()
    ]
        .into_iter().
        map(|m| m
            .into_iter()
            .map(String::from)
            .collect())
        .collect();
    println!("{:?}", accounts_merge(accounts));
}
