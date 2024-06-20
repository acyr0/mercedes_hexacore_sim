use enum_map::enum_map;
use lazy_static::lazy_static;

use crate::types::{BAColumn, HexacoreSkill, HexacoreSpec, Skill, BA};

pub const ENEMY_PDR: f64 = 300.0;

/// Damage + Boss Damage from the Stats window. Don't include the base 100% here.
pub const BOSS_DMG: f64 = 900.0;

/// IED from the Stats window.
pub const IED: f64 = 0.9774;
/// Any IED sources that are part of the Stats window calculation.
pub const IED_SOURCES: &'static [u8] = &[
    31, // Priere D'Aria + DCO
    20, // Luminous link
    30, // Event buff
    10, // CRA Hat
    5,  // CRA Top
    5,  // CRA Bottom
    20, // Weapon
    10, // 7 Day Monster Parker medal
    3,  // Familiar badge
    3,  // Familiar badge
    40, // Familiar effect
    10, // Arcane Umbra 3 set
    30, // Superior Gollux
    10, // Pitched 3 set
    5,  // Blaster legion
    5,  // Beast tamer legion
    40, // Legion board
    36, // 12 IED hyper
    10, // Ambition
];
/// Any IED sources that are not part of the Stats window calculation
pub const IED_DEBUFFS: &'static [u8] = &[];

pub const BA_DURATION_SEC: u64 = 9 * 60;

lazy_static! {
    pub static ref BA_SPEC: HexacoreSpec = HexacoreSpec(enum_map! {
        HexacoreSkill::UnfadingGlory => 1,
        HexacoreSkill::IshtarsRingVI => 1,
        HexacoreSkill::WrathSpikesTornadoVI => 0,

        HexacoreSkill::IrkallasWrath => 1,
        HexacoreSkill::RoyalKnights => 0,
        HexacoreSkill::SpiritOfElluel => 0,
        HexacoreSkill::SylvidiasFlight => 0,
    });
    pub static ref POSTNEWAGE_BA: BA = BA {
        columns: enum_map! {
            Skill::UnfadingGlory => BAColumn {
                contribution: 0.1120,
                times_used: 205,
            },
            Skill::IshtarsRing => BAColumn {
                contribution: 0.0672,
                times_used: 969,
            },
            Skill::IshtarsRingBrand => BAColumn {
                contribution: 0.0502,
                times_used: 314,
            },
            Skill::IrkallasWrath => BAColumn {
                contribution: 0.2801,
                times_used: 164,
            },
            Skill::RoyalKnights => BAColumn {
                contribution: 0.0743,
                times_used: 300,
            },
            Skill::SpiritOfElluel => BAColumn {
                contribution: 0.0218,
                times_used: 53,
            },
            Skill::SylvidiasFlight => BAColumn {
                contribution: 0.0,
                times_used: 0,
            },
            Skill::WrathOfEnlil => BAColumn {
                contribution: 0.0921,
                times_used: 174,
            },
            Skill::UnicornSpike => BAColumn {
                contribution: 0.0524,
                times_used: 148,
            },
            Skill::StaggeringStrike => BAColumn {
                contribution: 0.0466,
                times_used: 278,
            },
            Skill::ElementalKnights => BAColumn {
                contribution: 0.0345,
                times_used: 573,
            },
            Skill::SpikesRoyale => BAColumn {
                contribution: 0.0326,
                times_used: 124,
            },
            Skill::AdvancedFinalAttack => BAColumn {
                contribution: 0.0294,
                times_used: 1222,
            },
            Skill::LeapTornado => BAColumn {
                contribution: 0.0171,
                times_used: 88,
            },
            Skill::GustDive => BAColumn {
                contribution: 0.0141,
                times_used: 78,
            },
        },
    };
}
