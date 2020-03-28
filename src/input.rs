use cgmath::{Vector2};
use winit::event::{Event, MouseButton, ElementState};


#[derive(Default)]
pub struct EventBucket<'a>(pub Vec<Event<'a, ()>>);

pub struct ButtonState {
    pub pressed : bool,
    pub down    : bool,
}

impl ButtonState {
    pub fn new() -> Self {
        Self { pressed: false, down: false }
    }
}

pub struct MouseState {
    pub left     : ButtonState,
    pub right    : ButtonState,
    pub middle   : ButtonState,
    pub pos      : Vector2<f32>,
    pub last_pos : Vector2<f32>,
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            left   : ButtonState::new(),
            right  : ButtonState::new(),
            middle : ButtonState::new(),
            pos    : Vector2::new(0.0, 0.0),
            last_pos : Vector2::new(0.0, 0.0),
        }
    }

    pub fn update_from_mouse_button(&mut self, mouse_button: &MouseButton, state: &ElementState) {
        match state {
            ElementState::Pressed => {
                match mouse_button {
                    MouseButton::Left => {
                        self.left.down    = true;
                        self.left.pressed = true;
                    },
                    MouseButton::Right => {
                        self.right.down    = true;
                        self.right.pressed = true;
                    },
                    MouseButton::Middle => {
                        self.middle.down    = true;
                        self.middle.pressed = true;
                    },
                    _ => {}
                }
            },
            ElementState::Released => {
                match mouse_button {
                    MouseButton::Left => {
                        self.left.down    = false;
                        self.left.pressed = false;
                    },
                    MouseButton::Right => {
                        self.right.down    = false;
                        self.right.pressed = false;
                    },
                    MouseButton::Middle => {
                        self.middle.down    = false;
                        self.middle.pressed = false;
                    },
                    _ => {}
                }
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Key {
    E,
    S,
    D,
    F,
}

pub struct InputState {
    pub mouse: MouseState,
    pub keyboard: std::collections::HashMap<Key, ButtonState>,
}

impl InputState {
    pub fn new() -> Self {

        let mut keyboard =std::collections::HashMap::new();
        keyboard.insert(Key::E, ButtonState::new());
        keyboard.insert(Key::S, ButtonState::new());
        keyboard.insert(Key::D, ButtonState::new());
        keyboard.insert(Key::F, ButtonState::new());

        Self {
            mouse: MouseState::new(),
            keyboard,
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        return self.keyboard.get(&key).unwrap().down;
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        return self.keyboard.get(&key).unwrap().pressed;
    }

    pub fn update_from_event(&mut self, event: &winit::event::WindowEvent) {
        use winit::event::WindowEvent::*;
        use winit::event::VirtualKeyCode;
        use winit::event::ElementState;
        match event {
            KeyboardInput {input, ..} => {
                if let Some(kc)  = input.virtual_keycode {
                    if let Some(key) = &match kc {
                        VirtualKeyCode::E => Some(Key::E),
                        VirtualKeyCode::S => Some(Key::S),
                        VirtualKeyCode::D => Some(Key::D),
                        VirtualKeyCode::F => Some(Key::F),
                        _ => None,
                    } {
                        let state = self.keyboard.get_mut(key).unwrap();
                        match input.state {
                            ElementState::Pressed => {
                                state.down = true;
                                state.pressed = true;
                            },
                            ElementState::Released => {
                                state.down = false;
                                state.pressed = false;
                            },
                        }
                    }
                }
            },
            MouseInput { button, state, .. } => {
                self.mouse.update_from_mouse_button(button, state);
            },
            CursorMoved{ position, .. } => {
                self.mouse.last_pos = self.mouse.pos;
                self.mouse.pos = Vector2::new(position.x as f32, position.y as f32);
            }
            _ => ()
        }
    }
}