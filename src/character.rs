pub enum Alignment {
    Good,
    Neutral,
    Evil,
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Neutral
    }
}

pub struct Character {
    pub name: String,
    pub alignment: Alignment,
    pub armor_class: i32,
    pub damage: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub wisdom: u32,
    pub intelligence: u32,
    pub charisma: u32,
    pub experience_points: u64,
}

impl Character {
    pub fn fighter() -> Self {
            Self::default()
    }

    pub fn max_hit_points(&self) -> u32 {
        10 + ((self.level() as i32 - 1) * 5) as u32
    }

    

    pub fn is_dead(&self) -> bool {
        self.damage >= self.max_hit_points()
    }

    pub fn modifier_score(score: u32) -> i32 {
        match score {
            1 => -5,
            2 | 3 => -4,
            4 | 5 => -3,
            6 | 7 => -2,
            8 | 9 => -1,
            10 | 11 => 0,
            12 | 13 => 1,
            14 | 15 => 2,
            16 | 17 => 3,
            18 | 19 => 4,
            20 => 5,
            _ => 0,
        }
    }

    pub fn level(&self) -> u64 {
        1 + (self.experience_points / 1000)
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: String::new(),
            alignment: Alignment::default(),
            armor_class: 10,
            damage: 0,
            strength: 10,
            dexterity: 10,
            constitution: 10,
            wisdom: 10,
            intelligence: 10,
            charisma: 10,
            experience_points: 0,
        }
    }
}