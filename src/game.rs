#[derive(Debug)]
pub enum Version {
    Unknown = 0,
    Gm530 = 530,
    Gm600 = 600,
    Gm700 = 700,
    Gm800 = 800,
    Gm810 = 810,
}

impl Default for Version {
    fn default() -> Self {
        Version::Unknown
    }
}

#[derive(Default)]
pub struct Game {
    pub version: Version,
    pub debug: bool,
    pub pro: bool,
    pub game_id: u32,
    pub guid: [u32; 4],
}