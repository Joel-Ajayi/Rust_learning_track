use std::time::Duration;
use tokio::time::sleep;

#[tokio::main()]

async fn main() {
    // it is a zero cost abstraction until used
    // They can be cancelled
    // let f = my_function();
    // f.await

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

// trait  Future {
//     type Output;
//     fn poll(&mut self, wake:fn())-> Poll<Self::Output>;
// }

// enum Poll<T> {
//     Ready(T),
//     Pending,
// }

async fn my_function(i: i32) {
    println!("Hey {i} async func");
    let s1 = read_db().await;
    println!("First {i} res: {s1}");
    let s2 = read_db().await;
    println!("Second {i} res: {s2}");
}

async fn read_db() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_owned()
}
