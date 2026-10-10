fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
    let mut hashmap = std::collections::HashMap::<_, usize>::new();
    let n = messages.len();
    for i in 0..n {
        *hashmap.entry(senders[i].as_str()).or_default() += messages[i].split_whitespace().count();
    }

    hashmap
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(b.0)))
        .unwrap()
        .0
        .to_string()
}

pub fn main() {
    let messages = ["Hello userTwooo","Hi userThree","Wonderful day Alice","Nice day userThree"].into_iter().map(String::from).collect();
    let senders = ["Alice","userTwo","userThree","Alice"].into_iter().map(String::from).collect();
    println!("{}", largest_word_count(messages, senders));
}
