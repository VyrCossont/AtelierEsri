#include "Debug.hpp"

#include <cstdarg>
#include <cstdio>
#include <cstring>

#include <Devices.h>
#include <Files.h>
#include <Serial.h>

namespace AtelierEsri {

// We don't use result types in this file because debug logging is best effort
// and we'll almost always discard the result.

OSErr Debug::Printfln(const char *fmt, ...) {
  /// Original format string with a Mac newline on the end.
  char fmtBuffer[256];
  snprintf(fmtBuffer, sizeof fmtBuffer, "%s\r", fmt);

  char buffer[256];
  va_list args;
  va_start(args, fmt);
  vsnprintf(buffer, sizeof buffer, fmtBuffer, args);
  va_end(args);

  size_t num_bytes = strnlen(buffer, sizeof buffer);

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

OSErr Debug::FilePrint(size_t num_bytes, const char *buffer) {
  OSErr osErr;
  long count;

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunknown-escape-sequence"
  auto fileName = (ConstStr255Param) "\p:AtelierEsriLog.txt";
#pragma clang diagnostic pop

  // Volume 0 and directory 0 default to the current working directory.

  osErr = HCreate(0, 0, fileName, 'ttxt', 'TEXT');
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

  count = (long)num_bytes;
  osErr = FSWrite(fileRefNum, &count, buffer);
  if (osErr) {
    goto close;
  }

  FileParam fileParam;
  fileParam.ioFRefNum = fileRefNum;
  osErr = PBFlushFileSync((ParmBlkPtr)&fileParam);
#pragma clang diagnostic push
#pragma ide diagnostic ignored "ConstantConditionsOC"
  if (osErr) {
#pragma clang diagnostic push
#pragma ide diagnostic ignored "UnreachableCode"
    goto close;
#pragma clang diagnostic pop
  }
#pragma clang diagnostic pop

close:
  OSErr closeError = FSClose(fileRefNum);
  if (osErr == noErr && closeError != noErr) {
    return closeError;
  }

  return osErr;
}

OSErr Debug::SerialPrint(size_t num_bytes, const char *buffer) {
  OSErr osErr;

  // TODO: (Vyr) use Carbon equivalents of legacy serial API
#if !TARGET_API_MAC_CARBON
  short serialPortRefNum;
  short serConfig =
      (short)stop10 | (short)noParity | (short)data8 | (short)baud9600;

  /// .AOut is the modem port's output side.
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunknown-escape-sequence"
  auto driverName = (ConstStr255Param) "\p:.AOut";
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
  ioParam.ioReqCount = (long)std::min(num_bytes, sizeof ioBuffer);
  osErr = PBWriteSync((ParmBlkPtr)&ioParam);
#pragma clang diagnostic push
#pragma ide diagnostic ignored "ConstantConditionsOC"
  if (osErr) {
#pragma clang diagnostic push
#pragma ide diagnostic ignored "UnreachableCode"
    goto close;
#pragma clang diagnostic pop
  }
#pragma clang diagnostic pop

close:
  OSErr closeError = MacCloseDriver(serialPortRefNum);
  if (osErr == noErr && closeError != noErr) {
    return closeError;
  }
#endif

  return osErr;
}

} // namespace AtelierEsri
