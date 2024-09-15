mod connection;

use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // read exemple
    // let path = Path::new("foo.txt");
    // let mut file: File = File::open(&path).await?;

    // read up to 10 bytes
    // n represents the number of bytes read
    // let mut buffer: [u8; 10] = [0; 10];
    // let n: usize = file.read(&mut buffer[..]).await?;
    // println!("The bytes: {:?}", &buffer[..n]);

    // let mut buffer = Vec::new();
    // file.read_to_end(&mut buffer).await?;
    // println!("The bytes: {:?}", &buffer);
    // Ok(())
    // ----------------
    // write exemple
    // let mut file = File::create("fooWrite.txt").await?;

    // Writes some prefix of the byte string, but not necessarily all of it.
    // let n = file.write(b"some bytes").await?;
    // println!("Wrote the first {} bytes of 'some bytes'.", n);

    // file.write_all(b"some bytes").await?;
    // Ok(())

    //-------------------
    // copy exemple
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    Ok(())
}
