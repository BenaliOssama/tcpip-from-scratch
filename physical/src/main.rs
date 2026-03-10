use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = mpsc::channel(10);

    // push something in
    sender.send("hello").await.unwrap();

    // get it out
    let msg = receiver.recv().await.unwrap();
    println!("Got: {}", msg);
}
