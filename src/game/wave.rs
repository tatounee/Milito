use std::collections::HashMap;

use super::enemy::Enemy;

// Exemple:
// let lvl1 = wave![
//     1 => [1, 2,],
//     100 => [1],
// ];
#[macro_export]
macro_rules! wave {
    ($($secs:expr => [$($lvl:expr),+ $(,)?]),*  $(,)?) => {
        {
            let mut troops = std::collections::HashMap::new();
            $(
                let mut vec = Vec::new();
                $(
                    vec.push($lvl);
                )*
                let frame = $secs * crate::FPS;
                troops.insert(frame, vec);
            )*
            Wave {troops}
        }
    };
}

#[derive(Debug, Clone)]
pub struct Wave {
    pub troops: HashMap<u64, Vec<u8>>,
}

#[derive(Debug, Default, Clone)]
pub struct WaveLine {
    pub troops: HashMap<u64, u8>,
}

impl WaveLine {
    pub fn add_enemy(&mut self, frame: u64, level: u8) -> Option<u8> {
        self.troops.insert(frame, level)
    }
}

#[derive(Debug, Clone)]
pub struct IteratorWaveLine {
    frame: u64,
    frame_max: u64,
    troops: HashMap<u64, u8>,
}

impl IteratorWaveLine {
    pub fn is_ended(&self) -> bool {
        self.frame >= self.frame_max
    }
}

impl Iterator for IteratorWaveLine {
    type Item = Enemy;

    fn next(&mut self) -> Option<Self::Item> {
        if self.frame <= self.frame_max {
            let item = self.troops.remove(&self.frame);
            self.frame += 1;
            item.map(|lvl| Enemy::prefab(lvl)).flatten()
        } else {
            None
        }
    }
}

impl IntoIterator for WaveLine {
    type Item = Enemy;
    type IntoIter = IteratorWaveLine;

    fn into_iter(self) -> Self::IntoIter {
        let frame_max = self.troops.keys().max().cloned().unwrap_or(0);

        IteratorWaveLine {
            frame: 0,
            frame_max,
            troops: self.troops,
        }
    }
}
