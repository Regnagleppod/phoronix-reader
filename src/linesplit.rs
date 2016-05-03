pub fn split_by_chars(input: &str, length: usize) -> Vec<String> {
    let words: Vec<&str> = input.split_whitespace().collect();
    let output_capacity = input.len() + input.len() % length + 1;
    let mut lines: Vec<String> = Vec::with_capacity(output_capacity);
    let mut current_line = String::with_capacity(length);
    let (mut chars, mut initialized) = (0 as usize, false);
    for word in words {
        if chars + word.len() >= length {
            // if character length met or exceeded
            lines.push(current_line.clone());
            current_line.clear();
            current_line.reserve(length);
            current_line = String::from(word);
            chars = word.len();
        } else if !initialized {
            current_line = String::from(word);
            chars = word.len();
            initialized = true;
        } else {
            current_line = current_line + " " + &word;
            chars += word.len() + 1;
        }
    }
    if !current_line.is_empty() { lines.push(current_line); }
    lines
}
