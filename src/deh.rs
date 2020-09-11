use crate::info::MOBJINFO;
use crate::info::{MobjType, Spritenum, Statenum, SPRNAMES, STATES};
use crate::{
    sounds::{Music, Sfx, MUSIC, SFX},
    think::ActionF,
};
use lazy_static::lazy_static;
use parking_lot::RwLock;

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

pub fn build_bex_tables() {
    let mut i = 0;
    while i < Statenum::ExtraStates as usize {
        CODEPTR.write()[i] = STATES.read()[i].action;
        i += 1;
    }
    while i < Statenum::NumStates as usize {
        let mut state = &mut STATES.write()[i];
        state.sprite = Spritenum::TNT1;
        state.frame = 0;
        state.tics = -1;
        state.action = || ();
        state.nextstate = i;
        CODEPTR.write()[i] = state.action;
        i += 1;
    }

    for (i, sprname) in SPRNAMES.iter().enumerate() {
        SPRITENAMES.write()[i] = sprname;
    }
    for (i, music) in MUSIC.iter().enumerate().skip(1) {
        MUSICNAMES.write()[i] = music.name;
    }
    for (i, sfx) in SFX.iter().enumerate().skip(1) {
        SOUNDNAMES.write()[i] = sfx.name;
    }
    let upper_bound_remove_me = MOBJINFO.read().len();
    for i in 0..upper_bound_remove_me {
        let mobj = &mut MOBJINFO.write()[i];
        if i == MobjType::WOLFSS as usize || i == MobjType::POSSESSED as usize {
            mobj.droppeditem = MobjType::CLIP;
        } else if i == MobjType::SHOTGUY as usize {
            mobj.droppeditem = MobjType::SHOTGUN;
        } else if i == MobjType::CHAINGUY as usize {
            mobj.droppeditem = MobjType::CHAINGUN;
        } else {
            mobj.droppeditem = MobjType::NULL;
        }
    }
}
