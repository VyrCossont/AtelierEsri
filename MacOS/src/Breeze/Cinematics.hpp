#pragma once

#include <better-enums/enum.h>

#include <string>
#include <variant>

#include "Alchemy.hpp"
#include "EnumSet.hpp"

namespace Breeze {

struct CinematicCharacter {
  using ID = size_t;
  using Mood = size_t;

  ID id;
  Mood mood;
};

using CinematicText = std::string;

using CinematicBackground = size_t;

// NOLINTBEGIN(*-explicit-constructor, *-no-recursion)

BETTER_ENUM(CinematicCharacterSlot, uint8_t, Left, Right)

// NOLINTEND(*-explicit-constructor, *-no-recursion)

template <typename Enum, typename Element>
using EnumIndexedSparseArray =
    std::array<std::optional<Element>, max<Enum>()._to_integral() + 1>;

template <typename Enum, typename Element>
const Element* enum_indexed_sparse_array_get_ptr(
    const EnumIndexedSparseArray<Enum, Element>& array, const Enum index
) {
  const size_t slot = index._to_integral();
  if (array.size() <= slot || !array[slot].has_value()) {
    return nullptr;
  }
  return &*array[slot];
}

/// The page is now ready to display.
struct CinematicCommandCommit {};

struct CinematicCommandSetCharacter {
  CinematicCharacterSlot slot;
  CinematicCharacter character{};
};

/// As above but set only the mood.
struct CinematicCommandSetMood {
  CinematicCharacterSlot slot;
  CinematicCharacter::Mood mood{};
};

struct CinematicCommandClearCharacter {
  CinematicCharacterSlot slot;
};

struct CinematicCommandSetSpeaker {
  CinematicCharacterSlot slot;
};

struct CinematicCommandClearSpeaker {};

struct CinematicCommandSetText {
  CinematicText text;
};

struct CinematicCommandClearText {};

struct CinematicCommandSetBackground {
  CinematicBackground background;
};

struct CinematicCommandClearBackground {};

struct CinematicCommandSetMaterial {
  Material::ID material;
};

struct CinematicCommandClearMaterial {};

/// State delta.
using CinematicCommand = std::variant<
    CinematicCommandCommit,
    CinematicCommandSetCharacter,
    CinematicCommandSetMood,
    CinematicCommandClearCharacter,
    CinematicCommandSetSpeaker,
    CinematicCommandClearSpeaker,
    CinematicCommandSetText,
    CinematicCommandClearText,
    CinematicCommandSetBackground,
    CinematicCommandClearBackground,
    CinematicCommandSetMaterial,
    CinematicCommandClearMaterial>;

/// Accumulates state.
struct CinematicPlayer {
  std::optional<CinematicBackground> background;
  EnumIndexedSparseArray<CinematicCharacterSlot, CinematicCharacter> characters;
  std::optional<CinematicCharacterSlot> speaker;
  std::optional<CinematicText> text;
  std::optional<Material::ID> material;

  [[nodiscard]] const CinematicCharacter* Left() const;
  [[nodiscard]] const CinematicCharacter* Right() const;
  [[nodiscard]] const CinematicCharacter* Speaker() const;

  /// Returns true when a commit is received and the page should be shown.
  [[nodiscard]] bool Apply(const CinematicCommand& command);

  /// Discard all state.
  void Reset();
};

}  // namespace Breeze
