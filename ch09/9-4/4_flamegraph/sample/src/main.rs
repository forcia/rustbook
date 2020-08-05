use rand::Rng;

fn fn0(x: u64) -> u64 {
    if x % 2 == 0 {
        fn2(x)
    } else {
        fn3(x)
    }
}

fn fn1(x: u64) -> u64 {
    if x % 2 == 0 {
        fn4(x)
    } else {
        fn5(x)
    }
}

fn fn2(x: u64) -> u64 {
    repeat(x, |x| x / 3 + 20)
}

fn fn3(x: u64) -> u64 {
    repeat(x, |x| x / 5 / 7 + 10)
}

fn fn4(x: u64) -> u64 {
    repeat(x, |x| x * 11 + 20)
}

fn fn5(x: u64) -> u64 {
    repeat(x, |x| x * 13 * 5 + 10)
}

fn repeat<F>(x: u64, f: F) -> u64
where
    F: Fn(u64) -> u64,
{
    let mut y = x;
    for _ in 0..100 {
        y = f(y);
    }
    y
}

fn main() {
    let mut x = 1;
    let mut rng = rand::thread_rng();

    for _ in 0..100000000 {
        let i: u64 = rng.gen();
        match i % 6 {
            0 => x = fn0(x),
            1 => x = fn1(x),
            2 => x = fn2(x),
            3 => x = fn3(x),
            4 => x = fn4(x),
            5 => x = fn5(x),
            _ => (),
        }
    }

    println!("{}", x);
}
