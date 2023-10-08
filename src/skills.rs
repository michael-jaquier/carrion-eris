use crate::dice::{AdvantageState, Dice, Die};
use crate::enemies::Enemy;
use crate::player::ActionDice;
use crate::units::{Attribute, Attributes, DamageType};
use crate::{log_power_scale, sub_linear_scaling, AttributeScaling, ElementalScaling};
use eris_macro::{AttributeScaling, ElementalScaling, ErisDisplayEmoji, ErisValidEnum};
use serde::{Deserialize, Serialize};

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
pub enum Skill {
    #[stat("strength")]
    #[element("physical")]
    #[emoji("🗡️")]
    Slash,
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
    #[element("earth")]
    #[stat("constitution")]
    #[emoji("🌎")]
    EarthShatter,
    #[element("dark")]
    #[stat("intelligence")]
    #[emoji("🌑")]
    PoisonFlask,
    #[element("iron")]
    #[stat("strength")]
    #[emoji("🔩")]
    SteelRain,
    #[element("air")]
    #[stat("intelligence")]
    #[emoji("🌪️")]
    Tornado,
    #[element("holy")]
    #[stat("charisma")]
    #[emoji("🌟")]
    Rapture,
    #[element("physical")]
    #[stat("dexterity")]
    #[emoji("🗡️")]
    Backstab,
    #[element("earth")]
    #[stat("intelligence")]
    #[emoji("🌿")]
    Earthquake,
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
    pub(crate) fn attribute(&self, base_die: &mut ActionDice, attributes: &Attributes) {
        let n = if let Some(attribute) = AttributeScaling::scaling(self) {
            let attribute_value = attributes.get(&attribute);
            log_power_scale(attribute_value, None)
        } else {
            0
        } as usize;

        if let Some(attribute) = AttributeScaling::scaling(self) {
            match attribute {
                Attribute::Strength(_) => {
                    base_die.physical = Some(Dice::new(vec![Die::D20.into(); 1 + n]));
                }
                Attribute::Intelligence(_) => {
                    base_die.magical = Some(Dice::new(vec![Die::D20.into(); 4 + n]));
                }
                Attribute::Dexterity(_) => {
                    base_die.physical = Some(Dice::new(vec![Die::D12.into(); 2 + 2 * n]));
                }
                Attribute::Constitution(_) => {
                    base_die.physical = Some(Dice::new(vec![Die::D4.into(); 1 + 2 * n]));
                    base_die.magical = Some(Dice::new(vec![Die::D4.into(); 1 + 2 * n]));
                }
                Attribute::Wisdom(_) => {
                    base_die.magical = Some(Dice::new(vec![Die::D6.into(); 4 + 2 * n]));
                }
                Attribute::Charisma(_) => {
                    base_die.magical = Some(Dice::new(vec![Die::D8.into(); 3 + 3 * n]));
                }
            };
        };
    }

    pub(crate) fn elemental(&self, base_die: &mut ActionDice) {
        if let Some(elemental) = ElementalScaling::scaling(self) {
            match elemental {
                DamageType::Fire => {
                    base_die.set_critical_state(AdvantageState::Advantage);
                }
                DamageType::Water => {}
                DamageType::Earth => {}
                DamageType::Air => {}
                DamageType::Light => {}
                DamageType::Dark => {}
                DamageType::Iron => {}
                DamageType::Arcane => {}
                DamageType::Holy => {
                    base_die.set_critical_state(AdvantageState::Advantage);
                    base_die.magical = Some(Dice::new(vec![Die::D20.into(); 1]));
                }
                DamageType::NonElemental => {}
                DamageType::Physical => {}
                _ => {}
            }
        }
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
    pub(crate) fn attribute(&self, base_die: &mut ActionDice, attributes: &Attributes) {
        let n = if let Some(attribute) = AttributeScaling::scaling(self) {
            let attribute_value = attributes.get(&attribute);
            log_power_scale(attribute_value, None)
        } else {
            0
        } as usize;

        if let Some(attribute) = AttributeScaling::scaling(self) {
            match attribute {
                Attribute::Strength(_) => {
                    base_die.physical = Some(Dice::new(vec![Die::D4.into(); 4 + n]));
                }
                Attribute::Intelligence(_) => {
                    base_die.magical = Some(Dice::new(vec![Die::D4.into(); 4 + n]));
                }
                Attribute::Dexterity(_) => {
                    base_die.physical = Some(Dice::new(vec![Die::D4.into(); 4 + n]));
                }
                Attribute::Constitution(_) => {
                    base_die.physical = Some(Dice::new(vec![Die::D4.into(); 2 + n]));
                    base_die.magical = Some(Dice::new(vec![Die::D4.into(); 2 + n]));
                }
                Attribute::Wisdom(_) => {
                    base_die.magical = Some(Dice::new(vec![Die::D4.into(); 4 + n]));
                }
                Attribute::Charisma(_) => {
                    base_die.magical = Some(Dice::new(vec![Die::D4.into(); 4 + n]));
                }
            };
        };
    }

    pub(crate) fn elemental(&self, base_die: &mut ActionDice) {
        if let Some(elemental) = ElementalScaling::scaling(self) {
            match elemental {
                DamageType::Fire => {
                    base_die.set_critical_state(AdvantageState::Advantage);
                }
                DamageType::Water => {}
                DamageType::Earth => {}
                DamageType::Air => {}
                DamageType::Light => {}
                DamageType::Dark => {}
                DamageType::Iron => {}
                DamageType::Arcane => {}
                DamageType::Holy => {
                    base_die.set_critical_state(AdvantageState::Advantage);
                    base_die.magical = Some(Dice::new(vec![Die::D20.into(); 1]));
                }
                DamageType::NonElemental => {}
                DamageType::Physical => {}
                DamageType::Hope => {}
                DamageType::Despair => {}
                DamageType::Existential => {
                    base_die.magical = Some(Dice::new(vec![Die::D4.into(); 10]));
                }
                DamageType::Boss => {
                    base_die.set_critical_state(AdvantageState::Advantage);
                    base_die.magical = Some(Dice::new(vec![Die::D20.into(); 2]));
                    base_die.physical = Some(Dice::new(vec![Die::D20.into(); 2]));
                }

                DamageType::Prismatic => {}
                DamageType::Healing => {
                    base_die.magical = Some(Dice::new(vec![Die::D20.into(); 15]));
                }
            }
        }
    }

    fn level_scaling(&self, level: u32, base_die: &mut ActionDice) {
        base_die.add_existing_die(vec![Die::D12.into(); sub_linear_scaling(level) as usize]);
    }
    pub fn act(&self, enemy: &Enemy) -> ActionDice {
        let base_die = &mut ActionDice::default();
        self.attribute(base_die, &enemy.attributes);
        self.elemental(base_die);
        self.level_scaling(enemy.level, base_die);
        base_die.clone()
    }
}
