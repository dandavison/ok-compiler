use unicode_segmentation::UnicodeSegmentation;

fn f1(ss: &Vec<String>) {
    for s in ss.iter() {
        println!("{}", s);
    }
}

fn f2<I>(ss: I)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    for s in ss {
        println!("{}", s.as_ref());
    }
}

fn f3<'a, I>(ss: I)
where
    I: IntoIterator<Item = (usize, &'a str)>,
{
    for (_, s) in ss {
        println!("{}", s);
    }
}

pub fn main() {
    {
        let ss = vec!["x".to_string()];
        f1(&ss);
        println!("{}", ss.len());
    }
    {
        let ss = vec!["x".to_string()];
        f2(&ss);
        println!("{}", ss.len());
    }
    {
        let mut ss = "aá".grapheme_indices(true);
        f3(ss.by_ref());
        println!("{}", ss.count());
    }
}
