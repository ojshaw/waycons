use std::io;
use std::io::Read;
use std::thread::sleep;
use std::time::Duration;
use waycons::parse::parse;

fn main() {
    let mut bytes = Vec::new();
    io::stdin().read_to_end(&mut bytes).unwrap();
    let _input = String::from_utf8(bytes).unwrap();
    let mut conway = parse(&_input).expect("Conway failed to parse");

    let mut turn = 1;
    while !conway.all_dead() || turn < 1000 {
        conway = conway.update();
        println!("{}", conway.to_string(turn));

        turn += 1;
        sleep(Duration::from_millis(500));
    }
}
