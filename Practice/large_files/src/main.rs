use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

const BUFFER_SIZE: usize = 16;

fn main() -> std::io::Result<()> {
    let input_file = File::open("input.txt")?;
    let output_file = File::create("output.txt")?;

    let mut buf_reader = BufReader::new(input_file);
    let mut buf_writer = BufWriter::new(output_file);

    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        let byte_read = buf_reader.read(&mut buffer)?;
        if byte_read == 0 {
            break;
        }

        for byte in buffer.iter_mut().take(byte_read) {
            byte.make_ascii_uppercase();
        }
        buf_writer.write_all(&buffer[0..byte_read])?;
    }

    Ok(())
}
