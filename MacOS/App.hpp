#pragma once

#include "Control.hpp"
#include "GWorld.hpp"
#include "Game.hpp"
#include "Window.hpp"

namespace AtelierEsri {

/// Responsible for initing the Toolbox and running the event loop.
class App {
public:
  static App New();

  /// Run the event loop.
  void EventLoop();

private:
  App(Window gameWindow, ScrollBar gameVScrollBar, GWorld offscreenGWorld,
      Game game);

  static void SetupMenuBar();

  /// Returns true if we should quit.
  bool HandleMenuSelection(int32_t menuSelection);

  void AboutBox();

  /// Window in which the game world gets drawn.
  /// Auxiliary windows or dialogs may be used, but we'll always need this
  /// one.
  Window gameWindow;
  ScrollBar gameVScrollBar;

  GWorld offscreenGWorld;
  Game game;
};

} // namespace AtelierEsri
