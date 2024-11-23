use bitflags::bitflags;

pub const HEADER_SIZE: usize = 0x6A0;
pub const MAX_SCORE: u32 = 99999950;

#[derive(Copy, Clone, PartialEq)]
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


#[derive(Copy, Clone, PartialEq)]
pub enum PlayerCharacter {
    Mario,
    Luigi,
    BlueToad,
    YellowToad,
}

pub const PLAYER_COUNT: usize = 4;


pub const STAGE_COUNT: usize = 42;
pub const WORLD_COUNT: usize = 10;
pub const ACTUAL_WORLD_COUNT: usize = 9;

#[derive(PartialEq)]
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

#[derive(Copy, Clone, PartialEq)]
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

pub const HINT_MOVIE_TITLES: [&str; ACTUAL_HINT_MOVIE_COUNT] = [
    "1-1: Super Skills",
    "1-1: Infinite 1-Ups",
    "1-2: Infinite 1-Ups",
    "1-3: Secret Goal",
    "1-3: Super Skills",
    "1-T: Super Skills",
    "2-1: Star Coin",
    "2-1: Super Skills",
    "2-2: Star Coin",
    "2-2: Super Skills",
    "2-3: Infinite 1-Ups",
    "2-4: Secret Goal",
    "2-5: Infinite 1-Ups",
    "2-5: Super Skills",
    "2-6: Secret Goal",
    "3-1: Star Coin",
    "3-1: Super Skills",
    "3-2: Super Skills",
    "3-2: Infinite 1-Ups",
    "3-3: Star Coin",
    "3-3: Infinite 1-Ups",
    "3-3: Super Skills",
    "3-G: Secret Goal",
    "3-5: Super Skills",
    "4-1: Infinite 1-Ups",
    "4-2: Infinite 1-Ups",
    "4-2: Super Skills",
    "4-3: Star Coin",
    "4-3: Super Skills",
    "4-T: Secret Goal",
    "4-4: Star Coin",
    "4-G: Secret Goal",
    "4-C: Infinite 1-Ups",
    "5-1: Infinite 1-Ups",
    "5-3: Infinite 1-Ups",
    "5-G: Star Coin",
    "5-G: Secret Goal",
    "5-C: Super Skills",
    "6-1: Infinite 1-Ups",
    "6-2: Star Coin",
    "6-3: Star Coin",
    "6-3: Super Skills",
    "6-5: Secret Goal",
    "6-6: Secret Goal",
    "6-C: Super Skills",
    "7-1: Super Skills",
    "7-3: Star Coin",
    "7-T: Secret Goal",
    "7-G: Secret Goal",
    "7-4: Infinite 1-Ups",
    "7-C: Star Coin",
    "8-2: Secret Goal",
    "8-3: Super Skills",
    "8-T: Star Coin",
    "8-A: Infinite 1-Ups",
    "8-C: Super Skills",
    "9-1: Super Skills",
    "9-2: Super Skills",
    "9-3: Infinite 1-Ups",
    "9-3: Super Skills",
    "9-4: Star Coin",
    "9-5: Super Skills",
    "9-6: Star Coin",
    "9-7: Super Skills",
];

pub const POWERUP_NAMES: [&str; 7] = [
    "Mushroom",
    "Fire Flower",
    "Mini Mushroom",
    "Propeller Mushroom",
    "Penguin Suit",
    "Ice Flower",
    "Star"
];
    
pub const POWERUP_NAMES_2: [&str; 7] = [
    "None",
    "Mushroom",
    "Fire Flower",
    "Mini Mushroom",
    "Propeller Mushroom",
    "Penguin Suit",
    "Ice Flower",
];

pub const POWERUP_STOCK_MAX: u8 = 99;
pub const PLAYER_LIFE_MAX: u8 = 99;

pub const PLAYER_NAMES: [&str; 4] = [
    "Mario", "Luigi", "Blue Toad", "Yellow Toad"
];
