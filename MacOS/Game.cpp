#include "Game.hpp"

#include <ctgmath>

#include "Assets.h"
#include "Drawing.hpp"
#include "MaskedImage.hpp"

namespace AtelierEsri {

Game Game::Setup(Window &window) {
  MaskedImage spriteSheetImage = MaskedImage::Get(
      assetSpriteSheet00ImagePictResourceId,
      assetSpriteSheet00MaskPictResourceId,
      window
  );
  SpriteSheet spriteSheet = SpriteSheet::New(
      std::move(spriteSheetImage), assetSpriteSheet00RgnResourceId
  );
  auto game = Game(std::move(spriteSheet));

  // Give two of every raw material.
  const std::vector<Material> catalog = Material::Catalog();
  for (size_t materialIndex = 1; materialIndex < catalog.size();
       ++materialIndex) {
    const Breeze::Material &material = catalog[materialIndex].data;
    Breeze::Item item = {
        .material = material,
        .elements = material.elements,
        .elementValue = material.elementValue,
        .quality = 50,
        .categories = material.categories,
        .traits = material.traits
    };
    for (size_t itemIndex = 0; itemIndex < 2; ++itemIndex) {
      game.inventory.push_back(item);
    }
  }

  return game;
}

void Game::Update(const int16_t scrollBarPosition) {
  yOffset = static_cast<int16_t>(scrollBarPosition - 50);
}

void Game::Draw(GWorld &gWorld) {
  QD::Reset();

  GWorldActiveGuard activeGuard = gWorld.MakeActive();

  const Rect rect = gWorld.Bounds();
  const Pattern background = QD::Gray();
  FillRect(&rect, &background);

  const Rect dstRect = {yOffset, 0, static_cast<int16_t>(64 + yOffset), 64};
  spriteSheet.Draw(gWorld, assetSpriteSheet00AvatarEsriSpriteIndex, dstRect);

  const auto hex =
      Ngon({120, static_cast<int16_t>(120 + yOffset)}, 32, 6, M_PI + M_PI_2);
  {
    const ManagedPolygon polygon = hex.Polygon();
    OffsetPoly(polygon.get(), -2, -2);
    QD_CHECKED(ErasePoly(polygon.get()), "Couldn't clear hexagon");
    PenSize(4, 4);
    QD_CHECKED(FramePoly(polygon.get()), "Couldn't draw hexagon");
  }
  {
    const Pattern pattern = QD::White();
    PenSize(2, 2);
    for (uint8_t i = 0; i < 6; i++) {
      // ReSharper disable once CppUseStructuredBinding
      const Point center = hex[i];
      Rect nodeRect;
      constexpr int16_t nodeR = 6;
      nodeRect.left = static_cast<int16_t>(center.h - nodeR);
      nodeRect.right = static_cast<int16_t>(center.h + nodeR);
      nodeRect.top = static_cast<int16_t>(center.v - nodeR);
      nodeRect.bottom = static_cast<int16_t>(center.v + nodeR);
      FillOval(&nodeRect, &pattern);
      FrameOval(&nodeRect);

      if (i < 3) {
        Rect pipRect;
        pipRect.left = static_cast<int16_t>(center.h - 4);
        pipRect.right = static_cast<int16_t>(center.h + 4);
        pipRect.top = static_cast<int16_t>(center.v - 4);
        pipRect.bottom = static_cast<int16_t>(center.v + 4);
        spriteSheet.Draw(
            gWorld, assetSpriteSheet00ElementFireSpriteIndex, pipRect
        );
      }
    }
  }
}

Game::Game(SpriteSheet &&spriteSheet) : spriteSheet(std::move(spriteSheet)) {}

std::vector<Material> Material::Catalog() {
  const std::vector<Breeze::Material> breezeCatalog =
      Breeze::Material::Catalog();
  return {
      // Raw materials
      {.data = breezeCatalog[0],
       .name = "Bacon",
       .description = "A delicious slice of hickory-smoked pig meat. Thank you "
                      "for your service, Mr. Pig.",
       .spriteIndex = assetSpriteSheet00ItemBaconSpriteIndex},
      {.data = breezeCatalog[1],
       .name = "Bud",
       .description = "Part of a plant. What might it open to become?",
       .spriteIndex = assetSpriteSheet00ItemBudSpriteIndex},
      {.data = breezeCatalog[2],
       .name = "Crystal",
       .description = "A shiny chunk of rock quartz.",
       .spriteIndex = assetSpriteSheet00ItemCrystalSpriteIndex},
      {.data = breezeCatalog[3],
       .name = "Dragon Eye",
       .description = "Boy howdy, the dragon did not want to part with this.",
       .spriteIndex = assetSpriteSheet00ItemDragonEyeSpriteIndex},
      {.data = breezeCatalog[4],
       .name = "Dunkelheit",
       .description = "A mysterious flower. Obnoxiously expensive.",
       .spriteIndex = assetSpriteSheet00ItemDunkelheitSpriteIndex},
      {.data = breezeCatalog[5],
       .name = "Elerium",
       .description = "A crystal of a stable isotope of element 115. The key "
                      "to fine control of gravity.",
       .spriteIndex = assetSpriteSheet00ItemEleriumSpriteIndex},
      {.data = breezeCatalog[6],
       .name = "Feather",
       .description = "A pretty striped tail feather from a bird of prey.",
       .spriteIndex = assetSpriteSheet00ItemFeatherSpriteIndex},
      {.data = breezeCatalog[7],
       .name = "Flower 1",
       .description = "Some kind of flower.",
       .spriteIndex = assetSpriteSheet00ItemFlower1SpriteIndex},
      {.data = breezeCatalog[8],
       .name = "Flower 2",
       .description = "Some other kind of flower.",
       .spriteIndex = assetSpriteSheet00ItemFlower2SpriteIndex},
      {.data = breezeCatalog[9],
       .name = "Grapes",
       .description = "Leave these in the pantry for long enough and you're "
                      "halfway to wine.",
       .spriteIndex = assetSpriteSheet00ItemGrapesSpriteIndex},
      {.data = breezeCatalog[10],
       .name = "Grass",
       .description = "Small and green but has a lot of friends.",
       .spriteIndex = assetSpriteSheet00ItemGrassSpriteIndex},
      {.data = breezeCatalog[11],
       .name = "Gravistone",
       .description = "A hovering rock. May contain traces of elerium.",
       .spriteIndex = assetSpriteSheet00ItemGravistoneSpriteIndex},
      {.data = breezeCatalog[12],
       .name = "Herb",
       .description =
           "Flavorful plants from the garden. Esri, you shouldn't smoke these.",
       .spriteIndex = assetSpriteSheet00ItemHerbSpriteIndex},
      {.data = breezeCatalog[13],
       .name = "Leaf Down",
       .description = "A fallen leaf from a tree.",
       .spriteIndex = assetSpriteSheet00ItemLeafDownSpriteIndex},
      {.data = breezeCatalog[14],
       .name = "Leaf Triple",
       .description = "Sprig of triple taun, or possibly poison ivy. Note to "
                      "self: check forestry guide.",
       .spriteIndex = assetSpriteSheet00ItemLeafTripleSpriteIndex},
      {.data = breezeCatalog[15],
       .name = "Leaf Up",
       .description = "A perky leaf from a shrub.",
       .spriteIndex = assetSpriteSheet00ItemLeafUpSpriteIndex},
      {.data = breezeCatalog[16],
       .name = "Lump",
       .description = "An undifferentiated mass of who knows what.",
       .spriteIndex = assetSpriteSheet00ItemLumpSpriteIndex},
      {.data = breezeCatalog[17],
       .name = "Mushroom 1",
       .description = "Either delicious with wine and onions, or instantly "
                      "annihilates your liver. Maybe both.",
       .spriteIndex = assetSpriteSheet00ItemMushroom1SpriteIndex},
      {.data = breezeCatalog[18],
       .name = "Mushroom 2",
       .description = "A forest gnome could wear this as a hat.",
       .spriteIndex = assetSpriteSheet00ItemMushroom2SpriteIndex},
      {.data = breezeCatalog[19],
       .name = "Copper Ore",
       .description = "A hefty chunk of malachite with a pleasing green color.",
       .spriteIndex = assetSpriteSheet00ItemOreCopperSpriteIndex},
      {.data = breezeCatalog[20],
       .name = "Iron Ore",
       .description = "A shiny cluster of hematite orbs.",
       .spriteIndex = assetSpriteSheet00ItemOreIronSpriteIndex},
      {.data = breezeCatalog[21],
       .name = "Silver Ore",
       .description = "Not quite up to the gold standard.",
       .spriteIndex = assetSpriteSheet00ItemOreSilverSpriteIndex},
      {.data = breezeCatalog[22],
       .name = "Stygium Ore",
       .description = "A mysterious mineral that radiates darkness. Don't "
                      "leave it on the floor or you'll trip over it at night.",
       .spriteIndex = assetSpriteSheet00ItemOreStygiumSpriteIndex},
      {.data = breezeCatalog[23],
       .name = "Titanium Ore",
       .description = "A reddish-brown group of needle-like rutile crystals.",
       .spriteIndex = assetSpriteSheet00ItemOreTitaniumSpriteIndex},
      {.data = breezeCatalog[24],
       .name = "Page",
       .description = "Ripped out of a spellbook. One weird trick to make "
                      "witches hate you.",
       .spriteIndex = assetSpriteSheet00ItemPageSpriteIndex},
      {.data = breezeCatalog[25],
       .name = "Palm",
       .description = "Tropical plant material from a breezy beach somewhere.",
       .spriteIndex = assetSpriteSheet00ItemPalmSpriteIndex},
      {.data = breezeCatalog[26],
       .name = "Pendeloque",
       .description = "A crystal left behind by ghosts. Raises serious "
                      "questions about what happens to us when we die.",
       .spriteIndex = assetSpriteSheet00ItemPendeloqueSpriteIndex},
      {.data = breezeCatalog[27],
       .name = "Pods",
       .description = "Some sort of arthropod egg. I really hope these don't "
                      "hatch in the pantry.",
       .spriteIndex = assetSpriteSheet00ItemPodsSpriteIndex},
      {.data = breezeCatalog[28],
       .name = "Puniball",
       .description =
           "The core organ from a puni. Incredibly useful to alchemists.",
       .spriteIndex = assetSpriteSheet00ItemPuniballSpriteIndex},
      {.data = breezeCatalog[29],
       .name = "Giant Puniball",
       .description = "The core organ from a rare giant puni. Throbbing with "
                      "stored magic.",
       .spriteIndex = assetSpriteSheet00ItemPuniballGiantSpriteIndex},
      {.data = breezeCatalog[30],
       .name = "Rock",
       .description =
           "Everybody wants a rock to wind a piece of string around.",
       .spriteIndex = assetSpriteSheet00ItemRockSpriteIndex},
      {.data = breezeCatalog[31],
       .name = "Sand",
       .description =
           "The remnants of an ancient rock that just couldn't even.",
       .spriteIndex = assetSpriteSheet00ItemSandSpriteIndex},
      {.data = breezeCatalog[32],
       .name = "Seaweed 1",
       .description = "A ribbon-like algae. The extract is useful for baking.",
       .spriteIndex = assetSpriteSheet00ItemSeaweed1SpriteIndex},
      {.data = breezeCatalog[33],
       .name = "Seaweed 2",
       .description = "Fronds of a tangled underwater plant.",
       .spriteIndex = assetSpriteSheet00ItemSeaweed2SpriteIndex},
      {.data = breezeCatalog[34],
       .name = "Spider",
       .description = "A venomous predatory arachnid. Before you feel bad for "
                      "it, remember that the bigger ones will happily hunt us.",
       .spriteIndex = assetSpriteSheet00ItemSpiderSpriteIndex},
      {.data = breezeCatalog[35],
       .name = "Spirit",
       .description =
           "The semi-tangible remnant left behind when we defeated a massive "
           "monster. Is this a soul? Should the answer affect whether we use "
           "it in alchemy? Does this count as using every part of the animal?",
       .spriteIndex = assetSpriteSheet00ItemSpiritSpriteIndex},
      {.data = breezeCatalog[36],
       .name = "Steak",
       .description = "An uncomplicated slice of cow. Mmmmm.",
       .spriteIndex = assetSpriteSheet00ItemSteakSpriteIndex},
      {.data = breezeCatalog[37],
       .name = "Sulfur",
       .description = "A stinky yellow mineral found near volcanic vents.",
       .spriteIndex = assetSpriteSheet00ItemSulfurSpriteIndex},
      {.data = breezeCatalog[38],
       .name = "Uni",
       .description = "A spiky pod full of seeds. May explode when thrown.",
       .spriteIndex = assetSpriteSheet00ItemUniSpriteIndex},
      {.data = breezeCatalog[39],
       .name = "Water",
       .description = "It's what the pros drink. Stay hydrated!",
       .spriteIndex = assetSpriteSheet00ItemWaterSpriteIndex},
      {.data = breezeCatalog[40],
       .name = "Wood",
       .description = "Useful for construction, heating, and keeping the green "
                      "parts of trees off the ground.",
       .spriteIndex = assetSpriteSheet00ItemWoodSpriteIndex},
      {.data = breezeCatalog[41],
       .name = "Worm",
       .description = "A creepy crawler.",
       .spriteIndex = assetSpriteSheet00ItemWormSpriteIndex},

      // Basic recipes
      {.data = breezeCatalog[42],
       .name = "Copper Ingot",
       .description =
           "Your basic household metal. Shiny, provided you keep it polished.",
       .spriteIndex = assetSpriteSheet00ItemBarCopperSpriteIndex},
  };
}

}  // namespace AtelierEsri
