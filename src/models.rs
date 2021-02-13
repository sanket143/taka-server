use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum EventType {
  MOVE,
  WHEEL,
  CLICK1,
  CLICK2,
  KEYBOARD,
  UNKNOWN,
}

#[derive(Serialize, Debug)]
pub struct Value {
  pub x: i32,
  pub y: i32,
  pub val: i32,
}

#[derive(Serialize, Debug)]
pub struct Event {
  pub event_type: EventType,
  pub value: Value
}
