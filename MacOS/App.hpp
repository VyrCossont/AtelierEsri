#pragma once

#include "Alert.hpp"
#include "Error.hpp"
#include "Result.hpp"
#include "Window.hpp"

namespace AtelierEsri {

/// Responsible for initing the Toolbox and running the event loop.
class App {
public:
  App();

  /// Run the whole app. Returns `noErr` on success or an `OSErr` on failure.
  OSErr Run();

private:
  /// Set up Toolbox stuff.
  static void Initialize();

  /// Run the event loop.
  Result<std::monostate, Error> EventLoop();

  /// Error code for an app-specific abnormal exit.
  static OSErr appError;

  /// Display a fatal error.
  static void FatalError(const Error &error);

  /// Window in which the game world gets drawn.
  /// Auxiliary windows or dialogs may be used, but we'll always need this one.
  AtelierEsri::Window gameWindow;
};

} // namespace AtelierEsri
