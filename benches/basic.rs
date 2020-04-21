#![feature(test)]
extern crate test;

#[smol_attributes::bench]
async fn bench() {
    println!("hello world");
}
