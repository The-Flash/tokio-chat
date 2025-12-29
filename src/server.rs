use tokio::{io, net::{TcpListener, TcpStream}};

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process(socket).await {
                eprintln!("{:?}", e);
            };
        });
    }
}

async fn process(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
     let mut cursor = 0;
     let mut buffer = vec![0; 1024];
//     loop {
//         socket.readable().await?;
//         match socket.try_read(&mut msg) {
//             Ok(n) => {
//                 buffer.truncate(n);
//                 break;
//             }
//             Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
//                continue
//             }
//             Err(e) => {
//                 return Err(e.into());
//             }
//         }
//     }
//     println!("Message: {:?}", String::from_utf8_lossy(&buffer));
//     loop {
//        if buffer.len() == cursor {
//            buffer.resize(cursor * 2, 0);
//        }
//        let n = stream.try_read(&mut buffer[cursor..])?;
//        // if we are at the end of the stream
//        if 0 == n {
//            if cursor == 0 {
//                break;
//            } else {
//                return Err("Connection reset by peer".into());
//            }
//        } else {
//            cursor += n;
//        }
//     }
     loop {
        stream.readable().await?;
        match stream.try_read(&mut buffer[cursor..]) {
            Ok(0) => break,
            Ok(n) => {
                if buffer.len() == cursor {
                    buffer.resize(cursor * 2, 0);
                }
                cursor += n;
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            },
            Err(_e) => {
                return Err("Something went wrong".into());
            }
        }
     }
     println!("Message: {:?}", String::from_utf8_lossy(&buffer[..cursor]));
     Ok(())
}
