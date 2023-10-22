use crate::character::Character;
use crate::damage::{Damage, DamageBuilder, DamageType};
use crate::enemy::Enemy;
use crate::unit::Attributes;
use crate::{level_up_scaling, log_power_scale, AttributeScaling, ElementalScaling, EnemyEvents};
use eris_macro::{AttributeScaling, ElementalScaling, ErisDisplayEmoji, ErisValidEnum};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    AttributeScaling,
    ElementalScaling,
    ErisValidEnum,
    ErisDisplayEmoji,
    Copy,
)]
pub enum Skill {
    #[element("air")]
    #[stat("intelligence")]
    #[emoji("🌪️")]
    Tornado,
    #[element("arcane")]
    #[stat("intelligence")]
    #[emoji("🔮")]
    MagicMissile,
    #[element("fire")]
    #[stat("intelligence")]
    #[emoji("🔥")]
    FireBall,
    #[element("water")]
    #[stat("intelligence")]
    #[emoji("💧")]
    WaterBall,
    #[element("dark")]
    #[stat("intelligence")]
    #[emoji("🌑")]
    PoisonFlask,
    #[element("earth")]
    #[stat("intelligence")]
    #[emoji("🌿")]
    Earthquake,
    #[element("hope")]
    #[stat("intelligence")]
    #[emoji("🌟")]
    RadiantIntellect,
    #[stat("strength")]
    #[element("physical")]
    #[emoji("🗡️")]
    Slash,
    #[element("earth")]
    #[stat("constitution")]
    #[emoji("🌎")]
    EarthShatter,
    #[element("iron")]
    #[stat("strength")]
    #[emoji("🔩")]
    SteelRain,
    #[element("holy")]
    #[stat("charisma")]
    #[emoji("🌟")]
    Rapture,
    #[element("physical")]
    #[stat("dexterity")]
    #[emoji("🗡️")]
    Backstab,
    #[element("light")]
    #[stat("wisdom")]
    #[emoji("☀️")]
    HolySmite,
    #[element("holy")]
    #[stat("wisdom")]
    #[emoji("🙏")]
    DivineBlessing,
    #[stat("charisma")]
    #[element("existential")]
    #[emoji("🗨️")]
    SuicidalPersuasion,
    #[stat("charisma")]
    #[element("despair")]
    #[emoji("💋")]
    Seduction,
    #[stat("charisma")]
    #[element("prismatic")]
    #[emoji("🌀")]
    Mesmerize,
    #[stat("charisma")]
    #[element("fire")]
    #[emoji("🔥")]
    Excoriate,
    #[stat("dexterity")]
    #[element("arcane")]
    #[emoji("🔮")]
    ArcaneNeedle,
    #[stat("dexterity")]
    #[element("prismatic")]
    #[emoji("🌈")]
    PrismaticFlourish,
    #[stat("dexterity")]
    #[element("dark")]
    #[emoji("🌑")]
    ShadowStrike,
    #[stat("dexterity")]
    #[element("light")]
    #[emoji("☀️")]
    SolarFlareShot,
    #[stat("dexterity")]
    #[element("fire")]
    #[emoji("🔥")]
    FireDance,
    #[stat("strength")]
    #[element("physical")]
    #[emoji("🏋️")]
    PowerStrike,
    #[stat("strength")]
    #[element("earth")]
    #[emoji("🌋")]
    EarthquakeSlam,
    #[stat("strength")]
    #[element("iron")]
    #[emoji("⛓️")]
    IronFusillade,
    #[stat("strength")]
    #[element("prismatic")]
    #[emoji("🌈")]
    PrismaticHowl,
    #[stat("strength")]
    #[element("physical")]
    #[emoji("💪")]
    MightyBlow,
    #[stat("strength")]
    #[element("nonElemental")]
    #[emoji("🌟")]
    NebulaHammer,
    #[stat("strength")]
    #[element("physical")]
    #[emoji("⚔️")]
    BruteForce,
}

impl Skill {
    fn funky_scaling(&self, damage: &mut Damage, _player: &Character) {
        match self {
            Skill::Slash => {
                damage.damage += 10;
                damage.critical_multiplier = 2.0;
                damage.crit_chance += 0.1;
            }
            Skill::MagicMissile => {
                damage.number_of_hits = 5;
            }
            Skill::FireBall => {
                damage.critical_multiplier = 2.5;
                damage.crit_chance = 0.15;
            }
            Skill::WaterBall => {
                damage.damage = 12;
            }
            Skill::EarthShatter => {
                damage.damage = 20;
            }
            Skill::PoisonFlask => {
                damage.damage = 5;
            }
            Skill::SteelRain => {
                damage.damage += 5;
            }
            Skill::Tornado => {
                damage.damage += 5;
            }
            Skill::Rapture => {
                damage.damage += 5;
            }
            Skill::Backstab => {
                damage.damage += 5;
            }
            Skill::Earthquake => {
                damage.damage += 5;
            }
            Skill::HolySmite => {
                damage.damage += 5;
            }
            Skill::DivineBlessing => {
                damage.damage += 5;
            }
            Skill::SuicidalPersuasion => {
                damage.damage += 5;
            }
            Skill::Seduction => {
                damage.damage += 5;
            }
            Skill::Mesmerize => {
                damage.damage += 5;
            }
            Skill::Excoriate => {
                damage.damage += 5;
            }
            Skill::ArcaneNeedle => {
                damage.number_of_hits = 5;
            }
            Skill::PrismaticFlourish => {
                damage.damage += 5;
            }
            Skill::ShadowStrike => {
                damage.damage += 5;
            }
            Skill::SolarFlareShot => {
                damage.damage += 5;
            }
            Skill::FireDance => {
                damage.damage += 5;
            }
            Skill::PowerStrike => {
                damage.damage += 5;
            }
            Skill::EarthquakeSlam => {
                damage.damage += 5;
            }
            Skill::IronFusillade => {
                damage.damage += 5;
            }
            Skill::PrismaticHowl => {
                damage.damage += 5;
            }
            Skill::MightyBlow => {
                damage.damage += 50;
            }
            Skill::NebulaHammer => {
                damage.damage += 50;
            }
            Skill::BruteForce => {
                damage.damage += 25;
                damage.critical_multiplier += 2.0;
            }
            Skill::RadiantIntellect => {
                damage.crit_chance += 0.25;
                damage.critical_multiplier += 2.0;
                damage.number_of_hits = 3
            }
        }
    }

    pub fn base_damage(&self, player: &Character) -> Damage {
        let mut base = DamageBuilder::default()
            .dtype(self.element().unwrap_or_default())
            .damage(0)
            .build()
            .unwrap();

        self.funky_scaling(&mut base, player);

        let player_attributes = player.attributes.clone() + player.equipment.attribute();
        let attribute_bonus = self.attribute(&player_attributes) * player.level as i32;
        let elemental_bonus = self.elemental_scaling() * player.level as i32;
        base.damage +=
            (attribute_bonus + elemental_bonus).saturating_div(base.number_of_hits as i32);
        base
    }

    fn attribute(&self, attributes: &Attributes) -> i32 {
        let n = if let Some(attribute) = AttributeScaling::scaling(self) {
            let attribute_value = attributes.get(&attribute);
            log_power_scale(attribute_value, None)
        } else {
            0
        };
        n as i32
    }

    fn element(&self) -> Option<DamageType> {
        ElementalScaling::scaling(self)
    }

    fn elemental_scaling(&self) -> i32 {
        if let Some(elemental) = ElementalScaling::scaling(self) {
            let (bottom, top) = match elemental {
                DamageType::Fire => (0, 10),
                DamageType::Water => (0, 10),
                DamageType::Earth => (0, 10),
                DamageType::Air => (0, 10),
                DamageType::Light => (0, 10),
                DamageType::Dark => (0, 10),
                DamageType::Iron => (0, 10),
                DamageType::Arcane => (0, 10),
                DamageType::Holy => (0, 10),
                DamageType::NonElemental => (0, 10),
                DamageType::Physical => (0, 10),
                DamageType::Hope => (0, 10),
                DamageType::Despair => (0, 10),
                DamageType::Existential => (0, 10),
                DamageType::Boss => (0, 10),
                DamageType::Prismatic => (0, 10),
                DamageType::Healing => (0, 100),
                DamageType::Universal => (0, 10),
            };
            return thread_rng().gen_range(bottom..top);
        }
        thread_rng().gen_range(0..10)
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    AttributeScaling,
    ElementalScaling,
    ErisValidEnum,
    ErisDisplayEmoji,
)]
pub enum MobAction {
    #[element("physical")]
    #[stat("strength")]
    #[emoji("🦷")]
    Bite,
    #[element("physical")]
    #[stat("strength")]
    #[emoji("👊")]
    Claw,
    #[element("physical")]
    #[stat("strength")]
    #[emoji("🔪")]
    Stab,
    #[element("fire")]
    #[stat("intelligence")]
    #[emoji("🔥")]
    FireBall,
    #[element("holy")]
    #[stat("wisdom")]
    #[emoji("🌟")]
    SlimeAbsorb,
    #[element("physical")]
    #[stat("constitution")]
    #[emoji("👊")]
    Crush,
    #[element("physical")]
    #[stat("dexterity")]
    #[emoji("🗡️")]
    Riposte,
    #[element("dark")]
    #[stat("charisma")]
    #[emoji("👁️")]
    Glare,
    #[element("existential")]
    #[stat("charisma")]
    #[emoji("🔊")]
    MindBreak,
    #[element("fire")]
    #[stat("intelligence")]
    #[emoji("📛")]
    Burn,
    #[element("boss")]
    #[stat("constitution")]
    #[emoji("💥")]
    Explode,
    #[element("dark")]
    #[stat("intelligence")]
    #[emoji("☠️")]
    NecroticBlast,
    #[element("existential")]
    #[stat("intelligence")]
    #[emoji("🧟")]
    SummonUndead,
    #[element("physical")]
    #[stat("strength")]
    #[emoji("💥")]
    Smash,
    #[element("healing")]
    #[stat("constitution")]
    #[emoji("🔄")]
    Regenerate,
}

impl MobAction {
    pub fn base_damage(&self, enemy: &Enemy) -> Damage {
        let element = ElementalScaling::scaling(self).unwrap_or_default();
        let mut base = DamageBuilder::default()
            .dtype(element)
            .damage(enemy.kind.grade() as i32 * enemy.level as i32)
            .build()
            .unwrap();
        base.damage += self.attribute(&enemy.attributes);
        base.damage += self.elemental_scaling();
        base
    }

    pub(crate) fn attribute(&self, attributes: &Attributes) -> i32 {
        (if let Some(attribute) = AttributeScaling::scaling(self) {
            let attribute_value = attributes.get(&attribute);
            log_power_scale(attribute_value, None)
        } else {
            0
        }) as i32
    }
    fn elemental_scaling(&self) -> i32 {
        if let Some(elemental) = ElementalScaling::scaling(self) {
            let (bottom, top) = match elemental {
                DamageType::Fire => (0, 10),
                DamageType::Water => (0, 10),
                DamageType::Earth => (0, 10),
                DamageType::Air => (0, 10),
                DamageType::Light => (0, 10),
                DamageType::Dark => (0, 10),
                DamageType::Iron => (0, 10),
                DamageType::Arcane => (0, 10),
                DamageType::Holy => (0, 10),
                DamageType::NonElemental => (0, 10),
                DamageType::Physical => (0, 10),
                DamageType::Hope => (0, 10),
                DamageType::Despair => (0, 10),
                DamageType::Existential => (0, 10),
                DamageType::Boss => (0, 10),
                DamageType::Prismatic => (0, 10),
                DamageType::Healing => (0, 100),
                DamageType::Universal => (0, 10),
            };
            return thread_rng().gen_range(bottom..top);
        }
        thread_rng().gen_range(0..10)
    }
}

#[cfg(test)]
mod test {
    use crate::character::Character;
    use crate::class::Classes::Paladin;
    use crate::enemy::Mob;
    use crate::skill::SkillSet;
    use rand::random;

    #[test]
    fn slash_damage_within_range() {
        let me = Character::new("sdf".to_string(), 23, Paladin);
        let skill = crate::skill::Skill::Slash;
        let skill_set = SkillSet::new(skill);
        let mob: Mob = random();
        let enemy = mob.generate(&me);
        let damage = skill_set.act(&me, &enemy);
        assert!(damage.damage() < 150, "Damage was {:?}", damage);
    }
}

impl From<Skill> for SkillSet {
    fn from(value: Skill) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillSet {
    pub(crate) skill: Skill,
    pub(crate) experience: u64,
    pub(crate) active: bool,
    pub(crate) level: u32,
}

impl Default for SkillSet {
    fn default() -> Self {
        Self {
            skill: Skill::Slash,
            experience: 0,
            active: false,
            level: 1,
        }
    }
}

impl Display for SkillSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        string.push_str("```");
        string.push('\n');
        string.push_str(&format!("Skill: {}\n", self.skill));
        string.push_str(&format!("Level: {}\n", self.level));
        string.push_str(&format!("Experience: {}\n", self.experience));
        string.push('\n');
        string.push_str("```");
        write!(f, "{}", string)
    }
}

impl SkillSet {
    pub fn new(skill: Skill) -> Self {
        Self {
            skill,
            experience: 0,
            active: true,
            level: 1,
        }
    }

    pub fn try_level_up(&mut self) {
        while self.experience >= self.experience_to_next_level() {
            self.level += 1;
            self.experience = self
                .experience
                .saturating_sub(self.experience_to_next_level());
        }
    }

    pub fn skill(&self) -> Skill {
        self.skill
    }

    pub fn act(&self, player: &Character, _enemy: &Enemy) -> Damage {
        let mut base = self.skill().base_damage(player);
        base.damage += self.action_experience_scaling();
        base += player.mutations().get_damage(base.dtype()).clone();
        base.damage += *player.equipment.damage().get(&base.dtype).unwrap_or(&0);
        base.damage += *player
            .equipment
            .damage()
            .get(&DamageType::Universal)
            .unwrap_or(&0);
        base.number_of_hits += player.equipment.action_points() as u32;
        base
    }
    pub fn experience_to_next_level(&self) -> u64 {
        level_up_scaling(self.level, None)
    }

    pub fn action_experience_scaling(&self) -> i32 {
        (self.level * 10) as i32
    }
}
