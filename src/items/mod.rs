mod weapon;
mod projectile;

pub use weapon::Weapon;
pub use weapon::update_weapons;

pub use projectile::Projectile;
pub use projectile::projectile_movement;

pub use weapon::spawn_w_meteor;