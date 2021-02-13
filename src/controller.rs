use enigo::*;
use crate::models;

pub fn trigger(enigo: &mut Enigo, event: models::Event){
  match event.event_type {
    models::EventType::MOVE => {
      enigo.mouse_move_to(event.value.x, event.value.y);
    },
    _ => {

    }
  }
  
  println!("{:?}", event);
}