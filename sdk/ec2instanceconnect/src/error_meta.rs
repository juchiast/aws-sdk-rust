// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Either your AWS credentials are not valid or you do not have access to the EC2 instance.</p>
    AuthException(crate::error::AuthException),
    /// <p>The specified instance was not found.</p>
    Ec2InstanceNotFoundException(crate::error::Ec2InstanceNotFoundException),
    /// <p>Unable to connect because the instance is not in a valid state. Connecting to a stopped or terminated instance is not supported. If the instance is stopped, start your instance, and try to connect again.</p>
    Ec2InstanceStateInvalidException(crate::error::Ec2InstanceStateInvalidException),
    /// <p>The instance type is not supported for connecting via the serial console. Only Nitro instance types are currently supported.</p>
    Ec2InstanceTypeInvalidException(crate::error::Ec2InstanceTypeInvalidException),
    /// <p>The instance is currently unavailable. Wait a few minutes and try again.</p>
    Ec2InstanceUnavailableException(crate::error::Ec2InstanceUnavailableException),
    /// <p>One of the parameters is not valid.</p>
    InvalidArgsException(crate::error::InvalidArgsException),
    /// <p>Your account is not authorized to use the EC2 Serial Console. To authorize your account, run the EnableSerialConsoleAccess API. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_EnableSerialConsoleAccess.html">EnableSerialConsoleAccess</a> in the <i>Amazon EC2 API Reference</i>.</p>
    SerialConsoleAccessDisabledException(crate::error::SerialConsoleAccessDisabledException),
    /// <p>The instance currently has 1 active serial console session. Only 1 session is supported at a time.</p>
    SerialConsoleSessionLimitExceededException(
        crate::error::SerialConsoleSessionLimitExceededException,
    ),
    /// <p>Unable to start a serial console session. Please try again.</p>
    SerialConsoleSessionUnavailableException(
        crate::error::SerialConsoleSessionUnavailableException,
    ),
    /// <p>The service encountered an error. Follow the instructions in the error message and try again.</p>
    ServiceException(crate::error::ServiceException),
    /// <p>The requests were made too frequently and have been throttled. Wait a while and try again. To increase the limit on your request frequency, contact AWS Support.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AuthException(inner) => inner.fmt(f),
            Error::Ec2InstanceNotFoundException(inner) => inner.fmt(f),
            Error::Ec2InstanceStateInvalidException(inner) => inner.fmt(f),
            Error::Ec2InstanceTypeInvalidException(inner) => inner.fmt(f),
            Error::Ec2InstanceUnavailableException(inner) => inner.fmt(f),
            Error::InvalidArgsException(inner) => inner.fmt(f),
            Error::SerialConsoleAccessDisabledException(inner) => inner.fmt(f),
            Error::SerialConsoleSessionLimitExceededException(inner) => inner.fmt(f),
            Error::SerialConsoleSessionUnavailableException(inner) => inner.fmt(f),
            Error::ServiceException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SendSerialConsoleSSHPublicKeyError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::SendSerialConsoleSSHPublicKeyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::SendSerialConsoleSSHPublicKeyErrorKind::AuthException(inner) => Error::AuthException(inner),
                crate::error::SendSerialConsoleSSHPublicKeyErrorKind::Ec2InstanceNotFoundException(inner) => Error::Ec2InstanceNotFoundException(inner),
                crate::error::SendSerialConsoleSSHPublicKeyErrorKind::Ec2InstanceStateInvalidException(inner) => Error::Ec2InstanceStateInvalidException(inner),
                crate::error::SendSerialConsoleSSHPublicKeyErrorKind::Ec2InstanceTypeInvalidException(inner) => Error::Ec2InstanceTypeInvalidException(inner),
                crate::error::SendSerialConsoleSSHPublicKeyErrorKind::Ec2InstanceUnavailableException(inner) => Error::Ec2InstanceUnavailableException(inner),
                crate::error::SendSerialConsoleSSHPublicKeyErrorKind::InvalidArgsException(inner) => Error::InvalidArgsException(inner),
                crate::error::SendSerialConsoleSSHPublicKeyErrorKind::SerialConsoleAccessDisabledException(inner) => Error::SerialConsoleAccessDisabledException(inner),
                crate::error::SendSerialConsoleSSHPublicKeyErrorKind::SerialConsoleSessionLimitExceededException(inner) => Error::SerialConsoleSessionLimitExceededException(inner),
                crate::error::SendSerialConsoleSSHPublicKeyErrorKind::SerialConsoleSessionUnavailableException(inner) => Error::SerialConsoleSessionUnavailableException(inner),
                crate::error::SendSerialConsoleSSHPublicKeyErrorKind::ServiceException(inner) => Error::ServiceException(inner),
                crate::error::SendSerialConsoleSSHPublicKeyErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::SendSerialConsoleSSHPublicKeyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SendSSHPublicKeyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::SendSSHPublicKeyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendSSHPublicKeyErrorKind::AuthException(inner) => {
                    Error::AuthException(inner)
                }
                crate::error::SendSSHPublicKeyErrorKind::Ec2InstanceNotFoundException(inner) => {
                    Error::Ec2InstanceNotFoundException(inner)
                }
                crate::error::SendSSHPublicKeyErrorKind::Ec2InstanceStateInvalidException(
                    inner,
                ) => Error::Ec2InstanceStateInvalidException(inner),
                crate::error::SendSSHPublicKeyErrorKind::Ec2InstanceUnavailableException(inner) => {
                    Error::Ec2InstanceUnavailableException(inner)
                }
                crate::error::SendSSHPublicKeyErrorKind::InvalidArgsException(inner) => {
                    Error::InvalidArgsException(inner)
                }
                crate::error::SendSSHPublicKeyErrorKind::ServiceException(inner) => {
                    Error::ServiceException(inner)
                }
                crate::error::SendSSHPublicKeyErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::SendSSHPublicKeyErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
