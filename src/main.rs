use iocraft::ElementExt;
use std::io;
use std::io::Read;
use waycons::ui::App;

fn main() {
    let mut bytes = Vec::new();
    io::stdin().read_to_end(&mut bytes).unwrap();
    let input = String::from_utf8(bytes).unwrap();

    smol::block_on(iocraft::element!(App(input :input)).fullscreen()).unwrap();
}
