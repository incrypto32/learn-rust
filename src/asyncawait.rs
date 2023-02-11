// Simplfied version of future trait

// A future is simple state machine which can be polled to check if it is ready or not
// If it is ready it will return the value
// If it is not ready it will return Pending
// The future will be polled again when it is ready

////////////////////////////////////////////////////////
// trait Future {
//     type Output;
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
// }

// enum Poll<T> {
//     Ready(T),
//     Pending,
// }

// fn my_function() -> impl Future<Output = ()> {
//     async {
//         println!("Im an async function");
//     }
// }
////////////////////////////////////////////////////////

// For top most futures we need some code which will manually poll the future
// Programming language like C++ and Java have a runtime which will do this for us
// Rust does not have a runtime so we need to write this code ourselves
// So inorder to run async code we need to use a runtime like tokio or async-std
pub async fn run() {
    my_function().await;
}
async fn my_function() {
    println!("Im an async function");
    let s1 = read_from_database().await;
    println!("s1 : {}", s1);
    let s2 = read_from_database().await;
    println!("s2 : {}", s2);
}

async fn read_from_database() -> String {
    return "DB data".to_string();
}
