// use std::net::TcpListener;
use tokio::{io, net::TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    // let listener = TcpListener::bind("127.0.0.1:6142").await?;

    // loop {
    //     let (mut socket, _) = listener.accept().await?;

    //     tokio::spawn(async move {});
    // }
    //-------------------------
    // let socket = TcpStream::connect("127.0.0.1:6142").await?;
    // // io::split supports any value that implements AsyncRead + AsyncWrite and returns independent handles
    // let (mut rd, mut wr) = io::split(socket);

    // // Write data in the background to the writer of the socket
    // tokio::spawn(async move {
    //     wr.write_all(b"hello\r\n").await?;
    //     wr.write_all(b"world\r\n").await?;

    //     Ok::<_, io::Error>(())
    // });

    // // read data from the reader of the socket
    // let mut buffer = vec![0; 128];

    // loop {
    //     let n = rd.read(&mut buffer).await?;

    //     if n == 0 {
    //         break;
    //     }

    //     println!("Got {:?}", &buffer[..n]);
    // }
    //---------------------
    // copy with io::copy
    let socket = TcpStream::connect("127.0.0.1:6142").await?;

    tokio::spawn(async move {
        // io::split supports any value that implements AsyncRead + AsyncWrite and returns independent handles
        let (mut rd, mut wr) = io::split(socket);

        if io::copy(&mut rd, &mut wr).await.is_err() {
            eprintln!("failed to copy");
        }
    });

    Ok(())
    //------------------------
    // manual copy
    // let listener = TcpListener::bind("127.0.0.1:6142").await?;

    // loop {
    //     let (mut socket, _) = listener.accept().await?;

    //     tokio::spawn(async move {
    //         let mut buf = vec![0; 1024];

    //         loop {
    //             match socket.read(&mut buf).await {
    //                 // return value of `Ok(0)` signifies that the remote has closed
    //                 Ok(0) => return,
    //                 Ok(n) => {
    //                     // Copy the data back to socket
    //                     if socket.write_all(&buf[..n]).await.is_err() {
    //                         // Unexpecte socket error. There isn't much we can do here
    //                         // stop processing
    //                         return;
    //                     }
    //                 }
    //                 Err(_) => {
    //                     // Unexpected socket error. There isn't much we can do here
    //                     // stop processing
    //                     return;
    //                 }
    //             }
    //         }
    //     });
    // }
}
