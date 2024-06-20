use lazy_static::lazy_static;

use crate::config::{
    BA_DURATION_SEC, BA_SPEC, BOSS_DMG, ENEMY_PDR, IED, IED_DEBUFFS, IED_SOURCES, POSTNEWAGE_BA,
};
use crate::constants::FD_ENHANCEMENT;
use crate::types::{HexacoreSkill, HexacoreSpec, Skill, BA};

lazy_static! {
    pub static ref REMAINING_PDR: f64 = {
        let mut defense_pen = 1.0;
        for &ied in IED_SOURCES {
            defense_pen *= 1.0 - ied as f64 / 100.0;
        }

        // Check that the calculated IED matches the Stats window IED.
        assert_eq!(
            ((1.0 - defense_pen) * 1000.0).trunc(),
            (IED * 1000.0).trunc(),
            "Stats window IED doesn't match calculated IED"
        );

        for &ied in IED_DEBUFFS {
            defense_pen *= 1.0 - ied as f64 / 100.0;
        }
        defense_pen * ENEMY_PDR / 100.0
    };
}

fn simulate_origin(ba: &BA, old_origin: u8, cur_origin: u8, boss: f64, remaining_pdr: f64) -> f64 {
    fn unfading_glory_damage(level: u8) -> f64 {
        if level == 0 {
            return 0.0;
        }

        // TODO: Too lazy to sim all the damage here (and also not sure how it works), its about 2x
        // diff.
        (420 + level as u64 * 14) as f64
    }

    let old_damage: f64 = unfading_glory_damage(old_origin);
    let new_damage: f64 = unfading_glory_damage(cur_origin);

    fn breakpoint_multiplier(level: u8, boss: f64, remaining_pdr: f64) -> f64 {
        if level == 30 {
            (100.0 + boss + 50.0) / (100.0 + boss) * (1.0 - remaining_pdr * 0.50)
                / (1.0 - remaining_pdr)
        } else if level >= 20 {
            (100.0 + boss + 20.0) / (100.0 + boss) * (1.0 - remaining_pdr * 0.80)
                / (1.0 - remaining_pdr)
        } else if level >= 10 {
            (1.0 - remaining_pdr * 0.80) / (1.0 - remaining_pdr)
        } else {
            1.0
        }
    }

    let old_breakpoint_multiplier = breakpoint_multiplier(old_origin, boss, remaining_pdr);
    let new_breakpoint_multiplier = breakpoint_multiplier(cur_origin, boss, remaining_pdr);

    (new_damage / old_damage - 1.0)
        * (new_breakpoint_multiplier / old_breakpoint_multiplier)
        * ba.columns[Skill::UnfadingGlory].contribution
}

fn simulate_ishtars(
    ba: &BA,
    old_mastery1: u8,
    cur_mastery1: u8,
    old_mastery2: u8,
    cur_mastery2: u8,
) -> f64 {
    fn ishtars_damage(level: u8) -> f64 {
        if level > 0 {
            ((345 + level as u64 * 6) * 2) as f64
        } else {
            (318 * 2) as f64
        }
    }

    fn ishtars_brand_damage(level1: u8, level2: u8) -> f64 {
        if level1 > 0 {
            if level2 > 0 {
                ((4 * (400 + level1 as u64 * 7 + 30 + level2 as u64 * 7) * 3)
                    + (500 + level1 as u64 * 8 + 80 + level2 as u64 * 7) * 8) as f64
            } else {
                ((4 * (400 + level1 as u64 * 7) * 3) + (500 + level1 as u64 * 8) * 8) as f64
            }
        } else {
            0 as f64
        }
    }

    let old_damage_ishtars: f64 = ishtars_damage(old_mastery1);
    let new_damage_ishtars: f64 = ishtars_damage(cur_mastery1);

    let old_damage_brand: f64 = ishtars_brand_damage(old_mastery1, old_mastery2);
    let new_damage_brand: f64 = ishtars_brand_damage(cur_mastery1, cur_mastery2);

    (new_damage_ishtars / old_damage_ishtars - 1.0) * ba.columns[Skill::IshtarsRing].contribution
        + (new_damage_brand / old_damage_brand - 1.0)
            * ba.columns[Skill::IshtarsRingBrand].contribution
}

fn simulate_wrath(ba: &BA, old_mastery: u8, cur_mastery: u8) -> f64 {
    fn wrath_damage(ba: &BA, level: u8) -> f64 {
        if level > 0 {
            let enhanced_uses = BA_DURATION_SEC / 12;
            let unenhanced_uses = ba.columns[Skill::WrathOfEnlil].times_used - enhanced_uses;
            ((enhanced_uses * (605 + level as u64 * 11) * 10
                + unenhanced_uses * (550 + level as u64 * 10) * 10) as f64)
                / ((enhanced_uses + unenhanced_uses) as f64)
        } else {
            (515 * 10) as f64
        }
    }

    fn spikes_damage(ba: &BA, level: u8) -> f64 {
        if level > 0 {
            let enhanced_uses = BA_DURATION_SEC / 12;
            let unenhanced_uses = ba.columns[Skill::SpikesRoyale].times_used - enhanced_uses;
            ((enhanced_uses * (740 + level as u64 * 14) * 4
                + unenhanced_uses * (695 + level as u64 * 12) * 4) as f64)
                / ((enhanced_uses + unenhanced_uses) as f64)
        } else {
            (640 * 4) as f64
        }
    }

    fn tornado_damage(ba: &BA, level: u8) -> f64 {
        if level > 0 {
            let enhanced_uses = BA_DURATION_SEC / 6;
            let unenhanced_uses = ba.columns[Skill::LeapTornado].times_used - enhanced_uses;
            ((enhanced_uses * (590 + level as u64 * 9) * 4
                + unenhanced_uses * (510 + level as u64 * 9) * 4) as f64)
                / ((enhanced_uses + unenhanced_uses) as f64)
        } else {
            (500 * 4) as f64
        }
    }

    let old_damage_wrath: f64 = wrath_damage(ba, old_mastery);
    let new_damage_wrath: f64 = wrath_damage(ba, cur_mastery);

    let old_damage_spikes: f64 = spikes_damage(ba, old_mastery);
    let new_damage_spikes: f64 = spikes_damage(ba, cur_mastery);

    let old_damage_tornado: f64 = tornado_damage(ba, old_mastery);
    let new_damage_tornado: f64 = tornado_damage(ba, cur_mastery);

    (new_damage_wrath / old_damage_wrath - 1.0) * ba.columns[Skill::WrathOfEnlil].contribution
        + (new_damage_spikes / old_damage_spikes - 1.0)
            * ba.columns[Skill::SpikesRoyale].contribution
        + (new_damage_tornado / old_damage_tornado - 1.0)
            * ba.columns[Skill::LeapTornado].contribution
}

fn simulate_sylvidia(
    ba: &BA,
    old_enhance: u8,
    cur_enhance: u8,
    unfading_glory_contrib: f64,
    irkallas_fd_increase: f64,
) -> f64 {
    fn sylvidia_fd(level: u8) -> f64 {
        if level > 0 {
            ((level + 5) / 6) as f64
        } else {
            0 as f64
        }
    }

    let old_sylvidia_multi = 1.0 + sylvidia_fd(old_enhance) / 100.0;
    let new_sylvidia_multi = 1.0 + sylvidia_fd(cur_enhance) / 100.0;

    (new_sylvidia_multi / old_sylvidia_multi - 1.0)
        * (ba.columns[Skill::UnfadingGlory].contribution
            + unfading_glory_contrib
            + (1.0 + irkallas_fd_increase) * ba.columns[Skill::IrkallasWrath].contribution)
}

pub fn simulate_hexacores(spec: HexacoreSpec) -> f64 {
    let unfading_glory_contrib = simulate_origin(
        &POSTNEWAGE_BA,
        (*BA_SPEC).0[HexacoreSkill::UnfadingGlory],
        spec.0[HexacoreSkill::UnfadingGlory],
        BOSS_DMG,
        *REMAINING_PDR,
    );
    let ishtarsvi_contrib = simulate_ishtars(
        &POSTNEWAGE_BA,
        (*BA_SPEC).0[HexacoreSkill::IshtarsRingVI],
        spec.0[HexacoreSkill::IshtarsRingVI],
        (*BA_SPEC).0[HexacoreSkill::WrathSpikesTornadoVI],
        spec.0[HexacoreSkill::WrathSpikesTornadoVI],
    );
    let wrathvi_contrib = simulate_wrath(
        &POSTNEWAGE_BA,
        (*BA_SPEC).0[HexacoreSkill::WrathSpikesTornadoVI],
        spec.0[HexacoreSkill::WrathSpikesTornadoVI],
    );

    let irkallas_fd_increase = (1.0
        + FD_ENHANCEMENT[spec.0[HexacoreSkill::IrkallasWrath] as usize] as f64 / 100.0)
        / (1.0 + FD_ENHANCEMENT[BA_SPEC.0[HexacoreSkill::IrkallasWrath] as usize] as f64 / 100.0)
        - 1.0;
    let royal_knights_fd_increase = (1.0
        + FD_ENHANCEMENT[spec.0[HexacoreSkill::RoyalKnights] as usize] as f64 / 100.0)
        / (1.0 + FD_ENHANCEMENT[BA_SPEC.0[HexacoreSkill::RoyalKnights] as usize] as f64 / 100.0)
        - 1.0;

    let sylvidia_contrib = simulate_sylvidia(
        &POSTNEWAGE_BA,
        (*BA_SPEC).0[HexacoreSkill::SylvidiasFlight],
        spec.0[HexacoreSkill::SylvidiasFlight],
        unfading_glory_contrib,
        irkallas_fd_increase,
    );

    irkallas_fd_increase * POSTNEWAGE_BA.columns[Skill::IrkallasWrath].contribution
        + royal_knights_fd_increase * POSTNEWAGE_BA.columns[Skill::RoyalKnights].contribution
        + unfading_glory_contrib
        + sylvidia_contrib
        + ishtarsvi_contrib
        + wrathvi_contrib
}
