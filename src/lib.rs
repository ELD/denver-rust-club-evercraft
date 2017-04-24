#![allow(dead_code)]

enum Alignment {
    Good,
    Neutral,
    Evil,
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Neutral
    }
}

struct Character {
    name: String,
    alignment: Alignment,
    armor_class: i32,
    hit_points: i32,
}

impl Character {
    pub fn attack(&self, attackee: &Character, dice_roll: u32) -> Option<u32> {
        if dice_roll as i32 >= attackee.armor_class {
            if dice_roll == 20 {
                Some(2)
            } else {
                Some(1)
            }
        } else {
            None
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: String::new(),
            alignment: Alignment::default(),
            armor_class: 10,
            hit_points: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_defaults_to_10_ac_5_hp() {
        let character = Character::default();

        assert_eq!(10, character.armor_class);
        assert_eq!(5, character.hit_points);
    }

    #[test]
    fn a_player_can_successfully_attack_another_player() {
        let attacker = Character::default();
        let attackee = Character::default();
        let dice_roll: u32 = 10;

        let attack_result = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(1), attack_result);
    }

    #[test]
    fn a_player_can_unsuccessfully_attack_another_player() {
        let attacker = Character::default();
        let attackee = Character::default();
        let dice_roll: u32 = 9;

        let attack_result = attacker.attack(&attackee, dice_roll);
        assert_eq!(None, attack_result);
    }

    #[test]
    fn a_player_can_critically_hit_in_a_successful_attack() {
        let attacker = Character::default();
        let attackee = Character::default();
        let dice_roll: u32 = 20;

        let attack_result = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(2), attack_result);
    }
}
