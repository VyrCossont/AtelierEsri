#include "Game.hpp"

#include <ctgmath>

#include "Assets.h"
#include "Drawing.hpp"
#include "MaskedImage.hpp"

namespace AtelierEsri {

Game Game::Setup(Window &window) {
  MaskedImage spriteSheetImage =
      MaskedImage::Get(assetSpriteSheet00ImagePictResourceId,
                       assetSpriteSheet00MaskPictResourceId, window);
  SpriteSheet spriteSheet = SpriteSheet::New(std::move(spriteSheetImage),
                                             assetSpriteSheet00RgnResourceId);
  auto game = Game(std::move(spriteSheet));

  // Give two of every raw material.
  const std::vector<Material> catalog = Material::Catalog();
  for (size_t materialIndex = 1; materialIndex < catalog.size();
       ++materialIndex) {
    const Breeze::Material &material = catalog[materialIndex].data;
    Breeze::Item item = {.material = material,
                         .elements = material.elements,
                         .elementValue = material.elementValue,
                         .quality = 50,
                         .categories = material.categories,
                         .traits = material.traits};
    for (size_t itemIndex = 0; itemIndex < 2; ++itemIndex) {
      game.inventory.push_back(item);
    }
  }

  return game;
}

void Game::Update() {}

void Game::Draw(GWorld &gWorld) {
  QD::Reset();

  GWorldActiveGuard activeGuard = gWorld.MakeActive();

  const Rect rect = gWorld.Bounds();
  const Pattern background = QD::Gray();
  FillRect(&rect, &background);

  constexpr Rect dstRect = {0, 0, 64, 64};
  spriteSheet.Draw(gWorld, assetSpriteSheet00AvatarEsriSpriteIndex, dstRect);

  const auto hex = Ngon({120, 120}, 32, 6, M_PI + M_PI_2);
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
        spriteSheet.Draw(gWorld, assetSpriteSheet00ElementFireSpriteIndex,
                         pipRect);
      }
    }
  }
}

Game::Game(SpriteSheet &&spriteSheet) : spriteSheet(std::move(spriteSheet)) {}

std::vector<Material> Material::Catalog() {
  const std::vector<Breeze::Material> breezeCatalog =
      Breeze::Material::Catalog();
  return {
      {.data = breezeCatalog[1],
       .name = "Copper Ingot",
       .spriteIndex = assetSpriteSheet00ItemBarCopperSpriteIndex},

      // Raw materials
      {.data = breezeCatalog[0],
       .name = "Bacon",
       .spriteIndex = assetSpriteSheet00ItemBaconSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Bud",
       .spriteIndex = assetSpriteSheet00ItemBudSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Crystal",
       .spriteIndex = assetSpriteSheet00ItemCrystalSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "DragonEye",
       .spriteIndex = assetSpriteSheet00ItemDragonEyeSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Dunkelheit",
       .spriteIndex = assetSpriteSheet00ItemDunkelheitSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Elerium",
       .spriteIndex = assetSpriteSheet00ItemEleriumSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Feather",
       .spriteIndex = assetSpriteSheet00ItemFeatherSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Flower1",
       .spriteIndex = assetSpriteSheet00ItemFlower1SpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Flower2",
       .spriteIndex = assetSpriteSheet00ItemFlower2SpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Grapes",
       .spriteIndex = assetSpriteSheet00ItemGrapesSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Grass",
       .spriteIndex = assetSpriteSheet00ItemGrassSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Gravistone",
       .spriteIndex = assetSpriteSheet00ItemGravistoneSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Herb",
       .spriteIndex = assetSpriteSheet00ItemHerbSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "LeafDown",
       .spriteIndex = assetSpriteSheet00ItemLeafDownSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "LeafTriple",
       .spriteIndex = assetSpriteSheet00ItemLeafTripleSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "LeafUp",
       .spriteIndex = assetSpriteSheet00ItemLeafUpSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Lump",
       .spriteIndex = assetSpriteSheet00ItemLumpSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Mushroom1",
       .spriteIndex = assetSpriteSheet00ItemMushroom1SpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Mushroom2",
       .spriteIndex = assetSpriteSheet00ItemMushroom2SpriteIndex},
      {.data = breezeCatalog[0],
       .name = "OreCopper",
       .spriteIndex = assetSpriteSheet00ItemOreCopperSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "OreIron",
       .spriteIndex = assetSpriteSheet00ItemOreIronSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "OreSilver",
       .spriteIndex = assetSpriteSheet00ItemOreSilverSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "OreStygium",
       .spriteIndex = assetSpriteSheet00ItemOreStygiumSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "OreTitanium",
       .spriteIndex = assetSpriteSheet00ItemOreTitaniumSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Page",
       .spriteIndex = assetSpriteSheet00ItemPageSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Palm",
       .spriteIndex = assetSpriteSheet00ItemPalmSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Pendeloque",
       .spriteIndex = assetSpriteSheet00ItemPendeloqueSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Pods",
       .spriteIndex = assetSpriteSheet00ItemPodsSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Puniball",
       .spriteIndex = assetSpriteSheet00ItemPuniballSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "PuniballGiant",
       .spriteIndex = assetSpriteSheet00ItemPuniballGiantSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Rock",
       .spriteIndex = assetSpriteSheet00ItemRockSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Sand",
       .spriteIndex = assetSpriteSheet00ItemSandSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Seaweed1",
       .spriteIndex = assetSpriteSheet00ItemSeaweed1SpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Seaweed2",
       .spriteIndex = assetSpriteSheet00ItemSeaweed2SpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Spider",
       .spriteIndex = assetSpriteSheet00ItemSpiderSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Spirit",
       .spriteIndex = assetSpriteSheet00ItemSpiritSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Steak",
       .spriteIndex = assetSpriteSheet00ItemSteakSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Sulfur",
       .spriteIndex = assetSpriteSheet00ItemSulfurSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Uni",
       .spriteIndex = assetSpriteSheet00ItemUniSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Water",
       .spriteIndex = assetSpriteSheet00ItemWaterSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Wood",
       .spriteIndex = assetSpriteSheet00ItemWoodSpriteIndex},
      {.data = breezeCatalog[0],
       .name = "Worm",
       .spriteIndex = assetSpriteSheet00ItemWormSpriteIndex},
  };
}

} // namespace AtelierEsri
