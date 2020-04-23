enum ExitCodes {
    OK = 0,

    InvalidInput = 400,
    FileNotFound = 401,
    InvalidFile = 402,
    NoAccountFound = 403,
    NoAccountSelected = 404,
    NoMembershipFound = 405,
    NoTermsFound = 406,

    UnexpectedException = 500,
    FsOperationFailed = 501,
    ApiError = 502,
}
export = ExitCodes;
