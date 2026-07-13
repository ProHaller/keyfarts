use anyhow::Result;
use rdev::{
    Event, EventType,
    Key::{self},
    listen,
};
use rodio::{Decoder, MixerDeviceSink, Source, buffer::SamplesBuffer};
use strum::{EnumCount, EnumIter, IntoEnumIterator};

pub const FART_1: &[u8] = include_bytes!("../assets/fart_1.mp3");
pub const FART_2: &[u8] = include_bytes!("../assets/fart_2.mp3");
pub const FART_3: &[u8] = include_bytes!("../assets/fart_3.mp3");
pub const FART_4: &[u8] = include_bytes!("../assets/fart_4.mp3");
pub const FART_5: &[u8] = include_bytes!("../assets/fart_5.mp3");
pub const FART_6: &[u8] = include_bytes!("../assets/fart_6.mp3");
pub const FART_7: &[u8] = include_bytes!("../assets/fart_7.mp3");
pub const FART_8: &[u8] = include_bytes!("../assets/fart_8.mp3");
pub const FART_9: &[u8] = include_bytes!("../assets/fart_9.mp3");
pub const FART_10: &[u8] = include_bytes!("../assets/fart_10.mp3");
pub const FART_11: &[u8] = include_bytes!("../assets/fart_11.mp3");
pub const FART_12: &[u8] = include_bytes!("../assets/fart_12.mp3");

pub struct AudioPlayer {
    samples: [SamplesBuffer; Sound::COUNT],
    sink: MixerDeviceSink,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let samples = load_samples()?;
        let mut handle = rodio::DeviceSinkBuilder::open_default_sink()?;
        handle.log_on_drop(false);
        Ok(Self {
            samples,
            sink: handle,
        })
    }

    pub fn play(&self, sound: Sound) {
        self.sink.mixer().add(self.samples[sound as usize].clone());
    }
}

fn load_samples() -> Result<[SamplesBuffer; Sound::COUNT]> {
    let samples: Vec<SamplesBuffer> = Sound::iter()
        .map(|sound| Ok(Decoder::try_from(std::io::Cursor::new(sound.bytes()))?.record()))
        .collect::<Result<_>>()?;

    // Convert Vec to array (safe since we know the length)
    Ok(samples
        .try_into()
        .expect("Convert into array of size COUNT"))
}

pub fn keyboard_input(audio_player: AudioPlayer) -> Result<()> {
    let callback = move |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            audio_player.play(key_sound(key));
        }
    };

    listen(callback).map_err(|e| anyhow::anyhow!("keyboard listener failed: {e:?}"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCount)]
pub enum Sound {
    Letter,         // fart_1
    Digit,          // fart_2
    Edition,        // fart_3
    Arrow,          // fart_4
    Navigation,     // fart_5
    NumPad,         // fart_6
    NumPadModifyer, // fart_7
    Punctuation,    // fart_8
    Modifier,       // fart_9
    Enter,          // fart_10
    FnKey,          // fart_11
    Unknown,        // fart_12
}

impl Sound {
    const fn bytes(self) -> &'static [u8] {
        match self {
            Sound::Letter => FART_1,
            Sound::Digit => FART_2,
            Sound::Edition => FART_3,
            Sound::Arrow => FART_4,
            Sound::Navigation => FART_5,
            Sound::NumPad => FART_6,
            Sound::NumPadModifyer => FART_7,
            Sound::Punctuation => FART_8,
            Sound::Modifier => FART_9,
            Sound::Enter => FART_10,
            Sound::FnKey => FART_11,
            Sound::Unknown => FART_12,
        }
    }
}

pub fn key_sound(key: Key) -> Sound {
    use Key::*;
    use Sound::*;

    match key {
        KeyA | KeyB | KeyC | KeyD | KeyE | KeyF | KeyG | KeyH | KeyI | KeyJ | KeyK | KeyL
        | KeyM | KeyN | KeyO | KeyP | KeyQ | KeyR | KeyS | KeyT | KeyU | KeyV | KeyW | KeyX
        | KeyY | KeyZ => Letter,

        Num1 | Num2 | Num3 | Num4 | Num5 | Num6 | Num7 | Num8 | Num9 | Num0 => Digit,

        Backspace | CapsLock | Delete | Return | Space | Tab => Edition,

        Alt
        | AltGr
        | ControlLeft
        | ControlRight
        | MetaLeft
        | MetaRight
        | ShiftLeft
        | Key::Unknown(_)
        | ShiftRight
        | Function
        | F1
        | F10
        | F11
        | F12
        | F2
        | F3
        | F4
        | F5
        | F6
        | F7
        | F8
        | F9
        | PrintScreen
        | ScrollLock
        | Pause
        | NumLock => Modifier,

        End | Escape | Home | PageDown | PageUp | Insert => Navigation,

        DownArrow | LeftArrow | RightArrow | UpArrow => Arrow,

        BackQuote | Minus | Equal | LeftBracket | RightBracket | SemiColon | Quote | BackSlash
        | IntlBackslash | Comma | Dot | Slash => Punctuation,

        KpDelete | KpReturn | KpMinus | KpPlus | KpMultiply | KpDivide => NumPadModifyer,

        Kp0 | Kp1 | Kp2 | Kp3 | Kp4 | Kp5 | Kp6 | Kp7 | Kp8 | Kp9 => NumPad,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_sounds_decodes() {
        for sound in Sound::iter() {
            assert!(
                Decoder::try_from(std::io::Cursor::new(sound.bytes())).is_ok(),
                "{sound:?} failed to decode"
            );
        }
    }
}
