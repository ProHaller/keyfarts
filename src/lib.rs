use rdev::{Event, EventType, Key, listen};
use rodio::Decoder;
use std::fs::File;

mod sounds;

pub fn play(file_path: &'static str) {
    tokio::spawn(async move {
        // Get an OS-Sink handle to the default physical sound device.
        // Note that the playback stops when the handle is dropped.//!
        let mut handle =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        handle.log_on_drop(false);
        let _player = rodio::Player::connect_new(handle.mixer());
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = File::open(file_path).unwrap();
        // Decode that sound file into a source
        let source = Decoder::try_from(file).unwrap();
        // Play the sound directly on the device
        handle.mixer().add(source);
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
}

pub async fn keyboard_input() {
    // This will block.
    if let Err(error) = listen(fart_callback) {
        println!("Error: {:?}", error)
    }

    fn fart_callback(event: Event) {
        if let EventType::KeyPress(key) = event.event_type {
            match key {
                Key::Alt => play(sounds::FART_9),
                Key::AltGr => play(sounds::FART_9),
                Key::Backspace => play(sounds::FART_9),
                Key::CapsLock => play(sounds::FART_9),
                Key::ControlLeft => play(sounds::FART_9),
                Key::ControlRight => play(sounds::FART_9),
                Key::MetaLeft => play(sounds::FART_9),
                Key::MetaRight => play(sounds::FART_9),
                Key::Delete => play(sounds::FART_5),
                Key::End => play(sounds::FART_5),
                Key::Escape => play(sounds::FART_5),
                Key::Home => play(sounds::FART_5),
                Key::DownArrow => play(sounds::FART_4),
                Key::LeftArrow => play(sounds::FART_4),
                Key::RightArrow => play(sounds::FART_4),
                Key::UpArrow => play(sounds::FART_4),
                Key::PageDown => play(sounds::FART_5),
                Key::PageUp => play(sounds::FART_5),
                Key::Return => play(sounds::FART_10),
                Key::ShiftLeft => play(sounds::FART_9),
                Key::ShiftRight => play(sounds::FART_9),
                Key::Space => play(sounds::FART_4),
                Key::Tab => play(sounds::FART_4),
                Key::PrintScreen => play(sounds::FART_5),
                Key::ScrollLock => play(sounds::FART_5),
                Key::Pause => play(sounds::FART_5),
                Key::NumLock => play(sounds::FART_5),
                Key::BackQuote => play(sounds::FART_5),
                Key::F1 => play(sounds::FART_11),
                Key::F10 => play(sounds::FART_11),
                Key::F11 => play(sounds::FART_11),
                Key::F12 => play(sounds::FART_11),
                Key::F2 => play(sounds::FART_11),
                Key::F3 => play(sounds::FART_11),
                Key::F4 => play(sounds::FART_11),
                Key::F5 => play(sounds::FART_11),
                Key::F6 => play(sounds::FART_11),
                Key::F7 => play(sounds::FART_11),
                Key::F8 => play(sounds::FART_11),
                Key::F9 => play(sounds::FART_11),
                Key::Num1 => play(sounds::FART_2),
                Key::Num2 => play(sounds::FART_2),
                Key::Num3 => play(sounds::FART_2),
                Key::Num4 => play(sounds::FART_2),
                Key::Num5 => play(sounds::FART_2),
                Key::Num6 => play(sounds::FART_2),
                Key::Num7 => play(sounds::FART_2),
                Key::Num8 => play(sounds::FART_2),
                Key::Num9 => play(sounds::FART_2),
                Key::Num0 => play(sounds::FART_2),
                Key::Minus => play(sounds::FART_3),
                Key::Equal => play(sounds::FART_3),
                Key::KpDelete => play(sounds::FART_3),
                Key::Function => play(sounds::FART_3),
                Key::Unknown(_) => play(sounds::FART_12),
                Key::KeyQ => play(sounds::FART_1),
                Key::KeyW => play(sounds::FART_1),
                Key::KeyE => play(sounds::FART_1),
                Key::KeyR => play(sounds::FART_1),
                Key::KeyT => play(sounds::FART_1),
                Key::KeyY => play(sounds::FART_1),
                Key::KeyU => play(sounds::FART_1),
                Key::KeyI => play(sounds::FART_1),
                Key::KeyO => play(sounds::FART_1),
                Key::KeyP => play(sounds::FART_1),
                Key::LeftBracket => play(sounds::FART_1),
                Key::RightBracket => play(sounds::FART_1),
                Key::KeyA => play(sounds::FART_1),
                Key::KeyS => play(sounds::FART_1),
                Key::KeyD => play(sounds::FART_1),
                Key::KeyF => play(sounds::FART_1),
                Key::KeyG => play(sounds::FART_1),
                Key::KeyH => play(sounds::FART_1),
                Key::KeyJ => play(sounds::FART_1),
                Key::KeyK => play(sounds::FART_1),
                Key::KeyL => play(sounds::FART_1),
                Key::SemiColon => play(sounds::FART_1),
                Key::Quote => play(sounds::FART_1),
                Key::BackSlash => play(sounds::FART_1),
                Key::IntlBackslash => play(sounds::FART_1),
                Key::KeyZ => play(sounds::FART_1),
                Key::KeyX => play(sounds::FART_1),
                Key::KeyC => play(sounds::FART_1),
                Key::KeyV => play(sounds::FART_1),
                Key::KeyB => play(sounds::FART_1),
                Key::KeyN => play(sounds::FART_1),
                Key::KeyM => play(sounds::FART_1),
                Key::Comma => play(sounds::FART_1),
                Key::Dot => play(sounds::FART_1),
                Key::Slash => play(sounds::FART_1),
                Key::Insert => play(sounds::FART_1),
                Key::KpReturn => play(sounds::FART_1),
                Key::KpMinus => play(sounds::FART_1),
                Key::KpPlus => play(sounds::FART_1),
                Key::KpMultiply => play(sounds::FART_1),
                Key::KpDivide => play(sounds::FART_1),
                Key::Kp0 => play(sounds::FART_1),
                Key::Kp1 => play(sounds::FART_1),
                Key::Kp2 => play(sounds::FART_1),
                Key::Kp3 => play(sounds::FART_1),
                Key::Kp4 => play(sounds::FART_1),
                Key::Kp5 => play(sounds::FART_1),
                Key::Kp6 => play(sounds::FART_1),
                Key::Kp7 => play(sounds::FART_1),
                Key::Kp8 => play(sounds::FART_1),
                Key::Kp9 => play(sounds::FART_1),
            }
        };
    }
}
