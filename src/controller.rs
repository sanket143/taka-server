use enigo::*;
use crate::models;

pub fn trigger(enigo: &mut Enigo, event: models::Event){
  match event.event_type {
    models::EventType::MOVE => {
      enigo.mouse_move_to(event.value.x, event.value.y);
    },
    models::EventType::CLICK1 => {
      if event.value.val == 1 {
        enigo.mouse_down(MouseButton::Left);
      } else {
        enigo.mouse_up(MouseButton::Left);
      }
    },
    _ => {

    }
  }
  
  println!("{:?}", event);
}