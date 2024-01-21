pub fn pad(mut s: String, bookend: bool) -> String {
    // should be 54 lines
    let more_spaces: i32 = 94 - s.len() as i32;
    let each_side: i32 = more_spaces / 2;

    let mut left_side: String = String::new();
    let mut right_side: String = String::new();

    if !bookend {
        // add regular padding of spaces
        for _ in 0..each_side {
            left_side.push_str(" ");
            right_side.push_str(" ");
        }
        
    } else {
        // right: pad spaces but leave room for bookend
        for _ in 0..each_side - 1 {
            right_side.push_str(" ");
        }
        // add right bookend
        right_side.push_str("|");

        // left: add bookend first, then add spaces
        left_side.push_str("|");

        for _ in 0..each_side - 1 {
            left_side.push_str(" ");
        }
    }

    // add left extra if needed
    if left_side.len() + s.len() + right_side.len() < 94  {
        left_side.push_str(" ");
    }
        
    s.push_str(&right_side); // --- + ---
    left_side.push_str(&s); // --- + (--- + --)

    return left_side
}