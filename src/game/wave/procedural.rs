use std::collections::HashMap;

use crate::{
    game::enemy::EnemyProceced,
    log,
    utils::{rng, GetRandom, Median},
    FPS,
};

use super::Wave;

#[inline]
fn get_duration(level: u32) -> u32 {
    const S: f32 = 0.45;
    const U: f32 = 0.2;

    let x = rng() as f32 + 0.5;

    let lognormal = (1. / x * S * (2. * std::f32::consts::PI).sqrt())
        * (-(x.ln() - U).powi(2) / 2. * S.powi(2)).exp();

    let more_time = (lognormal - 1.) * 20.;

    ((rng() as f32 * 2.5 - 1.25) * more_time) as u32 + level * 12 + 10
}

#[test]
fn ter() {
    for i in 1..30 {
        let mut dura = 0;
        for _ in 0..10 {
            dura += get_duration(i);
        }
        dura /= 10;

        let mut _periode_count = 0;
        for _ in 0..10 {
            _periode_count += get_periode_count(dura);
        }
        _periode_count /= 10;

        let mut diff = 0;
        for _ in 0..10 {
            diff += get_difficulty(dura, i);
        }
        diff /= 10;
        println!("[{}]: {}", i, diff)
    }
}

#[inline]
fn get_periode_count(duration: u32) -> u32 {
    (duration / 24 + (rng() > 0.75) as u32 * 2 + (rng() > 0.75) as u32).max(1)
}

#[inline]
fn get_difficulty(duration: u32, level: u32) -> u32 {
    (duration as f64 * (1.8 + rng() / 7.) + 1.3f64.powi(level as i32 + 10) * 3.) as u32
}

#[inline]
fn get_shift(min: u32, max: f64, power: f64) -> u32 {
    let x = rng() - 1.;
    (max * max.powf(x * power)) as u32 + min
}

type Marker = u32;

#[derive(Debug)]
struct Periode {
    type_: PeriodeType,
    duration: Duration,
    difficulty: f32,
    enemies: EnemyStorage,
}

impl Periode {
    #[inline]
    fn power(&self) -> f32 {
        self.difficulty / self.duration.duration
    }

}

#[derive(Debug, Clone)]
struct Duration {
    start: f32,
    duration: f32,
}

impl Duration {
    #[inline]
    fn get_new_position(&self) -> u64 {
        let position_relative =
        (rng() * self.duration as f64 * FPS as f64).floor() as u64;
        position_relative + self.start as u64 * FPS
    }
}

#[derive(Debug)]
struct WaveMarked {
    markers: Vec<Marker>,
}

#[derive(Debug)]
struct WavePerioded {
    periodes: Vec<Periode>,
    difficulty: u32,
    level: u32,
}

impl Wave {
    pub fn generate(level: u32) -> Self {
        let duration = get_duration(level);
        let periode_count = get_periode_count(duration);
        let difficulty = get_difficulty(duration, level);

        let min_space = duration / (periode_count * 3);
        let max_space = (duration * 2) / periode_count;
        let frac = duration / 10;

        let mut wave = WaveMarked::random(duration, periode_count);

        let mut pass = 0;
        while !wave.spread_marker(min_space, max_space, frac, pass > 10) {
            pass += 1;
            wave = WaveMarked::random(duration, periode_count);
        }

        let mut wave = WavePerioded::from_markers(wave, difficulty, level);

        let brutal_wave_count = ((wave.len() - 1) / 2).saturating_sub(1);

        wave.assign_periode_type(brutal_wave_count);

        wave.assign_difficulty();

        wave.add_enemies();

        wave.pack_enemies();

        Self::from_wave_perioded(wave).unwrap()
    }

    fn from_wave_perioded(wave: WavePerioded) -> Option<Self> {
        let troops = wave
            .periodes
            .into_iter()
            .map(|periode| {
                if let EnemyStorage::Packed(storage) = periode.enemies {
                    storage
                } else {
                    HashMap::new()
                }
            })
            .reduce(|mut acc, enemies| {
                acc.extend(enemies);
                acc
            });

        troops.map(|troops| Self { troops })
    }
}

impl WaveMarked {
    fn random(duration: u32, periode_count: u32) -> Self {
        let mut markers = (0..periode_count - 1)
            .map(|_| (rng() * duration as f64) as Marker)
            .collect::<Vec<Marker>>();
        markers.sort_unstable();

        markers.push(duration);

        Self { markers }
    }

    fn spread_marker(&mut self, min_space: u32, max_space: u32, frac: u32, fuck_it: bool) -> bool {
        let mut pass = 0;

        while self.is_not_spread(min_space, max_space) {
            pass += 1;
            if pass > 50 {
                return fuck_it;
            }

            for i in 1..self.markers.len() - 2 {
                let marker = *self.markers.get(i).unwrap();

                let left = *self.markers.get(i - 1).unwrap_or(&0);
                let left_dist = marker - left;
                let left_too_close = left_dist < min_space;
                let left_too_far = left_dist > max_space;

                let right = *self.markers.get(i + 1).unwrap();
                let right_dist = right - marker;
                let right_too_close = right_dist < min_space;
                let right_too_far = right_dist > max_space;

                let marker = self.markers.get_mut(i).unwrap();

                if (left_too_close || right_too_far) && (!right_too_close || !left_too_far) {
                    let shift = get_shift(1, frac as f64, 0.8);
                    *marker += shift;
                } else if (!left_too_close || !right_too_far) && (right_too_close || left_too_far) {
                    let shift = get_shift(1, frac as f64, 0.8);
                    *marker -= marker.saturating_sub(shift);
                }

                self.markers.sort_unstable();
            }
        }

        true
    }

    #[inline]
    fn is_not_spread(&self, min_space: u32, max_space: u32) -> bool {
        let mut last_marker = 0;
        self.markers.iter().any(|marker| {
            let space = marker - last_marker;
            last_marker = *marker;
            min_space > space || space > max_space
        })
    }
}

impl WavePerioded {
    #[inline]
    fn len(&self) -> usize {
        self.periodes.len()
    }

    fn from_markers(markers: WaveMarked, difficulty: u32, level: u32) -> Self {
        let mut last_marker = 0;
        let periodes = markers
            .markers
            .into_iter()
            .map(|marker| {
                let periode = Periode {
                    type_: PeriodeType::Bruine,
                    duration: Duration {
                        start: last_marker as f32,
                        duration: (marker - last_marker) as f32,
                    },
                    difficulty: 0.,
                    enemies: EnemyStorage::Free(HashMap::new()),
                };
                last_marker = marker;
                periode
            })
            .collect::<Vec<Periode>>();

        Self {
            periodes,
            difficulty,
            level,
        }
    }

    fn assign_periode_type(&mut self, brutal_wave_count: usize) {
        if let Some(periode) = self.periodes.last_mut() {
            periode.type_ = PeriodeType::new_random_brutal()
        }

        let mut spaces = self.periodes[0..self.periodes.len().saturating_sub(1)]
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, p)| (i, p.duration.duration))
            .collect::<Vec<_>>();

        for _ in 0..brutal_wave_count {
            if let Some((space_idx, (idx, _))) = spaces
                .iter()
                .enumerate()
                .min_by(|(_, (_, a)), (_, (_, b))| a.partial_cmp(b).unwrap())
            {
                if let Some(periode) = self.periodes.get_mut(*idx) {
                    periode.type_ = PeriodeType::new_random_brutal()
                }
                spaces.remove(space_idx);
            }
        }

        while self.is_tree_brutal_aligned() {
            if spaces.is_empty() {
                break;
            }

            if let Some((space_idx, (idx, _))) = spaces
                .iter()
                .enumerate()
                .min_by(|(_, (_, a)), (_, (_, b))| a.partial_cmp(b).unwrap())
            {
                self.fix_one_of_tree_aligned();

                if let Some(new_periode) = self.periodes.get_mut(*idx) {
                    new_periode.type_ = PeriodeType::new_random_brutal();
                    spaces.remove(space_idx);
                }
            }
        }
    }

    fn assign_difficulty(&mut self) {
        let basic_diff = (self.difficulty / self.len() as u32) as f32;
        let peaceful_diff = basic_diff * 0.8;
        let brutal_diff = basic_diff * 1.2;

        let mut brutal_powers = Vec::new();
        let mut peaceful_powers = Vec::new();

        for periode in self.periodes.iter_mut() {
            if matches!(periode.type_, PeriodeType::Bruine) {
                periode.difficulty = peaceful_diff;
                peaceful_powers.push(periode.power())
            } else {
                periode.difficulty = brutal_diff;
                brutal_powers.push(periode.power())
            }
        }

        let brutal_power_median = brutal_powers.median().unwrap_or(0.);
        let mut peaceful_power_median = brutal_powers.median().unwrap_or(0.);

        if (brutal_power_median - peaceful_power_median) < brutal_power_median * 0.35 {
            peaceful_power_median -= brutal_power_median * 0.35;

            for periode in self.periodes.iter_mut() {
                if matches!(periode.type_, PeriodeType::Bruine)
                    && periode.power() < brutal_power_median
                {
                    periode.difficulty += brutal_power_median * 0.35;
                }
            }
        }

        for periode in self.periodes.iter_mut() {
            if matches!(periode.type_, PeriodeType::Bruine) {
                let difference = (peaceful_power_median - periode.power()).abs();
                if difference > peaceful_power_median / 2. {
                    if periode.power() > peaceful_power_median {
                        periode.difficulty /= 2.;
                    } else {
                        periode.difficulty *= 1.5;
                    }
                }
            }
        }
    }

    fn add_enemies(&mut self) {
        let proba_rank2_enemies = self.proba_to_have_rank2_enemies();

        for periode in self.periodes.iter_mut() {
            let mut diff_points = periode.difficulty as u32 * 2;
            let is_brutal_periode = periode.type_.is_brutal();

            if matches!(periode.type_, PeriodeType::Block) {
                let mut for_block = (diff_points as f32 * 0.9) as u32;
                diff_points = (diff_points as f32 * 0.1) as u32;

                let enemy_ref = EnemyProceced::new_random(proba_rank2_enemies);
                if let EnemyStorage::Free(ref mut storage) = periode.enemies {
                    storage.insert(enemy_ref.level, 0);
                }

                while for_block > 0 {
                    if for_block >= enemy_ref.weight {
                        if let EnemyStorage::Free(ref mut storage) = periode.enemies {
                            *storage.get_mut(&enemy_ref.level).unwrap() += 1;
                        }
                        for_block -= enemy_ref.weight
                    } else {
                        for_block = 0
                    }
                }
            }

            while diff_points > 0 {
                let enemy = EnemyProceced::new_random(proba_rank2_enemies);
                if !is_brutal_periode && enemy.level % 4 == 0 {
                    continue;
                }

                if enemy.weight < diff_points {
                    diff_points -= enemy.weight;

                    if let EnemyStorage::Free(ref mut storage) = periode.enemies {
                        let level = storage.entry(enemy.level).or_insert(0);
                        *level += 1;
                    }
                } else {
                    diff_points -= 1;
                }
            }
        }
    }

    fn pack_enemies(&mut self) {
        for periode in self.periodes.iter_mut() {
            let mut new_storage: HashMap<u64, Vec<u8>> = HashMap::new();
            let duration = periode.duration.clone();
            if let EnemyStorage::Free(ref mut storage) = periode.enemies {
                let mut levels = storage.keys().cloned().collect::<Vec<u8>>();

                while !levels.is_empty() {
                    let (level, idx) = levels
                        .get_random_and_index()
                        .map(|(lvl, idx)| (*lvl, idx))
                        .unwrap();

                    let quantity = storage.get_mut(&level).unwrap();
                    *quantity -= 1;
                    if *quantity == 0 {
                        levels.remove(idx);
                    }
                    let mut position = duration.get_new_position();
                    let mut echec = false;
                    let mut pass = 0;
                    while new_storage.contains_key(&position) && new_storage[&position].len() >= 5 {
                        pass += 1;
                        if pass > 10 {
                            echec = true;
                            break;
                        }
                        position = duration.get_new_position();
                    }

                    if !echec {
                        let frame = new_storage.entry(position).or_insert_with(Vec::new);
                        frame.push(level);
                    }
                }
            }

            periode.enemies = EnemyStorage::Packed(new_storage);
        }
    }

    fn is_tree_brutal_aligned(&self) -> bool {
        if let Some((_, periodes)) = self.periodes.split_last() {
            let mut count = 0;
            periodes.iter().any(|periode| {
                if matches!(periode.type_, PeriodeType::Bruine) {
                    count = 0;
                } else {
                    count += 1;
                }
                count == 3
            }) || count == 2
        } else {
            false
        }
    }

    fn fix_one_of_tree_aligned(&mut self) {
        if let Some((_, periodes)) = self.periodes.split_last() {
            let mut count = 0;
            if let Some((i, _)) = periodes.iter().enumerate().find(|periode| {
                if matches!(periode.1.type_, PeriodeType::Bruine) {
                    count = 0;
                } else {
                    count += 1;
                }
                count == 3
            }) {
                if let Some(periode) = self.periodes.get_mut(i - (rng() * 3.) as usize) {
                    periode.type_ = PeriodeType::Bruine
                }
            } else if count == 2 {
                let len = self.periodes.len();
                if let Some(periode) = self.periodes.get_mut(len - (rng() * 2.) as usize - 2) {
                    periode.type_ = PeriodeType::Bruine
                }
            }
        }
    }

    #[inline]
    fn proba_to_have_rank2_enemies(&self) -> f64 {
        ((self.level as f64 - 9.2).tanh() + 1.) * 0.5
    }
}

#[test]
fn ezr() {
    fn get_proba(level: u32) -> f64 {
        ((level as f64 - 9.2).tanh() + 1.) * 0.5
    }

    for i in 0..21 {
        let mut proba = 0.;
        for _ in 0..10 {
            proba += get_proba(i)
        }
        proba /= 10.;
        println!("Proba: {:.2}", proba);
    }
}

#[derive(Debug, Clone)]
enum PeriodeType {
    Bruine,
    Block,
    Group,
}

impl PeriodeType {
    #[inline]
    fn is_brutal(&self) -> bool {
        !matches!(self, Self::Bruine)
    }

    #[inline]
    fn new_random_brutal() -> Self {
        if rng() > 0.55 {
            PeriodeType::Block
        } else {
            PeriodeType::Group
        }
    }
}

#[derive(Debug)]
enum EnemyStorage {
    Free(HashMap<u8, u32>),        // lvl, quantity
    Packed(HashMap<u64, Vec<u8>>), // seconde, lvl
}
