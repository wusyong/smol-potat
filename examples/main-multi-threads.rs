use smol::Task;

#[smol_potat::main(threads = 3)]
async fn main() {
    Task::spawn(async {
        println!("Hello, world!");
    })
    .await;
}
