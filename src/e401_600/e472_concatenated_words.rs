use std::collections::HashSet;

const MOD: i64 = 2i64.pow(45)-1;
const BASE: i64 = 31;

#[derive(Debug)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    last: bool,
}

impl Trie {
    fn new() -> Self {
        const NONE: Option<Box<Trie>> = None;
        Self { children: [NONE; 26], last: false }
    }
}

fn dfs(
    trie: &Trie,
    words: usize,
    hashset: &mut HashSet<i64>,
    hash: i64,
    bytes: &mut Vec<u8>,
    result: &mut Vec<String>,
    memo: &mut HashSet<(Vec<u8>, i64)>
) {
    if memo.contains(&(bytes.clone(), hash)) {
        return;
    }

    if words > 1 && hash == 0 && trie.last {
        result.push(String::from_utf8(bytes.clone()).unwrap());
    }

    for i in 0..26 {
        if let Some(next_trie) = trie.children[i].as_deref() {
            let hash = (hash * BASE % MOD + (i+1) as i64) % MOD;
            bytes.push(i as u8 + b'a');
            if hashset.contains(&hash) {
                dfs(next_trie, words+1, hashset, 0, bytes, result, memo);
            }
            dfs(next_trie, words, hashset, hash, bytes, result, memo);
            bytes.pop();
        }
    }

    memo.insert((bytes.clone(), hash));
}

fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
    let mut hashset = HashSet::new();

    let mut trie = Box::new(Trie::new());
    for word in words {
        let mut trie = trie.as_mut();
        let mut hash = 0;
        for c in word.chars() {
            hash = (hash * BASE % MOD + (c as u8 - b'a' + 1) as i64) % MOD;
            trie = trie.children[(c as u8 - b'a') as usize].get_or_insert_with(|| Box::new(Trie::new()));
        }
        trie.last = true;
        hashset.insert(hash);
    }

    let mut res = vec![];
    dfs(trie.as_ref(), 0, &mut hashset, 0, &mut vec![], &mut res, &mut HashSet::new());

    res
}

pub fn main() {
    let words = ["cat","cats","catsdogcats","dog","dogcatsdog","hippopotamuses","rat","ratcatdogcat"].into_iter().map(String::from).collect();
    println!("{:?}", find_all_concatenated_words_in_a_dict(words));
}
