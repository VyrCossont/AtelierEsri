#pragma once

#include "Alert.hpp"
#include "Window.hpp"

namespace AtelierEsri {

/// Responsible for initing the Toolbox and running the event loop.
class App {
public:
  App();
  void Run();

private:
  AtelierEsri::Alert helloAlert;
  AtelierEsri::Window gameWindow;
  void Initialize();
};

} // namespace AtelierEsri
