enum ExitCodes {
  OK = 0,

  InvalidParameters = 100,
  DevelopmentModeOnly,
  FileError,
  ApiError = 200,
  UnsuccessfulRuntimeCall,
}
export = ExitCodes
