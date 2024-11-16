pub enum PlayerPowerup {
    None,
    Mushroom,
    FireFlower,
    MiniMushroom,
    PropellerMushroom,
    PenguinSuit,
    IceFlower,
}

pub const POWERUP_COUNT: usize = 7;

pub enum PlayerCharacter {
    Mario,
    Luigi,
    BlueToad,
    YellowToad,
}

pub const PLAYER_COUNT: usize = 4;


pub const STAGE_COUNT: usize = 42;
pub const WORLD_COUNT: usize = 10;

pub enum SaveFileRegion {
    NTSC,
    PAL,
    JPN,
    KOR,
    CHN,
    TW
}

pub const AMBUSH_ENEMY_COUNT: usize = 4;

// there are 64 hint movies, but 70 is the constant
pub const HINT_MOVIE_COUNT: usize = 70;
pub const ACTUAL_HINT_MOVIE_COUNT: usize = 64;
