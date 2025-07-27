use crate::assets::{asset_group_foreach, AssetGroup};
use crate::mac_assets::{MaskedPictAsset, RGNAsset};
use anyhow;
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use literally::bmap;
use regex::Regex;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

const CINEMATIC_ASSETS: &[AssetGroup] = &[AssetGroup {
    name: "cinematic",
    srcs: &["cinematics/*.aecinematic"],
}];

pub fn compile_cinematics(
    asset_base_dir: &Path,
    masked_pict_assets: &Vec<(String, Vec<MaskedPictAsset>)>,
    rgn_assets: &Vec<RGNAsset>,
    build_dir: &Path,
) -> anyhow::Result<PathBuf> {
    let (character_mood_sprite_indexes, background_resource_ids) =
        build_maps(masked_pict_assets, rgn_assets)?;

    let glob_match_fn = |_group_name: &str,
                         group_dir: &Path,
                         src: &Path,
                         base_name: &OsStr,
                         _ext: &str|
     -> anyhow::Result<()> {
        let mut dst = group_dir.join(base_name.to_string_lossy().to_case(Case::UpperCamel));
        dst.set_extension("cpp");
        translate_script(
            &character_mood_sprite_indexes,
            &background_resource_ids,
            base_name,
            src,
            &dst,
        )
    };

    let group_fn = |_group_name: &str, _group_dir: &Path| -> anyhow::Result<()> { Ok(()) };

    asset_group_foreach(
        CINEMATIC_ASSETS,
        asset_base_dir,
        build_dir,
        glob_match_fn,
        group_fn,
    )?;

    Ok(PathBuf::new())
}

lazy_static! {
    static ref CHARACTER_MOOD_SPRITE: Regex = Regex::new(r"^avatar_([A-Za-z]+)_(.+)$")
        .expect("Couldn't compile CHARACTER_MOOD_SPRITE regex");
}

fn build_maps(
    masked_pict_assets: &Vec<(String, Vec<MaskedPictAsset>)>,
    rgn_assets: &Vec<RGNAsset>,
) -> anyhow::Result<(BTreeMap<(usize, String), usize>, BTreeMap<String, i16>)> {
    let sprite_sheet_assets = masked_pict_assets
        .iter()
        .map(|(group_name, group_assets)| (group_name, group_assets))
        .find(|(group_name, _)| *group_name == "sprite_sheet")
        .expect("Couldn't find sprite_sheet asset group")
        .1;
    if sprite_sheet_assets.len() != 1 || rgn_assets.len() != 1 {
        anyhow::bail!("Multiple sprite sheets not supported yet");
    }

    let mut character_mood_sprite_indexes: BTreeMap<(usize, String), usize> = BTreeMap::new();
    for (index, (name, _)) in rgn_assets.first().unwrap().regions.iter().enumerate() {
        let Some(captures) = CHARACTER_MOOD_SPRITE.captures(name) else {
            continue;
        };
        let (_, [name, mood]) = captures.extract();
        let Some(id) = CHARACTER_IDS.get(name.to_uppercase().as_str()) else {
            anyhow::bail!("Couldn't find ID for character: {name}");
        };
        character_mood_sprite_indexes.insert((*id, mood.to_string()), index);
    }

    let background_resource_ids: BTreeMap<String, i16> = masked_pict_assets
        .iter()
        .map(|(group_name, group_assets)| (group_name, group_assets))
        .find(|(group_name, _)| *group_name == "scene")
        .expect("Couldn't find scene asset group")
        .1
        .iter()
        .map(|asset| (asset.base_name.clone(), asset.image_pict_resource_id))
        .collect();
    // TODO: we currently assume that the mask_pict_resource_id is unused,
    //  but should enforce this at the type level by having regular PICTs

    Ok((character_mood_sprite_indexes, background_resource_ids))
}

// TODO: copied from MacOS/Material.cpp but should be in a resource
const MATERIAL_NAMES: &[&str] = &[
    "Bacon",
    "Bud",
    "Crystal",
    "Dragon Eye",
    "Dunkelheit",
    "Elerium",
    "Feather",
    "Flower 1",
    "Flower 2",
    "Grapes",
    "Grass",
    "Gravistone",
    "Herb",
    "Leaf Down",
    "Leaf Triple",
    "Leaf Up",
    "Lump",
    "Mushroom 1",
    "Mushroom 2",
    "Copper Ore",
    "Iron Ore",
    "Silver Ore",
    "Stygium Ore",
    "Titanium Ore",
    "Page",
    "Palm",
    "Pendeloque",
    "Pods",
    "Puniball",
    "Giant Puniball",
    "Rock",
    "Sand",
    "Seaweed 1",
    "Seaweed 2",
    "Spider",
    "Spirit",
    "Steak",
    "Sulfur",
    "Uni",
    "Water",
    "Wood",
    "Worm",
    "Copper Ingot",
];

lazy_static! {
    // TODO: Character data doesn't exist in C++ yet, should be a resource anyway
    static ref CHARACTER_IDS: BTreeMap<&'static str, usize> = bmap! {
        "ESRI" => 0usize,
        "ALLIE" => 1usize,
        "SAE" => 2usize,
    };
}

trait ToCPP {
    fn to_cpp(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CinematicCharacter {
    id: usize,
    mood: usize,
}

impl ToCPP for CinematicCharacter {
    fn to_cpp(&self) -> String {
        format!(
            "CinematicCharacter{{.id={id}, .mood={mood}}}",
            id = self.id,
            mood = self.mood
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CinematicCharacterSlot {
    Left = 0,
    Right = 1,
}

impl TryFrom<&str> for CinematicCharacterSlot {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "left" => Self::Left,
            "right" => Self::Right,
            _ => anyhow::bail!("Invalid character slot: {value}"),
        })
    }
}

impl ToCPP for CinematicCharacterSlot {
    fn to_cpp(&self) -> String {
        match self {
            Self::Left => "CinematicCharacterSlot::Left".to_string(),
            Self::Right => "CinematicCharacterSlot::Right".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CinematicCommand {
    CinematicCommandCommit,
    CinematicCommandSetCharacter {
        slot: CinematicCharacterSlot,
        character: CinematicCharacter,
    },
    CinematicCommandSetMood {
        slot: CinematicCharacterSlot,
        mood: usize,
    },
    CinematicCommandClearCharacter {
        slot: CinematicCharacterSlot,
    },
    CinematicCommandSetSpeaker {
        slot: CinematicCharacterSlot,
    },
    CinematicCommandClearSpeaker,
    CinematicCommandSetText {
        text: String,
    },
    CinematicCommandClearText,
    CinematicCommandSetBackground {
        id: usize,
    },
    CinematicCommandClearBackground,
    CinematicCommandSetMaterial {
        id: usize,
    },
    CinematicCommandClearMaterial,
}

struct EncodeAsMacRoman<'a>(&'a str);

impl<'a> ToCPP for EncodeAsMacRoman<'a> {
    fn to_cpp(&self) -> String {
        // TODO: actually encode all the characters
        format!(
            r#""{escaped}""#,
            escaped = self
                .0
                .chars()
                .map(|c| match c {
                    '\\' => r"\\".to_string(),
                    '"' => r#"\""#.to_string(),
                    '…' => r"\xc9".to_string(),
                    _ => c.to_string(),
                })
                .collect::<String>()
        )
    }
}

impl ToCPP for CinematicCommand {
    fn to_cpp(&self) -> String {
        match self {
            CinematicCommand::CinematicCommandCommit => "CinematicCommandCommit{}".to_string(),
            CinematicCommand::CinematicCommandSetCharacter { slot, character } => {
                format!(
                    "CinematicCommandSetCharacter{{.slot={slot_cpp}, .character={character_cpp}}}",
                    slot_cpp = slot.to_cpp(),
                    character_cpp = character.to_cpp()
                )
            }
            CinematicCommand::CinematicCommandSetMood { slot, mood } => {
                format!(
                    "CinematicCommandSetMood{{.slot={slot_cpp}, .mood={mood}}}",
                    slot_cpp = slot.to_cpp(),
                )
            }
            CinematicCommand::CinematicCommandClearCharacter { slot } => {
                format!(
                    "CinematicCommandClearCharacter{{.slot={slot_cpp}}}",
                    slot_cpp = slot.to_cpp(),
                )
            }
            CinematicCommand::CinematicCommandSetSpeaker { slot } => {
                format!(
                    "CinematicCommandSetSpeaker{{.slot={slot_cpp}}}",
                    slot_cpp = slot.to_cpp(),
                )
            }
            CinematicCommand::CinematicCommandClearSpeaker => {
                "CinematicCommandClearSpeaker{}".to_string()
            }
            CinematicCommand::CinematicCommandSetText { text } => {
                format!(
                    "CinematicCommandSetText{{.text={text_cpp}}}",
                    text_cpp = EncodeAsMacRoman(text).to_cpp()
                )
            }
            CinematicCommand::CinematicCommandClearText => {
                "CinematicCommandClearText{}".to_string()
            }
            CinematicCommand::CinematicCommandSetBackground { id } => {
                format!("CinematicCommandSetBackground{{.background={id}}}")
            }
            CinematicCommand::CinematicCommandClearBackground => {
                "CinematicCommandClearBackground{}".to_string()
            }
            CinematicCommand::CinematicCommandSetMaterial { id } => {
                format!("CinematicCommandSetMaterial{{.material={id}}}")
            }
            CinematicCommand::CinematicCommandClearMaterial => {
                "CinematicCommandClearMaterial{}".to_string()
            }
        }
    }
}

lazy_static! {
    static ref SCRIPT_COMMENT: Regex =
        Regex::new(r"^#").expect("Couldn't compile SCRIPT_COMMENT regex");
    static ref SCRIPT_SET: Regex = Regex::new(r"^!set (background|material|left|right) (.+)$")
        .expect("Couldn't compile SCRIPT_SET regex");
    static ref SCRIPT_UNSET: Regex =
        Regex::new(r"^!unset (background|material|speaker|text|left|right)$")
            .expect("Couldn't compile SCRIPT_UNSET regex");
    static ref SCRIPT_SPEAKER: Regex =
        Regex::new(r"^([A-Z0-9_]+):$").expect("Couldn't compile SCRIPT_SPEAKER regex");
    static ref SCRIPT_MOOD: Regex =
        Regex::new(r"^\[([a-z0-9_]+)\]$").expect("Couldn't compile SCRIPT_MOOD regex");
}

fn translate_script(
    character_mood_sprite_indexes: &BTreeMap<(usize, String), usize>,
    background_resource_ids: &BTreeMap<String, i16>,
    base_name: &OsStr,
    input: &Path,
    output: &Path,
) -> anyhow::Result<()> {
    let mut set_character_slot: Option<CinematicCharacterSlot> = None;
    let mut set_character_id: Option<usize> = None;
    let mut characters: BTreeMap<CinematicCharacterSlot, CinematicCharacter> = BTreeMap::new();
    let mut speaker: Option<CinematicCharacterSlot> = None;

    let mut script: Vec<CinematicCommand> = Vec::new();

    let lookup_mood = |id: usize, name: &str| -> anyhow::Result<usize> {
        let Some(sprite_index) = character_mood_sprite_indexes.get(&(id, name.to_string())) else {
            let Some(character_name) =
                CHARACTER_IDS
                    .iter()
                    .find_map(|(character_name, character_id)| {
                        if *character_id == id {
                            Some(character_name)
                        } else {
                            None
                        }
                    })
            else {
                anyhow::bail!("Couldn't find mood for unknown character with ID {id}: {name}");
            };
            anyhow::bail!("Couldn't find mood for character {character_name}: {name}");
        };
        Ok(*sprite_index)
    };

    for result in BufReader::new(File::open(input)?).lines() {
        let line = result?;
        if line.is_empty() || SCRIPT_COMMENT.is_match(&line) {
            continue;
        } else if let Some(captures) = SCRIPT_SET.captures(&line) {
            let (_, [slot, name]) = captures.extract();
            match slot {
                "background" => {
                    let Some(resource_id) = background_resource_ids.get(name) else {
                        anyhow::bail!("Couldn't find background: {name}");
                    };
                    script.push(CinematicCommand::CinematicCommandSetBackground {
                        id: *resource_id as usize,
                    });
                }
                "material" => {
                    let Some(material_index) = MATERIAL_NAMES
                        .iter()
                        .position(|material_name| *material_name == name)
                    else {
                        anyhow::bail!("Couldn't find material: {name}");
                    };
                    script
                        .push(CinematicCommand::CinematicCommandSetMaterial { id: material_index });
                }
                "left" | "right" => {
                    if name != "…" {
                        anyhow::bail!("!set {slot} doesn't take a name, only '…', but got {name}");
                    }
                    set_character_slot = Some(CinematicCharacterSlot::try_from(slot)?);
                }
                _ => anyhow::bail!("Unknown slot for !set: {slot}"),
            }
        } else if let Some(captures) = SCRIPT_UNSET.captures(&line) {
            let (_, [slot]) = captures.extract();
            match slot {
                "background" => {
                    script.push(CinematicCommand::CinematicCommandClearBackground);
                }
                "material" => {
                    script.push(CinematicCommand::CinematicCommandClearMaterial);
                }
                "speaker" => {
                    script.push(CinematicCommand::CinematicCommandClearSpeaker);
                }
                "text" => {
                    script.push(CinematicCommand::CinematicCommandClearText);
                }
                "left" | "right" => {
                    let character_slot = CinematicCharacterSlot::try_from(slot)?;
                    if let Some(speaker_slot) = speaker {
                        if character_slot == speaker_slot {
                            script.push(CinematicCommand::CinematicCommandClearSpeaker);
                        }
                        speaker = None;
                    }
                    script.push(CinematicCommand::CinematicCommandClearCharacter {
                        slot: character_slot,
                    });
                }
                _ => anyhow::bail!("Unknown slot for !unset: {slot}"),
            }
        } else if let Some(captures) = SCRIPT_SPEAKER.captures(&line) {
            let (_, [name]) = captures.extract();
            let Some(id) = CHARACTER_IDS.get(name) else {
                anyhow::bail!("Couldn't find character: {name}");
            };
            let id = *id;
            if set_character_slot.is_some() {
                set_character_id = Some(id);
            } else {
                let Some((slot, _)) = characters.iter().find(|(_, character)| character.id == id)
                else {
                    anyhow::bail!("Character isn't on stage, can't set them as speaker: {name}");
                };
                let slot = *slot;
                script.push(CinematicCommand::CinematicCommandSetSpeaker { slot });
                speaker = Some(slot);
            }
        } else if let Some(captures) = SCRIPT_MOOD.captures(&line) {
            let (_, [name]) = captures.extract();
            if let Some(slot) = set_character_slot {
                let Some(id) = set_character_id else {
                    anyhow::bail!("Can't set mood for unknown character: {name}");
                };
                let mood = lookup_mood(id, name)?;
                let character = CinematicCharacter { id, mood };
                script.push(CinematicCommand::CinematicCommandSetCharacter { slot, character });
                characters.insert(slot, character);
                set_character_slot = None;
                set_character_id = None;
                speaker = Some(slot);
            } else {
                let Some(slot) = speaker else {
                    anyhow::bail!("Can't set mood for speaker if nobody is on stage");
                };
                let Some(character) = characters.get_mut(&slot) else {
                    anyhow::bail!("Speaker is slot {slot:?} but nobody is on stage there");
                };
                let mood = lookup_mood(character.id, name)?;
                script.push(CinematicCommand::CinematicCommandSetMood { slot, mood });
                character.mood = mood;
            }
        } else {
            script.push(CinematicCommand::CinematicCommandSetText {
                text: line.to_string(),
            });
            script.push(CinematicCommand::CinematicCommandCommit);
        }
    }

    if script.last() != Some(&CinematicCommand::CinematicCommandCommit) {
        script.push(CinematicCommand::CinematicCommandCommit);
    }

    let mut hpp_path = output.to_path_buf();
    hpp_path.set_extension("hpp");

    let mut hpp = BufWriter::new(File::create(&hpp_path)?);
    write!(hpp, "#pragma once\n\n")?;
    write!(hpp, "#include \"Breeze/Cinematics.hpp\"\n\n")?;
    write!(hpp, "using namespace Breeze;\n\n")?;
    write!(hpp, "namespace AtelierEsri {{\n\n")?;
    write!(
        hpp,
        "extern const std::vector<CinematicCommand> {id};\n\n",
        id = base_name.to_string_lossy().to_case(Case::UpperCamel)
    )?;
    write!(hpp, "}}  // namespace AtelierEsri\n")?;

    let header_rel_path = hpp_path
        .file_name()
        .ok_or(anyhow::anyhow!(
            "Couldn't get file name from header: {hpp_path}",
            hpp_path = hpp_path.display()
        ))?
        .to_string_lossy();

    let mut cpp = BufWriter::new(File::create(output)?);
    write!(cpp, "#include \"{header_rel_path}\"\n\n")?;
    write!(cpp, "namespace AtelierEsri {{\n\n")?;
    write!(
        cpp,
        "const std::vector<CinematicCommand> {id}{{\n",
        id = base_name.to_string_lossy().to_case(Case::UpperCamel)
    )?;
    for cmd in script {
        write!(cpp, "  {cmd_cpp},\n", cmd_cpp = cmd.to_cpp())?;
    }
    write!(cpp, "}};\n\n")?;
    write!(cpp, "}}  // namespace AtelierEsri\n")?;

    Ok(())
}
