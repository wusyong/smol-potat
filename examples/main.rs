#[smol_potat::main]
async fn main() {
    smol::spawn(async {
        println!("Hello, world!");
    })
    .await;
}
