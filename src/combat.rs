use std::cmp;

use character::{Character, Class};

pub fn resolve_combat(command: &AttackCommand, attacker: &mut Character, attackee: &mut Character) {
    attackee.damage += command.damage().unwrap_or(0) as u32;
    attacker.experience_points += command.experience_points();
}

pub type DiceRollModifier = i32;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct AttackCommand {
    pub dice_roll: u32,
    pub strength_modifier: DiceRollModifier,
    pub level_modifier: DiceRollModifier,
    pub dexterity_modifier: DiceRollModifier,
    pub constitution_modifier: DiceRollModifier,
    pub armor_class: i32,
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

impl Character {
    pub fn attack(&self, attackee: &Character, dice_roll: u32) -> AttackCommand {
        AttackCommand {
            dice_roll,
            level_modifier: self.level_modifier(),
            strength_modifier: Self::modifier_score(self.strength),
            dexterity_modifier: Self::modifier_score(attackee.dexterity),
            constitution_modifier: Self::modifier_score(attackee.constitution),
            armor_class: attackee.armor_class,
        }
    }

    fn level_modifier(&self) -> DiceRollModifier {
        match self.class {
            Class::Fighter => self.level() as DiceRollModifier,
            _ => (self.level() / 2) as DiceRollModifier
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_player_can_successfully_attack_another_player() {
        let attacker = Character::new(Class::Commoner);
        let attackee = Character::new(Class::Commoner);
        let dice_roll: u32 = 10;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(1), attack_command.damage());
    }

    #[test]
    fn a_player_can_unsuccessfully_attack_another_player() {
        let attacker = Character::new(Class::Commoner);
        let attackee = Character::new(Class::Commoner);
        let dice_roll: u32 = 9;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(None, attack_command.damage());
    }

    #[test]
    fn a_player_can_critically_hit_in_a_successful_attack() {
        let attacker = Character::new(Class::Commoner);
        let attackee = Character::new(Class::Commoner);
        let dice_roll: u32 = 20;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(1), attack_command.damage());
    }

    #[test]
    fn a_weak_character_can_still_do_damage() {
        let mut attacker = Character::new(Class::Commoner);
        attacker.strength = 6;
        let attackee = Character::new(Class::Commoner);
        let dice_roll = 12;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(1), attack_command.damage());
    }

    #[test]
    fn a_swole_character_does_more_damage() {
        let mut attacker = Character::new(Class::Commoner);
        attacker.strength = 15;
        let attackee = Character::new(Class::Commoner);
        let dice_roll = 12;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(3), attack_command.damage());
    }

    #[test]
    fn a_weak_character_does_modest_damage_in_a_critical_hit() {
        let mut attacker = Character::new(Class::Commoner);
        attacker.strength = 6;
        let attackee = Character::new(Class::Commoner);
        let dice_roll = 20;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(1), attack_command.damage());
    }

    #[test]
    fn a_players_damage_is_increased_when_damage_is_done() {
        let mut attacker = Character::new(Class::Commoner);
        let mut attackee = Character::new(Class::Commoner);
        let dice_roll: u32 = 15;
        let expected_damage = attackee.damage + 1;

        let attack_command = attacker.attack(&attackee, dice_roll);
        resolve_combat(&attack_command, &mut attacker, &mut attackee);
        assert_eq!(expected_damage, attackee.damage);
    }

    #[test]
    fn a_character_gains_experience_for_a_successful_attack() {
        let mut attacker = Character::new(Class::Commoner);
        let mut attackee = Character::new(Class::Commoner);
        let dice_roll: u32 = 15;

        assert_eq!(0, attacker.experience_points);

        resolve_combat(&attacker.attack(&attackee, dice_roll), &mut attacker, &mut attackee);
        assert_eq!(10, attacker.experience_points);
    }

    #[test]
    fn the_level_modifier_is_applied_to_attack_commands() {
        let attack_command = AttackCommand {
            level_modifier: 1,
            dice_roll: 1,
            strength_modifier: 0,
            dexterity_modifier: 0,
            constitution_modifier: 0,
            armor_class: 2,
        };

        assert_eq!(true, attack_command.succeeds());
    }

    #[test]
    fn as_a_fighter_my_attack_roll_is_increased_by_one_for_every_level() {
        let mut attacker = Character::new(Class::Fighter);
        attacker.experience_points = 3000;
        let attackee = Character::new(Class::Commoner);
        let dice_roll: u32 = 15;

        assert_eq!(4, attacker.level());

        let attack_command = attacker.attack(&attackee, dice_roll);

        assert_eq!(4, attack_command.level_modifier);
    }
}
