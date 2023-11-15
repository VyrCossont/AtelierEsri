#include <Dialogs.h>

#include "Alert.hpp"

namespace AtelierEsri {

Alert::Alert(ResourceID resourceID, AlertType alertType)
    : resourceID(resourceID), alertType(alertType) {}

DialogItemIndex Alert::Show() {
  // TODO: (Vyr) the second param should be an "event filter" UPP.
  //  See PSKM p. 361—362.
  switch (alertType) {
  case note:
    return NoteAlert(resourceID, nullptr);
  case caution:
    return CautionAlert(resourceID, nullptr);
  case stop:
    return StopAlert(resourceID, nullptr);
  case custom:
  default:
    return ::Alert(resourceID, nullptr);
  }
}

} // namespace AtelierEsri
