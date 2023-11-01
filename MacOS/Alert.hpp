#pragma once

#include <Dialogs.h>
#include <MacTypes.h>

namespace AtelierEsri {

enum AlertType {
  custom,
  note,
  caution,
  stop,
};

class Alert {
public:
  explicit Alert(SInt16 resourceID, AlertType alertType = custom);
  DialogItemIndex Show();

private:
  SInt16 resourceID;
  AlertType alertType;
};

} // namespace AtelierEsri
