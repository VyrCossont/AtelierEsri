#include "SynthesisDashboard.hpp"

#include "Design.hpp"

namespace AtelierEsri {

SynthesisDashboard::SynthesisDashboard(
    const Breeze::SynthesisState& state,
    const std::vector<Material>& catalog,
    const SpriteSheet& spriteSheet
)
    : state(state),
      catalog(catalog),
      spriteSheet(spriteSheet),
      elementValueDisplay(spriteSheet) {
  Layout();
}

void SynthesisDashboard::Refresh() {
  // ReSharper disable once CppUseStructuredBinding
  const Breeze::SynthesisResult result = state.Result();

  elementValueDisplay.elements = result.item.elements;
  elementValueDisplay.elementValue = result.item.elementValue;
}

void SynthesisDashboard::Draw() const {
  // ReSharper disable once CppUseStructuredBinding
  const Material& material = catalog[state.Output().id];

  const R2I iconRect = IconRect();
  spriteSheet.Draw(material.spriteIndex, iconRect);

  elementValueDisplay.Draw();

  const V2I labelOrigin = iconRect.origin + V2I{Design::MajorSpacing, 0};
  QD::MoveTo(labelOrigin);
  TextFont(0);
  TextSize(0);
  QD::DrawText(material.name);
}

void SynthesisDashboard::Layout() {
  const R2I iconRect = IconRect();
  elementValueDisplay.origin = {
      iconRect.Left(),
      iconRect.Bottom() + Design::MinorSpacing,
  };
}

R2I SynthesisDashboard::IconRect() const {
  return {
      bounds.origin + V2I{Design::MajorSpacing, Design::MinorSpacing},
      {MaterialIconWidth, MaterialIconWidth},
  };
}

}  // namespace AtelierEsri
