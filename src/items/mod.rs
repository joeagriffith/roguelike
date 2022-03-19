mod weapon;
mod projectile;

pub use weapon::Weapon;
pub use weapon::update_weapons_system;

pub use projectile::Projectile;
pub use projectile::projectile_movement_system;

pub use weapon::spawn_w_meteor;