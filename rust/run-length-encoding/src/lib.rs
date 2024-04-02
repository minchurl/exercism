pub fn encode(source: &str) -> String {
    let mut ret = String::new();
    let mut chars = source.chars().peekable();
    let mut curr_count = 0;

    while let Some(curr) = chars.next() {
        curr_count += 1;

        if chars.peek() != Some(&curr) {
            if curr_count > 1 {
                ret.push_str(&curr_count.to_string());
            }

            ret.push(curr);
            curr_count = 0;
        }
    }

    ret
}

pub fn decode(source: &str) -> String {
    let mut ret = String::new();
    let mut numeric_chars = String::new();

    for curr in source.chars() {
        if curr.is_numeric() {
            numeric_chars.push(curr);
        }
        else {
            ret.push_str(&curr.to_string().repeat(
                numeric_chars.parse::<usize>().unwrap_or(1)
            ));
            numeric_chars.clear();
        }
    }

    ret
}
