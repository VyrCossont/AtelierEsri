#pragma once

#include "Assets.h"
#include "Breeze/Cinematics.hpp"
#include "Control.hpp"
#include "Drawing.hpp"
#include "Game.hpp"
#include "MaskedImage.hpp"
#include "SpriteSheet.hpp"
#include "Window.hpp"

namespace AtelierEsri {

class CinematicGameMode final : public GameMode {
 public:
  explicit CinematicGameMode(
      Game& game,
      const std::vector<Breeze::CinematicCommand>& cinematic,
      const std::string& name
  );

 private:
  void Forward();
  void Back();

  void Draw() const;
  void Invalidate() const;

  void Advance();
  void Reset();

  Window window;
  Button forwardButton;
  Button backButton;

  Breeze::CinematicPlayer player;
  const std::vector<Breeze::CinematicCommand>& cinematic;
  std::vector<Breeze::CinematicCommand>::const_iterator position;

  std::optional<Picture> background;
  std::optional<SpriteSheet::SpriteIndex> leftCharacter;
  std::optional<SpriteSheet::SpriteIndex> rightCharacter;
  // TODO: speaker
  std::optional<std::string> text;
  std::optional<SpriteSheet::SpriteIndex> material;

  static constexpr SpriteSheet::PatchIndex Border =
      assetSpriteSheet00ItemBorder49PatchIndex;

  static constexpr R2I LeftSlotDecorationRect{{20, 200}, {80, 80}};
  static constexpr R2I LeftSlotCharacterRect{{28, 208}, {64, 64}};

  static constexpr R2I RightSlotDecorationRect{{300, 200}, {80, 80}};
  static constexpr R2I RightSlotCharacterRect{{308, 208}, {64, 64}};

  // TODO: material slot

  static constexpr R2I TextDecorationRect{{100, 200}, {200, 80}};
  static constexpr R2I TextLinesRect{{110, 210}, {180, 60}};
};

}  // namespace AtelierEsri
