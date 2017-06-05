#![allow(dead_code)]
use std::cmp;

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

pub struct Character {
    name: String,
    alignment: Alignment,
    armor_class: i32,
    damage: u32,
    strength: u32,
    dexterity: u32,
    constitution: u32,
    wisdom: u32,
    intelligence: u32,
    charisma: u32,
    experience_points: u64,
}

impl Character {
    pub fn max_hit_points(&self) -> u32 {
        10 + ((self.level() as i32 - 1) * 5) as u32
    }

    pub fn attack(&self, attackee: &Character, dice_roll: u32) -> AttackCommand {
        AttackCommand {
            dice_roll,
            level_modifier: (self.level() / 2) as DiceRollModifier,
            strength_modifier: Self::modifier_score(self.strength),
            dexterity_modifier: Self::modifier_score(attackee.dexterity),
            constitution_modifier: Self::modifier_score(attackee.constitution),
            armor_class: attackee.armor_class,
        }
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

pub fn resolve_combat(command: &AttackCommand, attacker: &mut Character, attackee: &mut Character) {
    attackee.damage += command.damage().unwrap_or(0) as u32;
    attacker.experience_points += command.experience_points();
}

type DiceRollModifier = i32;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct AttackCommand {
    dice_roll: u32,
    strength_modifier: DiceRollModifier,
    level_modifier: DiceRollModifier,
    dexterity_modifier: DiceRollModifier,
    constitution_modifier: DiceRollModifier,
    armor_class: i32,
}

impl AttackCommand {
    pub fn succeeds(&self) -> bool {
        (self.dice_roll as i32 + self.strength_modifier + self.level_modifier)
            >= (self.dexterity_modifier + self.armor_class)
    }

    pub fn is_critical(&self) -> bool {
        self.dice_roll == 20
    }

    pub fn damage(&self) -> Option<i32> {
        if !self.succeeds() {
            None
        } else if self.is_critical() {
            Some(cmp::max((2 * self.strength_modifier + 1), 1))
        } else {
            Some(cmp::max((1 + self.strength_modifier), 1))
        }
    }

    pub fn experience_points(&self) -> u64 {
        if self.succeeds() { 10 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_defaults_to_10_ac_5_hp() {
        let character = Character::default();

        assert_eq!(10, character.armor_class);
        assert_eq!(0, character.damage);
    }

    #[test]
    fn a_player_can_successfully_attack_another_player() {
        let attacker = Character::default();
        let attackee = Character::default();
        let dice_roll: u32 = 10;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(1), attack_command.damage());
    }

    #[test]
    fn a_player_can_unsuccessfully_attack_another_player() {
        let attacker = Character::default();
        let attackee = Character::default();
        let dice_roll: u32 = 9;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(None, attack_command.damage());
    }

    #[test]
    fn a_player_can_critically_hit_in_a_successful_attack() {
        let attacker = Character::default();
        let attackee = Character::default();
        let dice_roll: u32 = 20;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(1), attack_command.damage());
    }

    #[test]
    fn a_weak_character_can_still_do_damage() {
        let mut attacker = Character::default();
        attacker.strength = 6;
        let attackee = Character::default();
        let dice_roll = 12;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(1), attack_command.damage());
    }

    #[test]
    fn a_swole_character_does_more_damage() {
        let mut attacker = Character::default();
        attacker.strength = 15;
        let attackee = Character::default();
        let dice_roll = 12;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(3), attack_command.damage());
    }

    #[test]
    fn a_weak_character_does_modest_damage_in_a_critical_hit() {
        let mut attacker = Character::default();
        attacker.strength = 6;
        let attackee = Character::default();
        let dice_roll = 20;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(1), attack_command.damage());
    }

    #[test]
    fn a_players_damage_is_increased_when_damage_is_done() {
        let mut attacker = Character::default();
        let mut attackee = Character::default();
        let dice_roll: u32 = 15;
        let expected_damage = attackee.damage + 1;

        let attack_command = attacker.attack(&attackee, dice_roll);
        resolve_combat(&attack_command, &mut attacker, &mut attackee);
        assert_eq!(expected_damage, attackee.damage);
    }

    #[test]
    fn a_player_is_dead_if_hitpoints_are_zero() {
        let mut dead_player = Character::default();

        dead_player.damage = 10;

        assert!(dead_player.is_dead());
    }

    #[test]
    fn a_character_gains_experience_for_a_successful_attack() {
        let mut attacker = Character::default();
        let mut attackee = Character::default();
        let dice_roll: u32 = 15;

        assert_eq!(0, attacker.experience_points);

        resolve_combat(&attacker.attack(&attackee, dice_roll), &mut attacker, &mut attackee);
        assert_eq!(10, attacker.experience_points);
    }

    #[test]
    fn a_character_can_have_a_level() {
        let mut character = Character::default();

        assert_eq!(1, character.level());

        character.experience_points = 1000;

        assert_eq!(2, character.level());

        character.experience_points = 2000;

        assert_eq!(3, character.level());
    }

    #[test]
    fn a_character_has_increased_max_hit_points_based_on_level() {
        let mut character = Character::default();
        
        assert_eq!(10, character.max_hit_points());

        character.experience_points = 1000;
        assert_eq!(15, character.max_hit_points());

        character.experience_points = 2000;
        assert_eq!(20, character.max_hit_points());
    }


    #[test]
    fn a_character_has_plus_one_per_level_on_every_even_dice_roll_0_modifier() {
        let attacker = Character::default();
        let attackee = Character::default();
        let dice_roll: u32 = 15;

        assert_eq!(1, attacker.level());

        let attack_command = attacker.attack(&attackee, dice_roll);

        assert_eq!(0, attack_command.level_modifier);
    }

    #[test]
    fn a_character_has_plus_one_per_level_on_every_even_dice_roll_1_modifier() {
        let mut attacker = Character::default();
        attacker.experience_points = 1000;
        let attackee = Character::default();
        let dice_roll: u32 = 15;

        assert_eq!(2, attacker.level());

        let attack_command = attacker.attack(&attackee, dice_roll);

        assert_eq!(1, attack_command.level_modifier);
    }

    #[test]
    fn a_character_has_plus_one_per_level_on_every_even_dice_roll_2_modifier() {
        let mut attacker = Character::default();
        attacker.experience_points = 3000;
        let attackee = Character::default();
        let dice_roll: u32 = 15;

        assert_eq!(4, attacker.level());

        let attack_command = attacker.attack(&attackee, dice_roll);

        assert_eq!(2, attack_command.level_modifier);
    }

    #[test]
    fn the_level_modifier_is_applied_to_attack_commands() {
        let attack_command = AttackCommand{
            level_modifier: 1,
            dice_roll: 1,
            strength_modifier: 0,
            dexterity_modifier: 0,
            constitution_modifier: 0,
            armor_class: 2,
        };

        assert_eq!(true, attack_command.succeeds());
    }
}
