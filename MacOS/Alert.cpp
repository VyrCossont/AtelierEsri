#include <Dialogs.h>

#include "Alert.hpp"

namespace AtelierEsri {

Alert::Alert(SInt16 resourceID) { this->resourceID = resourceID; }

DialogItemIndex Alert::Show() {
  // TODO: (Vyr) the second param should be an "event filter" UPP.
  //  See PSKM p. 361—362.
  return NoteAlert(resourceID, nullptr);
}

} // namespace AtelierEsri
