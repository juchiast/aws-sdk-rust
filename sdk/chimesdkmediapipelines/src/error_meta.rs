// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The input parameters don't match the service's restrictions.</p>
    BadRequestException(crate::error::BadRequestException),
    /// <p>The client is permanently forbidden from making the request.</p>
    ForbiddenException(crate::error::ForbiddenException),
    /// <p>One or more of the resources in the request does not exist in the system.</p>
    NotFoundException(crate::error::NotFoundException),
    /// <p>The request exceeds the resource limit.</p>
    ResourceLimitExceededException(crate::error::ResourceLimitExceededException),
    /// <p>The service encountered an unexpected error.</p>
    ServiceFailureException(crate::error::ServiceFailureException),
    /// <p>The service is currently unavailable.</p>
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    /// <p>The client exceeded its request rate limit.</p>
    ThrottledClientException(crate::error::ThrottledClientException),
    /// <p>The client is not currently authorized to make the request.</p>
    UnauthorizedClientException(crate::error::UnauthorizedClientException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::ForbiddenException(inner) => inner.fmt(f),
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::ResourceLimitExceededException(inner) => inner.fmt(f),
            Error::ServiceFailureException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::ThrottledClientException(inner) => inner.fmt(f),
            Error::UnauthorizedClientException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateMediaCapturePipelineError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateMediaCapturePipelineError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateMediaCapturePipelineError> for Error {
    fn from(err: crate::error::CreateMediaCapturePipelineError) -> Self {
        match err.kind {
            crate::error::CreateMediaCapturePipelineErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::CreateMediaCapturePipelineErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::CreateMediaCapturePipelineErrorKind::ResourceLimitExceededException(
                inner,
            ) => Error::ResourceLimitExceededException(inner),
            crate::error::CreateMediaCapturePipelineErrorKind::ServiceFailureException(inner) => {
                Error::ServiceFailureException(inner)
            }
            crate::error::CreateMediaCapturePipelineErrorKind::ServiceUnavailableException(
                inner,
            ) => Error::ServiceUnavailableException(inner),
            crate::error::CreateMediaCapturePipelineErrorKind::ThrottledClientException(inner) => {
                Error::ThrottledClientException(inner)
            }
            crate::error::CreateMediaCapturePipelineErrorKind::UnauthorizedClientException(
                inner,
            ) => Error::UnauthorizedClientException(inner),
            crate::error::CreateMediaCapturePipelineErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::CreateMediaConcatenationPipelineError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::CreateMediaConcatenationPipelineError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateMediaConcatenationPipelineError> for Error {
    fn from(err: crate::error::CreateMediaConcatenationPipelineError) -> Self {
        match err.kind {
            crate::error::CreateMediaConcatenationPipelineErrorKind::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::error::CreateMediaConcatenationPipelineErrorKind::ForbiddenException(inner) => Error::ForbiddenException(inner),
            crate::error::CreateMediaConcatenationPipelineErrorKind::ResourceLimitExceededException(inner) => Error::ResourceLimitExceededException(inner),
            crate::error::CreateMediaConcatenationPipelineErrorKind::ServiceFailureException(inner) => Error::ServiceFailureException(inner),
            crate::error::CreateMediaConcatenationPipelineErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
            crate::error::CreateMediaConcatenationPipelineErrorKind::ThrottledClientException(inner) => Error::ThrottledClientException(inner),
            crate::error::CreateMediaConcatenationPipelineErrorKind::UnauthorizedClientException(inner) => Error::UnauthorizedClientException(inner),
            crate::error::CreateMediaConcatenationPipelineErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::CreateMediaLiveConnectorPipelineError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::CreateMediaLiveConnectorPipelineError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateMediaLiveConnectorPipelineError> for Error {
    fn from(err: crate::error::CreateMediaLiveConnectorPipelineError) -> Self {
        match err.kind {
            crate::error::CreateMediaLiveConnectorPipelineErrorKind::BadRequestException(inner) => Error::BadRequestException(inner),
            crate::error::CreateMediaLiveConnectorPipelineErrorKind::ForbiddenException(inner) => Error::ForbiddenException(inner),
            crate::error::CreateMediaLiveConnectorPipelineErrorKind::ResourceLimitExceededException(inner) => Error::ResourceLimitExceededException(inner),
            crate::error::CreateMediaLiveConnectorPipelineErrorKind::ServiceFailureException(inner) => Error::ServiceFailureException(inner),
            crate::error::CreateMediaLiveConnectorPipelineErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
            crate::error::CreateMediaLiveConnectorPipelineErrorKind::ThrottledClientException(inner) => Error::ThrottledClientException(inner),
            crate::error::CreateMediaLiveConnectorPipelineErrorKind::UnauthorizedClientException(inner) => Error::UnauthorizedClientException(inner),
            crate::error::CreateMediaLiveConnectorPipelineErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteMediaCapturePipelineError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteMediaCapturePipelineError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteMediaCapturePipelineError> for Error {
    fn from(err: crate::error::DeleteMediaCapturePipelineError) -> Self {
        match err.kind {
            crate::error::DeleteMediaCapturePipelineErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::DeleteMediaCapturePipelineErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::DeleteMediaCapturePipelineErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::DeleteMediaCapturePipelineErrorKind::ServiceFailureException(inner) => {
                Error::ServiceFailureException(inner)
            }
            crate::error::DeleteMediaCapturePipelineErrorKind::ServiceUnavailableException(
                inner,
            ) => Error::ServiceUnavailableException(inner),
            crate::error::DeleteMediaCapturePipelineErrorKind::ThrottledClientException(inner) => {
                Error::ThrottledClientException(inner)
            }
            crate::error::DeleteMediaCapturePipelineErrorKind::UnauthorizedClientException(
                inner,
            ) => Error::UnauthorizedClientException(inner),
            crate::error::DeleteMediaCapturePipelineErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteMediaPipelineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteMediaPipelineError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteMediaPipelineError> for Error {
    fn from(err: crate::error::DeleteMediaPipelineError) -> Self {
        match err.kind {
            crate::error::DeleteMediaPipelineErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::DeleteMediaPipelineErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::DeleteMediaPipelineErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::DeleteMediaPipelineErrorKind::ServiceFailureException(inner) => {
                Error::ServiceFailureException(inner)
            }
            crate::error::DeleteMediaPipelineErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::DeleteMediaPipelineErrorKind::ThrottledClientException(inner) => {
                Error::ThrottledClientException(inner)
            }
            crate::error::DeleteMediaPipelineErrorKind::UnauthorizedClientException(inner) => {
                Error::UnauthorizedClientException(inner)
            }
            crate::error::DeleteMediaPipelineErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetMediaCapturePipelineError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetMediaCapturePipelineError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetMediaCapturePipelineError> for Error {
    fn from(err: crate::error::GetMediaCapturePipelineError) -> Self {
        match err.kind {
            crate::error::GetMediaCapturePipelineErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::GetMediaCapturePipelineErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::GetMediaCapturePipelineErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::GetMediaCapturePipelineErrorKind::ServiceFailureException(inner) => {
                Error::ServiceFailureException(inner)
            }
            crate::error::GetMediaCapturePipelineErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::GetMediaCapturePipelineErrorKind::ThrottledClientException(inner) => {
                Error::ThrottledClientException(inner)
            }
            crate::error::GetMediaCapturePipelineErrorKind::UnauthorizedClientException(inner) => {
                Error::UnauthorizedClientException(inner)
            }
            crate::error::GetMediaCapturePipelineErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetMediaPipelineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetMediaPipelineError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetMediaPipelineError> for Error {
    fn from(err: crate::error::GetMediaPipelineError) -> Self {
        match err.kind {
            crate::error::GetMediaPipelineErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::GetMediaPipelineErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::GetMediaPipelineErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::GetMediaPipelineErrorKind::ServiceFailureException(inner) => {
                Error::ServiceFailureException(inner)
            }
            crate::error::GetMediaPipelineErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::GetMediaPipelineErrorKind::ThrottledClientException(inner) => {
                Error::ThrottledClientException(inner)
            }
            crate::error::GetMediaPipelineErrorKind::UnauthorizedClientException(inner) => {
                Error::UnauthorizedClientException(inner)
            }
            crate::error::GetMediaPipelineErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListMediaCapturePipelinesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListMediaCapturePipelinesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListMediaCapturePipelinesError> for Error {
    fn from(err: crate::error::ListMediaCapturePipelinesError) -> Self {
        match err.kind {
            crate::error::ListMediaCapturePipelinesErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::ListMediaCapturePipelinesErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::ListMediaCapturePipelinesErrorKind::ResourceLimitExceededException(
                inner,
            ) => Error::ResourceLimitExceededException(inner),
            crate::error::ListMediaCapturePipelinesErrorKind::ServiceFailureException(inner) => {
                Error::ServiceFailureException(inner)
            }
            crate::error::ListMediaCapturePipelinesErrorKind::ServiceUnavailableException(
                inner,
            ) => Error::ServiceUnavailableException(inner),
            crate::error::ListMediaCapturePipelinesErrorKind::ThrottledClientException(inner) => {
                Error::ThrottledClientException(inner)
            }
            crate::error::ListMediaCapturePipelinesErrorKind::UnauthorizedClientException(
                inner,
            ) => Error::UnauthorizedClientException(inner),
            crate::error::ListMediaCapturePipelinesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListMediaPipelinesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListMediaPipelinesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListMediaPipelinesError> for Error {
    fn from(err: crate::error::ListMediaPipelinesError) -> Self {
        match err.kind {
            crate::error::ListMediaPipelinesErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::ListMediaPipelinesErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::ListMediaPipelinesErrorKind::ResourceLimitExceededException(inner) => {
                Error::ResourceLimitExceededException(inner)
            }
            crate::error::ListMediaPipelinesErrorKind::ServiceFailureException(inner) => {
                Error::ServiceFailureException(inner)
            }
            crate::error::ListMediaPipelinesErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::ListMediaPipelinesErrorKind::ThrottledClientException(inner) => {
                Error::ThrottledClientException(inner)
            }
            crate::error::ListMediaPipelinesErrorKind::UnauthorizedClientException(inner) => {
                Error::UnauthorizedClientException(inner)
            }
            crate::error::ListMediaPipelinesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListTagsForResourceError> for Error {
    fn from(err: crate::error::ListTagsForResourceError) -> Self {
        match err.kind {
            crate::error::ListTagsForResourceErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ServiceFailureException(inner) => {
                Error::ServiceFailureException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ThrottledClientException(inner) => {
                Error::ThrottledClientException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::UnauthorizedClientException(inner) => {
                Error::UnauthorizedClientException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::TagResourceError> for Error {
    fn from(err: crate::error::TagResourceError) -> Self {
        match err.kind {
            crate::error::TagResourceErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::TagResourceErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::TagResourceErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::TagResourceErrorKind::ServiceFailureException(inner) => {
                Error::ServiceFailureException(inner)
            }
            crate::error::TagResourceErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::TagResourceErrorKind::ThrottledClientException(inner) => {
                Error::ThrottledClientException(inner)
            }
            crate::error::TagResourceErrorKind::UnauthorizedClientException(inner) => {
                Error::UnauthorizedClientException(inner)
            }
            crate::error::TagResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UntagResourceError> for Error {
    fn from(err: crate::error::UntagResourceError) -> Self {
        match err.kind {
            crate::error::UntagResourceErrorKind::BadRequestException(inner) => {
                Error::BadRequestException(inner)
            }
            crate::error::UntagResourceErrorKind::ForbiddenException(inner) => {
                Error::ForbiddenException(inner)
            }
            crate::error::UntagResourceErrorKind::NotFoundException(inner) => {
                Error::NotFoundException(inner)
            }
            crate::error::UntagResourceErrorKind::ServiceFailureException(inner) => {
                Error::ServiceFailureException(inner)
            }
            crate::error::UntagResourceErrorKind::ServiceUnavailableException(inner) => {
                Error::ServiceUnavailableException(inner)
            }
            crate::error::UntagResourceErrorKind::ThrottledClientException(inner) => {
                Error::ThrottledClientException(inner)
            }
            crate::error::UntagResourceErrorKind::UnauthorizedClientException(inner) => {
                Error::UnauthorizedClientException(inner)
            }
            crate::error::UntagResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
