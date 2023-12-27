#include "Debug.hpp"

#include <Devices.h>
#include <Files.h>
#include <Serial.h>

#include <cstdarg>
#include <cstdio>
#include <cstring>

namespace AtelierEsri {

// We don't use result types in this file because debug logging is best effort
// and we'll almost always discard the result.

OSErr Debug::Printfln(
    const char *fileName,
    const uint32_t line,
    const char *func,
    const char *fmt,
    ...
) {
  /// Original format string with location prefix and a Mac newline on the end.
  char fmtBuffer[256];
  snprintf(
      fmtBuffer,
      sizeof fmtBuffer,
      "%s:%lu (%s): %s\r",
      fileName,
      line,
      func,
      fmt
  );

  char buffer[256];
  va_list args;
  va_start(args, fmt);
  vsnprintf(buffer, sizeof buffer, fmtBuffer, args);
  va_end(args);

  const size_t num_bytes = strnlen(buffer, sizeof buffer);

  // ReSharper disable once CppJoinDeclarationAndAssignment
  OSErr osErr;

  osErr = FilePrint(num_bytes, buffer);
  if (osErr) {
    return osErr;
  }

  osErr = SerialPrint(num_bytes, buffer);
  if (osErr) {
    return osErr;
  }

  return noErr;
}

OSErr Debug::FilePrint(const size_t num_bytes, const char *buffer) {
  // ReSharper disable once CppJoinDeclarationAndAssignment
  OSErr osErr;
  long count;

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunknown-escape-sequence"
  const auto fileName =
      reinterpret_cast<ConstStr255Param>("\p:AtelierEsriLog.txt");
#pragma clang diagnostic pop

  // Volume 0 and directory 0 default to the current working directory.

  // ReSharper disable CppMultiCharacterLiteral
  osErr = HCreate(0, 0, fileName, 'ttxt', 'TEXT');
  // ReSharper restore CppMultiCharacterLiteral
  if (osErr && osErr != dupFNErr) {
    return osErr;
  }

  short fileRefNum;
  osErr = HOpenDF(0, 0, fileName, fsWrPerm, &fileRefNum);
  if (osErr) {
    return osErr;
  }

  // Seek to end.
  osErr = SetFPos(fileRefNum, fsFromLEOF, 0);
  if (osErr) {
    goto close;
  }

  count = static_cast<long>(num_bytes);
  osErr = FSWrite(fileRefNum, &count, buffer);
  if (osErr) {
    goto close;
  }

  FileParam fileParam;
  fileParam.ioFRefNum = fileRefNum;
  osErr = PBFlushFileSync(reinterpret_cast<ParmBlkPtr>(&fileParam));
  // ReSharper disable once CppDFAConstantConditions
  if (osErr) {
    // ReSharper disable once CppDFAUnreachableCode
    goto close;
  }

close:
  const OSErr closeError = FSClose(fileRefNum);
  if (osErr == noErr && closeError != noErr) {
    return closeError;
  }

  return osErr;
}

OSErr Debug::SerialPrint(const size_t num_bytes, const char *buffer) {
  OSErr osErr;

  // TODO: (Vyr) use Carbon equivalents of legacy serial API
#if !TARGET_API_MAC_CARBON
  short serialPortRefNum;
  constexpr short serConfig =
      static_cast<short>(stop10) | static_cast<short>(noParity) |
      static_cast<short>(data8) | static_cast<short>(baud9600);

  /// .AOut is the modem port's output side.
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunknown-escape-sequence"
  const auto driverName = reinterpret_cast<ConstStr255Param>("\p:.AOut");
#pragma clang diagnostic pop
  osErr = MacOpenDriver(driverName, &serialPortRefNum);
  if (osErr) {
    return osErr;
  }

  osErr = SerReset(serialPortRefNum, serConfig);
  if (osErr) {
    goto close;
  }

  char ioBuffer[256];
  memcpy(ioBuffer, buffer, sizeof ioBuffer);

  IOParam ioParam;
  ioParam.ioRefNum = serialPortRefNum;
  ioParam.ioBuffer = ioBuffer;
  ioParam.ioReqCount = static_cast<long>(std::min(num_bytes, sizeof ioBuffer));
  osErr = PBWriteSync(reinterpret_cast<ParmBlkPtr>(&ioParam));
  // ReSharper disable once CppDFAConstantConditions
  if (osErr) {
    // ReSharper disable once CppDFAUnreachableCode
    goto close;
  }

close:
  const OSErr closeError = MacCloseDriver(serialPortRefNum);
  if (osErr == noErr && closeError != noErr) {
    return closeError;
  }
#endif

  return osErr;
}

}  // namespace AtelierEsri
