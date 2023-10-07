/// This file is auto-generated. Do not edit!
use serde::Serialize;
use serde::Deserialize;
use crate::item_templates::{IndividualItem, SerialDie};
use crate::dice::Die;
use crate::items::EquipmentSlot;
use std::collections::{HashMap, HashSet};
use crate::items::Rarity;
use crate::units::DamageType;
use eris_macro::ErisConstructedTemplate;
use crate::item_templates::SerialAttributes;
use rand::{Rng, thread_rng};
use crate::enemies::MobGrade;
use std::f64::consts::E;
#[derive(Debug, Clone, PartialEq)]
pub struct VenomweaveGloves {}
impl VenomweaveGloves {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Venomweave Gloves".to_string(),
			description: "The Venomweave Gloves are an extraordinary creation designed for the master of poisons and toxins. Crafted from the hides of venomous creatures and enchanted with ancient alchemical sigils, these gloves empower the wearer to control and unleash potent venoms with deadly precision. The fingertips are adorned with shimmering emerald gems that glisten with a malevolent gleam.".to_string(),
			slot: EquipmentSlot::Hands,
			armor: SerialDie {die:Die::D4, quantity:0 }.to_die(),
			resistance: HashMap::from([(DamageType::Despair, SerialDie { die: Die::D12, quantity: 2 }.to_die()),]),
			damage: HashMap::from([(DamageType::Despair, SerialDie { die: Die::D10, quantity: 2 }.to_die()),]),
			attribute_bonus: SerialAttributes::zero().into(),
			action: 0,
			rarity: Rarity::Legendary,
			value: 300,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct PrismariPrimer {}
impl PrismariPrimer {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Prismari Primer".to_string(),
			description: "A magical tome filled with intricate diagrams and colorful illustrations. It grants the reader a deeper understanding of arcane arts.".to_string(),
			slot: EquipmentSlot::WondrousItem,
			armor: SerialDie {die:Die::D4, quantity:0 }.to_die(),
			resistance: HashMap::new(),
			damage: HashMap::new(),
			attribute_bonus: SerialAttributes { strength: 0, dexterity: 0, constitution: 0, intelligence: 5, wisdom: 0, charisma: 0 }.into(),
			action: 0,
			rarity: Rarity::Rare,
			value: 75,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct WandOfAbyssalDesolation {}
impl WandOfAbyssalDesolation {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Wand of Abyssal Desolation".to_string(),
			description: "The Wand of Abyssal Desolation is a sinister artifact, brimming with the malevolent forces of darkness and despair. It is a conduit to the very heart of existential dread. The wand's ebony wood is adorned with ominous runes that pulse with an eerie, purplish glow when it's in use.".to_string(),
			slot: EquipmentSlot::Weapon,
			armor: SerialDie {die:Die::D4, quantity:0 }.to_die(),
			resistance: HashMap::new(),
			damage: HashMap::from([(DamageType::Existential, SerialDie { die: Die::D12, quantity: 10 }.to_die()),(DamageType::Despair, SerialDie { die: Die::D20, quantity: 10 }.to_die()),]),
			attribute_bonus: SerialAttributes { strength: 0, dexterity: 0, constitution: 0, intelligence: 10, wisdom: 0, charisma: 5 }.into(),
			action: 0,
			rarity: Rarity::Legendary,
			value: 260,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct AmuletOfArcaneAscendance {}
impl AmuletOfArcaneAscendance {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Amulet of Arcane Ascendance".to_string(),
			description: "The Amulet of Arcane Ascendance is a relic of unparalleled mystic potency, a treasure coveted by scholars and sorcerers alike. Its centerpiece is a luminous sapphire, deep blue like the heart of a crystal-clear ocean, nestled within an ornate silver setting. The sapphire's facets shimmer with an ethereal radiance, revealing an otherworldly cosmos swirling within.".to_string(),
			slot: EquipmentSlot::Amulet,
			armor: SerialDie {die:Die::D4, quantity:0 }.to_die(),
			resistance: HashMap::from([(DamageType::Fire, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Air, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Prismatic, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Arcane, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Water, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Despair, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Earth, SerialDie { die: Die::D8, quantity: 2 }.to_die()),]),
			damage: HashMap::from([(DamageType::Fire, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Earth, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Air, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Arcane, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Water, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Prismatic, SerialDie { die: Die::D8, quantity: 2 }.to_die()),]),
			attribute_bonus: SerialAttributes::zero().into(),
			action: 0,
			rarity: Rarity::Artifact,
			value: 500,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct ArcaneReliquaryOfPower {}
impl ArcaneReliquaryOfPower {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Arcane Reliquary of Power".to_string(),
			description: "The Arcane Reliquary of Power is an artifact-tier wondrous item, a mystic container of boundless arcane energy. When worn as an amulet, it amplifies the wearer's arcane abilities, granting them immense power in both offense and defense.".to_string(),
			slot: EquipmentSlot::WondrousItem,
			armor: SerialDie {die:Die::D4, quantity:0 }.to_die(),
			resistance: HashMap::from([(DamageType::Arcane, SerialDie { die: Die::D20, quantity: 4 }.to_die()),]),
			damage: HashMap::from([(DamageType::Arcane, SerialDie { die: Die::D20, quantity: 4 }.to_die()),]),
			attribute_bonus: SerialAttributes { strength: 0, dexterity: 0, constitution: 0, intelligence: 6, wisdom: 0, charisma: 0 }.into(),
			action: 0,
			rarity: Rarity::Artifact,
			value: 300,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct AsgardianAegis {}
impl AsgardianAegis {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Asgardian Aegis".to_string(),
			description: "The Asgardian Aegis is a legendary shield that echoes with the power of the gods themselves. Forged in the divine fires of Asgard and imbued with the blessings of the divine pantheon, this shield is the stuff of myth and legend. Its surface is adorned with celestial runes that shimmer with a radiant aura, offering unparalleled protection to its wielder.".to_string(),
			slot: EquipmentSlot::Shield,
			armor: SerialDie {die:Die::D10, quantity:3 }.to_die(),
			resistance: HashMap::from([(DamageType::Boss, SerialDie { die: Die::D12, quantity: 2 }.to_die()),(DamageType::Despair, SerialDie { die: Die::D12, quantity: 2 }.to_die()),(DamageType::Existential, SerialDie { die: Die::D12, quantity: 2 }.to_die()),]),
			damage: HashMap::new(),
			attribute_bonus: SerialAttributes::zero().into(),
			action: 0,
			rarity: Rarity::Artifact,
			value: 500,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct HelmOfTheDwarfBetrayer {}
impl HelmOfTheDwarfBetrayer {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Helm of the Dwarf Betrayer".to_string(),
			description: "".to_string(),
			slot: EquipmentSlot::Helmet,
			armor: SerialDie {die:Die::D12, quantity:4 }.to_die(),
			resistance: HashMap::from([(DamageType::Iron, SerialDie { die: Die::D12, quantity: 4 }.to_die()),(DamageType::Despair, SerialDie { die: Die::D12, quantity: 4 }.to_die()),]),
			damage: HashMap::new(),
			attribute_bonus: SerialAttributes { strength: 10, dexterity: 0, constitution: 10, intelligence: 0, wisdom: 0, charisma: 0 }.into(),
			action: 0,
			rarity: Rarity::Legendary,
			value: 290,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct ColossusSlayerSHeartstone {}
impl ColossusSlayerSHeartstone {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Colossus Slayer's Heartstone".to_string(),
			description: "The Colossus Slayer's Heartstone is a legendary wondrous item, a relic of immense power. This crystalline heartstone, when worn as an amulet, channels the raw might of titans and colossi into the wearer's melee attacks, delivering devastating physical blows and fortifying their constitution.".to_string(),
			slot: EquipmentSlot::WondrousItem,
			armor: SerialDie {die:Die::D12, quantity:4 }.to_die(),
			resistance: HashMap::new(),
			damage: HashMap::from([(DamageType::Physical, SerialDie { die: Die::D20, quantity: 4 }.to_die()),]),
			attribute_bonus: SerialAttributes { strength: 4, dexterity: 4, constitution: 6, intelligence: 0, wisdom: 0, charisma: 0 }.into(),
			action: 0,
			rarity: Rarity::Legendary,
			value: 260,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct LegguardsOfInvincibleAegis {}
impl LegguardsOfInvincibleAegis {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Legguards of Invincible Aegis".to_string(),
			description: "The Legguards of Invincible Aegis are a legendary masterpiece forged in the heart of a celestial forge, said to be touched by the gods themselves. These legguards are adorned with intricate engravings of divine beings, and they grant the wearer an unmatched level of protection. The magical armor woven into them forms an impenetrable shield against all but the most devastating blows.".to_string(),
			slot: EquipmentSlot::Legs,
			armor: SerialDie {die:Die::D12, quantity:4 }.to_die(),
			resistance: HashMap::from([(DamageType::NonElemental, SerialDie { die: Die::D20, quantity: 1 }.to_die()),(DamageType::Arcane, SerialDie { die: Die::D8, quantity: 1 }.to_die()),(DamageType::Physical, SerialDie { die: Die::D8, quantity: 1 }.to_die()),(DamageType::Existential, SerialDie { die: Die::D8, quantity: 1 }.to_die()),(DamageType::Light, SerialDie { die: Die::D8, quantity: 1 }.to_die()),(DamageType::Hope, SerialDie { die: Die::D8, quantity: 1 }.to_die()),(DamageType::Despair, SerialDie { die: Die::D8, quantity: 1 }.to_die()),(DamageType::Holy, SerialDie { die: Die::D8, quantity: 1 }.to_die()),(DamageType::Iron, SerialDie { die: Die::D8, quantity: 1 }.to_die()),(DamageType::Dark, SerialDie { die: Die::D8, quantity: 1 }.to_die()),(DamageType::Prismatic, SerialDie { die: Die::D8, quantity: 1 }.to_die()),]),
			damage: HashMap::new(),
			attribute_bonus: SerialAttributes::zero().into(),
			action: 0,
			rarity: Rarity::Legendary,
			value: 500,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct RingOfBattleProwess {}
impl RingOfBattleProwess {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Ring of Battle Prowess".to_string(),
			description: "The Ring of Battle Prowess is a legendary ring forged for those who thrive in the heat of combat. It exudes an aura of unyielding determination, granting its wearer an extra action in battle, allowing them to strike with unmatched speed.".to_string(),
			slot: EquipmentSlot::Ring,
			armor: SerialDie {die:Die::D4, quantity:0 }.to_die(),
			resistance: HashMap::new(),
			damage: HashMap::from([(DamageType::Physical, SerialDie { die: Die::D10, quantity: 1 }.to_die()),]),
			attribute_bonus: SerialAttributes::zero().into(),
			action: 1,
			rarity: Rarity::Epic,
			value: 200,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct InfernoStaff {}
impl InfernoStaff {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Inferno Staff".to_string(),
			description: "The Inferno Staff is a legendary weapon, a conduit of scorching flames and searing heat. It is said to contain the essence of a raging inferno, ready to unleash its fiery wrath upon all who oppose its wielder.".to_string(),
			slot: EquipmentSlot::Weapon,
			armor: SerialDie {die:Die::D4, quantity:0 }.to_die(),
			resistance: HashMap::new(),
			damage: HashMap::from([(DamageType::Fire, SerialDie { die: Die::D20, quantity: 10 }.to_die()),]),
			attribute_bonus: SerialAttributes { strength: 0, dexterity: 0, constitution: 0, intelligence: 15, wisdom: 0, charisma: 0 }.into(),
			action: 0,
			rarity: Rarity::Legendary,
			value: 260,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct BootsOfElementalMastery {}
impl BootsOfElementalMastery {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Boots of Elemental Mastery".to_string(),
			description: "The Boots of Elemental Mastery are legendary footwear forged in the heart of an ancient volcano and imbued with the essence of the elements themselves. Each boot is adorned with intricate runes representing the elemental forces of fire, water, earth, and air. When worn, they grant the wearer unparalleled control over the elements.".to_string(),
			slot: EquipmentSlot::Feet,
			armor: SerialDie {die:Die::D12, quantity:4 }.to_die(),
			resistance: HashMap::from([(DamageType::Air, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Water, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Fire, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Earth, SerialDie { die: Die::D8, quantity: 2 }.to_die()),]),
			damage: HashMap::from([(DamageType::Fire, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Water, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Air, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Earth, SerialDie { die: Die::D8, quantity: 2 }.to_die()),]),
			attribute_bonus: SerialAttributes::zero().into(),
			action: 0,
			rarity: Rarity::Legendary,
			value: 300,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct DragonScaleArmor {}
impl DragonScaleArmor {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Dragon Scale Armor".to_string(),
			description: "Dragon scale mail is made of the scales of one kind of dragon. Sometimes dragons collect their cast-off scales and gift them to humanoids. Other times, hunters carefully skin and preserve the hide of a dead dragon. In either case, dragon scale mail is highly valued. While wearing this armor, you gain a 5% physical damage reduction, you have advantage on saving throws against elemental damage of the type that is determined by the kind of dragon that provided the scales. In addition you do additional physical and fire damage".to_string(),
			slot: EquipmentSlot::Armor,
			armor: SerialDie {die:Die::D4, quantity:2 }.to_die(),
			resistance: HashMap::from([(DamageType::Fire, SerialDie { die: Die::D8, quantity: 2 }.to_die()),]),
			damage: HashMap::from([(DamageType::Fire, SerialDie { die: Die::D8, quantity: 2 }.to_die()),(DamageType::Physical, SerialDie { die: Die::D8, quantity: 2 }.to_die()),]),
			attribute_bonus: SerialAttributes::zero().into(),
			action: 0,
			rarity: Rarity::VeryRare,
			value: 250,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct StoneOfStrength {}
impl StoneOfStrength {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Stone of Strength".to_string(),
			description: "A smooth, obsidian-like stone imbued with the essence of incredible strength. When held, it enhances the wearer's physical abilities.".to_string(),
			slot: EquipmentSlot::WondrousItem,
			armor: SerialDie {die:Die::D4, quantity:0 }.to_die(),
			resistance: HashMap::new(),
			damage: HashMap::new(),
			attribute_bonus: SerialAttributes { strength: 6, dexterity: 0, constitution: 0, intelligence: 0, wisdom: 0, charisma: 0 }.into(),
			action: 0,
			rarity: Rarity::Uncommon,
			value: 30,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct SwordOfAsbethathTheBetrayer {}
impl SwordOfAsbethathTheBetrayer {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Sword of Asbethath the Betrayer".to_string(),
			description: "".to_string(),
			slot: EquipmentSlot::Weapon,
			armor: SerialDie {die:Die::D4, quantity:0 }.to_die(),
			resistance: HashMap::from([(DamageType::Holy, SerialDie { die: Die::D8, quantity: 4 }.to_die()),(DamageType::Dark, SerialDie { die: Die::D8, quantity: 4 }.to_die()),]),
			damage: HashMap::from([(DamageType::Despair, SerialDie { die: Die::D100, quantity: 1 }.to_die()),(DamageType::Existential, SerialDie { die: Die::D20, quantity: 4 }.to_die()),]),
			attribute_bonus: SerialAttributes { strength: 10, dexterity: 0, constitution: 0, intelligence: 0, wisdom: 0, charisma: 10 }.into(),
			action: 0,
			rarity: Rarity::Artifact,
			value: 500,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct MemeweaveGloves {}
impl MemeweaveGloves {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Memeweave Gloves".to_string(),
			description: "The Venomweave Gloves are an extraordinary creation designed for the master of poisons and toxins. Crafted from the hides of venomous creatures and enchanted with ancient alchemical sigils, these gloves empower the wearer to control and unleash potent venoms with deadly precision. The fingertips are adorned with shimmering emerald gems that glisten with a malevolent gleam.".to_string(),
			slot: EquipmentSlot::Hands,
			armor: SerialDie {die:Die::D4, quantity:0 }.to_die(),
			resistance: HashMap::from([(DamageType::Despair, SerialDie { die: Die::D12, quantity: 2 }.to_die()),]),
			damage: HashMap::from([(DamageType::Despair, SerialDie { die: Die::D10, quantity: 2 }.to_die()),]),
			attribute_bonus: SerialAttributes::zero().into(),
			action: 0,
			rarity: Rarity::Common,
			value: 30,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct RingOfResilientFortitude {}
impl RingOfResilientFortitude {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Ring of Resilient Fortitude".to_string(),
			description: "The Ring of Resilient Fortitude is a masterwork of protection, designed to shield its wearer from harm. Crafted by ancient guardians, it bestows upon the wearer unparalleled resistance, making them nearly impervious to damage.".to_string(),
			slot: EquipmentSlot::Ring,
			armor: SerialDie {die:Die::D4, quantity:0 }.to_die(),
			resistance: HashMap::from([(DamageType::Air, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::Dark, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::Physical, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::Light, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::Prismatic, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::Despair, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::Earth, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::Arcane, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::Water, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::Existential, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::Iron, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::NonElemental, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::Fire, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::Holy, SerialDie { die: Die::D4, quantity: 1 }.to_die()),(DamageType::Hope, SerialDie { die: Die::D4, quantity: 1 }.to_die()),]),
			damage: HashMap::new(),
			attribute_bonus: SerialAttributes::zero().into(),
			action: 0,
			rarity: Rarity::Epic,
			value: 180,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct VeilOfEternalDreams {}
impl VeilOfEternalDreams {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Veil of Eternal Dreams".to_string(),
			description: "The Veil of Eternal Dreams is an enigmatic relic, unlike any other wondrous item known to mortals. Crafted by ancient dreamweavers, this ethereal veil transcends the boundaries of reality, allowing its wearer to traverse the realms of dreams and reality.".to_string(),
			slot: EquipmentSlot::WondrousItem,
			armor: SerialDie {die:Die::D6, quantity:2 }.to_die(),
			resistance: HashMap::from([(DamageType::Fire, SerialDie { die: Die::D12, quantity: 3 }.to_die()),(DamageType::Arcane, SerialDie { die: Die::D10, quantity: 3 }.to_die()),]),
			damage: HashMap::from([(DamageType::Prismatic, SerialDie { die: Die::D20, quantity: 4 }.to_die()),]),
			attribute_bonus: SerialAttributes { strength: 0, dexterity: 0, constitution: 0, intelligence: 10, wisdom: 10, charisma: 10 }.into(),
			action: 1,
			rarity: Rarity::Unique,
			value: 1,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct UnyieldingOnyxAssaultRing {}
impl UnyieldingOnyxAssaultRing {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Unyielding Onyx Assault Ring".to_string(),
			description: "This formidable ring is a testament to the power of the ancient Onyx gem it holds at its center. The gem, as dark as a moonless night, radiates an eerie yet captivating violet glow, hinting at the latent power contained within. Encasing the Onyx gem is a band of intricately engraved blackened steel, adorned with etchings of fierce, snarling beasts in the midst of battle. The ring is warm to the touch, and seems to pulse with a faint energy.".to_string(),
			slot: EquipmentSlot::Ring,
			armor: SerialDie {die:Die::D4, quantity:0 }.to_die(),
			resistance: HashMap::new(),
			damage: HashMap::from([(DamageType::Despair, SerialDie { die: Die::D12, quantity: 4 }.to_die()),(DamageType::Dark, SerialDie { die: Die::D12, quantity: 4 }.to_die()),(DamageType::Iron, SerialDie { die: Die::D12, quantity: 4 }.to_die()),(DamageType::Existential, SerialDie { die: Die::D12, quantity: 4 }.to_die()),(DamageType::Physical, SerialDie { die: Die::D12, quantity: 4 }.to_die()),]),
			attribute_bonus: SerialAttributes::zero().into(),
			action: 2,
			rarity: Rarity::Legendary,
			value: 290,
		}
	}
}
#[derive(Debug, Clone, PartialEq)]
pub struct RingOfArcaneMastery {}
impl RingOfArcaneMastery {
	pub fn generate(&self) -> IndividualItem {
		IndividualItem {
			name: "Ring of Arcane Mastery".to_string(),
			description: "The Ring of Arcane Mastery is an artifact of unparalleled magical power. Infused with the essence of arcane realms, it amplifies the wearer's magical abilities to astounding levels, enabling them to wield devastating arcane magic.".to_string(),
			slot: EquipmentSlot::Ring,
			armor: SerialDie {die:Die::D4, quantity:0 }.to_die(),
			resistance: HashMap::new(),
			damage: HashMap::from([(DamageType::Arcane, SerialDie { die: Die::D10, quantity: 1 }.to_die()),]),
			attribute_bonus: SerialAttributes::zero().into(),
			action: 0,
			rarity: Rarity::Epic,
			value: 200,
		}
	}
}
#[derive(ErisConstructedTemplate, Serialize, Deserialize, Hash, PartialEq, Clone, Copy, Debug, Eq)]
pub enum ItemsWeHave {
	VenomweaveGloves,
	PrismariPrimer,
	WandOfAbyssalDesolation,
	AmuletOfArcaneAscendance,
	ArcaneReliquaryOfPower,
	AsgardianAegis,
	HelmOfTheDwarfBetrayer,
	ColossusSlayerSHeartstone,
	LegguardsOfInvincibleAegis,
	RingOfBattleProwess,
	InfernoStaff,
	BootsOfElementalMastery,
	DragonScaleArmor,
	StoneOfStrength,
	SwordOfAsbethathTheBetrayer,
	MemeweaveGloves,
	RingOfResilientFortitude,
	VeilOfEternalDreams,
	UnyieldingOnyxAssaultRing,
	RingOfArcaneMastery,
}
impl ItemsWeHave {
pub fn drop_chance(level: u64, grade: MobGrade) -> Vec<ItemsWeHave> {
        let mut rng = thread_rng();
        let drop_probability =
            1_f64 - E.powf(-1_f64 * (level as f64).ln() / ((100 * grade as u64) as f64));
        let mut items = HashSet::new();
        let attempts = match grade {
            MobGrade::Weak => 1,
            MobGrade::Normal => 2,
            MobGrade::Strong => 3,
            MobGrade::Boss => 1000,
        };
        for _ in 0..attempts {
            if rng.gen_bool(drop_probability.abs().max(0.01)) {
                items.insert(ItemsWeHave::generate_random_item().expect("Failed to generate item"));
            };
        }
        items.into_iter().collect()
    }}
