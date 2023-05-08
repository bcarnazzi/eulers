use eulers::fibo::Fibo;

fn main() {
    let f = Fibo::new(0, 1);
    let r = f
        .take_while(|&f| f <= 4_000_000)
        .filter(|&f| f % 2 == 0)
        .sum::<u64>();

    println!("{}", r)
}
