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

use std::time::Duration;

use tokio::time::sleep;

// For top most futures we need some code which will manually poll the future
// Programming language like C++ and Java have a runtime which will do this for us
// Rust does not have a runtime so we need to write this code ourselves
// So inorder to run async code we need to use a runtime like tokio or async-std


pub async fn run() {
    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
async fn my_function(i: i32) {
    println!("[{i}] Im an async function");
    let s1 = read_from_database().await;
    println!("[{i}] s1 : {}", s1);
    let s2 = read_from_database().await;
    println!("[{i}] s2 : {}", s2);
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await;
    return "DB data".to_string();
}
