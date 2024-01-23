#include "Cinematics.hpp"

#include <sstream>

namespace Breeze {

const CinematicCharacter* CinematicPlayer::Left() const {
  constexpr CinematicCharacterSlot slot = CinematicCharacterSlot::Left;
  return enum_indexed_sparse_array_get_ptr(characters, slot);
}

const CinematicCharacter* CinematicPlayer::Right() const {
  constexpr CinematicCharacterSlot slot = CinematicCharacterSlot::Right;
  return enum_indexed_sparse_array_get_ptr(characters, slot);
}

const CinematicCharacter* CinematicPlayer::Speaker() const {
  if (!speaker.has_value()) {
    return nullptr;
  }
  return enum_indexed_sparse_array_get_ptr(characters, *speaker);
}

bool CinematicPlayer::Apply(const CinematicCommand& command) {
  // ReSharper disable CppRedundantElseKeywordInsideCompoundStatement
  // ReSharper disable CppDeclarationHidesLocal
  if (std::holds_alternative<CinematicCommandCommit>(command)) {
    return true;
  } else if (const auto cmd = std::get_if<CinematicCommandSetCharacter>(&command)) {
    characters[cmd->slot] = cmd->character;
  } else if (const auto cmd = std::get_if<CinematicCommandSetMood>(&command)) {
    if (!characters[cmd->slot].has_value()) {
      std::stringstream message;
      message << "Can't set mood of empty " << cmd->slot._to_string()
              << " character slot";
      throw std::runtime_error(message.str());
    }
    characters[cmd->slot]->mood = cmd->mood;
  } else if (const auto cmd = std::get_if<CinematicCommandClearCharacter>(&command)) {
    characters[cmd->slot].reset();
  } else if (const auto cmd = std::get_if<CinematicCommandSetSpeaker>(&command)) {
    if (!characters[cmd->slot].has_value()) {
      std::stringstream message;
      message << "Can't set empty " << cmd->slot._to_string()
              << " character slot as speaker";
      throw std::runtime_error(message.str());
    }
    speaker = cmd->slot;
  } else if (std::holds_alternative<CinematicCommandClearSpeaker>(command)) {
    speaker.reset();
  } else if (const auto cmd = std::get_if<CinematicCommandSetText>(&command)) {
    text = cmd->text;
  } else if (std::holds_alternative<CinematicCommandClearText>(command)) {
    text.reset();
  } else if (const auto cmd = std::get_if<CinematicCommandSetBackground>(&command)) {
    background = cmd->background;
  } else if (std::holds_alternative<CinematicCommandClearBackground>(command)) {
    background.reset();
  } else if (const auto cmd = std::get_if<CinematicCommandSetMaterial>(&command)) {
    material = cmd->material;
  } else if (std::holds_alternative<CinematicCommandClearMaterial>(command)) {
    material.reset();
  } else {
    throw std::invalid_argument("Unknown cinematic command type");
  }
  return false;
  // ReSharper restore CppDeclarationHidesLocal
  // ReSharper restore CppRedundantElseKeywordInsideCompoundStatement
}

void CinematicPlayer::Reset() {
  background.reset();
  for (auto& character : characters) {
    character.reset();
  }
  speaker.reset();
  text.reset();
  material.reset();

  std::vector<CinematicCommand> v{
      CinematicCommandClearBackground{},
      CinematicCommandSetBackground{5},
  };
}

}  // namespace Breeze