#include "Material.hpp"

#include <cassert>

#include "Assets.h"

namespace AtelierEsri {

std::vector<Material> Material::Catalog(
    const std::vector<Breeze::Material>& breezeCatalog
) {
  std::vector<Material> catalog = {
      // Raw materials
      {.name = "Bacon",
       .description = "A delicious slice of hickory-smoked pig meat. Thank you "
                      "for your service, Mr. Pig.",
       .spriteIndex = assetSpriteSheet00ItemBaconSpriteIndex},
      {.name = "Bud",
       .description = "Part of a plant. What might it open to become?",
       .spriteIndex = assetSpriteSheet00ItemBudSpriteIndex},
      {.name = "Crystal",
       .description = "A shiny chunk of rock quartz.",
       .spriteIndex = assetSpriteSheet00ItemCrystalSpriteIndex},
      {.name = "Dragon Eye",
       .description = "Boy howdy, the dragon did not want to part with this.",
       .spriteIndex = assetSpriteSheet00ItemDragonEyeSpriteIndex},
      {.name = "Dunkelheit",
       .description = "A mysterious flower. Obnoxiously expensive.",
       .spriteIndex = assetSpriteSheet00ItemDunkelheitSpriteIndex},
      {.name = "Elerium",
       .description = "A crystal of a stable isotope of element 115. The key "
                      "to fine control of gravity.",
       .spriteIndex = assetSpriteSheet00ItemEleriumSpriteIndex},
      {.name = "Feather",
       .description = "A pretty striped tail feather from a bird of prey.",
       .spriteIndex = assetSpriteSheet00ItemFeatherSpriteIndex},
      {.name = "Flower 1",
       .description = "Some kind of flower.",
       .spriteIndex = assetSpriteSheet00ItemFlower1SpriteIndex},
      {.name = "Flower 2",
       .description = "Some other kind of flower.",
       .spriteIndex = assetSpriteSheet00ItemFlower2SpriteIndex},
      {.name = "Grapes",
       .description = "Leave these in the pantry for long enough and you're "
                      "halfway to wine.",
       .spriteIndex = assetSpriteSheet00ItemGrapesSpriteIndex},
      {.name = "Grass",
       .description = "Small and green but has a lot of friends.",
       .spriteIndex = assetSpriteSheet00ItemGrassSpriteIndex},
      {.name = "Gravistone",
       .description = "A hovering rock. May contain traces of elerium.",
       .spriteIndex = assetSpriteSheet00ItemGravistoneSpriteIndex},
      {.name = "Herb",
       .description =
           "Flavorful plants from the garden. Esri, you shouldn't smoke these.",
       .spriteIndex = assetSpriteSheet00ItemHerbSpriteIndex},
      {.name = "Leaf Down",
       .description = "A fallen leaf from a tree.",
       .spriteIndex = assetSpriteSheet00ItemLeafDownSpriteIndex},
      {.name = "Leaf Triple",
       .description = "Sprig of triple taun, or possibly poison ivy. Note to "
                      "self: check forestry guide.",
       .spriteIndex = assetSpriteSheet00ItemLeafTripleSpriteIndex},
      {.name = "Leaf Up",
       .description = "A perky leaf from a shrub.",
       .spriteIndex = assetSpriteSheet00ItemLeafUpSpriteIndex},
      {.name = "Lump",
       .description = "An undifferentiated mass of who knows what.",
       .spriteIndex = assetSpriteSheet00ItemLumpSpriteIndex},
      {.name = "Mushroom 1",
       .description = "Either delicious with wine and onions, or instantly "
                      "annihilates your liver. Maybe both.",
       .spriteIndex = assetSpriteSheet00ItemMushroom1SpriteIndex},
      {.name = "Mushroom 2",
       .description = "A forest gnome could wear this as a hat.",
       .spriteIndex = assetSpriteSheet00ItemMushroom2SpriteIndex},
      {.name = "Copper Ore",
       .description = "A hefty chunk of malachite with a pleasing green color.",
       .spriteIndex = assetSpriteSheet00ItemOreCopperSpriteIndex},
      {.name = "Iron Ore",
       .description = "A shiny cluster of hematite orbs.",
       .spriteIndex = assetSpriteSheet00ItemOreIronSpriteIndex},
      {.name = "Silver Ore",
       .description = "Not quite up to the gold standard.",
       .spriteIndex = assetSpriteSheet00ItemOreSilverSpriteIndex},
      {.name = "Stygium Ore",
       .description = "A mysterious mineral that radiates darkness. Don't "
                      "leave it on the floor or you'll trip over it at night.",
       .spriteIndex = assetSpriteSheet00ItemOreStygiumSpriteIndex},
      {.name = "Titanium Ore",
       .description = "A reddish-brown group of needle-like rutile crystals.",
       .spriteIndex = assetSpriteSheet00ItemOreTitaniumSpriteIndex},
      {.name = "Page",
       .description = "Ripped out of a spellbook. One weird trick to make "
                      "witches hate you.",
       .spriteIndex = assetSpriteSheet00ItemPageSpriteIndex},
      {.name = "Palm",
       .description = "Tropical plant material from a breezy beach somewhere.",
       .spriteIndex = assetSpriteSheet00ItemPalmSpriteIndex},
      {.name = "Pendeloque",
       .description = "A crystal left behind by ghosts. Raises serious "
                      "questions about what happens to us when we die.",
       .spriteIndex = assetSpriteSheet00ItemPendeloqueSpriteIndex},
      {.name = "Pods",
       .description = "Some sort of arthropod egg. I really hope these don't "
                      "hatch in the pantry.",
       .spriteIndex = assetSpriteSheet00ItemPodsSpriteIndex},
      {.name = "Puniball",
       .description =
           "The core organ from a puni. Incredibly useful to alchemists.",
       .spriteIndex = assetSpriteSheet00ItemPuniballSpriteIndex},
      {.name = "Giant Puniball",
       .description = "The core organ from a rare giant puni. Throbbing with "
                      "stored magic.",
       .spriteIndex = assetSpriteSheet00ItemPuniballGiantSpriteIndex},
      {.name = "Rock",
       .description =
           "Everybody wants a rock to wind a piece of string around.",
       .spriteIndex = assetSpriteSheet00ItemRockSpriteIndex},
      {.name = "Sand",
       .description =
           "The remnants of an ancient rock that just couldn't even.",
       .spriteIndex = assetSpriteSheet00ItemSandSpriteIndex},
      {.name = "Seaweed 1",
       .description = "A ribbon-like algae. The extract is useful for baking.",
       .spriteIndex = assetSpriteSheet00ItemSeaweed1SpriteIndex},
      {.name = "Seaweed 2",
       .description = "Fronds of a tangled underwater plant.",
       .spriteIndex = assetSpriteSheet00ItemSeaweed2SpriteIndex},
      {.name = "Spider",
       .description = "A venomous predatory arachnid. Before you feel bad for "
                      "it, remember that the bigger ones will happily hunt us.",
       .spriteIndex = assetSpriteSheet00ItemSpiderSpriteIndex},
      {.name = "Spirit",
       .description =
           "The semi-tangible remnant left behind when we defeated a massive "
           "monster. Is this a soul? Should the answer affect whether we use "
           "it in alchemy? Does this count as using every part of the animal?",
       .spriteIndex = assetSpriteSheet00ItemSpiritSpriteIndex},
      {.name = "Steak",
       .description = "An uncomplicated slice of cow. Mmmmm.",
       .spriteIndex = assetSpriteSheet00ItemSteakSpriteIndex},
      {.name = "Sulfur",
       .description = "A stinky yellow mineral found near volcanic vents.",
       .spriteIndex = assetSpriteSheet00ItemSulfurSpriteIndex},
      {.name = "Uni",
       .description = "A spiky pod full of seeds. May explode when thrown.",
       .spriteIndex = assetSpriteSheet00ItemUniSpriteIndex},
      {.name = "Water",
       .description = "It's what the pros drink. Stay hydrated!",
       .spriteIndex = assetSpriteSheet00ItemWaterSpriteIndex},
      {.name = "Wood",
       .description = "Useful for construction, heating, and keeping the green "
                      "parts of trees off the ground.",
       .spriteIndex = assetSpriteSheet00ItemWoodSpriteIndex},
      {.name = "Worm",
       .description = "A creepy crawler.",
       .spriteIndex = assetSpriteSheet00ItemWormSpriteIndex},

      // Basic recipes
      {.name = "Copper Ingot",
       .description =
           "Your basic household metal. Shiny, provided you keep it polished.",
       .spriteIndex = assetSpriteSheet00ItemBarCopperSpriteIndex},
  };
  assert(breezeCatalog.size() == catalog.size());
  return catalog;
}

}  // namespace AtelierEsri
