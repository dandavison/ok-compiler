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

fn f21<'a, I>(ss: I)
where
    I: IntoIterator<Item = &'a String>,
{
    for s in ss {
        println!("{}", s);
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

fn consume_whitespace<I>(it: I) -> usize
where
    I: Iterator<Item = char>,
{
    it.take_while(|c| *c == ' ').count()
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
        f21(&ss);
        println!("{}", ss.len());
    }
    {
        let mut ss = "aá".grapheme_indices(true);
        f3(ss.by_ref());
        println!("{}", ss.count());
    }
    {
        let s = "  aaa";
        let mut it = s.chars();
        let n = (&mut it).take_while(|c| *c == ' ').count();
        println!(
            "consumed: {}, remaining: {}",
            n,
            it.collect::<String>().len()
        );
    }
    {
        let s = "  aaa";
        let mut it = s.chars();
        let n = consume_whitespace(&mut it);
        println!(
            "consumed: {}, remaining: {}",
            n,
            it.collect::<String>().len()
        );
    }
    {
        let mut it = "feather".chars();
        it.next();
        for c in it.by_ref().take(2) {
            println!("{}", c);
        }
        println!("--------------------");
        for c in it.take(2) {
            println!("{}", c);
        }
    }
    {
        println!("\n\n\n\n");
        let mut it = "gravity".chars();
        let itr = it.by_ref();
        itr.next();
        for c in itr.take(2) {
            println!("{}", c);
        }
        println!("--------------------");
        for c in itr.take(2) {
            println!("{}", c);
        }
    }
}
