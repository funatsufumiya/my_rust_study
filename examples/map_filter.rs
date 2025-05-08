fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_arr =
        arr.iter()
            .map(|x| x * 2)
            .filter(|x| *x > 5)
            .collect::<Vec<_>>();
    println!("{:?}", new_arr);
}