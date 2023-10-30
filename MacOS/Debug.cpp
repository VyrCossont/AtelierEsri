#include "Debug.hpp"

#include <cstdarg>
#include <cstdio>
#include <cstring>

#include <Devices.h>
#include <Files.h>
#include <Serial.h>

namespace AtelierEsri {

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

  OSErr error;

  error = FilePrint(num_bytes, buffer);
  if (error != noErr) {
    return error;
  }

  error = SerialPrint(num_bytes, buffer);
  if (error != noErr) {
    return error;
  }

  return noErr;
}

OSErr Debug::FilePrint(size_t num_bytes, const char *buffer) {
  OSErr error;
  long count;

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunknown-escape-sequence"
  auto fileName = (ConstStr255Param) "\p:AtelierEsriLog.txt";
#pragma clang diagnostic pop

  // Volume 0 and directory 0 default to the current working directory.

  error = HCreate(0, 0, fileName, 'ttxt', 'TEXT');
  if (error != noErr && error != dupFNErr) {
    return error;
  }

  short fileRefNum;
  error = HOpenDF(0, 0, fileName, fsWrPerm, &fileRefNum);
  if (error != noErr) {
    return error;
  }

  // Seek to end.
  error = SetFPos(fileRefNum, fsFromLEOF, 0);
  if (error != noErr) {
    goto close;
  }

  count = (long)num_bytes;
  error = FSWrite(fileRefNum, &count, buffer);
  if (error != noErr) {
    goto close;
  }

  FileParam fileParam;
  fileParam.ioFRefNum = fileRefNum;
  error = PBFlushFile((ParmBlkPtr)&fileParam, false);
  if (error != noErr) {
    goto close;
  }

close:
  OSErr closeError = FSClose(fileRefNum);
  if (error == noErr && closeError != noErr) {
    return closeError;
  }

  return error;
}

OSErr Debug::SerialPrint(size_t num_bytes, const char *buffer) {
  OSErr error;

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
  error = MacOpenDriver(driverName, &serialPortRefNum);
  if (error != noErr) {
    return error;
  }

  error = SerReset(serialPortRefNum, serConfig);
  if (error != noErr) {
    goto close;
  }

  char ioBuffer[256];
  memcpy(ioBuffer, buffer, sizeof ioBuffer);

  IOParam ioParam;
  ioParam.ioRefNum = serialPortRefNum;
  ioParam.ioBuffer = ioBuffer;
  ioParam.ioReqCount = (long)std::min(num_bytes, sizeof ioBuffer);
  error = PBWrite((ParmBlkPtr)&ioParam, false);
  if (error != noErr) {
    goto close;
  }

close:
  OSErr closeError = MacCloseDriver(serialPortRefNum);
  if (error == noErr && closeError != noErr) {
    return closeError;
  }
#endif

  return error;
}

} // namespace AtelierEsri
