use asr::{
    emulator::{ps2::Emulator}, 
    timer::{pause_game_time, resume_game_time}};

use std::collections::HashMap;

// supported game versions, (remix only for now)
#[derive(Eq, Hash, PartialEq)]
pub enum Version {
    V1_NTSC,
    V1_NTSCJ,
    V1_PAL,
    V2_NTSC,   
    V2_NTSCJ,
    V2_PAL,
    REMIX_NTSCJ,
    REMIX_PAL,
    REMIX_NTSC
}

// game variables we want to track
#[derive(Eq, Hash, PartialEq)]
pub enum GameVariables {
    version,
    loading,
}

pub struct Game<'a> {
    pub current: State,
    pub last: State,
    pub version: Version,
    pub pointers: HashMap<Version, HashMap<GameVariables, u32>>,
    pub process: &'a Emulator
}

pub struct State {
    pub loading: u32
}

// just a macro to make adding pointers easier
macro_rules! ap {
    ($s:ident [$ver:ident][$var:ident] = $addr:expr) => {{
        $s.pointers
            .entry(Version::$ver)
            .or_insert_with(std::collections::HashMap::new)
            .insert(GameVariables::$var, $addr);
    }};
}

// get value from PS2 memory
macro_rules! get_value {
    ($s:ident, $var:ident, $t:ty) => {{
        $s.pointers
            .get(&$s.version)
            .and_then(|m| m.get(&GameVariables::$var))
            .and_then(|addr| Emulator::read::<$t>(&$s.process, *addr).ok())
            .unwrap_or_default()
    }};
}

macro_rules! update_value {
    ($s:ident, $field:ident, $ty:ty) => {{
        $s.last.$field = $s.current.$field;
        $s.current.$field = get_value!($s, $field, $ty);
    }};
}


impl<'a> Game<'a> {
    // run when the game is first loaded
    pub fn new(emu: &'a Emulator) -> Self {
        let mut s = Self {
            current: State {loading: 0},
            last: State {loading: 0},
            version: Version::REMIX_NTSC,
            pointers: HashMap::new(),
            process: emu,
        };

        ap!{s[REMIX_NTSC][loading] = 0x6144BC};
        return s;
    }

    // runs on each tick
    pub fn update(&mut self) { 
        // update all necessary information here
        //self.update_game_version();
        self.update_values();

        self.is_loading();  // pause the game timer if loading

        // add more stuff here when needed
    }

    // runs when the timer is reset
    pub fn reset(&mut self) {   
        todo!("Implement mc3 reset logic");
    }

    // split the current segment
    pub fn split(&mut self) {   
        todo!("Implement mc3 split logic");
    }

    // returns true if the game is currently loading
    pub fn is_loading(&self) {
        if self.current.loading == self.last.loading {
            return;
        }

        if self.current.loading != 0 {
            pause_game_time();
        } else {
            resume_game_time();
        }
    }

    // logic to determine and set the game version
    fn update_game_version(&mut self) { 
        todo!("Implement game version detection logic");
    }

    // logic to read and update game values from memory
    fn update_values(&mut self) { 
        update_value!(self, loading, u32);
        // add more values here when needed
    }
}