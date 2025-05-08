fn add3(a: f32, b: f32, c: f32) -> f32 {
    a + b + c
}

fn main() {
    println!("add3(1.0, 2.0, 3.0) = {:?}", add3(1.0, 2.0, 3.0));

    let add2 = |a: f32, b: f32| add3(a, b, 0.0);
    println!("add2(1.0, 2.0) = {:?}", add2(1.0, 2.0));

    let ident = |a: f32| add2(a, 0.0);
    println!("ident(1.0) = {:?}", ident(1.0));
}