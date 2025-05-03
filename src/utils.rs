pub fn pad_left(s: &str, pad: char, len: usize) -> String {
    let padding = pad.to_string().repeat(len);
    format!("{}{}", padding, s)
}


