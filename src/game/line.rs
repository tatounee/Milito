
use super::NBR_OF_COLUMN;
use super::turret::Turret;
use super::projectile::Projectile;
use super::enemy::Enemy;

#[derive(Debug, Clone)]
pub struct Line {
    pub cells: Vec<Option<Box<dyn Turret>>>,
    pub shoots: Vec<Box<dyn Projectile>>,
    pub enemies: Vec<Box<dyn Enemy>>,
}

impl Line {
    pub(crate) fn delete_turret(&mut self, x: usize) -> u32 {
        let mut refund = 0;
        if let Some(ref turret) = self.cells[x] {
            refund = turret.refund()
        }
        self.cells[x] = None;
        refund
    }

    pub(crate) fn add_turret(&mut self, x: usize, turret: Box<dyn Turret>) -> u32 {
        let mut price = 0;
        if self.cells[x].is_none() {
            price = turret.price();
            self.cells[x] = Some(turret)
        }
        price
    }

    pub(crate) fn spawn_projectile(&mut self, projectile: Box<dyn Projectile>) {
        self.shoots.push(projectile)
    }

    pub(crate) fn spawn_projectiles<I: IntoIterator<Item = Box<dyn Projectile>>>(
        &mut self,
        projectiles: I,
    ) {
        self.shoots.extend(projectiles.into_iter())
    }

    pub(crate) fn process(&mut self) {
        self.process_shoots();

        self.process_ennemies();

        self.process_turrets();
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

    fn process_shoots(&mut self) {
        let mut attack_buf = Vec::new();
        let mut enemies = self.enemies.iter();
        self.shoots
            .iter_mut()
            .enumerate()
            .for_each(|(proj_index, proj)| {
                if enemies.by_ref().enumerate().all(|(enemy_index, enemy)| {
                    if enemy.hitbox().collide(proj.hitbox()) {
                        attack_buf.push((proj_index, enemy_index));
                        false
                    } else {
                        true
                    }
                }) {
                    proj.deplace()
                }
            });
        let mut dead_ennemies = Vec::new();
        for (proj_index, enemy_index) in attack_buf {
            let projectile = &self.shoots[proj_index];
            let enemy = self.enemies.get_mut(enemy_index).unwrap().as_mut();
            enemy.take_damage(projectile.damage());
            if enemy.get_life() == &0 {
                dead_ennemies.push(enemy_index);
            }
        }
        for dead_index in dead_ennemies {
            self.enemies.remove(dead_index);
        }
    }

    fn process_ennemies(&mut self) {
        let mut attack_buf = Vec::new();
        let mut cells = self.cells.iter();
        self.enemies
            .iter_mut()
            .enumerate()
            .for_each(|(enemy_index, enemy)| {
                // ENEMY WAIT
                enemy.wait();
                if cells.by_ref().enumerate().all(|(turret_index, turret)| {
                    if let Some(turret) = turret {
                        if turret.hitbox().collide(enemy.hitbox()) {
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
            let enemy = &self.enemies[enemy_index];
            if enemy.can_attack() {
                let turret = self.cells.get_mut(turret_index).unwrap().as_mut().unwrap();
                // enemy.penetrate(turret);
                turret.take_damage(enemy.damage());
                if turret.get_life() == &0 {
                    dead_turrets.push(turret_index);
                }
            }
        }
        for dead_index in dead_turrets {
            self.cells[dead_index] = None;
        }
    }
}

impl Default for Line {
    fn default() -> Self {
        Self {
            cells: vec![None; NBR_OF_COLUMN],
            shoots: Vec::new(),
            enemies: Vec::new(),
        }
    }
}
