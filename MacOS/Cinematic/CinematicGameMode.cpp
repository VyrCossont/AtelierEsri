#include "CinematicGameMode.hpp"

#include <sstream>
#include <stdexcept>

#include "AppResources.h"

namespace AtelierEsri {

CinematicGameMode::CinematicGameMode(
    Game& game,
    const std::vector<Breeze::CinematicCommand>& cinematic,
    const std::string& name
)
    : GameMode(game),
      // Yes, this is intentional. For now.
      window(atelierInteriorWINDResourceID),
      forwardButton(cinematicForwardButtonCNTLResourceID, window),
      backButton(cinematicBackButtonCNTLResourceID, window),
      cinematic(cinematic),
      position(cinematic.cbegin()) {
  window.onUpdate = [&]([[maybe_unused]] const Window& window) { Draw(); };
  window.Title(name);

  forwardButton.onClick = [&]([[maybe_unused]] const Button& button) {
    Forward();
  };

  backButton.onClick = [&]([[maybe_unused]] const Button& button) { Back(); };
}

// Used in lambda above.
// ReSharper disable once CppDFAUnreachableFunctionCall
void CinematicGameMode::Forward() { Advance(); }

// Used in lambda above.
// ReSharper disable once CppDFAUnreachableFunctionCall
void CinematicGameMode::Back() { Reset(); }

// ReSharper disable once CppDFAUnreachableFunctionCall
void CinematicGameMode::Draw() const {
  const GWorldActiveGuard activeGuard = window.MakeActivePort();
  QD::Reset();

  const Pattern pattern = QD::Black();
  const Rect windowRect = window.PortBounds();
  FillRect(&windowRect, &pattern);

  if (background) {
    background->Draw(windowRect);
  }

  if (leftCharacter) {
    game.MainSpriteSheet().Draw9Patch(Border, LeftSlotDecorationRect);
    const Rect rect = LeftSlotCharacterRect;
    FillRect(&rect, &pattern);
    game.MainSpriteSheet().Draw(*leftCharacter, LeftSlotCharacterRect);
  }

  if (rightCharacter) {
    game.MainSpriteSheet().Draw9Patch(Border, RightSlotDecorationRect);
    const Rect rect = RightSlotCharacterRect;
    FillRect(&rect, &pattern);
    game.MainSpriteSheet().Draw(*rightCharacter, RightSlotCharacterRect);
  }

  if (text) {
    // TODO: speaker indicator

    game.MainSpriteSheet().Draw9Patch(Border, TextDecorationRect);
    const Rect rect = TextLinesRect;
    FillRect(&rect, &pattern);

    // TODO: use TextEdit to draw multiple lines correctly
    const ChangeClip changeClip{TextLinesRect};
    QD::MoveTo(TextLinesRect.origin + V2I{0, 12});
    ForeColor(whiteColor);
    QD::DrawText(*text);
    ForeColor(blackColor);
  }
}

// NOLINTBEGIN(*-convert-member-functions-to-static)
// ReSharper disable once CppDFAUnreachableFunctionCall
void CinematicGameMode::Invalidate() const {
  // NOLINTEND(*-convert-member-functions-to-static)
  const GWorldActiveGuard activeGuard = window.MakeActivePort();
  const Rect windowRect = window.PortBounds();
#if TARGET_API_MAC_CARBON
  InvalWindowRect(window.Unmanaged(), &windowRect);
#else
  InvalRect(&windowRect);
#endif
}

// ReSharper disable once CppDFAUnreachableFunctionCall
void CinematicGameMode::Advance() {
  for (; position != cinematic.cend(); ++position) {
    if (player.Apply(*position)) {
      // Convert player IDs to resources and sprites.

      // These are not even remotely the same branch.
      if (player.background) {  // NOLINT(*-branch-clone)
        background.emplace(static_cast<ResourceID>(*player.background));
      } else {
        background.reset();
      }

      // Note: Breeze::CinematicCharacter::id is currently unused,
      // and ::mood can be directly converted into a sprite index.
      // TODO: draw character names using ::id

      if (const auto left = player.Left()) {
        leftCharacter = left->mood;
      } else {
        leftCharacter.reset();
      }

      if (const auto right = player.Right()) {
        rightCharacter = right->mood;
      } else {
        rightCharacter.reset();
      }

      // TODO: speaker

      text = player.text;

      if (player.material) {
        const auto& catalog = game.Catalog();
        const size_t id = *player.material;
        if (id >= catalog.size()) {
          std::stringstream message;
          message << "Invalid cinematic: material ID " << id
                  << " is missing from the game's material catalog";
          throw std::invalid_argument(message.str());
        }
        material = catalog[id].spriteIndex;
      } else {
        material.reset();
      }

      Invalidate();

      // Advance past the commit command.
      ++position;

      return;
    }
  }

  forwardButton.Enabled(false);

  // TODO: offer to show a log if the user wants them
  // TODO: tell the invoking game mode that we're done
}

// ReSharper disable once CppDFAUnreachableFunctionCall
void CinematicGameMode::Reset() {
  position = cinematic.cbegin();
  player.Reset();

  background.reset();
  leftCharacter.reset();
  rightCharacter.reset();
  // TODO: speaker
  text.reset();
  material.reset();

  forwardButton.Enabled(true);

  Invalidate();
}

}  // namespace AtelierEsri