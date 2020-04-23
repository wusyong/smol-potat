#![feature(test)]
extern crate test;

#[smol_potat::bench]
async fn bench() {
    println!("hello world");
}
