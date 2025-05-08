enum Val {
    Int(i32),
    Float(f32),
    Str(String),
}

impl Into<Val> for i32 {
    fn into(self) -> Val {
        Val::Int(self)
    }
}

impl Into<Val> for f32 {
    fn into(self) -> Val {
        Val::Float(self)
    }
}

impl Into<Val> for String {
    fn into(self) -> Val {
        Val::Str(self)
    }
}

fn pattern_match(x: Val) {
    match x {
        Val::Int(i) => println!("Int: {}", i),
        Val::Float(f) => println!("Float: {:?}", f),
        Val::Str(s) => println!("Str: {}", s),
    }
}

fn main() {
    let x = 1;
    pattern_match(x.into());

    let y = 3.0;
    pattern_match(y.into());

    let z = "hello".to_string();
    pattern_match(z.into());
}