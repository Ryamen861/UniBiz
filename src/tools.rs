pub fn pad(mut s: String) -> &str {
    // should be 54 lines
    let more_spaces: i32 = 54 - s.len() as i32 - 2;

    for i in 0..more_spaces {
        s.push_str(" ");
    }

    return " "
}