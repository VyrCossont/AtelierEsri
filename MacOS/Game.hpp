#pragma once

#include "Breeze/Alchemy.hpp"
#include "MaskedImage.hpp"
#include "Material.hpp"
#include "SpriteSheet.hpp"
#include "Window.hpp"

namespace AtelierEsri {

class Game;

class GameMode {
 public:
  virtual void Tick(uint64_t currentTimestampUsec);
  virtual ~GameMode() = default;

 protected:
  explicit GameMode(Game& game);

  Game& game;
};

class TitleScreenGameMode final : public GameMode {
 public:
  explicit TitleScreenGameMode(Game& game);
  void Tick(uint64_t currentTimestampUsec) override;

 private:
  /// Close this window and start the game proper.
  void EnterAtelier() const;

  Window window;
  MaskedImage titleScreen;
  /// Timestamp after which to close this window.
  uint64_t dismissTimestampUsec;
  /// Dismiss this window after this long.
  static constexpr uint64_t displayDurationUsec = 5'000'000;  // 5s
};

/// Holds a stack of game modes and distributes animation ticks to them.
/// Also holds data likely to be used by multiple game modes.
class Game {
 public:
  explicit Game();

  /// Called at approximately 60 Hz with current timestamp.
  /// Ticks all modes in the stack.
  ///
  /// Individual modes can decide to ignore ticks if backgrounded.
  void Tick(uint64_t currentTimestampUsec) const;

  /// Push a new game mode on top of the stack.
  void Push(GameMode* mode);

  /// Pop and delete the mode stack until we've popped off the requested mode.
  void PopTo(const GameMode* mode);

  /// Alchemy icons and avatars.
  [[nodiscard]] const SpriteSheet& MainSpriteSheet() const;

  /// Material data.
  [[nodiscard]] const std::vector<Breeze::Material>& BreezeCatalog() const;

  /// Material metadata (names and icons).
  [[nodiscard]] const std::vector<Material>& Catalog() const;

  /// All items in player's container.
  /// Intentionally mutable: some game modes will modify this.
  [[nodiscard]] Breeze::PlayerInventory& Inventory();

  // TODO: extract these to a Breeze alchemy level/skills class
  [[nodiscard]] Breeze::Quality PlayerMaxQuality();
  [[nodiscard]] int PlayerMaxPlacements();

 private:
  std::vector<GameMode*> modeStack;

  // Storage for resources used by multiple modes.

  SpriteSheet spriteSheet;
  std::vector<Breeze::Material> breezeCatalog;
  std::vector<Material> catalog;

  // Mutable persistent game state.
  // TODO: save/load games

  Breeze::PlayerInventory inventory;
};

}  // namespace AtelierEsri
