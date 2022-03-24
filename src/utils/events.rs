
pub struct KillEvent {
    pub xp_reward: f32,
}

pub struct DamageEvent {
    pub damage: f32,
}

pub struct GameOverEvent {
    pub kills: usize,
    pub level: usize,
}

pub struct LevelUpEvent {
}