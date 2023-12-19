#pragma once

#include "Breeze/Alchemy.hpp"
#include "Material.hpp"
#include "SpriteSheet.hpp"

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

class GameModeTitleScreen final : public GameMode {
 public:
  explicit GameModeTitleScreen(Game& game);
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

class GameModeAtelierInterior final : public GameMode {
 public:
  explicit GameModeAtelierInterior(Game& game);

 private:
  Window window;
  MaskedImage atelierInterior;
};

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

  /// Pop the mode stack until we've popped off the requested mode.
  void PopTo(const GameMode* mode);

 private:
  std::vector<GameMode*> modeStack;

  // Storage for things used by multiple modes.

  SpriteSheet spriteSheet;
  std::vector<Breeze::Material> breezeCatalog;
  std::vector<Material> catalog;
  Breeze::PlayerInventory inventory;
};

}  // namespace AtelierEsri
