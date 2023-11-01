/// Copied from https://yegor.pomortsev.com/post/result-type/

#pragma once

#include <variant>

#include "Error.hpp"

namespace AtelierEsri {
template <typename T> class Ok {
public:
  explicit constexpr Ok(T value) : value(std::move(value)) {}

  constexpr T &&take_value() { return std::move(value); }

  T value;
};

template <typename T> class Err {
public:
  explicit constexpr Err(T value) : value(std::move(value)) {}

  constexpr T &&take_value() { return std::move(value); }

  T value;
};

template <typename OkT, typename ErrT = Error> class Result {
public:
  using VariantT = std::variant<Ok<OkT>, Err<ErrT>>;

#pragma clang diagnostic push
#pragma ide diagnostic ignored "google-explicit-constructor"
  constexpr Result(Ok<OkT> value) : variant(std::move(value)) {}
  constexpr Result(Err<ErrT> value) : variant(std::move(value)) {}
#pragma clang diagnostic pop

  [[nodiscard]] constexpr bool is_ok() const {
    return std::holds_alternative<Ok<OkT>>(variant);
  }
  [[nodiscard]] constexpr bool is_err() const {
    return std::holds_alternative<Err<ErrT>>(variant);
  }

  constexpr OkT ok_value() const { return std::get<Ok<OkT>>(variant).value; }
  constexpr ErrT err_value() const {
    return std::get<Err<ErrT>>(variant).value;
  }

  constexpr OkT &&take_ok_value() {
    return std::get<Ok<OkT>>(variant).take_value();
  }
  constexpr ErrT &&take_err_value() {
    return std::get<Err<ErrT>>(variant).take_value();
  }

  VariantT variant;
};

using Unit = std::monostate;

} // namespace AtelierEsri

#pragma region Flow control macros

/// Flow control statement:
/// Evaluate an expression with a result type.
/// If the result is an error, return an error result of the appropriate type.
/// Otherwise, pass the success value through.
#define GUARD_LET_TRY(valueType, valueName, expr)                              \
  GUARD_LET_TRY_INNER_1(valueType, valueName, (expr), __COUNTER__)

#pragma region GUARD_LET_TRY implementation

/// Forces counter argument to be evaluated, giving us a unique-enough name.
#define GUARD_LET_TRY_INNER_1(valueType, valueName, expr, counter)             \
  GUARD_LET_TRY_INNER_2(valueType, valueName, (expr), counter)

/// Implements `GUARD_LET_TRY`.
#define GUARD_LET_TRY_INNER_2(valueType, valueName, expr, counter)             \
  auto guardLetTryResult##counter = (expr);                                    \
  if (guardLetTryResult##counter.is_err()) {                                   \
    return ::AtelierEsri::Err(guardLetTryResult##counter.take_err_value());    \
  }                                                                            \
  valueType valueName = guardLetTryResult##counter.take_ok_value()

#pragma endregion

/// Flow control statement:
/// Return an error result with a message and source location.
#define BAIL(message) return ::AtelierEsri::Err(ERROR((message)))

/// Flow control statement:
/// If a pointer type is null,
/// return an error result with a message and source location.
#define REQUIRE_NOT_NULL(valueName)                                            \
  do {                                                                         \
    if (valueName == nullptr) {                                                \
      BAIL("Requirement failed: " #valueName " is null");                      \
    }                                                                          \
  } while (false)

/// Flow control statement:
/// Return an error result with a message, OS error code, and source location.
#define OS_BAIL(message, osErr)                                                \
  return ::AtelierEsri::Err(OS_ERROR((message), (osErr)))

/// Flow control statement:
/// Evaluate an expression that returns an OS error code.
/// If there is no error, pass the result through.
/// Otherwise, return an error result with a message, OS error code, and source
/// location.
#define OS_CHECKED(expr, message)                                              \
  do {                                                                         \
    OSErr osErr = (expr);                                                      \
    if (osErr) {                                                               \
      OS_BAIL((message), osErr);                                               \
    }                                                                          \
  } while (false)

/// Flow control statement:
/// Evaluate an expression that can set a Color QuickDraw error code.
/// These are OS errors but are returned out of band:
/// https://preterhuman.net/macstuff/insidemac/QuickDraw/QuickDraw-255.html
/// If there is no error, pass the result through.
/// Otherwise, return an error result with a message, OS error code, and source
/// location.
#define QD_CHECKED(expr, message)                                              \
  (expr);                                                                      \
  do {                                                                         \
    OSErr qdError = QDError();                                                 \
    if (qdError) {                                                             \
      OS_BAIL((message), qdError);                                             \
    }                                                                          \
  } while (false)

#pragma endregion
