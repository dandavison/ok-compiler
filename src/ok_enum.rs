enum E1 {
    Ctor1,
}

impl E1 {
    pub fn format(&self) -> String {
        match self {
            E1::Ctor1 => format!("{}", "E1::Ctor1"),
        }
    }
}

enum E2 {
    Ctor1(E1),
}

impl E2 {
    pub fn format(&self) -> String {
        match self {
            E2::Ctor1(v) => format!("E2::Ctor1({})", v.format()),
        }
    }
}

enum E3<'a> {
    Ctor1(&'a str),
}

impl<'a> E3<'a> {
    pub fn format(&self) -> String {
        match self {
            E3::Ctor1(s) => format!("E3::Ctor1({})", s),
        }
    }
}

enum E4<'a> {
    Ctor1(E3<'a>),
}

impl<'a> E4<'a> {
    pub fn format(&self) -> String {
        match self {
            E4::Ctor1(v) => format!("E4::Ctor1({})", v.format()),
        }
    }
}

pub fn main() {
    let v1: E1 = E1::Ctor1;
    println!("v1 = {}", v1.format());

    let v2: E2 = E2::Ctor1(v1);
    println!("v2 = {}", v2.format());

    let v3: E3 = E3::Ctor1("feather");
    println!("v3 = {}", v3.format());

    let v4: E4 = E4::Ctor1(v3);
    println!("v4 = {}", v4.format());
}
