fn main() {
    let mut a = 0;
    let mut b = 1;

    #[allow(clippy::almost_swapped)]
    {
        a = b;
        b = a;
    }

    dbg!(a);
    dbg!(b);
}
