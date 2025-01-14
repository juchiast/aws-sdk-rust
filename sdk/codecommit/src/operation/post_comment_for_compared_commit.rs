// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl PostCommentForComparedCommitInput {
    /// Consumes the builder and constructs an Operation<[`PostCommentForComparedCommit`](crate::operation::post_comment_for_compared_commit::PostCommentForComparedCommit)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        mut self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::post_comment_for_compared_commit::PostCommentForComparedCommit,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(_config.use_dual_stack)
            .set_use_fips(_config.use_fips)
            .set_endpoint(_config.endpoint_url.clone())
            .build()
            .map_err(|err| {
                aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                Some(params),
            ),
            Err(e) => (Err(e), None),
        };
        if self.client_request_token.is_none() {
            self.client_request_token = Some(_config.make_token.make_idempotency_token());
        }
        let mut request = {
            fn uri_base(
                _input: &crate::operation::post_comment_for_compared_commit::PostCommentForComparedCommitInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::post_comment_for_compared_commit::PostCommentForComparedCommitInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "CodeCommit_20150413.PostCommentForComparedCommit",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_post_comment_for_compared_commit::ser_post_comment_for_compared_commit_input(&self)?
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::meta::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_credentials_cache(
            &mut request.properties_mut(),
            _config.credentials_cache.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::post_comment_for_compared_commit::PostCommentForComparedCommit::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "PostCommentForComparedCommit",
            "codecommit",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `PostCommentForComparedCommit`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct PostCommentForComparedCommit;
impl PostCommentForComparedCommit {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PostCommentForComparedCommit {
    type Output = std::result::Result<
        crate::operation::post_comment_for_compared_commit::PostCommentForComparedCommitOutput,
        crate::operation::post_comment_for_compared_commit::PostCommentForComparedCommitError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_post_comment_for_compared_commit::de_post_comment_for_compared_commit_http_error(response)
        } else {
            crate::protocol_serde::shape_post_comment_for_compared_commit::de_post_comment_for_compared_commit_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type PostCommentForComparedCommitErrorKind = PostCommentForComparedCommitError;
/// Error type for the `PostCommentForComparedCommitError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PostCommentForComparedCommitError {
    /// <p>The before commit ID and the after commit ID are the same, which is not valid. The before commit ID and the after commit ID must be different commit IDs.</p>
    BeforeCommitIdAndAfterCommitIdAreSameException(
        crate::types::error::BeforeCommitIdAndAfterCommitIdAreSameException,
    ),
    /// <p>A client request token is required. A client request token is an unique, client-generated idempotency token that, when provided in a request, ensures the request cannot be repeated with a changed parameter. If a request is received with the same parameters and a token is included, the request returns information about the initial request that used that token.</p>
    ClientRequestTokenRequiredException(crate::types::error::ClientRequestTokenRequiredException),
    /// <p>The comment is empty. You must provide some content for a comment. The content cannot be null.</p>
    CommentContentRequiredException(crate::types::error::CommentContentRequiredException),
    /// <p>The comment is too large. Comments are limited to 1,000 characters.</p>
    CommentContentSizeLimitExceededException(
        crate::types::error::CommentContentSizeLimitExceededException,
    ),
    /// <p>The specified commit does not exist or no commit was specified, and the specified repository has no default branch.</p>
    CommitDoesNotExistException(crate::types::error::CommitDoesNotExistException),
    /// <p>A commit ID was not specified.</p>
    CommitIdRequiredException(crate::types::error::CommitIdRequiredException),
    /// <p>An encryption integrity check failed.</p>
    EncryptionIntegrityChecksFailedException(
        crate::types::error::EncryptionIntegrityChecksFailedException,
    ),
    /// <p>An encryption key could not be accessed.</p>
    EncryptionKeyAccessDeniedException(crate::types::error::EncryptionKeyAccessDeniedException),
    /// <p>The encryption key is disabled.</p>
    EncryptionKeyDisabledException(crate::types::error::EncryptionKeyDisabledException),
    /// <p>No encryption key was found.</p>
    EncryptionKeyNotFoundException(crate::types::error::EncryptionKeyNotFoundException),
    /// <p>The encryption key is not available.</p>
    EncryptionKeyUnavailableException(crate::types::error::EncryptionKeyUnavailableException),
    /// <p>The client request token is not valid. Either the token is not in a valid format, or the token has been used in a previous request and cannot be reused.</p>
    IdempotencyParameterMismatchException(
        crate::types::error::IdempotencyParameterMismatchException,
    ),
    /// <p>The client request token is not valid.</p>
    InvalidClientRequestTokenException(crate::types::error::InvalidClientRequestTokenException),
    /// <p>The specified commit ID is not valid.</p>
    InvalidCommitIdException(crate::types::error::InvalidCommitIdException),
    /// <p>The location of the file is not valid. Make sure that you include the file name and extension.</p>
    InvalidFileLocationException(crate::types::error::InvalidFileLocationException),
    /// <p>The position is not valid. Make sure that the line number exists in the version of the file you want to comment on.</p>
    InvalidFilePositionException(crate::types::error::InvalidFilePositionException),
    /// <p>The specified path is not valid.</p>
    InvalidPathException(crate::types::error::InvalidPathException),
    /// <p>Either the enum is not in a valid format, or the specified file version enum is not valid in respect to the current file version.</p>
    InvalidRelativeFileVersionEnumException(
        crate::types::error::InvalidRelativeFileVersionEnumException,
    ),
    /// <p>A specified repository name is not valid.</p> <note>
    /// <p>This exception occurs only when a specified repository name is not valid. Other exceptions occur when a required repository parameter is missing, or when a specified repository does not exist.</p>
    /// </note>
    InvalidRepositoryNameException(crate::types::error::InvalidRepositoryNameException),
    /// <p>The specified path does not exist.</p>
    PathDoesNotExistException(crate::types::error::PathDoesNotExistException),
    /// <p>The folderPath for a location cannot be null.</p>
    PathRequiredException(crate::types::error::PathRequiredException),
    /// <p>The specified repository does not exist.</p>
    RepositoryDoesNotExistException(crate::types::error::RepositoryDoesNotExistException),
    /// <p>A repository name is required, but was not specified.</p>
    RepositoryNameRequiredException(crate::types::error::RepositoryNameRequiredException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for PostCommentForComparedCommitError {
    fn create_unhandled_error(
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled({
            let mut builder = aws_smithy_types::error::Unhandled::builder().source(source);
            builder.set_meta(meta);
            builder.build()
        })
    }
}
impl std::fmt::Display for PostCommentForComparedCommitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BeforeCommitIdAndAfterCommitIdAreSameException(_inner) => _inner.fmt(f),
            Self::ClientRequestTokenRequiredException(_inner) => _inner.fmt(f),
            Self::CommentContentRequiredException(_inner) => _inner.fmt(f),
            Self::CommentContentSizeLimitExceededException(_inner) => _inner.fmt(f),
            Self::CommitDoesNotExistException(_inner) => _inner.fmt(f),
            Self::CommitIdRequiredException(_inner) => _inner.fmt(f),
            Self::EncryptionIntegrityChecksFailedException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyAccessDeniedException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyDisabledException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyNotFoundException(_inner) => _inner.fmt(f),
            Self::EncryptionKeyUnavailableException(_inner) => _inner.fmt(f),
            Self::IdempotencyParameterMismatchException(_inner) => _inner.fmt(f),
            Self::InvalidClientRequestTokenException(_inner) => _inner.fmt(f),
            Self::InvalidCommitIdException(_inner) => _inner.fmt(f),
            Self::InvalidFileLocationException(_inner) => _inner.fmt(f),
            Self::InvalidFilePositionException(_inner) => _inner.fmt(f),
            Self::InvalidPathException(_inner) => _inner.fmt(f),
            Self::InvalidRelativeFileVersionEnumException(_inner) => _inner.fmt(f),
            Self::InvalidRepositoryNameException(_inner) => _inner.fmt(f),
            Self::PathDoesNotExistException(_inner) => _inner.fmt(f),
            Self::PathRequiredException(_inner) => _inner.fmt(f),
            Self::RepositoryDoesNotExistException(_inner) => _inner.fmt(f),
            Self::RepositoryNameRequiredException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for PostCommentForComparedCommitError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::BeforeCommitIdAndAfterCommitIdAreSameException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ClientRequestTokenRequiredException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CommentContentRequiredException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CommentContentSizeLimitExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CommitDoesNotExistException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::CommitIdRequiredException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionIntegrityChecksFailedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyAccessDeniedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyDisabledException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyNotFoundException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::EncryptionKeyUnavailableException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::IdempotencyParameterMismatchException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidClientRequestTokenException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidCommitIdException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidFileLocationException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidFilePositionException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidPathException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidRelativeFileVersionEnumException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidRepositoryNameException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::PathDoesNotExistException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::PathRequiredException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::RepositoryDoesNotExistException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::RepositoryNameRequiredException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::post_comment_for_compared_commit::PostCommentForComparedCommitError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for PostCommentForComparedCommitError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl PostCommentForComparedCommitError {
    /// Creates the `PostCommentForComparedCommitError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `PostCommentForComparedCommitError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
    pub fn generic(err: aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err.clone())
                .meta(err)
                .build(),
        )
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::BeforeCommitIdAndAfterCommitIdAreSameException(e) => e.meta(),
            Self::ClientRequestTokenRequiredException(e) => e.meta(),
            Self::CommentContentRequiredException(e) => e.meta(),
            Self::CommentContentSizeLimitExceededException(e) => e.meta(),
            Self::CommitDoesNotExistException(e) => e.meta(),
            Self::CommitIdRequiredException(e) => e.meta(),
            Self::EncryptionIntegrityChecksFailedException(e) => e.meta(),
            Self::EncryptionKeyAccessDeniedException(e) => e.meta(),
            Self::EncryptionKeyDisabledException(e) => e.meta(),
            Self::EncryptionKeyNotFoundException(e) => e.meta(),
            Self::EncryptionKeyUnavailableException(e) => e.meta(),
            Self::IdempotencyParameterMismatchException(e) => e.meta(),
            Self::InvalidClientRequestTokenException(e) => e.meta(),
            Self::InvalidCommitIdException(e) => e.meta(),
            Self::InvalidFileLocationException(e) => e.meta(),
            Self::InvalidFilePositionException(e) => e.meta(),
            Self::InvalidPathException(e) => e.meta(),
            Self::InvalidRelativeFileVersionEnumException(e) => e.meta(),
            Self::InvalidRepositoryNameException(e) => e.meta(),
            Self::PathDoesNotExistException(e) => e.meta(),
            Self::PathRequiredException(e) => e.meta(),
            Self::RepositoryDoesNotExistException(e) => e.meta(),
            Self::RepositoryNameRequiredException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::BeforeCommitIdAndAfterCommitIdAreSameException`.
    pub fn is_before_commit_id_and_after_commit_id_are_same_exception(&self) -> bool {
        matches!(
            self,
            Self::BeforeCommitIdAndAfterCommitIdAreSameException(_)
        )
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::ClientRequestTokenRequiredException`.
    pub fn is_client_request_token_required_exception(&self) -> bool {
        matches!(self, Self::ClientRequestTokenRequiredException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::CommentContentRequiredException`.
    pub fn is_comment_content_required_exception(&self) -> bool {
        matches!(self, Self::CommentContentRequiredException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::CommentContentSizeLimitExceededException`.
    pub fn is_comment_content_size_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::CommentContentSizeLimitExceededException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::CommitDoesNotExistException`.
    pub fn is_commit_does_not_exist_exception(&self) -> bool {
        matches!(self, Self::CommitDoesNotExistException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::CommitIdRequiredException`.
    pub fn is_commit_id_required_exception(&self) -> bool {
        matches!(self, Self::CommitIdRequiredException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::EncryptionIntegrityChecksFailedException`.
    pub fn is_encryption_integrity_checks_failed_exception(&self) -> bool {
        matches!(self, Self::EncryptionIntegrityChecksFailedException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::EncryptionKeyAccessDeniedException`.
    pub fn is_encryption_key_access_denied_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyAccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::EncryptionKeyDisabledException`.
    pub fn is_encryption_key_disabled_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyDisabledException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::EncryptionKeyNotFoundException`.
    pub fn is_encryption_key_not_found_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyNotFoundException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::EncryptionKeyUnavailableException`.
    pub fn is_encryption_key_unavailable_exception(&self) -> bool {
        matches!(self, Self::EncryptionKeyUnavailableException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::IdempotencyParameterMismatchException`.
    pub fn is_idempotency_parameter_mismatch_exception(&self) -> bool {
        matches!(self, Self::IdempotencyParameterMismatchException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::InvalidClientRequestTokenException`.
    pub fn is_invalid_client_request_token_exception(&self) -> bool {
        matches!(self, Self::InvalidClientRequestTokenException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::InvalidCommitIdException`.
    pub fn is_invalid_commit_id_exception(&self) -> bool {
        matches!(self, Self::InvalidCommitIdException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::InvalidFileLocationException`.
    pub fn is_invalid_file_location_exception(&self) -> bool {
        matches!(self, Self::InvalidFileLocationException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::InvalidFilePositionException`.
    pub fn is_invalid_file_position_exception(&self) -> bool {
        matches!(self, Self::InvalidFilePositionException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::InvalidPathException`.
    pub fn is_invalid_path_exception(&self) -> bool {
        matches!(self, Self::InvalidPathException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::InvalidRelativeFileVersionEnumException`.
    pub fn is_invalid_relative_file_version_enum_exception(&self) -> bool {
        matches!(self, Self::InvalidRelativeFileVersionEnumException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::InvalidRepositoryNameException`.
    pub fn is_invalid_repository_name_exception(&self) -> bool {
        matches!(self, Self::InvalidRepositoryNameException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::PathDoesNotExistException`.
    pub fn is_path_does_not_exist_exception(&self) -> bool {
        matches!(self, Self::PathDoesNotExistException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::PathRequiredException`.
    pub fn is_path_required_exception(&self) -> bool {
        matches!(self, Self::PathRequiredException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::RepositoryDoesNotExistException`.
    pub fn is_repository_does_not_exist_exception(&self) -> bool {
        matches!(self, Self::RepositoryDoesNotExistException(_))
    }
    /// Returns `true` if the error kind is `PostCommentForComparedCommitError::RepositoryNameRequiredException`.
    pub fn is_repository_name_required_exception(&self) -> bool {
        matches!(self, Self::RepositoryNameRequiredException(_))
    }
}
impl std::error::Error for PostCommentForComparedCommitError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BeforeCommitIdAndAfterCommitIdAreSameException(_inner) => Some(_inner),
            Self::ClientRequestTokenRequiredException(_inner) => Some(_inner),
            Self::CommentContentRequiredException(_inner) => Some(_inner),
            Self::CommentContentSizeLimitExceededException(_inner) => Some(_inner),
            Self::CommitDoesNotExistException(_inner) => Some(_inner),
            Self::CommitIdRequiredException(_inner) => Some(_inner),
            Self::EncryptionIntegrityChecksFailedException(_inner) => Some(_inner),
            Self::EncryptionKeyAccessDeniedException(_inner) => Some(_inner),
            Self::EncryptionKeyDisabledException(_inner) => Some(_inner),
            Self::EncryptionKeyNotFoundException(_inner) => Some(_inner),
            Self::EncryptionKeyUnavailableException(_inner) => Some(_inner),
            Self::IdempotencyParameterMismatchException(_inner) => Some(_inner),
            Self::InvalidClientRequestTokenException(_inner) => Some(_inner),
            Self::InvalidCommitIdException(_inner) => Some(_inner),
            Self::InvalidFileLocationException(_inner) => Some(_inner),
            Self::InvalidFilePositionException(_inner) => Some(_inner),
            Self::InvalidPathException(_inner) => Some(_inner),
            Self::InvalidRelativeFileVersionEnumException(_inner) => Some(_inner),
            Self::InvalidRepositoryNameException(_inner) => Some(_inner),
            Self::PathDoesNotExistException(_inner) => Some(_inner),
            Self::PathRequiredException(_inner) => Some(_inner),
            Self::RepositoryDoesNotExistException(_inner) => Some(_inner),
            Self::RepositoryNameRequiredException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::post_comment_for_compared_commit::_post_comment_for_compared_commit_output::PostCommentForComparedCommitOutput;

pub use crate::operation::post_comment_for_compared_commit::_post_comment_for_compared_commit_input::PostCommentForComparedCommitInput;

mod _post_comment_for_compared_commit_input;

mod _post_comment_for_compared_commit_output;

/// Builders
pub mod builders;
