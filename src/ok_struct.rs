use syntect::highlighting::StyleModifier;

#[derive(Debug)] // so it can be formatted with {:?}
struct MyStruct {
    k1: usize,
    k2: String,
}

fn f1() -> MyStruct {
    MyStruct {
        k1: 0,
        k2: "my_string".to_string(),
    }
}

fn make_vec<'a, T>(x: T) -> Vec<Vec<(T, &'a str)>> {
    let mut v = Vec::new();
    v.push(vec![(x, "a")]);
    // v.push(vec![(x, "b")]);
    v
}

pub fn main() {
    {
        let x = f1();
        let mut v = Vec::new();
        v.push(x);
        println!("{:?}", v);
        // v.push(x); // use of moved value: `x`
        // println!("{:?}", v);
    }
    {
        let x = f1();
        let mut v = Vec::new();
        v.push(vec![x]);
        println!("{:?}", v);
        // v.push(vec![x]); // use of moved value: `x`
        // println!("{:?}", v);
    }
    {
        let x = f1();
        let mut v = Vec::new();
        v.push(vec![(x, 1)]);
        println!("{:?}", v);
        // v.push(vec![(x, 2)]); // use of moved value: `x`
        // println!("{:?}", v);
    }
    {
        let x = StyleModifier::default();
        let v = make_vec(x);
        println!("{:?}", v);
    }
}
