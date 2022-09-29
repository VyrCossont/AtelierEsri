use crate::font_data::{BUILTIN, TINY};
use crate::gfx::{ngon, thick_line, Lo5SplitSprite};
use crate::gfx_data::{
    BAR_COPPER, BOMB, CURSOR_POINT, FLOWER1, ORE_COPPER, RUBY, SAND, SCROLL, TEST_TUBE, WATER, WOOD,
};
use crate::wasm4::rect;
use crate::{input, wasm4};
use enumset::{enum_set, EnumSet, EnumSetType};

const SPACE: f32 = 40.0;
const H_SPACE_MUL: f32 = 0.8660254037844386;

struct Material<'a> {
    name: &'a str,
    icon: &'a Lo5SplitSprite<'a>,
    categories: EnumSet<Category>,
    recipe: Option<Recipe<'a>>,
}

struct Recipe<'a> {
    nodes: &'a [RecipeNode<'a>],
}

type Quality = u16;

type ElementCount = u8;

struct RecipeNode<'a> {
    grid_pos: (i8, i8),
    /// Element used for display and for effect levels.
    element: Element,
    input: RecipeNodeInput<'a>,
    effects: &'a [RecipeNodeEffect],
    elemental_requirement: Option<RecipeNodeElementalRequirement>,
    quality_requirement: Option<Quality>,
    linked: &'a [usize],
}

enum Effect {
    Quality,
    SynthQuantity,
    FireDmg,
}

type EffectLevel = u8;

struct RecipeNodeEffect {
    id: Effect,
    level: EffectLevel,
    count: ElementCount,
}

struct RecipeNodeElementalRequirement {
    element: Element,
    count: ElementCount,
}

#[derive(EnumSetType)]
enum Element {
    Fire,
    Ice,
    Lightning,
    Wind,
}

enum RecipeNodeInput<'a> {
    Material(&'a Material<'a>),
    Category(Category),
}

/// Item category. Items may have more than one but should have at least one.
/// Probably won't use all of these.
#[derive(EnumSetType)]
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

impl Category {
    pub fn name(&self) -> &str {
        match self {
            Category::Water => "(Water)",
            Category::Plants => "(Plants)",
            Category::Flowers => "(Flowers)",
            Category::Ore => "(Ore)",
            Category::Sand => "(Sand)",
            Category::Neutralizers => "(Neutralizers)",
            Category::Bombs => "(Bombs)",
            _ => todo!(),
        }
    }
}

// An actual instance of a material.
struct Item<'a> {
    material: &'a Material<'a>,
    elements: EnumSet<Element>,
    element_value: ElementCount,
    quality: Quality,
    categories: EnumSet<Category>,
}

struct SynthesisNode<'a> {
    recipe_node: &'a RecipeNode<'a>,
    items: Vec<&'a Item<'a>>,
}

impl SynthesisNode<'_> {
    fn new<'a>(recipe_node: &'a RecipeNode) -> SynthesisNode<'a> {
        SynthesisNode {
            recipe_node,
            items: Vec::new(),
        }
    }

    fn pos(&self, grid_origin: (i32, i32)) -> (i32, i32) {
        (
            grid_origin.0 + (self.recipe_node.grid_pos.0 as f32 * SPACE * H_SPACE_MUL) as i32,
            grid_origin.1
                + ((self.recipe_node.grid_pos.1 as f32
                    + if self.recipe_node.grid_pos.0 % 2 == 1 {
                        0.5
                    } else {
                        0.0
                    })
                    * SPACE) as i32,
        )
    }

    fn draw(&self, grid_origin: (i32, i32)) {
        let pos = self.pos(grid_origin);
        ngon(pos, 12, 6, 0.0, 3, 3);
        unsafe { *wasm4::DRAW_COLORS = 0x22 };
        wasm4::oval(pos.0 - 10, pos.1 - 10, 20, 20);
        match &self.recipe_node.input {
            RecipeNodeInput::Material(material) => material.icon.blit(pos.0 - 8, pos.1 - 8, 0),
            RecipeNodeInput::Category(category) => {
                let metrics = TINY.metrics(category.name());
                unsafe { *wasm4::DRAW_COLORS = 0x340 };
                TINY.text(
                    category.name(),
                    pos.0 - metrics.0 as i32 / 2,
                    pos.1 - metrics.1 as i32 / 2,
                );
            }
        }
    }
}

struct SynthesisState<'a> {
    material: &'a Material<'a>,
    nodes: Vec<SynthesisNode<'a>>,
}

impl SynthesisState<'_> {
    fn new<'a>(material: &'a Material) -> SynthesisState<'a> {
        SynthesisState {
            material,
            nodes: material
                .recipe
                .as_ref()
                .unwrap()
                .nodes
                .iter()
                .map(|recipe_node| SynthesisNode::new(recipe_node))
                .collect::<Vec<_>>(),
        }
    }

    fn draw(&self, grid_origin: (i32, i32)) {
        unsafe { *wasm4::DRAW_COLORS = 0x22 };
        wasm4::rect(160 - 4 - 40, 4, 40, 40);
        self.material.icon.blit2x(160 - 40, 4 + 4);
        let mut banner_text = String::from("Synthesizing: ");
        banner_text.push_str(self.material.name);
        let metrics = TINY.metrics(banner_text.as_str());
        unsafe { *wasm4::DRAW_COLORS = 0x22 };
        rect(4, 4, 160 - 8, metrics.1 + 2);
        unsafe { *wasm4::DRAW_COLORS = 0x340 };
        TINY.text(banner_text.as_str(), 4, 4);

        for node in &self.nodes {
            let node_pos = node.pos(grid_origin);
            for linked_node_index in node.recipe_node.linked {
                let linked_pos = self.nodes[*linked_node_index].pos(grid_origin);
                unsafe { *wasm4::DRAW_COLORS = 0x2 };
                thick_line(node_pos.0, node_pos.1, linked_pos.0, linked_pos.1, 3, 3);
            }
            node.draw(grid_origin);
        }
    }
}

const MAT_CRIMSON_ORE: &Material = &Material {
    name: "Crimson Ore",
    icon: &ORE_COPPER,
    categories: enum_set!(Category::Ore),
    recipe: None,
};

const MAT_SAND: &Material = &Material {
    name: "Sand",
    icon: &SAND,
    categories: enum_set!(Category::Sand),
    recipe: None,
};

const MAT_WATER: &Material = &Material {
    name: "Drinking Water",
    icon: &WATER,
    categories: enum_set!(Category::Water),
    recipe: None,
};

const MAT_RED_FLOWER: &Material = &Material {
    name: "Red Flower",
    icon: &FLOWER1,
    categories: enum_set!(Category::Flowers),
    recipe: None,
};

const MAT_RED_NEUTRALIZER: &Material = &Material {
    name: "Red Neutralizer",
    icon: &TEST_TUBE,
    categories: enum_set!(Category::Neutralizers),
    recipe: Some(Recipe {
        nodes: &[
            RecipeNode {
                grid_pos: (0, 0),
                element: Element::Ice,
                // input: RecipeNodeInput::Category(Category::Water),
                input: RecipeNodeInput::Material(MAT_WATER),
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
                input: RecipeNodeInput::Material(MAT_RED_FLOWER),
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

static mut SYNTHESIS_STATE: Option<SynthesisState> = None;

pub fn init() {
    unsafe {
        SYNTHESIS_STATE = Some(SynthesisState::new(MAT_RED_NEUTRALIZER));
    }
}

pub fn update() {
    let synthesis_state = unsafe { SYNTHESIS_STATE.as_ref().unwrap() };
    synthesis_state.draw((wasm4::SCREEN_SIZE as i32 / 2, wasm4::SCREEN_SIZE as i32 / 2));

    CURSOR_POINT.draw(input::mouse().pos);
}
