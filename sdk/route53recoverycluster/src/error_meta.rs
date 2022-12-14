// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You don't have sufficient permissions to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>There was a conflict with this request. Try again.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>The cluster endpoint isn't available. Try another cluster endpoint.</p>
    EndpointTemporarilyUnavailableException(crate::error::EndpointTemporarilyUnavailableException),
    /// <p>There was an unexpected error during processing of the request.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The request references a routing control or control panel that was not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The request can't update that many routing control states at the same time. Try again with fewer routing control states.</p>
    ServiceLimitExceededException(crate::error::ServiceLimitExceededException),
    /// <p>The request was denied because of request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>There was a validation error on the request.</p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::EndpointTemporarilyUnavailableException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceLimitExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetRoutingControlStateError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetRoutingControlStateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetRoutingControlStateErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetRoutingControlStateErrorKind::EndpointTemporarilyUnavailableException(inner) => Error::EndpointTemporarilyUnavailableException(inner),
                crate::error::GetRoutingControlStateErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetRoutingControlStateErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetRoutingControlStateErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetRoutingControlStateErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetRoutingControlStateErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListRoutingControlsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListRoutingControlsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListRoutingControlsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListRoutingControlsErrorKind::EndpointTemporarilyUnavailableException(inner) => Error::EndpointTemporarilyUnavailableException(inner),
                crate::error::ListRoutingControlsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListRoutingControlsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ListRoutingControlsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::ListRoutingControlsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListRoutingControlsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateRoutingControlStateError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateRoutingControlStateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateRoutingControlStateErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::EndpointTemporarilyUnavailableException(inner) => Error::EndpointTemporarilyUnavailableException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UpdateRoutingControlStateErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateRoutingControlStatesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateRoutingControlStatesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateRoutingControlStatesErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::EndpointTemporarilyUnavailableException(inner) => Error::EndpointTemporarilyUnavailableException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::ServiceLimitExceededException(inner) => Error::ServiceLimitExceededException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UpdateRoutingControlStatesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
