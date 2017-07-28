pub fn list() -> Vec<&'static str> {
    let contents = include_str!("words.txt");

    let words: Vec<&'static str> = contents
        .split('\n')
        .collect();

    words
}
