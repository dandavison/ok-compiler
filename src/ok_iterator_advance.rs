fn prepare(line: &str) -> String {
    if !line.is_empty() {
        let mut line = line.chars();
        let prefix = line.next().unwrap();
        let output_line = line.collect::<String>();
        format!("{}{}", prefix, output_line)
    } else {
        "<empty line>".to_string()
    }
}

pub fn main() {
    let result = prepare("aábcdefghijklmn");
    println!("{}", result);
    // This example does work: it prints aábcdefghijklmn.

    // However, when calling a function like prepare in the context of a larger
    // project, the string returned from prepare is sometimes truncated on the
    // right-hand side, in apparently unpredictable locations, but apparently
    // always word boundaries of some sort. What mistake might I be making?
}
