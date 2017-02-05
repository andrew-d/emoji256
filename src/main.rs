use std::io::{self, Read, Write};


include!(concat!(env!("OUT_DIR"), "/table.rs"));


fn main() {
    assert_eq!(LOOKUP_TABLE.len(), 256);

    let stdin = io::stdin();
    let stdin_handle = stdin.lock();

    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    let mut buf = [0; 4];
    for b in stdin_handle.bytes() {
        let emoji = LOOKUP_TABLE[b.unwrap() as usize];
        let bytes = {
            let s = emoji.encode_utf8(&mut buf);
            s.len()
        };

        stdout_handle.write(&buf[0..bytes]).unwrap();
    }
}
