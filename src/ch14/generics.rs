pub mod functions {
    struct A;
    struct S(A);
    struct SGen<T>(T);
    fn reg_fn(_s: S) {}

    struct Fib {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fib {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            let new_next = self.curr + self.next;
            self.curr = self.next;
            self.next = new_next;
            Some(self.curr)
        }
    }

    fn fib() -> Fib {
        Fib { curr: 0, next: 1 }
    }

    pub fn test_fib() {
        let mut sequence = 0..3;
        println!("Four consecutive `next` calls on 0..3");
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());

        for i in 0..=3 {
            println!("> {}", i);
        }

        let mut Fibn = fib();
        for i in 0..=3 {
            println!("> {:?}", Fibn.next());
        }
        // The `take(n)` method reduces an `Iterator` to its first `n` terms.
        println!("The first four terms of the Fibonacci sequence are: ");
        for i in fib().take(4) {
            println!("> {}", i);
        }

        // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
        println!("The next four terms of the Fibonacci sequence are: ");
        for i in fib().skip(4).take(4) {
            println!("> {}", i);
        }
    }
}
