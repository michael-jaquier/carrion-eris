use crate::commands::character_trait;
use crate::enemies::{Alignment, Attack, AttackType, DamageType, Enemy, EnemyState, Mob};
use crate::{Attribute, CarrionError, Dice, Die};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serenity::Error::Client;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt::{write, Display, Formatter};
use std::ops::{Div, Range, Sub};
use tracing::info;
use tracing::log::debug;
use tracing_subscriber::fmt::{format, init};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, Eq, Hash)]
pub enum CharacterTraits {
    Robust,
    Nimble,
    Genius,
    Lucky,
    FolkHero,
    Charismatic,
    Strong,
    Hermit,
    Addict,
    Cursed,
    Unlucky,
    Righteous,
    Greedy,
    Keen,
}

impl Display for CharacterTraits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CharacterTraits::Robust => write!(f, "Robust"),
            CharacterTraits::Nimble => write!(f, "Nimble"),
            CharacterTraits::Genius => write!(f, "Genius"),
            CharacterTraits::Lucky => write!(f, "Lucky"),
            CharacterTraits::FolkHero => write!(f, "FolkHero"),
            CharacterTraits::Charismatic => write!(f, "Charismatic"),
            CharacterTraits::Strong => write!(f, "Strong"),
            CharacterTraits::Hermit => write!(f, "Hermit"),
            CharacterTraits::Addict => write!(f, "Addict"),
            CharacterTraits::Cursed => write!(f, "Cursed"),
            CharacterTraits::Unlucky => write!(f, "Unlucky"),
            CharacterTraits::Righteous => write!(f, "Righteous"),
            CharacterTraits::Greedy => write!(f, "Greedy"),
            CharacterTraits::Keen => write!(f, "Keen"),
        }
    }
}

impl CharacterTraits {
    pub fn attribute_mutator(&self, character_attributes: &mut CharacterAttributes) {
        match self {
            CharacterTraits::Robust => {
                character_attributes.strength.plus(1);
                character_attributes.constitution.plus(3);
            }
            CharacterTraits::Nimble => {
                character_attributes.dexterity.plus(3);
            }
            CharacterTraits::Genius => {
                character_attributes.intelligence.plus(3);
            }
            CharacterTraits::Lucky => {}
            CharacterTraits::FolkHero => {
                character_attributes.charisma.plus(2);
                character_attributes.constitution.plus(1);
            }
            CharacterTraits::Charismatic => {
                character_attributes.charisma.plus(3);
            }
            CharacterTraits::Strong => {
                character_attributes.strength.plus(3);
            }
            CharacterTraits::Hermit => {
                character_attributes.wisdom.plus(3);
            }
            CharacterTraits::Addict => {
                character_attributes.constitution.minus(3);
                character_attributes.intelligence.plus(5);
            }
            CharacterTraits::Cursed => {}
            CharacterTraits::Unlucky => {}
            CharacterTraits::Righteous => {}
            CharacterTraits::Greedy => {}
            CharacterTraits::Keen => {}
        }
    }

    pub fn valid_traits() -> String {
        let all = vec![
            CharacterTraits::Robust,
            CharacterTraits::Nimble,
            CharacterTraits::Genius,
            CharacterTraits::Lucky,
            CharacterTraits::FolkHero,
            CharacterTraits::Charismatic,
            CharacterTraits::Strong,
            CharacterTraits::Hermit,
            CharacterTraits::Addict,
            CharacterTraits::Cursed,
            CharacterTraits::Unlucky,
            CharacterTraits::Righteous,
            CharacterTraits::Greedy,
            CharacterTraits::Keen,
        ];
        all.iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join("\n ")
    }
}

impl TryFrom<String> for CharacterTraits {
    type Error = CarrionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "robust" => Ok(CharacterTraits::Robust),
            "nimble" => Ok(CharacterTraits::Nimble),
            "genius" => Ok(CharacterTraits::Genius),
            "lucky" => Ok(CharacterTraits::Lucky),
            "folkhero" => Ok(CharacterTraits::FolkHero),
            "charismatic" => Ok(CharacterTraits::Charismatic),
            "strong" => Ok(CharacterTraits::Strong),
            "hermit" => Ok(CharacterTraits::Hermit),
            "addict" => Ok(CharacterTraits::Addict),
            "cursed" => Ok(CharacterTraits::Cursed),
            "unlucky" => Ok(CharacterTraits::Unlucky),
            "righteous" => Ok(CharacterTraits::Righteous),
            "greedy" => Ok(CharacterTraits::Greedy),
            "keen" => Ok(CharacterTraits::Keen),
            _ => Err(CarrionError::ParseError(
                "Unable to parse character trait".to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum Classes {
    Warrior,
    Wizard,
    Sorcerer,
}

impl Classes {
    pub fn valid_classes() -> String {
        let all = vec![Classes::Warrior, Classes::Wizard, Classes::Sorcerer];
        all.iter()
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>()
            .join("\n ")
    }
    pub fn action(&self) -> PlayerAction {
        match self {
            Classes::Warrior => PlayerAction::Slash,
            Classes::Wizard => PlayerAction::MagicMissile,
            Classes::Sorcerer => PlayerAction::FireBall,
        }
    }

    pub fn hp_gain(&self, level: u32) -> u32 {
        match self {
            Classes::Warrior => 100 + (level * 25),
            Classes::Wizard => 75 + (level * 10),
            Classes::Sorcerer => 75 + (level * 15),
        }
    }
}

impl TryFrom<String> for Classes {
    type Error = CarrionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "warrior" => Ok(Classes::Warrior),
            "wizard" => Ok(Classes::Wizard),
            "sorcerer" => Ok(Classes::Sorcerer),
            _ => Err(CarrionError::ParseError(
                "Unable to parse class".to_string(),
            )),
        }
    }
}

impl Display for Classes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Classes::Warrior => write!(f, "Warrior 🪖"),
            Classes::Wizard => write!(f, "Wizard 🧙"),
            Classes::Sorcerer => write!(f, "Sorcerer 🧙"),
        }
    }
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AttackModifier {
    pub(crate) flat: u32,
    pub(crate) multiplier: u32,
    pub(crate) crit_chance: Dice,
    pub(crate) crit_multiplier: f64,
}

impl AttackModifier {
    pub fn new(flat: u32, multiplier: u32, crit_chance: Dice, crit_multiplier: f64) -> Self {
        Self {
            flat,
            multiplier,
            crit_chance,
            crit_multiplier,
        }
    }
}

impl Default for AttackModifier {
    fn default() -> Self {
        Self {
            flat: 1,
            multiplier: 1,
            crit_chance: Default::default(),
            crit_multiplier: 1.3,
        }
    }
}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AttackModifiers {
    pub(crate) magic: AttackModifier,
    pub(crate) physical: AttackModifier,
}

type PhysicalMagical = ((u32, bool), (u32, bool));
impl AttackModifiers {
    pub fn builder(player: &Character, enemy: &Enemy) -> AttackModifiers {
        AttackModifiers::default()
            .apply_level_scaling(player, &player.class.action())
            .apply_skill(&player.class.action())
            .apply_attributes(enemy, player, &player.class.action())
            .apply_traits(player, enemy)
            .apply_vulnerability(enemy, &player.class.action())
            .clone()
    }

    fn apply_level_scaling(
        &mut self,
        player: &Character,
        action: &PlayerAction,
    ) -> &mut AttackModifiers {
        match action.action_type() {
            ActionType::Physical => {
                self.physical.flat += player.level * 5;
            }
            ActionType::Magical => {
                self.magic.flat += player.level * 5;
            }
            ActionType::Mixed => {
                self.magic.flat += player.level * 5;
                self.physical.flat += player.level * 5;
            }
        }
        self
    }

    fn apply_skill(&mut self, action: &PlayerAction) -> &mut AttackModifiers {
        let base = action.action_base_damage();
        match action.action_type() {
            ActionType::Physical => {
                self.physical.flat += base.roll_all();
            }
            ActionType::Magical => {
                self.magic.flat += base.roll_all();
            }
            ActionType::Mixed => {
                self.magic.flat += base.roll_all().div(2);
                self.physical.flat += base.roll_all().div(2);
            }
        }
        self
    }

    fn apply_attributes(
        &mut self,
        enemy: &Enemy,
        player: &Character,
        action: &PlayerAction,
    ) -> &mut AttackModifiers {
        let attribute = action.action_attribute_modifiers();
        let modifier = match attribute {
            Attribute::Strength(_) => player
                .attributes
                .strength
                .absolute_difference(&enemy.attributes.strength),
            Attribute::Intelligence(_) => player
                .attributes
                .intelligence
                .absolute_difference(&enemy.attributes.intelligence),
            Attribute::Dexterity(_) => player
                .attributes
                .dexterity
                .absolute_difference(&enemy.attributes.dexterity),
            Attribute::Constitution(_) => player
                .attributes
                .constitution
                .absolute_difference(&enemy.attributes.constitution),
            Attribute::Wisdom(_) => player
                .attributes
                .wisdom
                .absolute_difference(&enemy.attributes.wisdom),
            Attribute::Charisma(_) => player
                .attributes
                .charisma
                .absolute_difference(&enemy.attributes.charisma),
        };

        match action.action_type() {
            ActionType::Physical => {
                self.physical.flat += (self.physical.flat as i32 + modifier).max(0) as u32;
            }
            ActionType::Magical => {
                self.magic.flat += (self.magic.flat as i32 + modifier).max(0) as u32;
            }
            ActionType::Mixed => {
                self.magic.flat += (self.magic.flat as i32 + modifier).max(0).div(2) as u32;
                self.physical.flat += (self.physical.flat as i32 + modifier).max(0).div(2) as u32;
            }
        }
        self
    }

    fn apply_vulnerability(
        &mut self,
        enemy: &Enemy,
        action: &PlayerAction,
    ) -> &mut AttackModifiers {
        let vulnerability = enemy.kind.vulnerability();
        let action_element = action.action_element();

        if action_element.contains(&vulnerability) && action.action_type() == ActionType::Magical {
            self.magic.multiplier += 1;
            self.magic.flat += 5;
        }

        if action_element.contains(&vulnerability) && action.action_type() == ActionType::Physical {
            self.physical.multiplier += 1;
            self.physical.flat += 5;
        }
        self
    }
    pub fn new(magic: AttackModifier, physical: AttackModifier) -> Self {
        Self { magic, physical }
    }

    fn magical_crit(&self) -> bool {
        self.magic.crit_chance.success(None)
    }
    fn physical_crit(&self) -> bool {
        self.physical.crit_chance.success(None)
    }

    fn physical_range(&self) -> (u32, bool) {
        let mut rng = thread_rng();
        let crit = self.physical_crit();
        let physical_scaled = self.physical.flat * self.physical.multiplier;
        let physical_crit = if crit {
            (physical_scaled as f64 * self.physical.crit_multiplier) as u32
        } else {
            physical_scaled
        };
        assert!(physical_crit > 0, "Physical crit is {}", physical_crit);
        (rng.gen_range(physical_crit..physical_crit * 2), crit)
    }

    fn magical_range(&self) -> (u32, bool) {
        let mut rng = thread_rng();
        let crit = self.magical_crit();
        let magical_scaled = self.magic.flat * self.magic.multiplier;
        let magical_crit = if crit {
            (magical_scaled as f64 * self.magic.crit_multiplier) as u32
        } else {
            magical_scaled
        };
        assert!(magical_crit > 0, "Magical crit is {}", magical_crit);
        (rng.gen_range(magical_crit..magical_crit * 2), crit)
    }

    pub fn generate_damage_values(&self) -> PhysicalMagical {
        (self.physical_range(), self.magical_range())
    }
    fn apply_traits(&mut self, player: &Character, enemy: &Enemy) -> &mut AttackModifiers {
        for tr in player.traits.iter() {
            match tr {
                CharacterTraits::Robust => {}
                CharacterTraits::Nimble => {}
                CharacterTraits::Genius => {
                    self.magic.multiplier += 1;
                    self.magic.flat += 5;
                }
                CharacterTraits::Lucky => {
                    self.magic.crit_chance.advantage();
                    self.physical.crit_chance.advantage();
                }
                CharacterTraits::FolkHero => match enemy.kind.alignment() {
                    Alignment::ChaoticEvil => {
                        self.magic.flat += 5;
                        self.physical.flat += 5;
                    }
                    Alignment::ChaoticNeutral => {
                        self.magic.flat += 5;
                        self.physical.flat += 5;
                    }
                    _ => {}
                },
                CharacterTraits::Charismatic => {}
                CharacterTraits::Strong => {
                    self.physical.flat += 5;
                }
                CharacterTraits::Hermit => {}
                CharacterTraits::Addict => {
                    self.physical.flat -= (self.physical.flat as f64 * 0.5) as u32;
                }
                CharacterTraits::Cursed => {
                    self.magic.crit_chance.disadvantage();
                    self.magic.flat -= (self.magic.flat as f64 * 0.5) as u32;
                }
                CharacterTraits::Unlucky => {
                    self.magic.crit_chance.disadvantage();
                    self.physical.crit_chance.disadvantage();
                }
                CharacterTraits::Righteous => match enemy.kind.alignment() {
                    Alignment::LawfulEvil => {
                        self.magic.flat += (self.magic.flat as f64 * 1.5) as u32;
                        self.physical.flat += (self.magic.flat as f64 * 1.5) as u32
                    }
                    Alignment::NeutralEvil => {
                        self.magic.flat += (self.magic.flat as f64 * 1.5) as u32;
                        self.physical.flat += (self.magic.flat as f64 * 1.5) as u32;
                    }
                    Alignment::ChaoticEvil => {
                        self.magic.flat += (self.magic.flat as f64 * 1.5) as u32;
                        self.physical.flat += (self.magic.flat as f64 * 1.5) as u32;
                    }
                    _ => {}
                },
                CharacterTraits::Greedy => {}
                CharacterTraits::Keen => {
                    self.magic.crit_chance.stored_target = Some(18);
                    self.physical.crit_chance.stored_target = Some(18);
                }
            }
        }
        self
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, Eq, Hash)]
pub enum ActionType {
    Physical,
    Magical,
    Mixed,
}

impl Default for AttackModifiers {
    fn default() -> Self {
        Self {
            magic: AttackModifier::default(),
            physical: AttackModifier::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerAction {
    Slash,
    MagicMissile,
    FireBall,
}

impl Display for PlayerAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerAction::Slash => write!(f, "Slash"),
            PlayerAction::MagicMissile => write!(f, "Magic Missile"),
            PlayerAction::FireBall => write!(f, "Fire Ball"),
        }
    }
}

impl PlayerAction {
    pub fn damage(&self, modifiers: AttackModifiers) -> (u32, bool) {
        let (physical_damage, magical_damage) = modifiers.generate_damage_values();
        match self {
            PlayerAction::Slash => physical_damage,
            PlayerAction::MagicMissile => magical_damage,
            PlayerAction::FireBall => magical_damage,
        }
    }

    pub fn act(&self, player: &Character, enemy: &Enemy) -> Attack {
        let attack_modifiers = AttackModifiers::builder(player, enemy);
        let (damage, critical) = self.damage(attack_modifiers);
        let attack_type = match self {
            PlayerAction::Slash => AttackType::Physical(damage),
            PlayerAction::MagicMissile => AttackType::Magical(damage),
            PlayerAction::FireBall => AttackType::Magical(damage),
        };

        Attack {
            attack_type,
            damage_type: self.action_element(),
            critical,
        }
    }

    pub fn action_element(&self) -> Vec<DamageType> {
        match self {
            PlayerAction::Slash => {
                vec![DamageType::Physical]
            }
            PlayerAction::MagicMissile => {
                vec![DamageType::Arcane]
            }
            PlayerAction::FireBall => {
                vec![DamageType::Fire]
            }
        }
    }

    pub fn action_type(&self) -> ActionType {
        match self {
            PlayerAction::Slash => ActionType::Physical,
            PlayerAction::MagicMissile => ActionType::Magical,
            PlayerAction::FireBall => ActionType::Magical,
        }
    }

    pub fn action_base_damage(&self) -> Dice {
        match self {
            PlayerAction::Slash => Dice::new(Die::D20, 1, None),
            PlayerAction::MagicMissile => Dice::new(Die::D6, 3, Some(19)),
            PlayerAction::FireBall => Dice::new(Die::D12, 2, None),
        }
    }

    pub fn action_attribute_modifiers(&self) -> Attribute {
        match self {
            PlayerAction::Slash => Attribute::Strength(0),
            PlayerAction::MagicMissile => Attribute::Charisma(0),
            PlayerAction::FireBall => Attribute::Intelligence(0),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterAttributes {
    pub(crate) strength: Attribute,
    pub(crate) intelligence: Attribute,
    pub(crate) dexterity: Attribute,
    pub(crate) constitution: Attribute,
    pub(crate) wisdom: Attribute,
    pub(crate) charisma: Attribute,
}

impl From<&Classes> for CharacterAttributes {
    fn from(class: &Classes) -> Self {
        let mut ca = Self::default();
        match class {
            Classes::Warrior => {
                ca.strength = Attribute::Strength(17);
                ca.constitution = Attribute::Constitution(15);
            }

            Classes::Wizard => {
                ca.intelligence = Attribute::Intelligence(17);
                ca.charisma = Attribute::Charisma(7);
            }
            Classes::Sorcerer => {
                ca.charisma = Attribute::Charisma(17);
                ca.intelligence = Attribute::Intelligence(15);
            }
        }
        ca
    }
}

impl From<&Mob> for CharacterAttributes {
    fn from(enemy: &Mob) -> Self {
        let mut ca = Self::default();
        match enemy {
            Mob::Orc => {
                ca.strength = Attribute::Strength(17);
                ca.intelligence = Attribute::Intelligence(1);
            }
            Mob::Elf => {
                ca.intelligence = Attribute::Intelligence(22);
                ca.dexterity = Attribute::Dexterity(19);
                ca.constitution = Attribute::Constitution(3);
            }
        }
        ca
    }
}

impl Default for CharacterAttributes {
    fn default() -> Self {
        Self {
            strength: Attribute::Strength(7),
            intelligence: Attribute::Intelligence(7),
            dexterity: Attribute::Dexterity(0),
            constitution: Attribute::Constitution(0),
            wisdom: Attribute::Wisdom(0),
            charisma: Attribute::Charisma(0),
        }
    }
}

impl Sub for CharacterAttributes {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            strength: self.strength - rhs.strength,
            intelligence: self.intelligence - rhs.intelligence,
            dexterity: self.dexterity - rhs.dexterity,
            constitution: self.constitution - rhs.constitution,
            wisdom: self.wisdom - rhs.wisdom,
            charisma: self.charisma - rhs.charisma,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleInfo {
    pub action: PlayerAction,
    pub damage: i32,
    pub player_name: String,
    pub monster_name: String,
    pub kill: bool,
    pub critical: bool,
    pub leveled_up: bool,
    pub monster_hp: i32,
    pub traits_available: u32,
}

impl BattleInfo {
    pub fn new(
        action: PlayerAction,
        damage: i32,
        player_name: String,
        monster_name: String,
        kill: bool,
        critical: bool,
        leveled_up: bool,
        monster_hp: i32,
        traits_available: u32,
    ) -> Self {
        Self {
            action,
            damage,
            player_name,
            monster_name,
            kill,
            critical,
            leveled_up,
            monster_hp,
            traits_available,
        }
    }
}

impl Display for BattleInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        string.push_str("\n🗡️");
        string.push_str("\n\t");
        string.push_str("🎲\t");
        string.push_str(&self.player_name);
        string.push_str(" attacked the ");
        string.push_str(&self.monster_name);
        string.push_str(" with ");
        string.push_str(&self.action.to_string());
        string.push_str(" dealing ");
        string.push_str(&self.damage.to_string());
        string.push_str(" damage!");
        string.push_str("\t🎲");
        if self.critical {
            string.push_str(" 💥 Critical hit! 💥");
        }
        if self.kill {
            string.push_str("\n\t");
            string.push_str("☠️\t");
            string.push_str("Killing blow");
            string.push_str("\t☠️");
        }
        if self.leveled_up {
            string.push_str("\n\t");
            string.push_str("🎉\t");
            string.push_str("Leveled up!");
            string.push_str("\t🎉")
        }
        if self.traits_available > 0 {
            string.push_str("\n\t");
            string.push_str("🎉\t");
            string.push_str("Trait available!");
            string.push_str("\t🎉")
        }
        string.push_str("\n🗡️\n");
        write!(f, "{}", string)
    }
}

pub struct CharacterDefenses {
    pub(crate) dodge: Dice,
    pub(crate) magical: Dice,
    pub(crate) physical: Dice,
}

impl CharacterDefenses {
    pub fn new(dodge: Dice, magical: Dice, physical: Dice) -> Self {
        Self {
            dodge,
            magical,
            physical,
        }
    }
}

impl From<&mut Character> for CharacterDefenses {
    fn from(character: &mut Character) -> Self {
        let dodge_bonus = [&CharacterTraits::Nimble, &CharacterTraits::Lucky];
        let dodge_hits = dodge_bonus
            .iter()
            .filter(|&&item| character.traits.contains(&item))
            .count() as u32;

        let mut dodge = match character.attributes.dexterity.inner() {
            0..=5 => Dice::new(Die::D20, 1 + dodge_hits, None),
            6..=10 => Dice::new(Die::D20, 2 + dodge_hits, None),
            11..=15 => Dice::new(Die::D20, 3 + dodge_hits, None),
            16..=20 => Dice::new(Die::D20, 4 + dodge_hits, None),
            _ => Dice::new(Die::D20, 1 + dodge_hits, None),
        };

        if dodge_hits > 0 {
            dodge.set_target(19);
        }

        let physical = match character.attributes.constitution.inner() {
            0..=5 => Dice::new(Die::D20, 1, None),
            6..=10 => Dice::new(Die::D20, 2, None),
            11..=15 => Dice::new(Die::D20, 3, None),
            16..=20 => Dice::new(Die::D20, 4, None),
            _ => Dice::new(Die::D20, 1, None),
        };

        let magical = match character.attributes.wisdom.inner() {
            0..=5 => Dice::new(Die::D20, 1, None),
            6..=10 => Dice::new(Die::D20, 2, None),
            11..=15 => Dice::new(Die::D20, 3, None),
            16..=20 => Dice::new(Die::D20, 4, None),
            _ => Dice::new(Die::D20, 1, None),
        };

        Self {
            dodge,
            magical,
            physical,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub(crate) level: u32,
    pub(crate) name: String,
    pub(crate) user_id: u64,
    pub(crate) class: Classes,
    pub(crate) max_hp: u32,
    pub(crate) hp: i32,
    pub(crate) experience: u32,
    pub(crate) attributes: CharacterAttributes,
    pub(crate) traits: HashSet<CharacterTraits>,
    pub(crate) available_traits: u32,
}

impl Character {
    pub fn experience_to_next_level(&self) -> u32 {
        self.level.pow(2) + self.level * 1000
    }
    pub fn new(name: String, user_id: u64, class: Classes) -> Self {
        let max_hp = class.hp_gain(1);
        Self {
            level: 1,
            name,
            user_id,
            class,
            max_hp,
            hp: max_hp as i32,
            experience: 0,
            attributes: (&class).into(),
            traits: HashSet::new(),
            available_traits: 0,
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.max_hp = self.class.hp_gain(self.level);
        self.hp = self.max_hp as i32;
        self.experience = self
            .experience
            .checked_sub(self.experience_to_next_level())
            .unwrap_or(0);
    }

    pub fn rest(&mut self) {
        self.hp = self.max_hp as i32;
    }

    fn difference(lhs: Attribute, rhs: &CharacterAttributes) -> i32 {
        match lhs {
            Attribute::Strength(v) => v as i32 - *rhs.strength as i32,
            Attribute::Intelligence(v) => v as i32 - *rhs.intelligence as i32,
            Attribute::Dexterity(v) => v as i32 - *rhs.dexterity as i32,
            Attribute::Constitution(v) => v as i32 - *rhs.constitution as i32,
            Attribute::Wisdom(v) => v as i32 - *rhs.wisdom as i32,
            Attribute::Charisma(v) => v as i32 - *rhs.charisma as i32,
        }
    }
    pub fn player_action(&mut self, enemy: &mut Enemy) -> BattleInfo {
        let action = self.class.action();
        let act = action.act(&self, enemy);
        match act.attack_type {
            AttackType::Physical(d) => {
                enemy.health -= d as i32;
                debug!(
                    "{} attacked {} for {} damage! {} has {} hp",
                    self.name, enemy.kind, d, enemy.kind, enemy.health
                )
            }
            AttackType::Magical(d) => {
                enemy.health -= d as i32;
                debug!(
                    "{} attacked {} for {} damage! {} has {} hp",
                    self.name, enemy.kind, d, enemy.kind, enemy.health
                )
            }
        }

        let mut level = false;
        if enemy.health <= 0 {
            enemy.state = EnemyState::Dead;
            self.experience += enemy.experience;
            while self.experience >= self.experience_to_next_level() {
                self.level_up();
                level = true;
                if self.level % 10 == 0 {
                    self.available_traits += 1;
                }
            }
        }

        BattleInfo {
            action,
            damage: act.attack_type.inner() as i32,
            player_name: self.name.clone(),
            monster_name: enemy.kind.to_string(),
            kill: enemy.health <= 0,
            critical: act.critical,
            leveled_up: level,
            monster_hp: enemy.health,
            traits_available: self.available_traits,
        }
    }

    pub fn enemy_action(&mut self, enemy: &Enemy) {
        let action = enemy.action();
        let defense: CharacterDefenses = self.into();
        match action {
            AttackType::Physical(damage) => {
                let dodge = defense.physical.success(None);
                if dodge {
                    info!("{} dodged the attack!", self.name);
                } else {
                    self.hp -= damage as i32;
                    debug!(
                        "{} was attacked by {} for {} damage! {} has {} hp",
                        self.name, enemy.kind, damage, self.name, self.hp
                    )
                }
            }
            AttackType::Magical(damage) => {
                self.hp -= damage as i32;
            }
        }
        self.hp = self.hp.max(0);
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}\n level: {}\n class {}",
            self.name, self.level, self.class
        )
    }
}

#[cfg(test)]
mod test {
    use crate::player::AttackModifiers;

    #[test]
    fn attack_modifiers() {
        let mut attack_modifiers = AttackModifiers::default();
        let (physical, magical) = attack_modifiers.generate_damage_values();
        assert!(physical.0 > 0);
        assert!(magical.0 > 0);
    }
}
