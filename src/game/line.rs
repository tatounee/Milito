use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::VecDeque;

use super::components::Collide;
use super::enemy::Enemy;
use super::projectile::Projectile;
use super::turret::Turret;
use super::wave::{IteratorWaveLine, WaveLine};
use super::{Defeat, Reward, BOARD_LENGHT, CELL_SIZE, NBR_OF_COLUMN};
use crate::log;

fn is_enemies_in_front(coord: &[f32], x: f32) -> bool {
    coord
        .iter()
        .any(|coord| &x <= coord && coord < &(CELL_SIZE * 8. + 6.))
}

#[derive(Debug, Clone)]
pub struct Line {
    pub cells: Vec<Option<Turret>>,
    pub projectiles: RefCell<Vec<Projectile>>,
    pub enemies: RefCell<Vec<Enemy>>,
    pub waves: RefCell<VecDeque<WaveLine>>,
    nbr_of_wave: usize,
    current_wave: Option<RefCell<IteratorWaveLine>>,
}

impl Line {
    pub(crate) fn skip_one_wave(&mut self) -> u32 {
        let mut wave = self
            .waves
            .borrow_mut()
            .pop_front()
            .unwrap_or_default()
            .into_iter();
        let mut reward = 0;
        while !wave.is_ended() {
            if let Some(enemy) = wave.next() {
                reward += enemy.reward()
            }
        }
        reward
    }

    pub fn delete_turret(&mut self, x: usize) -> u32 {
        let mut refund = 0;
        if let Some(ref turret) = self.cells[x] {
            refund = turret.refund()
        }
        self.cells[x] = None;
        refund
    }

    pub fn add_turret(&mut self, x: usize, turret: Turret) -> u32 {
        let mut price = 0;
        if self.cells[x].is_none() {
            price = turret.price();
            self.cells[x] = Some(turret.set_x((x + 1) as f32 * CELL_SIZE + 6.))
        }
        price
    }

    #[inline]
    pub fn set_waves(&mut self, waves: RefCell<VecDeque<WaveLine>>) {
        self.nbr_of_wave = waves.borrow().len();
        self.waves = waves;
    }

    #[inline]
    pub fn wave(&self) -> usize {
        self.nbr_of_wave - self.waves.borrow().len()
    }

    #[inline]
    pub fn spawn_projectile(&mut self, projectile: Projectile) {
        self.projectiles.borrow_mut().push(projectile)
    }

    #[inline]
    pub fn spawn_projectiles<I: IntoIterator<Item = Projectile>>(&mut self, projectiles: I) {
        self.projectiles
            .borrow_mut()
            .extend(projectiles.into_iter())
    }

    #[inline]
    pub fn use_god(&mut self) -> Reward {
        let dead_enemies = self
            .enemies
            .borrow_mut()
            .iter_mut()
            .enumerate()
            .flat_map(|(i, enemy)| {
                enemy.take_damage((enemy.max_life() / 1.25) as u32);
                if enemy.is_dead() {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        let mut enemies = self.enemies.borrow_mut();
        dead_enemies
            .into_iter()
            .rev()
            .map(|index| enemies.remove(index).reward())
            .sum()
    }

    #[inline]
    pub fn start_next_wave(&mut self) -> Option<()> {
        self.current_wave = Some(RefCell::new(
            self.waves.borrow_mut().pop_front()?.into_iter(),
        ));
        Some(())
    }

    #[inline]
    pub fn is_wave_ended(&self) -> bool {
        if let Some(wave) = &self.current_wave {
            wave.borrow().is_ended()
        } else {
            true
        }
    }

    #[inline]
    pub fn is_remaining_enemies(&self) -> bool {
        !self.enemies.borrow().is_empty()
    }

    #[inline]
    fn enemies_coord(&self) -> Vec<f32> {
        self.enemies
            .borrow()
            .iter()
            .map(|e| e.hitbox().end())
            .collect::<Vec<f32>>()
    }

    pub fn process(&mut self) -> (Reward, Defeat) {
        let reward = self.process_projectiles();

        let defeat = self.process_enemies();

        self.process_turrets();

        self.spawn_new_enemies();

        (reward, defeat)
    }

    fn process_projectiles(&mut self) -> Reward {
        let mut buf_attack = Vec::new();
        let mut del_projs = Vec::new();
        let mut move_projs = Vec::new();
        let mut dead_enemies = Vec::new();
        let mut reward = 0;

        self.projectiles
            .borrow_mut()
            .iter_mut()
            .enumerate()
            .for_each(|(proj_index, proj)| {
                if let Some(impact_idx) = proj.take_next_impact() {
                    buf_attack.push((proj_index, impact_idx));
                } else {
                    move_projs.push(proj_index);
                }
            });

        buf_attack.sort_unstable_by(|(_, enmy1), (_, enmy2)| enmy2.cmp(enmy1));

        for (proj_index, enemy_index) in buf_attack {
            let projectile = &self.projectiles.borrow()[proj_index];
            let mut bren = self.enemies.borrow_mut();
            let enemy = bren.get_mut(enemy_index).unwrap();

            if !dead_enemies.contains(&enemy_index) {
                enemy.take_damage(projectile.damage());
                del_projs.push(proj_index);
                if enemy.is_dead() {
                    dead_enemies.push(enemy_index)
                }
            } else {
                move_projs.push(proj_index);
            }
        }

        dead_enemies.sort_unstable_by(|enmy1, enmy2| enmy2.cmp(enmy1));
        for dead_index in dead_enemies {
            reward += self.enemies.borrow_mut().remove(dead_index).reward();
        }

        {
            let mut projectiles = self.projectiles.borrow_mut();
            for proj_index in move_projs {
                let proj = projectiles.get_mut(proj_index).unwrap();
                if let Some(idx) = self
                    .enemies
                    .borrow()
                    .iter()
                    .enumerate()
                    .filter(|(_, enemy)| enemy.hitbox().end() >= proj.hitbox().start())
                    .min_by(|(_, enmy1), (_, enmy2)| {
                        (enmy1.hitbox().start() - proj.hitbox().end())
                            .partial_cmp(&(enmy2.hitbox().start() - proj.hitbox().end()))
                            .unwrap()
                    })
                    .map(|(idx, enemy)| {
                        if (enemy.hitbox().start() + enemy.speed())
                            - (proj.hitbox().end() + proj.speed())
                            <= 0.
                        {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .flatten()
                {
                    proj.add_next_impact(idx)
                }
                proj.deplace();
                if proj.x() > BOARD_LENGHT as f32 {
                    del_projs.push(proj_index)
                }
            }
        }

        del_projs.sort_unstable_by(|proj1, proj2| proj2.cmp(proj1));
        let mut projectiles = self.projectiles.borrow_mut();
        for proj in del_projs {
            projectiles.remove(proj);
        }

        reward
    }

    fn process_enemies(&mut self) -> bool {
        let mut attack_buf = Vec::new();
        let mut defeat = false;
        self.enemies
            .borrow_mut()
            .iter_mut()
            .enumerate()
            .for_each(|(enemy_index, enemy)| {
                // ENEMY WAIT
                enemy.wait();
                if self.cells.iter().enumerate().all(|(turret_index, turret)| {
                    if let Some(turret) = turret {
                        if turret.collide(enemy) {
                            attack_buf.push((enemy_index, turret_index));
                            false
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                }) {
                    enemy.deplace();
                    if enemy.x() < -10. {
                        defeat = true;
                    }
                }
            });
        let mut dead_turrets = Vec::new();
        for (enemy_index, turret_index) in attack_buf {
            let mut bren = self.enemies.borrow_mut();
            let enemy = bren.get_mut(enemy_index).unwrap();
            if enemy.can_attack() {
                let turret = self.cells.get_mut(turret_index).unwrap().as_mut().unwrap();
                turret.take_damage(enemy.attack());
                if turret.is_dead() {
                    dead_turrets.push(turret_index);
                }
            }
        }
        for dead_index in dead_turrets {
            self.cells[dead_index] = None;
        }

        defeat
    }

    fn process_turrets(&mut self) {
        let mut shoots_buf = Vec::with_capacity(7);
        let enemies_coord = self.enemies_coord();
        self.cells.iter_mut().for_each(|turret| {
            if let Some(turret) = turret {
                // TURRET WAIT
                turret.wait();
                if turret.can_attack()
                    && is_enemies_in_front(&enemies_coord, turret.hitbox().start())
                {
                    shoots_buf.push(turret.shoot().unwrap())
                }
            }
        });
        self.spawn_projectiles(shoots_buf);
    }

    fn spawn_new_enemies(&mut self) {
        if let Some(wave) = &self.current_wave {
            if let Some(enemy) = wave.borrow_mut().next() {
                self.enemies.borrow_mut().push(enemy);
            }
        }
    }
}

impl Default for Line {
    fn default() -> Self {
        Self {
            cells: vec![None; NBR_OF_COLUMN],
            projectiles: RefCell::new(Vec::new()),
            enemies: RefCell::new(Vec::new()),
            waves: RefCell::new(VecDeque::new()),
            nbr_of_wave: 0,
            current_wave: None,
        }
    }
}
