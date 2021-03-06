use std::cmp;

use character::{Character, Class, Alignment};

pub fn resolve_combat(command: &AttackCommand, attacker: &mut Character, attackee: &mut Character) {
    attackee.damage += command.damage().unwrap_or(0) as u32;
    attacker.experience_points += command.experience_points();
}

pub type DiceRollModifier = i32;

#[derive(Debug, PartialEq, Eq)]
pub struct AttackCommand {
    pub dice_roll: u32,
    pub attack_modifier: DiceRollModifier,
    pub level_modifier: DiceRollModifier,
    pub defense_dexterity_modifier: DiceRollModifier,
    pub constitution_modifier: DiceRollModifier,
    pub defense_wisdom_modifier: DiceRollModifier,
    pub armor_class: i32,
    pub critical_hit_multiplier: i32,
    pub minimum_damage: i32,
    pub alignment_damage_modifier: DiceRollModifier,
}

impl AttackCommand {
    pub fn succeeds(&self) -> bool {
        (self.dice_roll as i32 + self.attack_modifier + self.level_modifier) >=
            (self.defense_dexterity_modifier + self.armor_class + self.defense_wisdom_modifier)
    }

    pub fn is_critical(&self) -> bool {
        self.dice_roll == 20
    }

    pub fn damage(&self) -> Option<i32> {
        let additional_dmg = self.attack_modifier + self.alignment_damage_modifier;
        if !self.succeeds() {
            None
        } else if self.is_critical() {
            Some(cmp::max(
                (self.critical_hit_multiplier * additional_dmg + 1),
                self.minimum_damage,
            ))
        } else {
            Some(cmp::max((1 + additional_dmg), self.minimum_damage))
        }
    }

    pub fn experience_points(&self) -> u64 {
        if self.succeeds() { 10 } else { 0 }
    }
}

impl Character {
    pub fn attack(&self, attackee: &Character, dice_roll: u32) -> AttackCommand {
        let attackee_defense_dexterity_modifier = match self.class {
            Class::Rogue => cmp::min(attackee.dexterity_modifier(), 0),
            _ => attackee.dexterity_modifier(),
        };

        let attack_modifier = match self.class {
            Class::Rogue => self.dexterity_modifier(),
            _ => self.strength_modifier(),
        };

        let critical_hit_multiplier = match self.class {
            Class::Rogue => 3,
            Class::Paladin if attackee.alignment == Alignment::Evil => 3,
            _ => 2,
        };

        let minimum_damage = match self.class {
            Class::Monk => 3,
            _ => 1,
        };

        let defense_wisdom_modifier = match attackee.class {
            Class::Monk => cmp::max(attackee.wisdom_modifier(), 0),
            _ => 0,
        };

        let alignment_damage_modifier =
            if self.class == Class::Paladin && attackee.alignment == Alignment::Evil {
                2
            } else {
                0
            };

        AttackCommand {
            dice_roll,
            level_modifier: self.level_modifier(),
            attack_modifier: attack_modifier,
            defense_dexterity_modifier: attackee_defense_dexterity_modifier,
            constitution_modifier: attackee.constitution_modifier(),
            defense_wisdom_modifier,
            armor_class: attackee.armor_class,
            critical_hit_multiplier,
            minimum_damage,
            alignment_damage_modifier,
        }
    }

    fn level_modifier(&self) -> DiceRollModifier {
        match self.class {
            Class::Fighter | Class::Paladin => self.level() as DiceRollModifier,
            Class::Monk => (self.level() * 2 / 3) as DiceRollModifier,
            _ => (self.level() / 2) as DiceRollModifier,
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
        let mut attacker = Character::new(Class::Commoner);
        attacker.strength = 15;
        let attackee = Character::new(Class::Commoner);
        let dice_roll: u32 = 20;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(5), attack_command.damage());
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

        resolve_combat(
            &attacker.attack(&attackee, dice_roll),
            &mut attacker,
            &mut attackee,
        );
        assert_eq!(10, attacker.experience_points);
    }

    #[test]
    fn the_level_modifier_is_applied_to_attack_commands() {
        let attack_command = AttackCommand {
            level_modifier: 1,
            dice_roll: 1,
            attack_modifier: 0,
            defense_dexterity_modifier: 0,
            constitution_modifier: 0,
            armor_class: 2,
            defense_wisdom_modifier: 0,
            critical_hit_multiplier: 2,
            minimum_damage: 1,
            alignment_damage_modifier: 0,
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

    #[test]
    fn as_a_rogue_a_critical_hit_does_triple_damage() {
        let mut attacker = Character::new(Class::Rogue);
        attacker.dexterity = 12;
        let attackee = Character::new(Class::Commoner);
        let dice_roll: u32 = 20;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(4), attack_command.damage());
    }

    #[test]
    fn as_a_rogue_ignores_a_positive_defense_dexterity_modifier() {
        // TODO: directly setup the attack command
        let attacker = Character::new(Class::Rogue);
        let mut attackee = Character::new(Class::Commoner);
        attackee.armor_class = 10;
        attackee.dexterity = 12;
        let dice_roll: u32 = 10;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(true, attack_command.succeeds());
    }

    #[test]
    fn as_a_rogue_does_not_ignore_a_negative_defense_dexterity_modifier() {
        // TODO: directly setup the attack command
        let attacker = Character::new(Class::Rogue);
        let mut attackee = Character::new(Class::Commoner);
        attackee.armor_class = 11;
        attackee.dexterity = 8;
        let dice_roll: u32 = 10;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(true, attack_command.succeeds());
    }

    #[test]
    fn as_a_rogue_adds_dexterity_modifier_to_attacks() {
        // TODO: directly setup the attack command
        let mut attacker = Character::new(Class::Rogue);
        let attackee = Character::new(Class::Commoner);
        attacker.dexterity = 12;
        let dice_roll: u32 = 9;


        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(true, attack_command.succeeds());
    }

    #[test]
    fn a_monk_does_at_least_three_points_of_damage_on_an_attack() {
        let mut attacker = Character::new(Class::Monk);
        attacker.strength = 6;
        let attackee = Character::new(Class::Commoner);
        let dice_roll = 20;

        let attack_command = attacker.attack(&attackee, dice_roll);
        assert_eq!(Some(3), attack_command.damage());
    }

    #[test]
    fn a_monk_adds_a_positive_defense_wisdom_modifier() {
        let attacker = Character::new(Class::Commoner);
        let mut attackee = Character::new(Class::Monk);
        attackee.wisdom = 16;
        let dice_roll = 12;

        let attack_command = attacker.attack(&attackee, dice_roll);

        assert_eq!(attack_command.defense_wisdom_modifier, 3);
    }

    #[test]
    fn a_monk_does_not_add_a_negative_defense_wisdom_modifier() {
        let attacker = Character::new(Class::Commoner);
        let mut attackee = Character::new(Class::Monk);
        attackee.wisdom = 6;
        let dice_roll = 10;

        let attack_command = attacker.attack(&attackee, dice_roll);

        assert_eq!(attack_command.defense_wisdom_modifier, 0);
    }

    #[test]
    fn anyone_other_than_a_monk_has_wisdom_modifier() {
        let attacker = Character::new(Class::Commoner);
        let mut attackee = Character::new(Class::Commoner);
        attackee.wisdom = 16;
        let dice_roll = 8;

        let attack_command = attacker.attack(&attackee, dice_roll);

        assert_eq!(attack_command.defense_wisdom_modifier, 0);
    }

    #[test]
    fn defense_wisdom_modifier_is_applied_when_calculating_success() {
        let attack_command = AttackCommand {
            level_modifier: 0,
            dice_roll: 10,
            attack_modifier: 0,
            defense_dexterity_modifier: 0,
            constitution_modifier: 0,
            armor_class: 0,
            defense_wisdom_modifier: 11,
            critical_hit_multiplier: 0,
            minimum_damage: 1,
            alignment_damage_modifier: 0,
        };

        assert_eq!(attack_command.succeeds(), false);
    }

    impl Character {
        fn set_level(&mut self, level: u64) {
            if level == 0 {
                self.experience_points = 0;
            } else {
                self.experience_points = (level - 1) * 1000;
            }
        }
    }

    #[test]
    fn as_a_monk_an_attack_roll_is_increased_by_1_every_2nd_and_3rd_level() {
        let mut attacker = Character::new(Class::Monk);
        attacker.set_level(1);
        let attackee = Character::new(Class::Commoner);

        let attack_command = attacker.attack(&attackee, 10);
        assert_eq!(attack_command.level_modifier, 0);

        attacker.set_level(2);
        let attack_command = attacker.attack(&attackee, 10);
        assert_eq!(attack_command.level_modifier, 1);

        attacker.set_level(3);
        let attack_command = attacker.attack(&attackee, 10);
        assert_eq!(attack_command.level_modifier, 2);

        attacker.set_level(4);
        let attack_command = attacker.attack(&attackee, 10);
        assert_eq!(attack_command.level_modifier, 2);

        attacker.set_level(5);
        let attack_command = attacker.attack(&attackee, 10);
        assert_eq!(attack_command.level_modifier, 3);

        attacker.set_level(6);
        let attack_command = attacker.attack(&attackee, 10);
        assert_eq!(attack_command.level_modifier, 4);
    }

    #[test]
    fn a_paladin_does_2_extra_damage_to_evil_characters() {
        let attacker = Character::new(Class::Paladin);
        let mut attackee = Character::new(Class::Commoner);
        attackee.alignment = Alignment::Evil;

        let attack_command = attacker.attack(&attackee, 10);
        assert_eq!(attack_command.alignment_damage_modifier, 2);
        assert_eq!(attack_command.damage(), Some(3));

        let mut attackee = Character::new(Class::Commoner);
        attackee.alignment = Alignment::Good;

        let attack_command = attacker.attack(&attackee, 10);
        assert_eq!(attack_command.alignment_damage_modifier, 0);
        assert_eq!(attack_command.damage(), Some(1));
    }

    #[test]
    fn a_paladin_does_6_extra_damage_on_critical_hit_to_evil_characters() {
        let attacker = Character::new(Class::Paladin);
        let mut attackee = Character::new(Class::Commoner);
        attackee.alignment = Alignment::Evil;

        let attack_command = attacker.attack(&attackee, 20);
        assert_eq!(attack_command.alignment_damage_modifier, 2);
        assert_eq!(attack_command.critical_hit_multiplier, 3);
        assert_eq!(attack_command.damage(), Some(7));

        let mut attackee = Character::new(Class::Commoner);
        attackee.alignment = Alignment::Good;

        let attack_command = attacker.attack(&attackee, 20);
        assert_eq!(attack_command.alignment_damage_modifier, 0);
        assert_eq!(attack_command.critical_hit_multiplier, 2);
        assert_eq!(attack_command.damage(), Some(1));
    }

    #[test]
    fn as_a_paladin_my_attack_roll_is_increased_by_one_for_every_level() {
        let mut attacker = Character::new(Class::Paladin);
        attacker.experience_points = 3000;
        let attackee = Character::new(Class::Commoner);
        let dice_roll: u32 = 15;

        assert_eq!(4, attacker.level());

        let attack_command = attacker.attack(&attackee, dice_roll);

        assert_eq!(4, attack_command.level_modifier);
    }
}
