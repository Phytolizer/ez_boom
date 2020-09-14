use crate::info::MOBJINFO;
use crate::info::{MobjType, Spritenum, Statenum, SPRNAMES, STATES};
use crate::{
    sounds::{Music, Sfx, MUSIC, SFX},
    think::ActionF,
};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use strum::IntoEnumIterator;

// TODO delete these dumb statics
lazy_static! {
    pub(crate) static ref CODEPTR: RwLock<[ActionF; Statenum::NumStates as usize]> =
        RwLock::new([|| (); Statenum::NumStates as usize]);
    pub(crate) static ref SPRITENAMES: RwLock<[&'static str; Spritenum::NumSprites as usize + 1]> =
        RwLock::new([""; Spritenum::NumSprites as usize + 1]);
    pub(crate) static ref MUSICNAMES: RwLock<[&'static str; Music::NUMMUSIC as usize + 1]> =
        RwLock::new([""; Music::NUMMUSIC as usize + 1]);
    pub(crate) static ref SOUNDNAMES: RwLock<[&'static str; Sfx::NUMSFX as usize + 1]> =
        RwLock::new([""; Sfx::NUMSFX as usize + 1]);
}

/// Set up the initial BEX tables, initializing them with their default values.
pub fn build_bex_tables() {
    let mut i = 0;
    while i < Statenum::ExtraStates as usize {
        CODEPTR.write()[i] = STATES.read()[i].action;
        i += 1;
    }
    while i < Statenum::NumStates as usize {
        let mut state = &mut STATES.write()[i];
        // invisible sprite
        state.sprite = Spritenum::TNT1;
        state.frame = 0;
        state.tics = -1;
        state.action = || ();
        state.nextstate = i;
        CODEPTR.write()[i] = state.action;
        i += 1;
    }

    // initialize runtime-modifiable tables with the constant initial values

    for (i, sprname) in SPRNAMES.iter().enumerate() {
        SPRITENAMES.write()[i] = sprname;
    }
    for (i, music) in MUSIC.iter().enumerate().skip(1) {
        MUSICNAMES.write()[i] = music.name;
    }
    for (i, sfx) in SFX.iter().enumerate().skip(1) {
        SOUNDNAMES.write()[i] = sfx.name;
    }
    // set dropped items for enemies that drop items
    let mut mobjinfo = MOBJINFO.write();
    for i in MobjType::iter() {
        let mobj = match mobjinfo.get_mut(&i) {
            Some(v) => v,
            None => continue,
        };
        if i == MobjType::WOLFSS || i == MobjType::POSSESSED {
            mobj.droppeditem = MobjType::CLIP;
        } else if i == MobjType::SHOTGUY {
            mobj.droppeditem = MobjType::SHOTGUN;
        } else if i == MobjType::CHAINGUY {
            mobj.droppeditem = MobjType::CHAINGUN;
        } else {
            mobj.droppeditem = MobjType::NULL;
        }
    }
}
