/// why didn't this work?

lazy_static! {
    static ref ANSI_16_COLORS: HashMap<String, u8> = {
        vec![
            ("black".to_string(), 0),
            ("red".to_string(), 1),
            ("green".to_string(), 2),
            ("yellow".to_string(), 3),
            ("blue".to_string(), 4),
            ("magenta".to_string(), 5),
            ("purple".to_string(), 5),
            ("cyan".to_string(), 6),
            ("white".to_string(), 7),
            ("bright-black".to_string(), 8),
            ("brightblack".to_string(), 8),
            ("bright-red".to_string(), 9),
            ("brightred".to_string(), 9),
            ("bright-green".to_string(), 10),
            ("brightgreen".to_string(), 10),
            ("bright-yellow".to_string(), 11),
            ("brightyellow".to_string(), 11),
            ("bright-blue".to_string(), 12),
            ("brightblue".to_string(), 12),
            ("bright-magenta".to_string(), 13),
            ("brightmagenta".to_string(), 13),
            ("bright-purple".to_string(), 13),
            ("brightpurple".to_string(), 13),
            ("bright-cyan".to_string(), 14),
            ("brightcyan".to_string(), 14),
            ("bright-white".to_string(), 15),
            ("brightwhite".to_string(), 15),
        ]
        .into_iter()
        .collect()
    };
}

// See
// https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
pub fn ansi_16_color_name_to_number(name: &str) -> Option<u8> {
    ANSI_16_COLORS.get(name).map(|n| *n)
}

fn ansi_16_color_number_to_name(n: u8) -> Option<String> {
    ANSI_16_COLORS
        .iter()
        .filter(|(_, _n)| **_n == n)
        .map(|(k, _)| k.to_string())
        .nth(1)
}
