use std::net::UdpSocket;
use std::str;
use enigo::*;
use crate::models;
use crate::controller;

pub fn serve(){
  let mut buf = [0; 2048];
  let addr = format!("0.0.0.0:{}", 41439);
  let socket = match UdpSocket::bind(addr) {
    Ok(s) => s,
    Err(e) => panic!("couldn't bind socket: {}", e),
  };

  let mut enigo = Enigo::new();

  loop {
    match socket.recv_from(&mut buf) {
      Ok((amt, _src)) => {
        let reply = str::from_utf8(&buf[..amt]).unwrap_or(""); 
        let resp = String::from(reply);
        let resp = resp.split(" ");
        let resp = resp.collect::<Vec<&str>>();

        let event = match resp[0] {
          "M" => models::Event {
            event_type: models::EventType::MOVE,
            value: models::Value {
              x: resp[1].parse().unwrap(),
              y: resp[2].parse().unwrap(),
              val: 1,
            }
          },
          "O" => models::Event {
            event_type: models::EventType::CLICK1,
            value: models::Value {
              x: 0,
              y: 0,
              val: resp[1].parse().unwrap(),
            },
          },
          "P" => models::Event {
            event_type: models::EventType::CLICK2,
            value: models::Value {
              x: 0,
              y: 0,
              val: 1,
            },
          },
          "K" => models::Event {
            event_type: models::EventType::KEYBOARD,
            value: models::Value {
              x: 0,
              y: 0,
              val: 1,
            },
          },
          "W" => models::Event {
            event_type: models::EventType::WHEEL,
            value: models::Value {
              x: 0,
              y: 0,
              val: 1,
            },
          },
          _ => models::Event {
            event_type: models::EventType::UNKNOWN,
            value: models::Value {
              x: 0,
              y: 0,
              val: 1,
            },
          },
        };

        controller::trigger(&mut enigo, event);
      }
      Err(e) => {
        println!("couldn't recieve a datagram: {}", e);
      }
    }
  }
}