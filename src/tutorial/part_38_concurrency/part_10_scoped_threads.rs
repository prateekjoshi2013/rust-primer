use std::thread;
/*
> Create a scope for spawning scoped threads.

> The function passed to scope will be provided
a Scope object, through which scoped threads
can be spawned.

> Unlike non-scoped threads, scoped threads can
borrow non-'static data, as the scope guarantees
all threads will be joined at the end of the scope.

> All threads spawned within the scope that haven't
been manually joined will be automatically joined
before this function returns.

*/
pub fn main() {
    let mut vec = vec![1, 2, 3];
    let mut x = 0;
    thread::scope(|some_scope| {
        some_scope.spawn(|| {
            println!("I am first thread in the scope");
            println!("{:?}", vec);
        });
        some_scope.spawn(|| {
            println!("I am second thread in the scope");
            x += 45;
        });
    });

    println!("the threads are now complete");
    vec.push(5);
    println!("x:{:?} and vec:{:?}", x, vec);
}
