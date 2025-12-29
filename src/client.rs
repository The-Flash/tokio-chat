use tokio::net::TcpStream;

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
       let stream = TcpStream::connect("127.0.0.1:3000").await?;
        stream.try_write(b"Hello server\n")?;
        stream.try_write(b"Hello server\n")?;
        stream.try_write(b"Hello server\n")?;
        stream.try_write(b"Hello server\n")?;
        stream.try_write(b"Hello server\n")?;
    }
}
