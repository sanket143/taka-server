pub mod controller;
pub mod models;
mod udp_server;

fn main() {
  udp_server::serve();
}
