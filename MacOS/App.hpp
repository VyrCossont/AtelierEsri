#pragma once

#include "Control.hpp"
#include "GWorld.hpp"
#include "Game.hpp"
#include "Window.hpp"

namespace AtelierEsri {

/// Responsible for initing the Toolbox and running the event loop.
class App {
 public:
  App();

  /// Run the event loop.
  void EventLoop();

 private:
  static void SetupMenuBar();

  /// Returns true if we should quit.
  bool HandleMenuSelection(int32_t menuSelection);

  void AboutBox();

  static void DiskInserted(const EventRecord& event);

  /// For activate and update events only, return the associated `Window`,
  /// if there is one. Result will be null otherwise.
  static Window* EventWindow(const EventRecord& event);

  /// Window in which the game world gets drawn.
  /// Auxiliary windows or dialogs may be used, but we'll always need this
  /// one.
  Window gameWindow;
  ScrollBar gameVScrollBar;

  GWorld offscreenGWorld;
  Game game;
};

}  // namespace AtelierEsri
