use std::cell::RefCell;
use std::collections::VecDeque;

use super::components::Collide;
use super::enemy::Enemy;
use super::projectile::Projectile;
use super::turret::Turret;
use super::{BOARD_LENGHT, NBR_OF_COLUMN};
use super::wave::{IteratorWaveLine, WaveLine};
use crate::log;

#[derive(Debug, Clone)]
pub struct Line {
    pub cells: Vec<Option<Turret>>,
    pub projectiles: RefCell<Vec<Projectile>>,
    pub enemies: RefCell<Vec<Enemy>>,
    pub waves: RefCell<VecDeque<WaveLine>>,
    current_wave: Option<RefCell<IteratorWaveLine>>,
}

impl Line {
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
            self.cells[x] = Some(turret.set_x(((x + 1) * 12 + 6) as f32))
        }
        price
    }

    pub fn spawn_projectile(&mut self, projectile: Projectile) {
        self.projectiles.borrow_mut().push(projectile)
    }

    pub fn spawn_projectiles<I: IntoIterator<Item = Projectile>>(&mut self, projectiles: I) {
        self.projectiles.borrow_mut().extend(projectiles.into_iter())
    }

    pub fn kill_all(&mut self) -> u32 {
        self.enemies
            .borrow_mut()
            .drain(..)
            .map(|enemy| enemy.reward())
            .sum()
    }

    pub fn next_wave(&mut self) -> Option<()> {
        self.current_wave = Some(RefCell::new(self.waves.borrow_mut().pop_front()?.into_iter()));
        Some(())
    }

    pub fn is_wave_ended(&self) -> bool {
        if let Some(wave) = &self.current_wave {
            wave.borrow().is_ended()
        } else {
            true
        }
    }

    pub fn remaining_enemies(&self) -> bool {
        !self.enemies.borrow().is_empty()
    }

    pub fn process(&mut self) {
        self.process_projectiles();

        self.process_enemies();

        self.process_turrets();

        self.spawn_new_enemies();
    }

    fn process_turrets(&mut self) {
        let mut shoots_buf = Vec::with_capacity(7);
        self.cells.iter_mut().for_each(|turret| {
            if let Some(turret) = turret {
                // TURRET WAIT
                turret.wait();
                if turret.can_attack() {
                    shoots_buf.push(turret.shoot().unwrap())
                }
            }
        });
        self.spawn_projectiles(shoots_buf);
    }

    fn process_projectiles(&mut self) {
        // TODO: delete proj out
        let mut attack_buf = Vec::new();
        let mut proj_buf = Vec::new();
        {
            let bren = self.enemies.borrow();
            let mut enemies = bren.iter();
            self.projectiles
                .borrow_mut()
                .iter_mut()
                .enumerate()
                .for_each(|(proj_index, proj)| {
                    if enemies.by_ref().enumerate().all(|(enemy_index, enemy)| {
                        if enemy.collide(proj) {
                            attack_buf.push((proj_index, enemy_index));
                            proj_buf.push(proj_index);
                            false
                        } else {
                            true
                        }
                    }) {
                        proj.deplace()
                    }
                    if proj.x() > BOARD_LENGHT as f32 {
                        proj_buf.push(proj_index)
                    }
                });
        }

        attack_buf.sort_unstable_by(|(_, enmy1), (_, enmy2)| enmy2.cmp(enmy1));
        proj_buf.sort_unstable_by(|proj1, proj2| proj2.cmp(proj1));

        let mut dead_enemies = Vec::new();
        for (proj_index, enemy_index) in attack_buf {
            {
                let projectile = &self.projectiles.borrow()[proj_index];
                let mut bren = self.enemies.borrow_mut();
                let enemy = bren.get_mut(enemy_index).unwrap();
                enemy.take_damage(projectile.damage());
                if enemy.is_dead() {
                    dead_enemies.push(enemy_index);
                }
            }
        }
        for dead_index in dead_enemies {
            self.enemies.borrow_mut().remove(dead_index);
        }

        let mut projectiles = self.projectiles.borrow_mut();
        for index in proj_buf {
            projectiles.remove(index);
        }
    }

    fn process_enemies(&mut self) {
        let mut attack_buf = Vec::new();
        let mut cells = self.cells.iter();
        self.enemies
            .borrow_mut()
            .iter_mut()
            .enumerate()
            .for_each(|(enemy_index, enemy)| {
                // ENEMY WAIT
                enemy.wait();
                if cells.by_ref().enumerate().all(|(turret_index, turret)| {
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
                    enemy.deplace()
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
            current_wave: None,
        }
    }
}
