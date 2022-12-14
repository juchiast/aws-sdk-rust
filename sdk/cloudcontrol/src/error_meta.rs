// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The resource with the name requested already exists.</p>
    AlreadyExistsException(crate::error::AlreadyExistsException),
    /// <p>The specified client token has already been used in another resource request.</p>
    /// <p>It's best practice for client tokens to be unique for each resource operation request. However, client token expire after 36 hours.</p>
    ClientTokenConflictException(crate::error::ClientTokenConflictException),
    /// <p>The resource is currently being modified by another operation.</p>
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    /// <p>Another resource operation is currently being performed on this resource.</p>
    ConcurrentOperationException(crate::error::ConcurrentOperationException),
    /// <p>The resource handler has returned that the downstream service generated an error that doesn't map to any other handler error code.</p>
    GeneralServiceException(crate::error::GeneralServiceException),
    /// <p>The resource handler has failed without a returning a more specific error code. This can include timeouts.</p>
    HandlerFailureException(crate::error::HandlerFailureException),
    /// <p>The resource handler has returned that an unexpected error occurred within the resource handler.</p>
    HandlerInternalFailureException(crate::error::HandlerInternalFailureException),
    /// <p>The resource handler has returned that the credentials provided by the user are invalid.</p>
    InvalidCredentialsException(crate::error::InvalidCredentialsException),
    /// <p>The resource handler has returned that invalid input from the user has generated a generic exception.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>The resource handler has returned that the request couldn't be completed due to networking issues, such as a failure to receive a response from the server.</p>
    NetworkFailureException(crate::error::NetworkFailureException),
    /// <p>The resource handler has returned that the downstream resource failed to complete all of its ready-state checks.</p>
    NotStabilizedException(crate::error::NotStabilizedException),
    /// <p>One or more properties included in this resource operation are defined as create-only, and therefore can't be updated.</p>
    NotUpdatableException(crate::error::NotUpdatableException),
    /// <p>Cloud Control API hasn't received a valid response from the resource handler, due to a configuration error. This includes issues such as the resource handler returning an invalid response, or timing out.</p>
    PrivateTypeException(crate::error::PrivateTypeException),
    /// <p>A resource operation with the specified request token can't be found.</p>
    RequestTokenNotFoundException(crate::error::RequestTokenNotFoundException),
    /// <p>The resource is temporarily unavailable to be acted upon. For example, if the resource is currently undergoing an operation and can't be acted upon until that operation is finished.</p>
    ResourceConflictException(crate::error::ResourceConflictException),
    /// <p>A resource with the specified identifier can't be found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The resource handler has returned that the downstream service returned an internal error, typically with a <code>5XX HTTP</code> status code.</p>
    ServiceInternalErrorException(crate::error::ServiceInternalErrorException),
    /// <p>The resource handler has returned that a non-transient resource limit was reached on the service side.</p>
    ServiceLimitExceededException(crate::error::ServiceLimitExceededException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The specified extension doesn't exist in the CloudFormation registry.</p>
    TypeNotFoundException(crate::error::TypeNotFoundException),
    /// <p>The specified resource doesn't support this resource operation.</p>
    UnsupportedActionException(crate::error::UnsupportedActionException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AlreadyExistsException(inner) => inner.fmt(f),
            Error::ClientTokenConflictException(inner) => inner.fmt(f),
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::ConcurrentOperationException(inner) => inner.fmt(f),
            Error::GeneralServiceException(inner) => inner.fmt(f),
            Error::HandlerFailureException(inner) => inner.fmt(f),
            Error::HandlerInternalFailureException(inner) => inner.fmt(f),
            Error::InvalidCredentialsException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::NetworkFailureException(inner) => inner.fmt(f),
            Error::NotStabilizedException(inner) => inner.fmt(f),
            Error::NotUpdatableException(inner) => inner.fmt(f),
            Error::PrivateTypeException(inner) => inner.fmt(f),
            Error::RequestTokenNotFoundException(inner) => inner.fmt(f),
            Error::ResourceConflictException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceInternalErrorException(inner) => inner.fmt(f),
            Error::ServiceLimitExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::TypeNotFoundException(inner) => inner.fmt(f),
            Error::UnsupportedActionException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CancelResourceRequestError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CancelResourceRequestError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CancelResourceRequestErrorKind::ConcurrentModificationException(
                    inner,
                ) => Error::ConcurrentModificationException(inner),
                crate::error::CancelResourceRequestErrorKind::RequestTokenNotFoundException(
                    inner,
                ) => Error::RequestTokenNotFoundException(inner),
                crate::error::CancelResourceRequestErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateResourceErrorKind::AlreadyExistsException(inner) => {
                    Error::AlreadyExistsException(inner)
                }
                crate::error::CreateResourceErrorKind::ClientTokenConflictException(inner) => {
                    Error::ClientTokenConflictException(inner)
                }
                crate::error::CreateResourceErrorKind::ConcurrentOperationException(inner) => {
                    Error::ConcurrentOperationException(inner)
                }
                crate::error::CreateResourceErrorKind::GeneralServiceException(inner) => {
                    Error::GeneralServiceException(inner)
                }
                crate::error::CreateResourceErrorKind::HandlerFailureException(inner) => {
                    Error::HandlerFailureException(inner)
                }
                crate::error::CreateResourceErrorKind::HandlerInternalFailureException(inner) => {
                    Error::HandlerInternalFailureException(inner)
                }
                crate::error::CreateResourceErrorKind::InvalidCredentialsException(inner) => {
                    Error::InvalidCredentialsException(inner)
                }
                crate::error::CreateResourceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::CreateResourceErrorKind::NetworkFailureException(inner) => {
                    Error::NetworkFailureException(inner)
                }
                crate::error::CreateResourceErrorKind::NotStabilizedException(inner) => {
                    Error::NotStabilizedException(inner)
                }
                crate::error::CreateResourceErrorKind::NotUpdatableException(inner) => {
                    Error::NotUpdatableException(inner)
                }
                crate::error::CreateResourceErrorKind::PrivateTypeException(inner) => {
                    Error::PrivateTypeException(inner)
                }
                crate::error::CreateResourceErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::CreateResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateResourceErrorKind::ServiceInternalErrorException(inner) => {
                    Error::ServiceInternalErrorException(inner)
                }
                crate::error::CreateResourceErrorKind::ServiceLimitExceededException(inner) => {
                    Error::ServiceLimitExceededException(inner)
                }
                crate::error::CreateResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::CreateResourceErrorKind::TypeNotFoundException(inner) => {
                    Error::TypeNotFoundException(inner)
                }
                crate::error::CreateResourceErrorKind::UnsupportedActionException(inner) => {
                    Error::UnsupportedActionException(inner)
                }
                crate::error::CreateResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteResourceErrorKind::AlreadyExistsException(inner) => {
                    Error::AlreadyExistsException(inner)
                }
                crate::error::DeleteResourceErrorKind::ClientTokenConflictException(inner) => {
                    Error::ClientTokenConflictException(inner)
                }
                crate::error::DeleteResourceErrorKind::ConcurrentOperationException(inner) => {
                    Error::ConcurrentOperationException(inner)
                }
                crate::error::DeleteResourceErrorKind::GeneralServiceException(inner) => {
                    Error::GeneralServiceException(inner)
                }
                crate::error::DeleteResourceErrorKind::HandlerFailureException(inner) => {
                    Error::HandlerFailureException(inner)
                }
                crate::error::DeleteResourceErrorKind::HandlerInternalFailureException(inner) => {
                    Error::HandlerInternalFailureException(inner)
                }
                crate::error::DeleteResourceErrorKind::InvalidCredentialsException(inner) => {
                    Error::InvalidCredentialsException(inner)
                }
                crate::error::DeleteResourceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::DeleteResourceErrorKind::NetworkFailureException(inner) => {
                    Error::NetworkFailureException(inner)
                }
                crate::error::DeleteResourceErrorKind::NotStabilizedException(inner) => {
                    Error::NotStabilizedException(inner)
                }
                crate::error::DeleteResourceErrorKind::NotUpdatableException(inner) => {
                    Error::NotUpdatableException(inner)
                }
                crate::error::DeleteResourceErrorKind::PrivateTypeException(inner) => {
                    Error::PrivateTypeException(inner)
                }
                crate::error::DeleteResourceErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::DeleteResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteResourceErrorKind::ServiceInternalErrorException(inner) => {
                    Error::ServiceInternalErrorException(inner)
                }
                crate::error::DeleteResourceErrorKind::ServiceLimitExceededException(inner) => {
                    Error::ServiceLimitExceededException(inner)
                }
                crate::error::DeleteResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DeleteResourceErrorKind::TypeNotFoundException(inner) => {
                    Error::TypeNotFoundException(inner)
                }
                crate::error::DeleteResourceErrorKind::UnsupportedActionException(inner) => {
                    Error::UnsupportedActionException(inner)
                }
                crate::error::DeleteResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetResourceErrorKind::AlreadyExistsException(inner) => {
                    Error::AlreadyExistsException(inner)
                }
                crate::error::GetResourceErrorKind::GeneralServiceException(inner) => {
                    Error::GeneralServiceException(inner)
                }
                crate::error::GetResourceErrorKind::HandlerFailureException(inner) => {
                    Error::HandlerFailureException(inner)
                }
                crate::error::GetResourceErrorKind::HandlerInternalFailureException(inner) => {
                    Error::HandlerInternalFailureException(inner)
                }
                crate::error::GetResourceErrorKind::InvalidCredentialsException(inner) => {
                    Error::InvalidCredentialsException(inner)
                }
                crate::error::GetResourceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::GetResourceErrorKind::NetworkFailureException(inner) => {
                    Error::NetworkFailureException(inner)
                }
                crate::error::GetResourceErrorKind::NotStabilizedException(inner) => {
                    Error::NotStabilizedException(inner)
                }
                crate::error::GetResourceErrorKind::NotUpdatableException(inner) => {
                    Error::NotUpdatableException(inner)
                }
                crate::error::GetResourceErrorKind::PrivateTypeException(inner) => {
                    Error::PrivateTypeException(inner)
                }
                crate::error::GetResourceErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::GetResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetResourceErrorKind::ServiceInternalErrorException(inner) => {
                    Error::ServiceInternalErrorException(inner)
                }
                crate::error::GetResourceErrorKind::ServiceLimitExceededException(inner) => {
                    Error::ServiceLimitExceededException(inner)
                }
                crate::error::GetResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetResourceErrorKind::TypeNotFoundException(inner) => {
                    Error::TypeNotFoundException(inner)
                }
                crate::error::GetResourceErrorKind::UnsupportedActionException(inner) => {
                    Error::UnsupportedActionException(inner)
                }
                crate::error::GetResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetResourceRequestStatusError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetResourceRequestStatusError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetResourceRequestStatusErrorKind::RequestTokenNotFoundException(
                    inner,
                ) => Error::RequestTokenNotFoundException(inner),
                crate::error::GetResourceRequestStatusErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListResourceRequestsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListResourceRequestsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListResourceRequestsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListResourcesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListResourcesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListResourcesErrorKind::AlreadyExistsException(inner) => {
                    Error::AlreadyExistsException(inner)
                }
                crate::error::ListResourcesErrorKind::GeneralServiceException(inner) => {
                    Error::GeneralServiceException(inner)
                }
                crate::error::ListResourcesErrorKind::HandlerFailureException(inner) => {
                    Error::HandlerFailureException(inner)
                }
                crate::error::ListResourcesErrorKind::HandlerInternalFailureException(inner) => {
                    Error::HandlerInternalFailureException(inner)
                }
                crate::error::ListResourcesErrorKind::InvalidCredentialsException(inner) => {
                    Error::InvalidCredentialsException(inner)
                }
                crate::error::ListResourcesErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::ListResourcesErrorKind::NetworkFailureException(inner) => {
                    Error::NetworkFailureException(inner)
                }
                crate::error::ListResourcesErrorKind::NotStabilizedException(inner) => {
                    Error::NotStabilizedException(inner)
                }
                crate::error::ListResourcesErrorKind::NotUpdatableException(inner) => {
                    Error::NotUpdatableException(inner)
                }
                crate::error::ListResourcesErrorKind::PrivateTypeException(inner) => {
                    Error::PrivateTypeException(inner)
                }
                crate::error::ListResourcesErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::ListResourcesErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListResourcesErrorKind::ServiceInternalErrorException(inner) => {
                    Error::ServiceInternalErrorException(inner)
                }
                crate::error::ListResourcesErrorKind::ServiceLimitExceededException(inner) => {
                    Error::ServiceLimitExceededException(inner)
                }
                crate::error::ListResourcesErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListResourcesErrorKind::TypeNotFoundException(inner) => {
                    Error::TypeNotFoundException(inner)
                }
                crate::error::ListResourcesErrorKind::UnsupportedActionException(inner) => {
                    Error::UnsupportedActionException(inner)
                }
                crate::error::ListResourcesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateResourceErrorKind::AlreadyExistsException(inner) => {
                    Error::AlreadyExistsException(inner)
                }
                crate::error::UpdateResourceErrorKind::ClientTokenConflictException(inner) => {
                    Error::ClientTokenConflictException(inner)
                }
                crate::error::UpdateResourceErrorKind::ConcurrentOperationException(inner) => {
                    Error::ConcurrentOperationException(inner)
                }
                crate::error::UpdateResourceErrorKind::GeneralServiceException(inner) => {
                    Error::GeneralServiceException(inner)
                }
                crate::error::UpdateResourceErrorKind::HandlerFailureException(inner) => {
                    Error::HandlerFailureException(inner)
                }
                crate::error::UpdateResourceErrorKind::HandlerInternalFailureException(inner) => {
                    Error::HandlerInternalFailureException(inner)
                }
                crate::error::UpdateResourceErrorKind::InvalidCredentialsException(inner) => {
                    Error::InvalidCredentialsException(inner)
                }
                crate::error::UpdateResourceErrorKind::InvalidRequestException(inner) => {
                    Error::InvalidRequestException(inner)
                }
                crate::error::UpdateResourceErrorKind::NetworkFailureException(inner) => {
                    Error::NetworkFailureException(inner)
                }
                crate::error::UpdateResourceErrorKind::NotStabilizedException(inner) => {
                    Error::NotStabilizedException(inner)
                }
                crate::error::UpdateResourceErrorKind::NotUpdatableException(inner) => {
                    Error::NotUpdatableException(inner)
                }
                crate::error::UpdateResourceErrorKind::PrivateTypeException(inner) => {
                    Error::PrivateTypeException(inner)
                }
                crate::error::UpdateResourceErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::UpdateResourceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateResourceErrorKind::ServiceInternalErrorException(inner) => {
                    Error::ServiceInternalErrorException(inner)
                }
                crate::error::UpdateResourceErrorKind::ServiceLimitExceededException(inner) => {
                    Error::ServiceLimitExceededException(inner)
                }
                crate::error::UpdateResourceErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::UpdateResourceErrorKind::TypeNotFoundException(inner) => {
                    Error::TypeNotFoundException(inner)
                }
                crate::error::UpdateResourceErrorKind::UnsupportedActionException(inner) => {
                    Error::UnsupportedActionException(inner)
                }
                crate::error::UpdateResourceErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
