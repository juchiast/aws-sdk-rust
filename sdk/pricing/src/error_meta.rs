// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The pagination token expired. Try again without a pagination token.</p>
    ExpiredNextTokenException(crate::error::ExpiredNextTokenException),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalErrorException(crate::error::InternalErrorException),
    /// <p>The pagination token is invalid. Try again without a pagination token.</p>
    InvalidNextTokenException(crate::error::InvalidNextTokenException),
    /// <p>One or more parameters had an invalid value.</p>
    InvalidParameterException(crate::error::InvalidParameterException),
    /// <p>The requested resource can't be found.</p>
    NotFoundException(crate::error::NotFoundException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ExpiredNextTokenException(inner) => inner.fmt(f),
            Error::InternalErrorException(inner) => inner.fmt(f),
            Error::InvalidNextTokenException(inner) => inner.fmt(f),
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeServicesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeServicesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeServicesErrorKind::ExpiredNextTokenException(inner) => {
                    Error::ExpiredNextTokenException(inner)
                }
                crate::error::DescribeServicesErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::DescribeServicesErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::DescribeServicesErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeServicesErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DescribeServicesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetAttributeValuesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetAttributeValuesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetAttributeValuesErrorKind::ExpiredNextTokenException(inner) => {
                    Error::ExpiredNextTokenException(inner)
                }
                crate::error::GetAttributeValuesErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::GetAttributeValuesErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::GetAttributeValuesErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::GetAttributeValuesErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::GetAttributeValuesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetProductsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetProductsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetProductsErrorKind::ExpiredNextTokenException(inner) => {
                    Error::ExpiredNextTokenException(inner)
                }
                crate::error::GetProductsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::GetProductsErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::GetProductsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::GetProductsErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::GetProductsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
