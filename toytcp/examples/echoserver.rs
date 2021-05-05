use anyhow::Result;
use std::{env, net::Ipv4Addr, str};
use toytcp::tcp::TCP;

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  let addr: Ipv4Addr = args[1].parse()?;
  let port: u16 = args[2].parse()?;
  echo_server(addr, port)?;
  Ok(())
}

fn echo_server(local_addr: Ipv4Addr, local_port: u16) -> Result<()> {
  let tcp = TCP::new();
  let listening_socket = tcp.listen(local_addr, local_port)?;
  dbg!("listening..");
  loop {
    let connected_socket = tcp.accept(listening_socket)?;
    dbg!("accepted!", connected_socket.1, connected_socket.3);
  }
}
