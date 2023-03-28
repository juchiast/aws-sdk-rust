// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `GetRecommendations` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetRecommendationsError {
    /// Kind of error that occurred.
    pub kind: GetRecommendationsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for GetRecommendationsError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: GetRecommendationsErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `GetRecommendations` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetRecommendationsErrorKind {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInputException(crate::error::InvalidInputException),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
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
impl std::fmt::Display for GetRecommendationsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetRecommendationsErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            GetRecommendationsErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            GetRecommendationsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetRecommendationsError {
    fn code(&self) -> Option<&str> {
        GetRecommendationsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetRecommendationsError {
    /// Creates a new `GetRecommendationsError`.
    pub fn new(kind: GetRecommendationsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetRecommendationsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetRecommendationsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `GetRecommendationsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetRecommendationsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `GetRecommendationsErrorKind::InvalidInputException`.
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetRecommendationsErrorKind::InvalidInputException(_)
        )
    }
    /// Returns `true` if the error kind is `GetRecommendationsErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetRecommendationsErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for GetRecommendationsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetRecommendationsErrorKind::InvalidInputException(_inner) => Some(_inner),
            GetRecommendationsErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            GetRecommendationsErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// <p>The specified resource does not exist.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ResourceNotFoundException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ResourceNotFoundException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFoundException {}
/// See [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
pub mod resource_not_found_exception {

    /// A builder for [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message,
            }
        }
    }
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
    pub fn builder() -> crate::error::resource_not_found_exception::Builder {
        crate::error::resource_not_found_exception::Builder::default()
    }
}

/// <p>Provide a valid value for the field or parameter.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InvalidInputException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl InvalidInputException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidInputException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidInputException")?;
        if let Some(inner_2) = &self.message {
            {
                write!(f, ": {}", inner_2)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InvalidInputException {}
/// See [`InvalidInputException`](crate::error::InvalidInputException).
pub mod invalid_input_exception {

    /// A builder for [`InvalidInputException`](crate::error::InvalidInputException).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidInputException`](crate::error::InvalidInputException).
        pub fn build(self) -> crate::error::InvalidInputException {
            crate::error::InvalidInputException {
                message: self.message,
            }
        }
    }
}
impl InvalidInputException {
    /// Creates a new builder-style object to manufacture [`InvalidInputException`](crate::error::InvalidInputException).
    pub fn builder() -> crate::error::invalid_input_exception::Builder {
        crate::error::invalid_input_exception::Builder::default()
    }
}

/// Error type for the `GetPersonalizedRanking` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetPersonalizedRankingError {
    /// Kind of error that occurred.
    pub kind: GetPersonalizedRankingErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for GetPersonalizedRankingError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: GetPersonalizedRankingErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `GetPersonalizedRanking` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetPersonalizedRankingErrorKind {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInputException(crate::error::InvalidInputException),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
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
impl std::fmt::Display for GetPersonalizedRankingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetPersonalizedRankingErrorKind::InvalidInputException(_inner) => _inner.fmt(f),
            GetPersonalizedRankingErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            GetPersonalizedRankingErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetPersonalizedRankingError {
    fn code(&self) -> Option<&str> {
        GetPersonalizedRankingError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetPersonalizedRankingError {
    /// Creates a new `GetPersonalizedRankingError`.
    pub fn new(kind: GetPersonalizedRankingErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetPersonalizedRankingError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetPersonalizedRankingErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
            meta: Default::default(),
        }
    }

    /// Creates the `GetPersonalizedRankingError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetPersonalizedRankingErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `GetPersonalizedRankingErrorKind::InvalidInputException`.
    pub fn is_invalid_input_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetPersonalizedRankingErrorKind::InvalidInputException(_)
        )
    }
    /// Returns `true` if the error kind is `GetPersonalizedRankingErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetPersonalizedRankingErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for GetPersonalizedRankingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetPersonalizedRankingErrorKind::InvalidInputException(_inner) => Some(_inner),
            GetPersonalizedRankingErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            GetPersonalizedRankingErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

///
/// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
///
/// When logging an error from the SDK, it is recommended that you either wrap the error in
/// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
/// error reporter library that visits the error's cause/source chain, or call
/// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
///
#[derive(Debug)]
pub struct Unhandled {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
}
impl Unhandled {
    #[allow(unused)]
    pub(crate) fn new(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self { source }
    }
}
impl std::fmt::Display for Unhandled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "unhandled error")
    }
}
impl std::error::Error for Unhandled {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref() as _)
    }
}
