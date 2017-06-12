use std::cmp;

use character::Character;

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
            level_modifier: (self.level() / 2) as DiceRollModifier,
            strength_modifier: Self::modifier_score(self.strength),
            dexterity_modifier: Self::modifier_score(attackee.dexterity),
            constitution_modifier: Self::modifier_score(attackee.constitution),
            armor_class: attackee.armor_class,
        }
    }
}
