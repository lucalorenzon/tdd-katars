use crate::helpers::{delay, green, yellow};


#[allow(dead_code)]
mod leap_year;
#[allow(dead_code)]
mod fizzbuzz;
#[allow(dead_code)]
mod helpers;

async fn hello(who: &str) -> String {
    format!("Hello {who}")
}

#[tokio::main]
async fn main() {
    let result = delay(200).await;
    let message = hello("World").await;
    let duration = format!("{result}ms");

    println!("{} after {}", green(&message), yellow(&duration) );
}
