use smol::Task;

#[smol_potat::main]
async fn main() {
    Task::spawn(async {
        println!("Hello, world!");
    })
    .await;
}
