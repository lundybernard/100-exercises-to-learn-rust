//! TODO: get the code to compile by **re-ordering** the statements
//!  in the `example` function. You're not allowed to change the
//!  `spawner` function nor what each line does in `example`.
//!   You can wrap existing statements in blocks `{}` if needed.
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    // Official solution. I believe that wrapping the first two lines in brackets
    // allows the non_send variable to go out of scope, before it has a chance
    // to interfere with the yield invocation.
    {
        let non_send = Rc::new(1);
        println!("{}", non_send);
    }
    yield_now().await;
}
