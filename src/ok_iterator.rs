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
}
