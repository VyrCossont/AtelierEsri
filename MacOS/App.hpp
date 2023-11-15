#pragma once

#include "Alert.hpp"
#include "Error.hpp"
#include "GWorld.hpp"
#include "Game.hpp"
#include "Resource.hpp"
#include "Result.hpp"
#include "Window.hpp"

namespace AtelierEsri {

/// Responsible for initing the Toolbox and running the event loop.
class App {
public:
  static Result<App> New() noexcept;

  /// Run the event loop.
  Result<Unit> EventLoop() noexcept;

private:
  App(Window gameWindow, GWorld offscreenGWorld, Game game) noexcept;

  static Result<Unit> SetupMenuBar() noexcept;

  /// Returns true if we should quit.
  bool HandleMenuSelection(int32_t menuSelection) noexcept;

  void AboutBox() noexcept;

  /// Window in which the game world gets drawn.
  /// Auxiliary windows or dialogs may be used, but we'll always need this
  /// one.
  Window gameWindow;

  GWorld offscreenGWorld;
  Game game;
};

} // namespace AtelierEsri
