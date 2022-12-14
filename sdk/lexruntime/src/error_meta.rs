// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Either the Amazon Lex bot is still building, or one of the dependent services (Amazon Polly, AWS Lambda) failed with an internal service error.</p>
    BadGatewayException(crate::error::BadGatewayException),
    /// <p> Request validation failed, there is no usable message in the context, or the bot build failed, is still in progress, or contains unbuilt changes. </p>
    BadRequestException(crate::error::BadRequestException),
    /// <p> Two clients are using the same AWS account, Amazon Lex bot, and user ID. </p>
    ConflictException(crate::error::ConflictException),
    /// <p> One of the dependencies, such as AWS Lambda or Amazon Polly, threw an exception. For example, </p>
    /// <ul>
    /// <li> <p>If Amazon Lex does not have sufficient permissions to call a Lambda function.</p> </li>
    /// <li> <p>If a Lambda function takes longer than 30 seconds to execute.</p> </li>
    /// <li> <p>If a fulfillment Lambda function returns a <code>Delegate</code> dialog action without removing any slot values.</p> </li>
    /// </ul>
    DependencyFailedException(crate::error::DependencyFailedException),
    /// <p>Internal service error. Retry the call.</p>
    InternalFailureException(crate::error::InternalFailureException),
    /// <p>Exceeded a limit.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>This exception is not used.</p>
    LoopDetectedException(crate::error::LoopDetectedException),
    /// <p>The accept header in the request does not have a valid value.</p>
    NotAcceptableException(crate::error::NotAcceptableException),
    /// <p>The resource (such as the Amazon Lex bot or an alias) that is referred to is not found.</p>
    NotFoundException(crate::error::NotFoundException),
    /// <p>The input speech is too long.</p>
    RequestTimeoutException(crate::error::RequestTimeoutException),
    /// <p>The Content-Type header (<code>PostContent</code> API) has an invalid value. </p>
    UnsupportedMediaTypeException(crate::error::UnsupportedMediaTypeException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadGatewayException(inner) => inner.fmt(f),
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::DependencyFailedException(inner) => inner.fmt(f),
            Error::InternalFailureException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::LoopDetectedException(inner) => inner.fmt(f),
            Error::NotAcceptableException(inner) => inner.fmt(f),
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::RequestTimeoutException(inner) => inner.fmt(f),
            Error::UnsupportedMediaTypeException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteSessionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteSessionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteSessionErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DeleteSessionErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::DeleteSessionErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::DeleteSessionErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::DeleteSessionErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DeleteSessionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSessionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSessionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetSessionErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::GetSessionErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::GetSessionErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::GetSessionErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::GetSessionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PostContentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PostContentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PostContentErrorKind::BadGatewayException(inner) => {
                    Error::BadGatewayException(inner)
                }
                crate::error::PostContentErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::PostContentErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::PostContentErrorKind::DependencyFailedException(inner) => {
                    Error::DependencyFailedException(inner)
                }
                crate::error::PostContentErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::PostContentErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::PostContentErrorKind::LoopDetectedException(inner) => {
                    Error::LoopDetectedException(inner)
                }
                crate::error::PostContentErrorKind::NotAcceptableException(inner) => {
                    Error::NotAcceptableException(inner)
                }
                crate::error::PostContentErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::PostContentErrorKind::RequestTimeoutException(inner) => {
                    Error::RequestTimeoutException(inner)
                }
                crate::error::PostContentErrorKind::UnsupportedMediaTypeException(inner) => {
                    Error::UnsupportedMediaTypeException(inner)
                }
                crate::error::PostContentErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PostTextError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PostTextError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PostTextErrorKind::BadGatewayException(inner) => {
                    Error::BadGatewayException(inner)
                }
                crate::error::PostTextErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::PostTextErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::PostTextErrorKind::DependencyFailedException(inner) => {
                    Error::DependencyFailedException(inner)
                }
                crate::error::PostTextErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::PostTextErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::PostTextErrorKind::LoopDetectedException(inner) => {
                    Error::LoopDetectedException(inner)
                }
                crate::error::PostTextErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::PostTextErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutSessionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutSessionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutSessionErrorKind::BadGatewayException(inner) => {
                    Error::BadGatewayException(inner)
                }
                crate::error::PutSessionErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::PutSessionErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::PutSessionErrorKind::DependencyFailedException(inner) => {
                    Error::DependencyFailedException(inner)
                }
                crate::error::PutSessionErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::PutSessionErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::PutSessionErrorKind::NotAcceptableException(inner) => {
                    Error::NotAcceptableException(inner)
                }
                crate::error::PutSessionErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::PutSessionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
