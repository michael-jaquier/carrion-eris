use heck::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SerialAttributes {
    pub(crate) strength: i32,
    pub(crate) dexterity: i32,
    pub(crate) constitution: i32,
    pub(crate) intelligence: i32,
    pub(crate) wisdom: i32,
    pub(crate) charisma: i32,
}

impl SerialAttributes {
    pub fn zero() -> SerialAttributes {
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
    pub(crate) armor: Option<SerialDie>,
    pub(crate) resistance: Option<HashMap<String, SerialDie>>,
    pub(crate) damage: Option<HashMap<String, SerialDie>>,
    pub(crate) attribute_bonus: Option<SerialAttributes>,
    pub(crate) action: Option<u32>,
    pub(crate) rarity: String,
    pub(crate) value: u64,
}

fn main() -> std::io::Result<()> {
    // Path to the directory containing your YAML files.
    let yaml_dir = "items/";
    // Read the file names in the directory.
    let entries = fs::read_dir(yaml_dir).unwrap();
    // Create a vector to hold your structs.
    let mut struct_vec: Vec<IndividualItem> = Vec::new();
    // Read the file names in the directory.
    for entry in entries {
        if let Ok(entry) = entry {
            // Get the file path.
            let file_path = entry.path();

            // Check if the file is a YAML file.
            if let Some(extension) = file_path.extension() {
                if extension == "yaml" {
                    // Read the YAML file and deserialize it into your struct.
                    let file_content = fs::read_to_string(&file_path)
                        .expect(format!("Failed to read file {:?}", file_path).as_str());
                    let yaml_struct: IndividualItem = serde_yaml::from_str(&file_content)
                        .expect(format!("Failed to parse file {:?}", file_path).as_str());

                    // Add the deserialized struct to the vector.
                    struct_vec.push(yaml_struct);
                }
            }
        }
    }

    let mut source_code = String::new();
    source_code.push_str("/// This file is auto-generated. Do not edit!\n");
    source_code.push_str("use serde::Serialize;\n");
    source_code.push_str("use serde::Deserialize;\n");
    source_code.push_str("use crate::item_templates::{IndividualItem, SerialDie};\n");
    source_code.push_str("use crate::dice::Die;\n");
    source_code.push_str("use crate::items::EquipmentSlot;\n");
    source_code.push_str("use std::collections::{HashMap, HashSet};\n");
    source_code.push_str("use crate::items::Rarity;\n");
    source_code.push_str("use crate::units::DamageType;\n");
    source_code.push_str("use eris_macro::ErisConstructedTemplate;\n");
    source_code.push_str("use crate::item_templates::SerialAttributes;\n");
    source_code.push_str("use rand::{Rng, thread_rng};\n");
    source_code.push_str("use crate::enemies::MobGrade;\n");
    source_code.push_str("use std::f64::consts::E;\n");
    for st in &struct_vec {
        let struct_name = st.name.to_pascal_case();
        source_code.push_str(&format!("#[derive(Debug, Clone, PartialEq)]\n"));
        source_code.push_str(&format!("pub struct {} {{}}", struct_name));
        source_code.push_str(&format!("\n"));
        source_code.push_str(&format!("impl {} {{", struct_name));
        source_code.push_str(&format!("\n"));
        let function = format!("\tpub fn generate(&self) -> IndividualItem {{");
        source_code.push_str(&function);
        source_code.push_str(&format!("\n\t\t"));
        source_code.push_str(&format!("IndividualItem {{"));
        source_code.push_str(&format!("\n\t\t\t"));
        source_code.push_str(&format!("name: \"{}\".to_string(),", st.name));
        source_code.push_str(&format!("\n\t\t\t"));
        source_code.push_str(&format!("description: \"{}\".to_string(),", st.description));
        source_code.push_str(&format!("\n\t\t\t"));
        source_code.push_str(&format!("slot: EquipmentSlot::{},", st.slot));
        source_code.push_str(&format!("\n\t\t\t"));
        if let Some(armor) = st.armor.clone() {
            source_code.push_str(&format!(
                "armor: SerialDie {{die:Die::{}, quantity:{} }}.to_die(),",
                armor.die, armor.quantity
            ));
        } else {
            source_code.push_str(&format!(
                "armor: SerialDie {{die:Die::D4, quantity:0 }}.to_die(),"
            ));
        }
        source_code.push_str(&format!("\n\t\t\t"));
        if let Some(resistance) = st.resistance.clone() {
            let tuple_vec: Vec<(_, _)> = resistance.iter().map(|(k, v)| (k, v)).collect();
            let mut ds = String::new();
            ds.push_str("[");
            for (k, v) in tuple_vec {
                ds.push_str(&format!(
                    "(DamageType::{}, SerialDie {{ die: Die::{}, quantity: {} }}.to_die()),",
                    k, v.die, v.quantity
                ));
            }
            ds.push_str("]");
            source_code.push_str(&format!("resistance: HashMap::from({}),", ds));
        } else {
            source_code.push_str(&format!("resistance: HashMap::new(),"));
        }
        source_code.push_str(&format!("\n\t\t\t"));
        if let Some(damage) = st.damage.clone() {
            let tuple_vec: Vec<(_, _)> = damage.iter().map(|(k, v)| (k, v)).collect();
            let mut ds = String::new();
            ds.push_str("[");
            for (k, v) in tuple_vec {
                ds.push_str(&format!(
                    "(DamageType::{}, SerialDie {{ die: Die::{}, quantity: {} }}.to_die()),",
                    k, v.die, v.quantity
                ));
            }
            ds.push_str("]");
            source_code.push_str(&format!("damage: HashMap::from({}),", ds));
        } else {
            source_code.push_str(&format!("damage: HashMap::new(),"));
        }
        source_code.push_str(&format!("\n\t\t\t"));
        if let Some(attribute_bonus) = st.attribute_bonus.clone() {
            source_code.push_str(&format!("attribute_bonus: {:?}.into(),", attribute_bonus));
        } else {
            source_code.push_str(&format!(
                "attribute_bonus: SerialAttributes::zero().into(),"
            ));
        }
        source_code.push_str(&format!("\n\t\t\t"));
        source_code.push_str(&format!("action: {},", st.action.unwrap_or(0)));
        source_code.push_str(&format!("\n\t\t\t"));
        source_code.push_str(&format!("rarity: Rarity::{},", st.rarity));
        source_code.push_str(&format!("\n\t\t\t"));
        source_code.push_str(&format!("value: {},", st.value));
        source_code.push_str(&format!("\n\t\t}}\n"));
        source_code.push_str(&format!("\t}}\n"));

        source_code.push_str(&format!("}}\n"));
    }
    source_code.push_str("#[derive(ErisConstructedTemplate, Serialize, Deserialize, Hash, PartialEq, Clone, Copy, Debug, Eq)]\n");
    source_code.push_str("pub enum ItemsWeHave {\n");
    for st in &struct_vec {
        source_code.push_str(&format!("\t{},\n", st.name.to_pascal_case()));
    }
    source_code.push_str("}\n");
    source_code.push_str("impl ItemsWeHave {\n");
    source_code.push_str("pub fn drop_chance(level: u64, grade: MobGrade) -> Vec<ItemsWeHave> {
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
                items.insert(ItemsWeHave::generate_random_item().expect(\"Failed to generate item\"));
            };
        }
        items.into_iter().collect()
    }");
    source_code.push_str("}\n");

    // Define the output file path.
    let output_path = "src/constructed.rs";

    // Create the output directory if it doesn't exist.
    fs::create_dir_all(Path::new(output_path).parent().unwrap()).unwrap();

    // Write the Rust source code to the output file.
    let mut output_file = File::create(output_path).expect("Failed to create output file");
    output_file
        .write_all(source_code.as_bytes())
        .expect("Failed to write output file");
    Ok(())
}
