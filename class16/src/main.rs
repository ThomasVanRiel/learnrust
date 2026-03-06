fn make_multiplier(n: &str) -> impl Fn(i32) -> String {
    // move |x| n * x
    move |x| format!("{}{}", n, x)
}

fn apply_twice<F: FnMut(i32) -> i32>(mut f: F, x: i32) -> i32 {
    let tmp = f(x);
    f(tmp)
}

fn main() {
    let name = String::from("Thomas");
    let greet = || println!("{}", name);
    greet();
    println!("{}", name);

    let res = apply_twice(|x| x + 3, 10);
    println!("{res}");
    let mut calls = 0;
    let result = apply_twice(
        |x| {
            calls += 1;
            x + 3
        },
        10,
    );
    println!("result: {}, calls: {}", result, calls);
    let name2 = String::from("Thomas2");
    let handle = std::thread::spawn(move || {
        println!("{}", name2);
    });
    handle.join().unwrap();

    let double = make_multiplier(name.as_str());
    let triple = make_multiplier(name.as_str());
    println!("{}", double(5));
    println!("{}", triple(5));
}
