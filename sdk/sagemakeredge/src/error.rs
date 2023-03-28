// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `SendHeartbeat` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct SendHeartbeatError {
    /// Kind of error that occurred.
    pub kind: SendHeartbeatErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for SendHeartbeatError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: SendHeartbeatErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `SendHeartbeat` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum SendHeartbeatErrorKind {
    /// <p>An internal failure occurred. Try your request again. If the problem persists, contact Amazon Web Services customer support.</p>
    InternalServiceException(crate::error::InternalServiceException),
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
impl std::fmt::Display for SendHeartbeatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            SendHeartbeatErrorKind::InternalServiceException(_inner) => _inner.fmt(f),
            SendHeartbeatErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for SendHeartbeatError {
    fn code(&self) -> Option<&str> {
        SendHeartbeatError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl SendHeartbeatError {
    /// Creates a new `SendHeartbeatError`.
    pub fn new(kind: SendHeartbeatErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `SendHeartbeatError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: SendHeartbeatErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `SendHeartbeatError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: SendHeartbeatErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
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
    /// Returns `true` if the error kind is `SendHeartbeatErrorKind::InternalServiceException`.
    pub fn is_internal_service_exception(&self) -> bool {
        matches!(
            &self.kind,
            SendHeartbeatErrorKind::InternalServiceException(_)
        )
    }
}
impl std::error::Error for SendHeartbeatError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            SendHeartbeatErrorKind::InternalServiceException(_inner) => Some(_inner),
            SendHeartbeatErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// <p>An internal failure occurred. Try your request again. If the problem persists, contact Amazon Web Services customer support.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InternalServiceException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl InternalServiceException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalServiceException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServiceException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InternalServiceException {}
/// See [`InternalServiceException`](crate::error::InternalServiceException).
pub mod internal_service_exception {

    /// A builder for [`InternalServiceException`](crate::error::InternalServiceException).
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
        /// Consumes the builder and constructs a [`InternalServiceException`](crate::error::InternalServiceException).
        pub fn build(self) -> crate::error::InternalServiceException {
            crate::error::InternalServiceException {
                message: self.message,
            }
        }
    }
}
impl InternalServiceException {
    /// Creates a new builder-style object to manufacture [`InternalServiceException`](crate::error::InternalServiceException).
    pub fn builder() -> crate::error::internal_service_exception::Builder {
        crate::error::internal_service_exception::Builder::default()
    }
}

/// Error type for the `GetDeviceRegistration` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetDeviceRegistrationError {
    /// Kind of error that occurred.
    pub kind: GetDeviceRegistrationErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for GetDeviceRegistrationError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: GetDeviceRegistrationErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `GetDeviceRegistration` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetDeviceRegistrationErrorKind {
    /// <p>An internal failure occurred. Try your request again. If the problem persists, contact Amazon Web Services customer support.</p>
    InternalServiceException(crate::error::InternalServiceException),
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
impl std::fmt::Display for GetDeviceRegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetDeviceRegistrationErrorKind::InternalServiceException(_inner) => _inner.fmt(f),
            GetDeviceRegistrationErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetDeviceRegistrationError {
    fn code(&self) -> Option<&str> {
        GetDeviceRegistrationError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetDeviceRegistrationError {
    /// Creates a new `GetDeviceRegistrationError`.
    pub fn new(kind: GetDeviceRegistrationErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetDeviceRegistrationError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetDeviceRegistrationErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
            meta: Default::default(),
        }
    }

    /// Creates the `GetDeviceRegistrationError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetDeviceRegistrationErrorKind::Unhandled(crate::error::Unhandled::new(
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
    /// Returns `true` if the error kind is `GetDeviceRegistrationErrorKind::InternalServiceException`.
    pub fn is_internal_service_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetDeviceRegistrationErrorKind::InternalServiceException(_)
        )
    }
}
impl std::error::Error for GetDeviceRegistrationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetDeviceRegistrationErrorKind::InternalServiceException(_inner) => Some(_inner),
            GetDeviceRegistrationErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// Error type for the `GetDeployments` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetDeploymentsError {
    /// Kind of error that occurred.
    pub kind: GetDeploymentsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for GetDeploymentsError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: GetDeploymentsErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `GetDeployments` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetDeploymentsErrorKind {
    /// <p>An internal failure occurred. Try your request again. If the problem persists, contact Amazon Web Services customer support.</p>
    InternalServiceException(crate::error::InternalServiceException),
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
impl std::fmt::Display for GetDeploymentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetDeploymentsErrorKind::InternalServiceException(_inner) => _inner.fmt(f),
            GetDeploymentsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetDeploymentsError {
    fn code(&self) -> Option<&str> {
        GetDeploymentsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetDeploymentsError {
    /// Creates a new `GetDeploymentsError`.
    pub fn new(kind: GetDeploymentsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetDeploymentsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetDeploymentsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
            meta: Default::default(),
        }
    }

    /// Creates the `GetDeploymentsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetDeploymentsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
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
    /// Returns `true` if the error kind is `GetDeploymentsErrorKind::InternalServiceException`.
    pub fn is_internal_service_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetDeploymentsErrorKind::InternalServiceException(_)
        )
    }
}
impl std::error::Error for GetDeploymentsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetDeploymentsErrorKind::InternalServiceException(_inner) => Some(_inner),
            GetDeploymentsErrorKind::Unhandled(_inner) => Some(_inner),
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
