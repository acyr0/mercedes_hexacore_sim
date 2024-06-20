use lazy_static::lazy_static;

cfg_if::cfg_if! {
    if #[cfg(feature = "sol_erda")] {
        const COST_TO_NEXT_SKILL: [u16; 30] = [
            0, // 0
            1, 1, 1, 2, 2, 2, 3, 3, 10, // 9
            3, 3, 4, 4, 4, 4, 4, 4, 5, 15, // 19
            5, 5, 5, 5, 5, 6, 6, 6, 7, 20, // 29
        ];

        const COST_TO_NEXT_MASTERY: [u16; 30] = [
            3, // 0
            1, 1, 1, 1, 1, 1, 2, 2, 5, // 9
            2, 2, 2, 2, 2, 2, 2, 2, 3, 8, // 19
            3, 3, 3, 3, 3, 3, 3, 3, 4, 10, // 29
        ];

        const COST_TO_NEXT_ENHANCEMENT: [u16; 30] = [
            4, // 0
            1, 1, 1, 2, 2, 2, 3, 3, 8, // 9
            3, 3, 3, 3, 3, 3, 3, 3, 4, 12, // 19
            4, 4, 4, 4, 4, 5, 5, 5, 6, 15, // 29
        ];
    } else {
        const COST_TO_NEXT_SKILL: [u16; 30] = [
            0, // 0
            30, 35, 40, 45, 50, 55, 60, 65, 200, // 9
            80, 90, 100, 110, 120, 130, 140, 150, 160, 350, // 19
            170, 180, 190, 200, 210, 220, 230, 240, 250, 500, // 29
        ];

        const COST_TO_NEXT_MASTERY: [u16; 30] = [
            50, // 0
            15, 18, 20, 23, 25, 28, 30, 33, 100, // 9
            40, 45, 50, 55, 60, 65, 70, 75, 80, 175, // 19
            85, 90, 95, 100, 105, 110, 115, 120, 125, 250, // 29
        ];

        const COST_TO_NEXT_ENHANCEMENT: [u16; 30] = [
            75, // 0
            23, 27, 30, 34, 38, 42, 45, 49, 150, // 9
            60, 68, 75, 83, 90, 98, 105, 113, 120, 263, // 19
            128, 135, 143, 150, 158, 165, 173, 180, 188, 375, // 29
        ];
    }
}

lazy_static! {
    pub static ref TOTAL_COST_SKILL: [u16; 31] = {
        let mut ret = [0; 31];
        let mut c = 0;
        for i in 0..=30 {
            ret[i] = c;
            if i == 30 {
                break;
            }
            c += COST_TO_NEXT_SKILL[i];
        }
        ret
    };
    pub static ref TOTAL_COST_MASTERY: [u16; 31] = {
        let mut ret = [0; 31];
        let mut c = 0;
        for i in 0..=30 {
            ret[i] = c;
            if i == 30 {
                break;
            }
            c += COST_TO_NEXT_MASTERY[i];
        }
        ret
    };
    pub static ref TOTAL_COST_ENHANCEMENT: [u16; 31] = {
        let mut ret = [0; 31];
        let mut c = 0;
        for i in 0..=30 {
            ret[i] = c;
            if i == 30 {
                break;
            }
            c += COST_TO_NEXT_ENHANCEMENT[i];
        }
        ret
    };
}

pub const FD_ENHANCEMENT: [u16; 31] = [
    0, // 0
    11, 12, 13, 14, 15, 16, 17, 18, 19, 25, // 10
    26, 27, 28, 29, 30, 31, 32, 33, 34, 40, // 20
    41, 42, 43, 44, 45, 46, 47, 48, 49, 60, // 30
];
