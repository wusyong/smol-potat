#![no_implicit_prelude]

use ::std::panic;

#[::smol_potat::main]
async fn main() {
    //
}

#[::smol_potat::test]
async fn test() -> ::std::io::Result<()> {
    ::std::assert_eq!(2 * 2, 4);
    ::std::result::Result::Ok(())
}
