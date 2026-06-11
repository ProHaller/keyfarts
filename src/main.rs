use keyfarts::keyboard_input;

mod sounds;

#[tokio::main]
async fn main() {
    keyboard_input().await;
}
