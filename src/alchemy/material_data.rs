use crate::alchemy::{
    Category, Effect, Element, Material, Recipe, RecipeNode, RecipeNodeEffect, RecipeNodeInput,
};
use crate::asset_data;
use enumset::enum_set;

pub const CRIMSON_ORE: &Material = &Material {
    name: "Crimson Ore",
    icon: asset_data::item::ORE_COPPER,
    categories: enum_set!(Category::Ore),
    recipe: None,
};

pub const SAND: &Material = &Material {
    name: "Sand",
    icon: asset_data::item::SAND,
    categories: enum_set!(Category::Sand),
    recipe: None,
};

pub const WATER: &Material = &Material {
    name: "Drinking Water",
    icon: asset_data::item::WATER,
    categories: enum_set!(Category::Water),
    recipe: None,
};

pub const RED_FLOWER: &Material = &Material {
    name: "Red Flower",
    icon: asset_data::item::FLOWER1,
    categories: enum_set!(Category::Flowers),
    recipe: None,
};

pub const RED_NEUTRALIZER: &Material = &Material {
    name: "Red Neutralizer",
    icon: asset_data::item::TEST_TUBE,
    categories: enum_set!(Category::Neutralizers),
    recipe: Some(Recipe {
        nodes: &[
            RecipeNode {
                grid_pos: (0, 0),
                element: Element::Ice,
                // input: RecipeNodeInput::Category(Category::Water),
                input: RecipeNodeInput::Material(WATER),
                effects: &[RecipeNodeEffect {
                    id: Effect::Quality,
                    level: 1,
                    count: 2,
                }],
                elemental_requirement: None,
                quality_requirement: None,
                linked: &[1],
            },
            RecipeNode {
                grid_pos: (1, 0),
                element: Element::Fire,
                // input: RecipeNodeInput::Category(Category::Flowers),
                input: RecipeNodeInput::Material(RED_FLOWER),
                effects: &[RecipeNodeEffect {
                    id: Effect::Quality,
                    level: 2,
                    count: 2,
                }],
                elemental_requirement: None,
                quality_requirement: None,
                linked: &[],
            },
        ],
    }),
};
