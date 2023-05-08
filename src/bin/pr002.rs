use eulers::fibo::Fibo;
use ibig::UBig;

fn main() {
    let f = Fibo::new(0, 1);
    let r = f
        .take_while(|f| f <= &UBig::from(4_000_000 as u32))
        .filter(|f| f % 2 == 0)
        .fold(UBig::from(0 as u32), |b1, b2| b1 + b2);

    println!("{}", r);
}
