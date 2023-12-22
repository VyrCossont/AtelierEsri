#pragma once

namespace AtelierEsri {

/// Layout constants, some from the Apple HIG.
struct Design {
  /// Corresponds to "A" spacing in Figure 6-21 of the HIG (p. 197, PDF p. 221).
  static constexpr int MinorSpacing = 13;
  /// Corresponds to "B" spacing in Figure 6-21 of the HIG (p. 197, PDF p. 221).
  static constexpr int MajorSpacing = 23;
  /// Standard height for buttons (HIG p. 205, PDF p. 228).
  static constexpr int ButtonHeight = 20;
  /// Standard width for "OK" and "Cancel" buttons (HIG p. 204, PDF p. 228).
  static constexpr int ButtonWidth = 59;

  // App-specific stuff below.

  /// Spacing between very small components.
  static constexpr int SpacingMini = 2;

  /// For very small components with rounded rect borders.
  static constexpr int CornerRadiusMini = 2;
};

}  // namespace AtelierEsri
