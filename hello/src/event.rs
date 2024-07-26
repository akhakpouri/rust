#[derive(Debug)]
pub struct MouseClick {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
pub struct KeyPress(pub String, pub char);

#[derive(Debug)]
pub enum WebEvent {
    Load(bool),
    Click(MouseClick),
    Keys(KeyPress),
}
