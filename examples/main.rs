use smol::Task;

#[smol_attributes::main]
async fn main() {
    Task::spawn(async {
        println!("Hello, world!");
    })
    .await;
}
