use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct HitBox(f32);

#[derive(Debug, Component)]
pub struct AttackRange(f32);

#[derive(Debug, Component)]
pub struct Damage(f32);

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub sprite_bundle: SpriteBundle,
    pub hit_box: HitBox,
    pub attack_range: AttackRange,
    pub damage: Damage,
}

#[derive(Debug, Component)]
pub struct Projectile {
    pub start_position: Vec2,
}

#[derive(Debug, Component)]
pub struct Weapon {
    pub damage: f32,
    pub attack_range: f32,
    pub attack_speed: f32,
    pub projectile_image: Handle<Image>,
}