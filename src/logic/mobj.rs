use bitflags::bitflags;

bitflags! {
    pub(crate) struct MobjFlag: u64 {
        const NONE = 0x0;
        /// Calls mobj::special_thing() when touched!
        const SPECIAL = 0x1;
        /// Blocks other things.
        const SOLID = 0x2;
        /// Can be hit.
        const SHOOTABLE = 0x4;
        /// Don't use sector links, i.e. invisible but touchable
        const NOSECTOR = 0x8;
        /// Don't use blocklinks, i.e. inert but displayable
        const NOBLOCKMAP = 0x10;
        /// Not activated by sound
        const AMBUSH = 0x20;
        /// Will try to attack soon
        const JUSTHIT = 0x40;
        /// Will take at least one step before attacking again
        const JUSTATTACKED = 0x80;
        /// Initial position hanging from ceiling
        const SPAWNCEILING = 0x100;
        /// Gravity does not apply
        const NOGRAVITY = 0x200;
        /// Allow to jump from high places
        const DROPOFF = 0x400;
        /// Picks up items
        const PICKUP = 0x800;
        /// Player cheat
        const NOCLIP = 0x1000;
        /// Keep info about sliding along walls (player)
        const SLIDE = 0x2000;
        /// Allow moving to any height, no gravity
        ///
        /// e.g. cacodemon, pain elemental
        const FLOAT = 0x4000;
        /// Don't cross lines or look at heights on teleport
        const TELEPORT = 0x8000;
        /// Don't hit the same species, explode on block
        /// Player missiles/fireballs from enemies
        const MISSILE = 0x10000;
        /// Dropped by a demon, i.e. not placed by the level
        const DROPPED = 0x20000;
        /// Use fuzzy drawing for this
        const SHADOW = 0x40000;
        /// Don't bleed when shot, use bullet puff instead
        ///
        /// e.g. barrels, shootable furniture
        const NOBLOOD = 0x80000;
        /// Don't stop moving halfway off a step, i.e. slide down
        /// all the way
        const CORPSE = 0x100000;
        /// Floating to a specific height, so don't float to
        /// the target right now.
        const INFLOAT = 0x200000;
        /// On kill, count this item towards the total
        const COUNTKILL = 0x400000;
        /// On pickup, count towards item total
        const COUNTITEM = 0x800000;
        /// Use special handling for a skull in flight
        ///
        /// (ed. note: these things are annoying!)
        const SKULLFLY = 0x1000000;
        /// Don't spawn this in deathmatch mode.
        const NOTDMATCH = 0x2000000;
        /// Player sprites in multiplayer are modified using
        /// an internal color lookup table.
        /// If the value is one of the three below, use a
        /// lookup table.
        const TRANSLATION = 0xc000000;
        const TRANSLATION1 = 0x4000000;
        const TRANSLATION2 = 0x8000000;
        /// Docs just say ???. I agree.
        const TRANSSHIFT = 26;

        const UNUSED2 = 0x10000000;
        const UNUSED3 = 0x20000000;

        /// Translucent sprite, maybe
        const TRANSLUCENT = 0x40000000;

        const TOUCHY = 0x100000000;
        const BOUNCES = 0x200000000;
        const FRIEND = 0x400000000;
        const RESURRECTED = 0x1000000000;
        const NO_DEPTH_TEST = 0x2000000000;
        const FOREGROUND = 0x4000000000;
        const PLAYERSPRITE = 0x8000000000;

        /// Not targetted when it hurts something else
        const NOTARGET = 0x10000000000;
        /// fly mode active
        const FLY = 0x20000000000;
    }
}

impl Into<u64> for MobjFlag {
    fn into(self) -> u64 {
        self.bits
    }
}
