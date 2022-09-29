mod material_data;

use crate::font_data::TINY;
use crate::gfx::{ngon, thick_line, Lo5SplitSprite};
use crate::gfx_data::CURSOR_POINT;
use crate::{input, wasm4};
use enumset::{EnumSet, EnumSetType};

const SPACE: f32 = 40.0;
const H_SPACE_MUL: f32 = 0.8660254037844386;

pub struct Material<'a> {
    name: &'a str,
    icon: &'a Lo5SplitSprite<'a>,
    categories: EnumSet<Category>,
    recipe: Option<Recipe<'a>>,
}

pub struct Recipe<'a> {
    nodes: &'a [RecipeNode<'a>],
}

pub type Quality = u16;

pub type ElementCount = u8;

pub struct RecipeNode<'a> {
    grid_pos: (i8, i8),
    /// Element used for display and for effect levels.
    element: Element,
    input: RecipeNodeInput<'a>,
    effects: &'a [RecipeNodeEffect],
    elemental_requirement: Option<RecipeNodeElementalRequirement>,
    quality_requirement: Option<Quality>,
    parent: Option<usize>,
}

pub enum Effect {
    Quality,
    SynthQuantity,
    FireDmg,
}

pub type EffectLevel = u8;

pub struct RecipeNodeEffect {
    id: Effect,
    level: EffectLevel,
    count: ElementCount,
}

pub struct RecipeNodeElementalRequirement {
    element: Element,
    count: ElementCount,
}

#[derive(EnumSetType)]
pub enum Element {
    Fire,
    Ice,
    Lightning,
    Wind,
}

pub enum RecipeNodeInput<'a> {
    Material(&'a Material<'a>),
    Category(Category),
}

/// Item category. Items may have more than one but should have at least one.
/// Probably won't use all of these.
#[derive(EnumSetType)]
pub enum Category {
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

    fn center(&self, grid_origin: (i32, i32)) -> (i32, i32) {
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
        // Draw shape (normally a hexagon)
        let center = self.center(grid_origin);
        ngon(center, 13, 6, 0.0, 3, 4);
        unsafe { *wasm4::DRAW_COLORS = 0x22 };
        wasm4::oval(center.0 - 11, center.1 - 11, 21, 21);

        // Draw material icon or category name
        match &self.recipe_node.input {
            RecipeNodeInput::Material(material) => {
                material.icon.blit(center.0 - 8, center.1 - 8, 0)
            }
            RecipeNodeInput::Category(category) => {
                let metrics = TINY.metrics(category.name());
                let shadow_metrics = (metrics.0 + 2, metrics.1 + 2);
                unsafe { *wasm4::DRAW_COLORS = 0x22 };
                wasm4::rect(
                    center.0 - shadow_metrics.0 as i32 / 2,
                    center.1 - shadow_metrics.1 as i32 / 2,
                    shadow_metrics.0,
                    shadow_metrics.1,
                );
                unsafe { *wasm4::DRAW_COLORS = 0x340 };
                TINY.text(
                    category.name(),
                    center.0 - metrics.0 as i32 / 2,
                    center.1 - metrics.1 as i32 / 2,
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
        wasm4::rect(4 - 1, 4 - 1, 160 - 8, metrics.1 + 2);
        unsafe { *wasm4::DRAW_COLORS = 0x340 };
        TINY.text(banner_text.as_str(), 4, 4);

        for node in &self.nodes {
            let node_pos = node.center(grid_origin);
            if let Some(parent_node_index) = node.recipe_node.parent {
                let linked_pos = self.nodes[parent_node_index].center(grid_origin);
                unsafe { *wasm4::DRAW_COLORS = 0x2 };
                thick_line(node_pos.0, node_pos.1, linked_pos.0, linked_pos.1, 3, 3);
            }
            node.draw(grid_origin);
        }
    }
}

static mut SYNTHESIS_STATE: Option<SynthesisState> = None;

pub fn init() {
    unsafe {
        SYNTHESIS_STATE = Some(SynthesisState::new(material_data::RED_NEUTRALIZER));
    }
}

pub fn update() {
    let synthesis_state = unsafe { SYNTHESIS_STATE.as_ref().unwrap() };
    synthesis_state.draw((wasm4::SCREEN_SIZE as i32 / 2, wasm4::SCREEN_SIZE as i32 / 2));

    CURSOR_POINT.draw(input::mouse().pos);
}
