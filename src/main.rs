use std::io::{self, Read, Write};


include!(concat!(env!("OUT_DIR"), "/table.rs"));


const CHUNK_SIZE: usize = 10 * 1024;


fn main() {
    assert_eq!(LOOKUP_TABLE.len(), 256);

    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();

    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    let mut readbuf = [0; CHUNK_SIZE];
    while let Ok(bytes_read) = stdin_handle.read(&mut readbuf) {
        if bytes_read == 0 {
            break;
        }

        let emoji = &readbuf[0..bytes_read]
            .iter()
            .flat_map(|b| LOOKUP_TABLE[*b as usize].iter())
            .cloned()
            .collect::<Vec<u8>>();

        stdout_handle.write(&*emoji).unwrap();
    }
}
