#[smol_potat::main(threads = 3)]
async fn main() {
    smol::spawn(async {
        println!("Hello, world!");
    })
    .await;
}
