// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFoundException(crate::error::ContainerNotFoundException),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>Could not perform an operation on an object that does not exist.</p>
    ObjectNotFoundException(crate::error::ObjectNotFoundException),
    /// <p>The requested content range is not valid.</p>
    RequestedRangeNotSatisfiableException(crate::error::RequestedRangeNotSatisfiableException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ContainerNotFoundException(inner) => inner.fmt(f),
            Error::InternalServerError(inner) => inner.fmt(f),
            Error::ObjectNotFoundException(inner) => inner.fmt(f),
            Error::RequestedRangeNotSatisfiableException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteObjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteObjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteObjectErrorKind::ContainerNotFoundException(inner) => {
                    Error::ContainerNotFoundException(inner)
                }
                crate::error::DeleteObjectErrorKind::InternalServerError(inner) => {
                    Error::InternalServerError(inner)
                }
                crate::error::DeleteObjectErrorKind::ObjectNotFoundException(inner) => {
                    Error::ObjectNotFoundException(inner)
                }
                crate::error::DeleteObjectErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeObjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeObjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeObjectErrorKind::ContainerNotFoundException(inner) => {
                    Error::ContainerNotFoundException(inner)
                }
                crate::error::DescribeObjectErrorKind::InternalServerError(inner) => {
                    Error::InternalServerError(inner)
                }
                crate::error::DescribeObjectErrorKind::ObjectNotFoundException(inner) => {
                    Error::ObjectNotFoundException(inner)
                }
                crate::error::DescribeObjectErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetObjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetObjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetObjectErrorKind::ContainerNotFoundException(inner) => {
                    Error::ContainerNotFoundException(inner)
                }
                crate::error::GetObjectErrorKind::InternalServerError(inner) => {
                    Error::InternalServerError(inner)
                }
                crate::error::GetObjectErrorKind::ObjectNotFoundException(inner) => {
                    Error::ObjectNotFoundException(inner)
                }
                crate::error::GetObjectErrorKind::RequestedRangeNotSatisfiableException(inner) => {
                    Error::RequestedRangeNotSatisfiableException(inner)
                }
                crate::error::GetObjectErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListItemsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListItemsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListItemsErrorKind::ContainerNotFoundException(inner) => {
                    Error::ContainerNotFoundException(inner)
                }
                crate::error::ListItemsErrorKind::InternalServerError(inner) => {
                    Error::InternalServerError(inner)
                }
                crate::error::ListItemsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutObjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutObjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PutObjectErrorKind::ContainerNotFoundException(inner) => {
                    Error::ContainerNotFoundException(inner)
                }
                crate::error::PutObjectErrorKind::InternalServerError(inner) => {
                    Error::InternalServerError(inner)
                }
                crate::error::PutObjectErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
