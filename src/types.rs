use enum_map::{Enum, EnumMap};

use crate::constants::{TOTAL_COST_ENHANCEMENT, TOTAL_COST_MASTERY, TOTAL_COST_SKILL};

#[derive(Debug, Enum)]
pub enum Skill {
    IshtarsRing,
    IshtarsRingBrand,
    WrathOfEnlil,
    SpikesRoyale,
    AdvancedFinalAttack,
    LeapTornado,
    GustDive,
    UnicornSpike,
    StaggeringStrike,
    ElementalKnights,

    IrkallasWrath,
    RoyalKnights,
    SpiritOfElluel,
    SylvidiasFlight,

    UnfadingGlory,
}

#[derive(Debug)]
pub struct BA {
    // The sum of the contributions of all of the columns here should equal 1.
    pub columns: EnumMap<Skill, BAColumn>,
}

#[derive(Debug)]
pub struct BAColumn {
    pub contribution: f64,
    pub times_used: u64,
}

#[derive(Debug, Clone, Copy, Enum)]
pub enum HexacoreSkill {
    UnfadingGlory,
    IshtarsRingVI,
    WrathSpikesTornadoVI,

    IrkallasWrath,
    RoyalKnights,
    SpiritOfElluel,
    SylvidiasFlight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HexacoreSpec(pub EnumMap<HexacoreSkill, u8>);

impl HexacoreSpec {
    pub fn valid(&self) -> bool {
        (1..=30).contains(&self.0[HexacoreSkill::UnfadingGlory])
            && (0..=30).contains(&self.0[HexacoreSkill::IshtarsRingVI])
            && (0..=30).contains(&self.0[HexacoreSkill::WrathSpikesTornadoVI])
            && (0..=30).contains(&self.0[HexacoreSkill::IrkallasWrath])
            && (0..=30).contains(&self.0[HexacoreSkill::RoyalKnights])
            && (0..=30).contains(&self.0[HexacoreSkill::SpiritOfElluel])
            && (0..=30).contains(&self.0[HexacoreSkill::SylvidiasFlight])
    }

    pub fn cost(&self) -> u16 {
        TOTAL_COST_SKILL[self.0[HexacoreSkill::UnfadingGlory] as usize]
            + TOTAL_COST_MASTERY[self.0[HexacoreSkill::IshtarsRingVI] as usize]
            + TOTAL_COST_MASTERY[self.0[HexacoreSkill::WrathSpikesTornadoVI] as usize]
            + TOTAL_COST_ENHANCEMENT[self.0[HexacoreSkill::IrkallasWrath] as usize]
            + TOTAL_COST_ENHANCEMENT[self.0[HexacoreSkill::RoyalKnights] as usize]
            + TOTAL_COST_ENHANCEMENT[self.0[HexacoreSkill::SpiritOfElluel] as usize]
            + TOTAL_COST_ENHANCEMENT[self.0[HexacoreSkill::SylvidiasFlight] as usize]
    }
}
