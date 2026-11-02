//! Wave composition and difficulty scaling.

use rand::Rng;
use rand::seq::SliceRandom;

use super::config::TOTAL_WAVES;

/// Enemy archetypes. Stats live in [`EnemyKind::stats`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EnemyKind {
    Grunt,
    Runner,
    Tank,
    Boss,
}

/// Base stats before wave multipliers are applied.
#[derive(Clone, Copy, Debug)]
pub struct EnemyStats {
    pub name: &'static str,
    pub hp: f32,
    pub speed: f32,
    pub reward: i32,
    pub leak: i32,
    pub radius: f32,
}

impl EnemyKind {
    pub const ALL: [EnemyKind; 4] = [
        EnemyKind::Grunt,
        EnemyKind::Runner,
        EnemyKind::Tank,
        EnemyKind::Boss,
    ];

    pub fn stats(self) -> EnemyStats {
        match self {
            EnemyKind::Grunt => EnemyStats {
                name: "Grunt",
                hp: 40.0,
                speed: 55.0 / 40.0,
                reward: 8,
                leak: 1,
                radius: 10.0 / 40.0,
            },
            EnemyKind::Runner => EnemyStats {
                name: "Runner",
                hp: 22.0,
                speed: 105.0 / 40.0,
                reward: 8,
                leak: 1,
                radius: 8.0 / 40.0,
            },
            EnemyKind::Tank => EnemyStats {
                name: "Tank",
                hp: 160.0,
                speed: 34.0 / 40.0,
                reward: 20,
                leak: 2,
                radius: 14.0 / 40.0,
            },
            EnemyKind::Boss => EnemyStats {
                name: "Boss",
                hp: 380.0,
                speed: 28.0 / 40.0,
                reward: 60,
                leak: 5,
                radius: 18.0 / 40.0,
            },
        }
    }
}

/// Per-wave multipliers and timing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaveScaling {
    pub hp_mult: f32,
    pub speed_mult: f32,
    pub spawn_interval: f32,
    pub clear_bonus: i32,
}

pub fn wave_scaling(wave: u32) -> WaveScaling {
    WaveScaling {
        hp_mult: 1.0 + 0.13 * (wave.saturating_sub(1)) as f32,
        speed_mult: 1.0 + 0.015 * (wave.saturating_sub(1)) as f32,
        spawn_interval: (0.9 - 0.035 * wave as f32).max(0.30),
        clear_bonus: 45 + 14 * wave as i32,
    }
}

/// Compose a wave: a core of grunts, runners from wave 3, tanks from wave 5
/// and a boss every fifth wave. The order is shuffled so leaks feel varied.
pub fn build_wave<R: Rng + ?Sized>(wave: u32, rng: &mut R) -> Vec<EnemyKind> {
    let mut kinds = Vec::new();

    kinds.extend(std::iter::repeat_n(EnemyKind::Grunt, (3 + wave) as usize));
    if wave >= 3 {
        kinds.extend(std::iter::repeat_n(EnemyKind::Runner, (wave - 2) as usize));
    }
    if wave >= 5 {
        kinds.extend(std::iter::repeat_n(
            EnemyKind::Tank,
            (1 + (wave - 5) / 3) as usize,
        ));
    }
    if wave.is_multiple_of(5) {
        kinds.extend(std::iter::repeat_n(EnemyKind::Boss, (wave / 5) as usize));
    }

    kinds.shuffle(rng);
    kinds
}

/// Which edge gates are active for a wave.
///
/// Gate order is left, top, right, bottom. Early waves probe one side at a
/// time; later waves open more gates until every direction is under pressure.
pub fn spawn_indices(wave: u32) -> Vec<usize> {
    match wave {
        0 => vec![0],
        1 => vec![0],
        2 => vec![1],
        3 => vec![2],
        4 => vec![3],
        5 => vec![0, 2],
        6 => vec![1, 3],
        7 => vec![0, 1, 2],
        8 => vec![1, 2, 3],
        _ => vec![0, 1, 2, 3],
    }
}

/// Format a wave counter for the HUD.
pub fn wave_label(wave: u32) -> String {
    format!("{}/{}", wave.min(TOTAL_WAVES), TOTAL_WAVES)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    fn counts(kinds: &[EnemyKind]) -> (usize, usize, usize, usize) {
        let mut out = (0, 0, 0, 0);
        for kind in kinds {
            match kind {
                EnemyKind::Grunt => out.0 += 1,
                EnemyKind::Runner => out.1 += 1,
                EnemyKind::Tank => out.2 += 1,
                EnemyKind::Boss => out.3 += 1,
            }
        }
        out
    }

    #[test]
    fn wave_one_is_only_grunts() {
        let mut rng = StdRng::seed_from_u64(1);
        let kinds = build_wave(1, &mut rng);
        assert_eq!(counts(&kinds), (4, 0, 0, 0));
    }

    #[test]
    fn wave_five_has_all_archetypes() {
        let mut rng = StdRng::seed_from_u64(5);
        let kinds = build_wave(5, &mut rng);
        let (grunts, runners, tanks, bosses) = counts(&kinds);
        assert_eq!(grunts, 8);
        assert_eq!(runners, 3);
        assert_eq!(tanks, 1);
        assert_eq!(bosses, 1);
        assert_eq!(kinds.len(), 13);
    }

    #[test]
    fn wave_fifteen_has_three_bosses() {
        let mut rng = StdRng::seed_from_u64(15);
        let kinds = build_wave(15, &mut rng);
        assert_eq!(counts(&kinds).3, 3);
    }

    #[test]
    fn scaling_matches_design() {
        let first = wave_scaling(1);
        assert_eq!(first.hp_mult, 1.0);
        assert_eq!(first.speed_mult, 1.0);
        assert!((first.spawn_interval - 0.865).abs() < 1e-6);
        assert_eq!(first.clear_bonus, 59);

        let late = wave_scaling(15);
        assert!((late.hp_mult - 2.82).abs() < 1e-5);
        assert!((late.speed_mult - 1.21).abs() < 1e-5);
        assert_eq!(late.spawn_interval, 0.375);
        assert_eq!(late.clear_bonus, 255);
    }

    #[test]
    fn interval_never_drops_below_floor() {
        assert_eq!(wave_scaling(100).spawn_interval, 0.30);
    }

    #[test]
    fn gates_open_up_as_waves_progress() {
        assert_eq!(spawn_indices(1), vec![0]);
        assert_eq!(spawn_indices(4), vec![3]);
        assert_eq!(spawn_indices(5), vec![0, 2]);
        assert_eq!(spawn_indices(7), vec![0, 1, 2]);
        assert_eq!(spawn_indices(9), vec![0, 1, 2, 3]);
        assert_eq!(spawn_indices(15), vec![0, 1, 2, 3]);
    }

    #[test]
    fn same_seed_same_order() {
        let mut a = StdRng::seed_from_u64(9);
        let mut b = StdRng::seed_from_u64(9);
        assert_eq!(build_wave(7, &mut a), build_wave(7, &mut b));
    }
}
