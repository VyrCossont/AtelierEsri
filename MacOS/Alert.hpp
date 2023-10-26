#pragma once

#include <Dialogs.h>
#include <MacTypes.h>

namespace AtelierEsri {

class Alert {
public:
  explicit Alert(SInt16 resourceID);
  DialogItemIndex Show();

private:
  SInt16 resourceID;
};

} // namespace AtelierEsri
