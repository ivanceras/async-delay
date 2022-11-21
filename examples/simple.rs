use async_delay::Throttle;
use std::future::ready;

#[tokio::main]
async fn main() {
    let throttle = Throttle::from_interval(10);
    println!("calling throttled..");
    for i in 0..100 {
        let ret = throttle
            .call(|| {
                println!("hello world!");
                ready(42)
            })
            .await;
        println!("ret: {}", ret);
    }
}
