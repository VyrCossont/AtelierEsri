#pragma once

#include <Dialogs.h>
#include <MacTypes.h>

#include "Resource.hpp"

namespace AtelierEsri {

enum AlertType {
  custom,
  note,
  caution,
  stop,
};

class Alert {
public:
  explicit Alert(ResourceID resourceID, AlertType alertType = custom);
  DialogItemIndex Show() const;

private:
  SInt16 resourceID;
  AlertType alertType;
};

} // namespace AtelierEsri
