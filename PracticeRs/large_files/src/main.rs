use std::io::{Read, Write};

const BUFFER_SIZE: usize = 10;
fn main() -> std::io::Result<()> {
    let input_file = std::fs::File::open("input.txt")?;
    let out_file = std::fs::File::create("output.txt")?;

    let mut in_reader = std::io::BufReader::new(input_file);
    let mut in_writer = std::io::BufWriter::new(out_file);
    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        let byte_read = in_reader.read(&mut buffer)?;
        if byte_read == 0 {
            break;
        }

        for byte in buffer.iter_mut().take(byte_read) {
            byte.make_ascii_uppercase();
        }

        in_writer.write_all(&buffer[0..byte_read])?;
    }

    Ok(())
}
