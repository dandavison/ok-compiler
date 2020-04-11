use unicode_segmentation::UnicodeSegmentation;

pub fn main() {
    {
        let mut line = "aábcdefghijklmn".graphemes(true);
        let prefix = line.next().unwrap(); // we can see that this won't panic
        let output_line = line.collect::<String>();
        println!("{}", prefix);
        println!("{}", output_line);
    }
}
