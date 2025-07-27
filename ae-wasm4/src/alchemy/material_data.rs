use crate::alchemy::{
    Category, Effect, Element, Material, Recipe, RecipeNode, RecipeNodeEffect,
    RecipeNodeElementalRequirement, RecipeNodeInput,
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

pub const GASOLINE: &Material = &Material {
    name: "Gasoline",
    icon: asset_data::item::POTION_DARK,
    categories: enum_set!(Category::Water | Category::Fuel),
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
                input: RecipeNodeInput::Material(WATER),
                effects: &[
                    RecipeNodeEffect {
                        id: Effect::Quality,
                        level: 1,
                        count: 1,
                    },
                    RecipeNodeEffect {
                        id: Effect::Quality,
                        level: 2,
                        count: 2,
                    },
                    RecipeNodeEffect {
                        id: Effect::Quality,
                        level: 3,
                        count: 3,
                    },
                ],
                elemental_requirement: None,
                quality_requirement: None,
                parent: None,
            },
            RecipeNode {
                grid_pos: (1, 0),
                element: Element::Fire,
                input: RecipeNodeInput::Category(Category::Flowers),
                effects: &[RecipeNodeEffect {
                    id: Effect::FireDmg,
                    level: 2,
                    count: 2,
                }],
                elemental_requirement: None,
                quality_requirement: None,
                parent: Some(0),
            },
            RecipeNode {
                grid_pos: (2, 0),
                element: Element::Fire,
                input: RecipeNodeInput::Category(Category::Fuel),
                effects: &[RecipeNodeEffect {
                    id: Effect::FireDmg,
                    level: 2,
                    count: 2,
                }],
                elemental_requirement: Some(RecipeNodeElementalRequirement {
                    element: Element::Fire,
                    count: 2,
                }),
                quality_requirement: None,
                parent: Some(1),
            },
        ],
    }),
};
