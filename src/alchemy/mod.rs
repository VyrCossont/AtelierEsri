mod material_data;

use crate::font_data::TINY;
use crate::gfx::{ngon, ngon_points, thick_line, Lo5SplitSprite};
use crate::gfx_data::CURSOR_POINT;
use crate::{asset_data, input, wasm4};
use enumset::{enum_set, EnumSet, EnumSetType};
use std::f32::consts::PI;

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

impl Element {
    pub const ALL: EnumSet<Element> =
        enum_set!(Element::Fire | Element::Ice | Element::Lightning | Element::Wind);

    pub fn icon(&self) -> &Lo5SplitSprite {
        match self {
            Element::Fire => asset_data::element::FIRE7,
            Element::Ice => asset_data::element::ICE7,
            Element::Lightning => asset_data::element::LIGHTNING7,
            Element::Wind => asset_data::element::WIND7,
        }
    }
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
            Category::Uni => "(Uni)",
            Category::Flowers => "(Flowers)",
            Category::Medicinal => "(Medicinal)",
            Category::Poisons => "(Poisons)",
            Category::Elixirs => "(Elixirs)",
            Category::Sand => "(Sand)",
            Category::Stone => "(Stone)",
            Category::Ore => "(Ore)",
            Category::Gemstones => "(Gemstones)",
            Category::Gunpowder => "(Gunpowder)",
            Category::Fuel => "(Fuel)",
            Category::Edibles => "(Edibles)",
            Category::Fruit => "(Fruit)",
            Category::Beehives => "(Beehives)",
            Category::Mushrooms => "(Mushrooms)",
            Category::Seafood => "(Seafood)",
            Category::Bugs => "(Bugs)",
            Category::Threads => "(Threads)",
            Category::Lumber => "(Lumber)",
            Category::Gases => "(Gases)",
            Category::Puniballs => "(Puniballs)",
            Category::AnimalProducts => "(AnimalProducts)",
            Category::DragonMaterials => "(DragonMaterials)",
            Category::Magical => "(Magical)",
            Category::Neutralizers => "(Neutralizers)",
            Category::GeneralGoods => "(GeneralGoods)",
            Category::Metal => "(Metal)",
            Category::Jewels => "(Jewels)",
            Category::Spices => "(Spices)",
            Category::Seeds => "(Seeds)",
            Category::Food => "(Food)",
            Category::Medicine => "(Medicine)",
            Category::Bombs => "(Bombs)",
            Category::MagicTools => "(MagicTools)",
            Category::Ingots => "(Ingots)",
            Category::Cloth => "(Cloth)",
            Category::Weapons => "(Weapons)",
            Category::Armor => "(Armor)",
            Category::Accessories => "(Accessories)",
            Category::Tools => "(Tools)",
            Category::Furniture => "(Furniture)",
            Category::KeyItems => "(KeyItems)",
            Category::Essence => "(Essence)",
        }
    }

    /// Shorter codes that fit in a zoomed-out node.
    pub fn fourcc(&self) -> &str {
        match self {
            Category::Water => "(Watr)",
            Category::Plants => "(Plnt)",
            Category::Uni => "(Uni)",
            Category::Flowers => "(Flwr)",
            Category::Medicinal => "(Mdcl)",
            Category::Poisons => "(Pois)",
            Category::Elixirs => "(Elix)",
            Category::Sand => "(Sand)",
            Category::Stone => "(Ston)",
            Category::Ore => "(Ore)",
            Category::Gemstones => "(Gems)",
            Category::Gunpowder => "(Gnpw)",
            Category::Fuel => "(Fuel)",
            Category::Edibles => "(Edbl)",
            Category::Fruit => "(Frut)",
            Category::Beehives => "(Bhvs)",
            Category::Mushrooms => "(Mush)",
            Category::Seafood => "(Sfud)",
            Category::Bugs => "(Bugs)",
            Category::Threads => "(Thrd)",
            Category::Lumber => "(Lumb)",
            Category::Gases => "(Gas)",
            Category::Puniballs => "(Puni)",
            Category::AnimalProducts => "(Anim)",
            Category::DragonMaterials => "(Drgn)",
            Category::Magical => "(Magc)",
            Category::Neutralizers => "(Neut)",
            Category::GeneralGoods => "(GnGd)",
            Category::Metal => "(Metl)",
            Category::Jewels => "(Jewl)",
            Category::Spices => "(Spic)",
            Category::Seeds => "(Seed)",
            Category::Food => "(Food)",
            Category::Medicine => "(Mdcn)",
            Category::Bombs => "(Bomb)",
            Category::MagicTools => "(MgTl)",
            Category::Ingots => "(Ingt)",
            Category::Cloth => "(Clth)",
            Category::Weapons => "(Weap)",
            Category::Armor => "(Armr)",
            Category::Accessories => "(Accs)",
            Category::Tools => "(Tool)",
            Category::Furniture => "(Furn)",
            Category::KeyItems => "(Key)",
            Category::Essence => "(Esnc)",
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
        let radius = 13;
        // Draw shape (normally a hexagon)
        let center = self.center(grid_origin);
        ngon(center, radius, 6, 0.0, 3, 4);
        unsafe { *wasm4::DRAW_COLORS = 0x22 };
        wasm4::oval(center.0 - 11, center.1 - 11, 21, 21);

        // Draw material icon or category name
        match &self.recipe_node.input {
            RecipeNodeInput::Material(material) => {
                material.icon.blit(center.0 - 8, center.1 - 8, 0)
            }
            RecipeNodeInput::Category(category) => {
                let metrics = TINY.metrics(category.fourcc());
                let shadow_metrics = (metrics.0 + 2, metrics.1 + 2);
                unsafe { *wasm4::DRAW_COLORS = 0x22 };
                wasm4::rect(
                    center.0 - shadow_metrics.0 as i32 / 2 - 1,
                    center.1 - shadow_metrics.1 as i32 / 2 - 1,
                    shadow_metrics.0,
                    shadow_metrics.1,
                );
                unsafe { *wasm4::DRAW_COLORS = 0x340 };
                TINY.text(
                    category.fourcc(),
                    center.0 - metrics.0 as i32 / 2 - 1,
                    center.1 - metrics.1 as i32 / 2 - 1,
                );
            }
        }

        // Draw effect level element icons.
        if let Some((effect, mut value)) = self.active_effect() {
            let mut slots = effect.count;
            for slot_center in ngon_points(6, center.into(), radius + 2, PI / -3.0) {
                if slots == 0 {
                    break;
                }
                let icon = if value > 0 {
                    self.recipe_node.element.icon()
                } else {
                    asset_data::element::EMPTY7
                };
                icon.blit(slot_center.x - 4, slot_center.y - 4, 0);
                value = value.saturating_sub(1);
                slots -= 1;
            }
        }

        // Draw lock.
        // TODO: unlock when parent meets requirements
        if let Some(req) = &self.recipe_node.elemental_requirement {
            asset_data::element::LOCK7.blit(center.0 - 10, center.1 + 4, 0);

            req.element.icon().blit(center.0 - 2, center.1 + 5, 0);

            let unlock_count = req.count.to_string();
            let metrics = TINY.metrics(&unlock_count);
            let shadow_metrics = (metrics.0 + 2, metrics.1 + 2);
            unsafe { *wasm4::DRAW_COLORS = 0x22 };
            wasm4::rect(
                center.0 + 8 - shadow_metrics.0 as i32 / 2 - 1,
                center.1 + 8 - shadow_metrics.1 as i32 / 2 - 1,
                shadow_metrics.0,
                shadow_metrics.1,
            );
            unsafe { *wasm4::DRAW_COLORS = 0x340 };
            TINY.text(
                &unlock_count,
                center.0 + 8 - metrics.0 as i32 / 2 - 1,
                center.1 + 8 - metrics.1 as i32 / 2 - 1,
            );
        }
    }

    /// Total element value of items on this node that match the node's element.
    /// TODO: Effect Spread +N effects on materials can go both forwards and backwards.
    ///     We'll need to scan the graph in both directions up to max(N) nodes away.
    ///     Note that Effect Spread doesn't go through nodes with halos around them.
    fn item_value(&self) -> ElementCount {
        let mut value = 0;
        for item in &self.items {
            if item.elements.contains(self.recipe_node.element.clone()) {
                value += item.element_value;
            }
        }
        value
    }

    /// Return the effect we're currently trying to fill,
    /// and how many elements we have filled on the current level.
    /// If we have enough element value to fill up the last effect,
    /// return the last effect, completely filled.
    /// Some nodes may not have any effects so we don't return anything.
    /// TODO: are there real recipes with nodes with no effects?
    fn active_effect(&self) -> Option<(&RecipeNodeEffect, ElementCount)> {
        let mut value = self.item_value();
        for effect in self.recipe_node.effects {
            if value < effect.count {
                return Some((effect, value));
            }
            value = value.saturating_sub(effect.count);
        }
        return self
            .recipe_node
            .effects
            .last()
            .map(|effect| (effect, effect.count));
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
        // Draw recipe metadata.
        unsafe { *wasm4::DRAW_COLORS = 0x22 };
        wasm4::rect(160 - 40, 0, 40, 40);
        self.material.icon.blit2x(160 - 40 + 4, 4);
        let mut banner_text = String::from("Synthesizing: ");
        banner_text.push_str(self.material.name);
        let metrics = TINY.metrics(banner_text.as_str());
        unsafe { *wasm4::DRAW_COLORS = 0x22 };
        wasm4::rect(0, 0, 160 - 8, metrics.1 + 2);
        unsafe { *wasm4::DRAW_COLORS = 0x340 };
        TINY.text(banner_text.as_str(), 1, 1);

        // Draw node interconnect lines.
        // TODO: nodes should draw their own interconnect lines
        //  so they can decide whether to show them active or not.
        for node in &self.nodes {
            let node_pos = node.center(grid_origin);
            if let Some(parent_node_index) = node.recipe_node.parent {
                let linked_pos = self.nodes[parent_node_index].center(grid_origin);
                unsafe { *wasm4::DRAW_COLORS = 0x2 };
                thick_line(node_pos.0, node_pos.1, linked_pos.0, linked_pos.1, 3, 3);
            }
        }

        // Draw nodes on top of the lines.
        for node in &self.nodes {
            node.draw(grid_origin);
        }

        // Draw inventory.
        unsafe { *wasm4::DRAW_COLORS = 0x22 };
        wasm4::rect(0, 100, 100, 60);
        unsafe { *wasm4::DRAW_COLORS = 0x340 };
        TINY.text("Inventory:", 1, 100 + 1);
        for (i, item) in unsafe { &INVENTORY }.iter().enumerate() {
            unsafe { *wasm4::DRAW_COLORS = 0x23 };
            let y = 100 + 6 + i as i32 * 17;
            wasm4::rect(0, y, 100, 17 + 1);
            item.material.icon.blit(1, y + 1, 0);
            let x_col2 = 16 + 2;
            unsafe { *wasm4::DRAW_COLORS = 0x210 };
            TINY.text(item.material.name, x_col2, y + 2);
            let y_element = y + 9;
            let mut x_element = x_col2;
            for element in Element::ALL {
                let icon = if item.elements.contains(element) {
                    element.icon()
                } else {
                    asset_data::element::EMPTY7
                };
                icon.blit(x_element, y_element, 0);
                x_element += 8;
            }
            unsafe { *wasm4::DRAW_COLORS = 0x210 };
            let y_row2_text = y_element + 1;
            x_element += 1;
            TINY.text(&item.element_value.to_string(), x_element, y_row2_text);
            let mut quality = "Quality: ".to_string();
            quality.push_str(&item.quality.to_string());
            let quality_metrics = TINY.metrics(&quality);
            TINY.text(&quality, 100 - 2 - quality_metrics.0 as i32, y_row2_text);

            if unsafe { INVENTORY_SELECTION[i] } {
                unsafe { *wasm4::DRAW_COLORS = 0x4 };
                thick_line(1, y + 4, 3, y + 6, 3, 3);
                thick_line(3, y + 6, 7, y + 1, 3, 3);
                unsafe { *wasm4::DRAW_COLORS = 0x1 };
                wasm4::line(1 + 1, y + 4 + 1, 3 + 1, y + 6 + 1);
                wasm4::line(3 + 1, y + 6 + 1, 7 + 1, y + 1 + 1);
            }
        }
    }
}

static mut SYNTHESIS_STATE: Option<SynthesisState> = None;

static mut INVENTORY: Vec<Item> = vec![];
static mut INVENTORY_SELECTION: Vec<bool> = vec![];

pub fn init() {
    unsafe {
        INVENTORY.push(Item {
            material: &material_data::GASOLINE,
            elements: enum_set!(Element::Fire),
            element_value: 2,
            quality: 93,
            categories: enum_set!(Category::Water),
        });
        INVENTORY_SELECTION.push(false);

        INVENTORY.push(Item {
            material: &material_data::WATER,
            elements: enum_set!(Element::Ice),
            element_value: 2,
            quality: 58,
            categories: enum_set!(Category::Water),
        });
        INVENTORY_SELECTION.push(true);

        let mut synthesis_state = SynthesisState::new(material_data::RED_NEUTRALIZER);
        synthesis_state.nodes[0].items.push(&INVENTORY[1]);
        SYNTHESIS_STATE = Some(synthesis_state);
    }
}

pub fn update() {
    let synthesis_state = unsafe { SYNTHESIS_STATE.as_ref().unwrap() };
    synthesis_state.draw((wasm4::SCREEN_SIZE as i32 / 2, wasm4::SCREEN_SIZE as i32 / 2));

    CURSOR_POINT.draw(input::mouse().pos);
}
