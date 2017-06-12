#![allow(dead_code)]
mod character;
mod combat;

#[cfg(test)]
mod tests {
    use super::character::Character;
    use super::combat::*;

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
    fn as_a_player_i_can_be_a_fighter() {
        let mut attacker = Character::fighter();
        attacker.experience_points = 3000;
        let attackee = Character::default();
        let dice_roll: u32 = 15;

        assert_eq!(4, attacker.level());

        let attack_command = attacker.attack(&attackee, dice_roll);

        assert_eq!(4, attack_command.level_modifier);
    }
}
