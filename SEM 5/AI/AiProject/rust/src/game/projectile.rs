//! Homing projectiles. They track a target id so removing enemies from the
//! active list never leaves a dangling reference.

use super::enemy::Enemy;
use super::position::Position;
use super::tower::TowerKind;

#[derive(Clone, Debug)]
pub struct Projectile {
    pub id: u64,
    pub pos: Position,
    pub target_id: u64,
    pub kind: TowerKind,
    pub damage: f32,
    pub speed: f32,
    pub alive: bool,
}

impl Projectile {
    pub fn new(
        id: u64,
        origin: Position,
        target_id: u64,
        kind: TowerKind,
        damage: f32,
        speed: f32,
    ) -> Self {
        Self {
            id,
            pos: origin,
            target_id,
            kind,
            damage,
            speed,
            alive: true,
        }
    }

    pub fn update(&mut self, dt: f32, enemies: &mut [Enemy]) {
        if !self.alive {
            return;
        }

        let Some(target_index) = enemies.iter().position(|enemy| enemy.id == self.target_id) else {
            self.alive = false;
            return;
        };
        if !enemies[target_index].alive || enemies[target_index].leaked {
            self.alive = false;
            return;
        }

        let distance = self.pos.distance_to(enemies[target_index].pos);
        let step = self.speed * dt;
        if distance <= step || distance < 1e-3 {
            self.pos = enemies[target_index].pos;
            self.alive = false;
            self.impact(target_index, enemies);
        } else {
            let scale = step / distance;
            self.pos.x += (enemies[target_index].pos.x - self.pos.x) * scale;
            self.pos.y += (enemies[target_index].pos.y - self.pos.y) * scale;
        }
    }

    fn impact(&self, target_index: usize, enemies: &mut [Enemy]) {
        let stats = self.kind.stats();
        let impact = enemies[target_index].pos;

        if stats.splash > 0.0 {
            for enemy in enemies.iter_mut() {
                if !enemy.alive || enemy.leaked {
                    continue;
                }
                let distance = enemy.pos.distance_to(impact);
                if distance <= stats.splash + enemy.radius * 0.5 {
                    let falloff = 1.0 - 0.4 * (distance / stats.splash).clamp(0.0, 1.0);
                    damage(enemy, self.damage * falloff);
                }
            }
        } else {
            damage(&mut enemies[target_index], self.damage);
        }

        if stats.slow < 1.0 {
            let target = &mut enemies[target_index];
            if target.alive {
                target.apply_slow(stats.slow, stats.slow_duration);
            }
        }

        if stats.chain > 0 {
            let mut candidates: Vec<(usize, f32)> = enemies
                .iter()
                .enumerate()
                .filter(|(index, enemy)| {
                    *index != target_index
                        && enemy.alive
                        && !enemy.leaked
                        && enemy.pos.distance_to(impact) <= stats.chain_range
                })
                .map(|(index, enemy)| (index, enemy.pos.distance_to(impact)))
                .collect();
            candidates.sort_by(|a, b| a.1.total_cmp(&b.1));
            for (index, _) in candidates.into_iter().take(stats.chain as usize) {
                damage(&mut enemies[index], self.damage * 0.6);
            }
        }
    }
}

fn damage(enemy: &mut Enemy, amount: f32) {
    if !enemy.alive {
        return;
    }
    enemy.hp -= amount;
    if enemy.hp <= 0.0 {
        enemy.hp = 0.0;
        enemy.alive = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::cell::Cell;
    use crate::game::wave::EnemyKind;

    fn enemy(id: u64) -> Enemy {
        Enemy::new(
            id,
            EnemyKind::Grunt,
            vec![Cell::new(5, 5), Cell::new(6, 5)],
            Cell::new(6, 5),
            1.0,
            1.0,
        )
    }

    #[test]
    fn hits_target_and_deals_damage() {
        let mut enemies = [enemy(3)];
        let start_hp = enemies[0].hp;
        let mut projectile =
            Projectile::new(1, Position::new(0.0, 0.0), 3, TowerKind::Basic, 15.0, 100.0);
        projectile.update(1.0, &mut enemies);
        assert_eq!(enemies[0].hp, start_hp - 15.0);
        assert!(!projectile.alive);
    }

    #[test]
    fn kills_target_at_zero_hp() {
        let mut target = enemy(4);
        target.hp = 5.0;
        let mut enemies = [target];
        let mut projectile =
            Projectile::new(1, Position::new(0.0, 0.0), 4, TowerKind::Basic, 20.0, 100.0);
        projectile.update(1.0, &mut enemies);
        assert_eq!(enemies[0].hp, 0.0);
        assert!(!enemies[0].alive);
    }

    #[test]
    fn expires_when_target_is_gone() {
        let mut projectile = Projectile::new(
            1,
            Position::new(0.0, 0.0),
            99,
            TowerKind::Basic,
            10.0,
            100.0,
        );
        projectile.update(0.1, &mut []);
        assert!(!projectile.alive);
    }

    #[test]
    fn splash_damages_neighbours() {
        let first = enemy(6);
        let mut second = enemy(7);
        second.pos = Position::new(5.2, 5.0);
        let mut enemies = [first, second];
        enemies[0].hp = 100.0;
        enemies[1].hp = 100.0;
        let mut projectile = Projectile::new(
            1,
            Position::new(0.0, 0.0),
            6,
            TowerKind::Cannon,
            20.0,
            100.0,
        );
        projectile.update(10.0, &mut enemies);
        assert!(enemies[0].hp < 100.0);
        assert!(enemies[1].hp < 100.0);
    }

    #[test]
    fn frost_rounds_apply_a_slow() {
        let mut enemies = [enemy(8)];
        let mut projectile =
            Projectile::new(1, Position::new(0.0, 0.0), 8, TowerKind::Frost, 5.0, 100.0);
        projectile.update(10.0, &mut enemies);
        assert!(enemies[0].slow_timer > 0.0);
        assert!(enemies[0].slow_factor < 1.0);
    }

    #[test]
    fn arc_rounds_chain_to_a_second_target() {
        let first = enemy(9);
        let mut second = enemy(10);
        second.pos = Position::new(6.2, 5.0);
        let mut enemies = [first, second];
        let mut projectile =
            Projectile::new(1, Position::new(0.0, 0.0), 9, TowerKind::Tesla, 20.0, 100.0);
        projectile.update(10.0, &mut enemies);
        assert!(enemies[0].hp < 100.0);
        assert!(enemies[1].hp < 100.0);
    }

    #[test]
    fn travels_towards_the_target() {
        let target = enemy(5);
        let mut enemies = [target];
        let start = Position::new(0.0, 0.0);
        let mut projectile = Projectile::new(1, start, 5, TowerKind::Basic, 10.0, 1.0);
        projectile.update(1.0, &mut enemies);
        assert!(projectile.pos.distance_to(start) > 0.9);
        assert!(projectile.alive);
    }
}
