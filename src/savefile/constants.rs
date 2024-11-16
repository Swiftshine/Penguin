use bitflags::bitflags;

pub const HEADER_SIZE: usize = 0x6A0;

#[derive(Copy, Clone)]
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


#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum StartingMushroomKind {
    None,
    Star,
    Item,
    OneUp,
    StarRescue,
    ItemRescue,
    OneUpRescue,
}

#[derive(Copy, Clone)]
pub enum EnemyDirection {
    ToNextNode, // "forwards" to the next node
    ToPreviousNode, // "backwards" to the previous node
    FirstTimeValue, // the initial value that was set prior to entering the world for the first time
}

bitflags!{
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GameCompletionFlags: u8 {
        const SaveEmpty           = 0b00000001;
        const FinalBossBeaten     = 0b00000010;
        const AllGoals            = 0b00000100;
        const AllStarCoinsReg     = 0b00001000;
        const AllStarCoinsSpe     = 0b00010000;
        const GameCompleted       = 0b00100000;
        const SuperGuideTriggered = 0b01000000;
    }
    
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PlayerCreationFlags: u8 {
        const None = 0;
        const StarPower  = 0b00000001;
        const Yoshi      = 0b00000010;
        const Bubble     = 0b00000100;
        const RescueToad = 0b00001000;
    }
}

