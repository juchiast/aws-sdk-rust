// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Access is denied. Your account is not authorized to perform this operation.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>The Data Store is in a transition state and the user requested action can not be performed.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>Unknown error occurs in the service.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p> The requested Data Store was not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The user has exceeded their maximum number of allowed calls to the given API. </p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The user input parameter was invalid.</p>
    ValidationException(crate::error::ValidationException),
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
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateFHIRDatastoreError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateFHIRDatastoreError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateFHIRDatastoreError> for Error {
    fn from(err: crate::error::CreateFHIRDatastoreError) -> Self {
        match err.kind {
            crate::error::CreateFHIRDatastoreErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::CreateFHIRDatastoreErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::CreateFHIRDatastoreErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::CreateFHIRDatastoreErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::CreateFHIRDatastoreErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteFHIRDatastoreError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteFHIRDatastoreError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteFHIRDatastoreError> for Error {
    fn from(err: crate::error::DeleteFHIRDatastoreError) -> Self {
        match err.kind {
            crate::error::DeleteFHIRDatastoreErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::DeleteFHIRDatastoreErrorKind::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::error::DeleteFHIRDatastoreErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DeleteFHIRDatastoreErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DeleteFHIRDatastoreErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DeleteFHIRDatastoreErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DeleteFHIRDatastoreErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeFHIRDatastoreError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeFHIRDatastoreError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeFHIRDatastoreError> for Error {
    fn from(err: crate::error::DescribeFHIRDatastoreError) -> Self {
        match err.kind {
            crate::error::DescribeFHIRDatastoreErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeFHIRDatastoreErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeFHIRDatastoreErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DescribeFHIRDatastoreErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeFHIRDatastoreErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeFHIRExportJobError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeFHIRExportJobError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeFHIRExportJobError> for Error {
    fn from(err: crate::error::DescribeFHIRExportJobError) -> Self {
        match err.kind {
            crate::error::DescribeFHIRExportJobErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeFHIRExportJobErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeFHIRExportJobErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DescribeFHIRExportJobErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeFHIRExportJobErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeFHIRImportJobError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeFHIRImportJobError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeFHIRImportJobError> for Error {
    fn from(err: crate::error::DescribeFHIRImportJobError) -> Self {
        match err.kind {
            crate::error::DescribeFHIRImportJobErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::DescribeFHIRImportJobErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::DescribeFHIRImportJobErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::DescribeFHIRImportJobErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::DescribeFHIRImportJobErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListFHIRDatastoresError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListFHIRDatastoresError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListFHIRDatastoresError> for Error {
    fn from(err: crate::error::ListFHIRDatastoresError) -> Self {
        match err.kind {
            crate::error::ListFHIRDatastoresErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListFHIRDatastoresErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListFHIRDatastoresErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListFHIRDatastoresErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListFHIRExportJobsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListFHIRExportJobsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListFHIRExportJobsError> for Error {
    fn from(err: crate::error::ListFHIRExportJobsError) -> Self {
        match err.kind {
            crate::error::ListFHIRExportJobsErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListFHIRExportJobsErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListFHIRExportJobsErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListFHIRExportJobsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListFHIRExportJobsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListFHIRExportJobsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListFHIRImportJobsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListFHIRImportJobsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListFHIRImportJobsError> for Error {
    fn from(err: crate::error::ListFHIRImportJobsError) -> Self {
        match err.kind {
            crate::error::ListFHIRImportJobsErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::ListFHIRImportJobsErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::ListFHIRImportJobsErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListFHIRImportJobsErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::ListFHIRImportJobsErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListFHIRImportJobsErrorKind::Unhandled(inner) => {
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
            crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartFHIRExportJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartFHIRExportJobError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StartFHIRExportJobError> for Error {
    fn from(err: crate::error::StartFHIRExportJobError) -> Self {
        match err.kind {
            crate::error::StartFHIRExportJobErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::StartFHIRExportJobErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::StartFHIRExportJobErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::StartFHIRExportJobErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::StartFHIRExportJobErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::StartFHIRExportJobErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartFHIRImportJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::StartFHIRImportJobError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StartFHIRImportJobError> for Error {
    fn from(err: crate::error::StartFHIRImportJobError) -> Self {
        match err.kind {
            crate::error::StartFHIRImportJobErrorKind::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::error::StartFHIRImportJobErrorKind::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::error::StartFHIRImportJobErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::StartFHIRImportJobErrorKind::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::error::StartFHIRImportJobErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::StartFHIRImportJobErrorKind::Unhandled(inner) => {
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
            crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::TagResourceErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
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
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::error::UntagResourceErrorKind::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::error::UntagResourceErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
