use std::{collections::HashMap, fs::File};

use heck::*;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SerialDie {
    pub(crate) die: String,
    pub(crate) quantity: usize,
}

impl SerialDie {
    pub fn new(die: String, quantity: usize) -> Self {
        Self { die, quantity }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Attributes {
    pub(crate) strength: i32,
    pub(crate) dexterity: i32,
    pub(crate) constitution: i32,
    pub(crate) intelligence: i32,
    pub(crate) wisdom: i32,
    pub(crate) charisma: i32,
}

impl Attributes {
    pub fn zero() -> Attributes {
        Self {
            strength: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            wisdom: 0,
            charisma: 0,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IndividualItem {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) slot: String,
    pub(crate) armor: Option<i32>,
    pub(crate) evasion: Option<i32>,
    pub(crate) resistance: Option<HashMap<String, i32>>,
    pub(crate) damage: Option<HashMap<String, i32>>,
    pub(crate) attribute_bonus: Option<Attributes>,
    pub(crate) action: Option<u32>,
    pub(crate) rarity: String,
    pub(crate) points: Option<u64>,
}

pub enum ResistCategories {
    Elemental,
    Physical,
    NonElemental,
    Boss,
    Prismatic,
    Universal,
}

pub fn parse_items() -> std::io::Result<String> {
    let items_directories = "items/";
    let mut items = Vec::new();
    for entry in WalkDir::new(items_directories)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            let file = File::open(path)?;
            let item: IndividualItem = serde_yaml::from_reader(file)
                .expect(&format!("Failed to parse item, {}", path.display()));
            items.push(item);
        }
    }

    let mut source_code = base_source();
    for item in &items {
        source_code.push_str(&struct_conversion(&item));
    }
    source_code.push_str(&items_impls(&items));

    Ok(source_code)
}

fn items_impls(struct_vec: &Vec<IndividualItem>) -> String {
    let mut source_code = String::new();
    source_code.push_str("#[derive(ErisValidEnum, ErisConstructedTemplate, ErisDisplayEmoji, Serialize, Deserialize, Hash, PartialEq, Clone, Copy, Debug, Eq)]\n");
    source_code.push_str("pub enum ItemsWeHave {\n");
    for st in struct_vec {
        source_code.push_str(&format!("\t{},\n", st.name.to_pascal_case()));
    }
    source_code.push_str("}\n");
    source_code.push_str("impl ItemsWeHave {\n");
    source_code.push_str(
        "\tpub fn drop_chance(level: u64, grade: MobGrade) -> Vec<ItemsWeHave> {
        let mut rng = thread_rng();
        let drop_probability = level as f64 / 20000.0;
        let mut items = HashSet::new();
        let attempts = grade as u64;
        for _ in 0..attempts {
            if rng.gen_bool(drop_probability.abs()) {
                items.insert(ItemsWeHave::generate_random_item().expect(\"Failed to generate item\"));
            };
        }
        items.into_iter().collect()
    }");
    source_code.push('}');
    source_code
}

fn struct_conversion(st: &IndividualItem) -> String {
    let mut source_code = String::new();
    let struct_name = st.name.to_pascal_case();
    source_code.push_str("#[derive(Debug, Clone, PartialEq)]\n");
    source_code.push_str(&format!("pub struct {} {{}}", struct_name));
    source_code.push('\n');
    source_code.push_str(&format!("impl {} {{", struct_name));
    source_code.push('\n');
    source_code.push_str("\tpub fn generate(&self) -> IndividualItem {");
    source_code.push_str("\n\t\t");
    source_code.push_str("IndividualItem {");
    source_code.push_str("\n\t\t\t");
    source_code.push_str(&format!("name: \"{}\".to_string(),", st.name));
    source_code.push_str("\n\t\t\t");
    source_code.push_str(&format!("description: \"{}\".to_string(),", st.description));
    source_code.push_str("\n\t\t\t");
    source_code.push_str(&format!("slot: EquipmentSlot::{},", st.slot));
    source_code.push_str("\n\t\t\t");
    let armor = st.armor.unwrap_or_default();
    source_code.push_str(&format!("armor: {},", armor));
    source_code.push_str("\n\t\t\t");
    let evasion = st.evasion.unwrap_or_default();
    source_code.push_str(&format!("dodge: {},", evasion));
    source_code.push_str("\n\t\t\t");
    let resistance = hash_map_mapping(st.resistance.clone(), "ResistCategories".to_string());
    source_code.push_str(&format!("resistance: {},", resistance));
    source_code.push_str("\n\t\t\t");
    let damage = hash_map_mapping(st.damage.clone(), "DamageType".to_string());
    source_code.push_str(&format!("damage: {},", damage));
    source_code.push_str("\n\t\t\t");
    let attribute_bonus = st.attribute_bonus.clone().unwrap_or_default();
    source_code.push_str(&format!("attribute_bonus: {:?},", attribute_bonus));
    source_code.push_str("\n\t\t\t");
    let action = st.action.unwrap_or_default();
    source_code.push_str(&format!("action: {},", action));
    source_code.push_str("\n\t\t\t");
    source_code.push_str(&format!("rarity: Rarity::{},", st.rarity));
    source_code.push_str("\n\t\t\t");
    source_code.push_str(&format!("points: {},", st.points.unwrap_or_default()));
    source_code.push_str("\n\t\t");
    source_code.push_str("}\n");
    source_code.push_str("\t}\n");
    source_code.push_str("}\n");
    source_code
}

fn hash_map_mapping(map: Option<HashMap<String, i32>>, prefix: String) -> String {
    if map.is_none() {
        return "HashMap::new()".to_string();
    }
    let map = map.unwrap_or_default();
    let tuple_vec: Vec<(_, _)> = map.iter().map(|(k, v)| (k, v)).collect();
    let mut ds = String::new();
    ds.push('[');
    for (k, v) in tuple_vec {
        ds.push_str(&format!("({}::{}, {}),", prefix, k, v));
    }
    ds.push(']');
    format!("HashMap::from({})", ds)
}

fn base_source() -> String {
    let mut source_code = String::new();
    source_code.push_str("#[rustfmt::skip]");
    source_code.push_str("/// @generated\n");
    source_code.push_str("/// This file is auto-generated. Do not edit!\n");
    source_code.push_str("use serde::Serialize;\n");
    source_code.push_str("use serde::Deserialize;\n");
    source_code.push_str("use crate::item::IndividualItem;\n");
    source_code.push_str("use crate::item::EquipmentSlot;\n");
    source_code.push_str("use std::collections::{HashMap, HashSet};\n");
    source_code.push_str("use crate::item::Rarity;\n");
    source_code.push_str("use crate::damage::DamageType;\n");
    source_code.push_str("use crate::damage::ResistCategories;\n");
    source_code.push_str("use eris_macro::ErisConstructedTemplate;\n");
    source_code.push_str("use crate::unit::Attributes;\n");
    source_code.push_str("use rand::{Rng, thread_rng};\n");
    source_code.push_str("use crate::enemy::MobGrade;\n");
    source_code.push_str("use eris_macro::ErisValidEnum;\n");
    source_code.push_str("use eris_macro::ErisDisplayEmoji;\n");
    source_code
}
