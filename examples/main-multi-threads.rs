use smol::Executor;

#[smol_potat::main(threads = 3)]
async fn main() {
    let ex = Executor::new();

    ex.run(async {
        println!("Hello, world!");
    })
    .await;
}
