#[derive(Debug,PartialEq, Eq, Clone, Copy)]
pub enum Class {
    Fighter,
    Rogue,
    Monk,
    Commoner,
}

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
    pub class: Class,
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
    pub fn new(class: Class) -> Self {
        Self {
            name: String::new(),
            class: class,
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

    pub fn max_hit_points(&self) -> u32 {
        let hit_points_per_level = match self.class {
            Class::Fighter => 10,
            Class::Monk => 6,
            _ => 5,
        };
        
        10 + ((self.level() as i32 - 1) * hit_points_per_level) as u32
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_defaults_to_10_ac_5_hp() {
        let character = Character::new(Class::Commoner);

        assert_eq!(10, character.armor_class);
        assert_eq!(0, character.damage);
    }

    #[test]
    fn a_player_is_dead_if_hitpoints_are_zero() {
        let mut dead_player = Character::new(Class::Commoner);

        dead_player.damage = 10;

        assert!(dead_player.is_dead());
    }

    #[test]
    fn a_character_can_have_a_level() {
        let mut character = Character::new(Class::Commoner);

        assert_eq!(1, character.level());

        character.experience_points = 1000;

        assert_eq!(2, character.level());

        character.experience_points = 2000;

        assert_eq!(3, character.level());
    }

    #[test]
    fn a_character_has_increased_max_hit_points_based_on_level() {
        let mut character = Character::new(Class::Commoner);
        
        assert_eq!(10, character.max_hit_points());

        character.experience_points = 1000;
        assert_eq!(15, character.max_hit_points());

        character.experience_points = 2000;
        assert_eq!(20, character.max_hit_points());
    }


    #[test]
    fn a_character_has_plus_one_per_level_on_every_even_dice_roll_0_modifier() {
        let attacker = Character::new(Class::Commoner);
        let attackee = Character::new(Class::Commoner);
        let dice_roll: u32 = 15;

        assert_eq!(1, attacker.level());

        let attack_command = attacker.attack(&attackee, dice_roll);

        assert_eq!(0, attack_command.level_modifier);
    }

    #[test]
    fn a_character_has_plus_one_per_level_on_every_even_dice_roll_1_modifier() {
        let mut attacker = Character::new(Class::Commoner);
        attacker.experience_points = 1000;
        let attackee = Character::new(Class::Commoner);
        let dice_roll: u32 = 15;

        assert_eq!(2, attacker.level());

        let attack_command = attacker.attack(&attackee, dice_roll);

        assert_eq!(1, attack_command.level_modifier);
    }

    #[test]
    fn a_character_has_plus_one_per_level_on_every_even_dice_roll_2_modifier() {
        let mut attacker = Character::new(Class::Commoner);
        attacker.experience_points = 3000;
        let attackee = Character::new(Class::Commoner);
        let dice_roll: u32 = 15;

        assert_eq!(4, attacker.level());

        let attack_command = attacker.attack(&attackee, dice_roll);

        assert_eq!(2, attack_command.level_modifier);
    }

    #[test]
    fn as_a_fighter_i_have_10_hp_per_level_instead_of_five() {
        let mut character = Character::new(Class::Fighter);
        character.experience_points = 3000;

        assert_eq!(40, character.max_hit_points());
    }

    #[test]
    fn a_war_monk_has_six_hitpoints_per_level() {
        let mut monk = Character::new(Class::Monk);

        assert_eq!(10, monk.max_hit_points());

        monk.experience_points = 1000;
        assert_eq!(16, monk.max_hit_points());

        monk.experience_points = 2000;
        assert_eq!(22, monk.max_hit_points());
    }
}
