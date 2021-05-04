use anyhow::{anyhow, Result};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, str, thread};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let addr = &args[1];
        return echo_server(addr)
    } else {
        return Err(anyhow!("missing addr"))
    }
}

fn echo_server(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address)?;
    loop {
        let (mut stream, _) = listener.accept()?;
        thread::spawn(move || {
            let mut buffer = [0u8; 1024];
            loop {
                let nbytes = stream.read(&mut buffer).unwrap();
                if nbytes == 0 {
                    return;
                }
                println!("{}", str::from_utf8(&buffer[..nbytes]).unwrap());
                stream.write_all(&buffer[..nbytes]).unwrap();
            }
        });
    }
}
