use ibig::UBig;

pub struct Fibo {
    current: UBig,
    next: UBig,
}

impl Fibo {
    pub fn new(u0: usize, u1: usize) -> Self {
        Fibo {
            current: UBig::from(u0),
            next: UBig::from(u1),
        }
    }
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibo {
    // We can refer to this type using Self::Item
    type Item = UBig;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.clone();

        (self.current, self.next) = (self.next.clone(), current.clone() + self.next.clone());

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
        // will never return `None`, and `Some` is always returned.
        Some(current)
    }
}
