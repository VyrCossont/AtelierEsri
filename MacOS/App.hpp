#pragma once

#include "Alert.hpp"

namespace AtelierEsri {

/// Responsible for initing the Toolbox and running the event loop.
class App {
public:
  App();
  void Run();

private:
  AtelierEsri::Alert helloAlert;
  void Initialize();
};

} // namespace AtelierEsri
