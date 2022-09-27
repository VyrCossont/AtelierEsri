use anyhow;
use enumset::{EnumSet, EnumSetType};
use indexmap::IndexMap;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct Items {
    materials: IndexMap<MaterialId, Material>,
}

type MaterialId = String;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct Material {
    categories: EnumSet<Category>,
    #[serde(default)]
    recipe: Option<Recipe>,
}

type RecipeNodeId = String;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct Recipe {
    nodes: IndexMap<RecipeNodeId, RecipeNode>,
    links: Vec<(RecipeNodeId, RecipeNodeId)>,
}

type Quality = u16;

type ElementCount = u8;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct RecipeNode {
    /// Element used for display and for effect levels.
    element: Element,
    input: RecipeNodeInput,
    effects: Vec<RecipeNodeEffect>,
    #[serde(default)]
    elemental_requirement: Option<RecipeNodeElementalRequirement>,
    #[serde(default)]
    quality_requirement: Option<Quality>,
}

type EffectId = String;

type EffectLevel = u8;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct RecipeNodeEffect {
    id: EffectId,
    level: EffectLevel,
    count: ElementCount,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct RecipeNodeElementalRequirement {
    element: Element,
    count: ElementCount,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum Element {
    Fire,
    Ice,
    Lightning,
    Wind,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum RecipeNodeInput {
    Material(MaterialId),
    Category(Category),
}

/// Item category. Items may have more than one but should have at least one.
/// Probably won't use all of these.
#[derive(EnumSetType, Debug, Serialize, Deserialize, JsonSchema)]
#[enumset(serialize_as_list)]
#[serde(rename_all = "snake_case")]
enum Category {
    Water,
    Plants,
    Uni,
    Flowers,
    Medicinal,
    Poisons,
    Elixirs,
    Sand,
    Stone,
    Ore,
    Gemstones,
    Gunpowder,
    Fuel,
    Edibles,
    Fruit,
    Beehives,
    Mushrooms,
    Seafood,
    Bugs,
    Threads,
    Lumber,
    Gases,
    Puniballs,
    AnimalProducts,
    DragonMaterials,
    Magical,
    Neutralizers,
    GeneralGoods,
    Metal,
    Jewels,
    Spices,
    Seeds,
    Food,
    Medicine,
    Bombs,
    MagicTools,
    Ingots,
    Cloth,
    Weapons,
    Armor,
    Accessories,
    Tools,
    Furniture,
    KeyItems,
    Essence,
}

pub fn schema(output_path: &Path) -> anyhow::Result<()> {
    let schema = schema_for!(Items);
    File::create(output_path)?.write(&serde_json::to_vec_pretty(&schema)?)?;
    Ok(())
}

pub fn code(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    todo!()
}
