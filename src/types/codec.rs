#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codec {
    Avc,
    Hevc,
    Xvid,
    Mpeg,
    Av1,
}

impl Codec {
    pub fn as_str(&self) -> &'static str {
        match self {
            Codec::Avc => "avc",
            Codec::Hevc => "hevc",
            Codec::Xvid => "xvid",
            Codec::Mpeg => "mpeg",
            Codec::Av1 => "av1",
        }
    }
}
