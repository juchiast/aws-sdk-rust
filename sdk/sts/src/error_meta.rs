// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The web identity token that was passed is expired or is not valid. Get a new identity token from the identity provider and then retry the request.</p>
    ExpiredTokenException(crate::error::ExpiredTokenException),
    /// <p>The request could not be fulfilled because the identity provider (IDP) that was asked to verify the incoming identity token could not be reached. This is often a transient error caused by network conditions. Retry the request a limited number of times so that you don't exceed the request rate. If the error persists, the identity provider might be down or not responding.</p>
    IdpCommunicationErrorException(crate::error::IdpCommunicationErrorException),
    /// <p>The identity provider (IdP) reported that authentication failed. This might be because the claim is invalid.</p>
    /// <p>If this error is returned for the <code>AssumeRoleWithWebIdentity</code> operation, it can also mean that the claim has expired or has been explicitly revoked. </p>
    IdpRejectedClaimException(crate::error::IdpRejectedClaimException),
    /// <p>The error returned if the message passed to <code>DecodeAuthorizationMessage</code> was invalid. This can happen if the token contains invalid characters, such as linebreaks. </p>
    InvalidAuthorizationMessageException(crate::error::InvalidAuthorizationMessageException),
    /// <p>The web identity token that was passed could not be validated by Amazon Web Services. Get a new identity token from the identity provider and then retry the request.</p>
    InvalidIdentityTokenException(crate::error::InvalidIdentityTokenException),
    /// <p>The request was rejected because the policy document was malformed. The error message describes the specific error.</p>
    MalformedPolicyDocumentException(crate::error::MalformedPolicyDocumentException),
    /// <p>The request was rejected because the total packed size of the session policies and session tags combined was too large. An Amazon Web Services conversion compresses the session policy document, session policy ARNs, and session tags into a packed binary format that has a separate limit. The error message indicates by percentage how close the policies and tags are to the upper size limit. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_session-tags.html">Passing Session Tags in STS</a> in the <i>IAM User Guide</i>.</p>
    /// <p>You could receive this error even though you meet other defined session policy and session tag limits. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html#reference_iam-limits-entity-length">IAM and STS Entity Character Limits</a> in the <i>IAM User Guide</i>.</p>
    PackedPolicyTooLargeException(crate::error::PackedPolicyTooLargeException),
    /// <p>STS is not activated in the requested region for the account that is being asked to generate credentials. The account administrator must use the IAM console to activate STS in that region. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and Deactivating Amazon Web Services STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
    RegionDisabledException(crate::error::RegionDisabledException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ExpiredTokenException(inner) => inner.fmt(f),
            Error::IdpCommunicationErrorException(inner) => inner.fmt(f),
            Error::IdpRejectedClaimException(inner) => inner.fmt(f),
            Error::InvalidAuthorizationMessageException(inner) => inner.fmt(f),
            Error::InvalidIdentityTokenException(inner) => inner.fmt(f),
            Error::MalformedPolicyDocumentException(inner) => inner.fmt(f),
            Error::PackedPolicyTooLargeException(inner) => inner.fmt(f),
            Error::RegionDisabledException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AssumeRoleError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::AssumeRoleError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::AssumeRoleErrorKind::ExpiredTokenException(inner) => {
                    Error::ExpiredTokenException(inner)
                }
                crate::error::AssumeRoleErrorKind::MalformedPolicyDocumentException(inner) => {
                    Error::MalformedPolicyDocumentException(inner)
                }
                crate::error::AssumeRoleErrorKind::PackedPolicyTooLargeException(inner) => {
                    Error::PackedPolicyTooLargeException(inner)
                }
                crate::error::AssumeRoleErrorKind::RegionDisabledException(inner) => {
                    Error::RegionDisabledException(inner)
                }
                crate::error::AssumeRoleErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AssumeRoleWithSAMLError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AssumeRoleWithSAMLError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::AssumeRoleWithSAMLErrorKind::ExpiredTokenException(inner) => {
                    Error::ExpiredTokenException(inner)
                }
                crate::error::AssumeRoleWithSAMLErrorKind::IdpRejectedClaimException(inner) => {
                    Error::IdpRejectedClaimException(inner)
                }
                crate::error::AssumeRoleWithSAMLErrorKind::InvalidIdentityTokenException(inner) => {
                    Error::InvalidIdentityTokenException(inner)
                }
                crate::error::AssumeRoleWithSAMLErrorKind::MalformedPolicyDocumentException(
                    inner,
                ) => Error::MalformedPolicyDocumentException(inner),
                crate::error::AssumeRoleWithSAMLErrorKind::PackedPolicyTooLargeException(inner) => {
                    Error::PackedPolicyTooLargeException(inner)
                }
                crate::error::AssumeRoleWithSAMLErrorKind::RegionDisabledException(inner) => {
                    Error::RegionDisabledException(inner)
                }
                crate::error::AssumeRoleWithSAMLErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AssumeRoleWithWebIdentityError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AssumeRoleWithWebIdentityError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::AssumeRoleWithWebIdentityErrorKind::ExpiredTokenException(inner) => Error::ExpiredTokenException(inner),
                crate::error::AssumeRoleWithWebIdentityErrorKind::IdpCommunicationErrorException(inner) => Error::IdpCommunicationErrorException(inner),
                crate::error::AssumeRoleWithWebIdentityErrorKind::IdpRejectedClaimException(inner) => Error::IdpRejectedClaimException(inner),
                crate::error::AssumeRoleWithWebIdentityErrorKind::InvalidIdentityTokenException(inner) => Error::InvalidIdentityTokenException(inner),
                crate::error::AssumeRoleWithWebIdentityErrorKind::MalformedPolicyDocumentException(inner) => Error::MalformedPolicyDocumentException(inner),
                crate::error::AssumeRoleWithWebIdentityErrorKind::PackedPolicyTooLargeException(inner) => Error::PackedPolicyTooLargeException(inner),
                crate::error::AssumeRoleWithWebIdentityErrorKind::RegionDisabledException(inner) => Error::RegionDisabledException(inner),
                crate::error::AssumeRoleWithWebIdentityErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DecodeAuthorizationMessageError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DecodeAuthorizationMessageError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DecodeAuthorizationMessageErrorKind::InvalidAuthorizationMessageException(inner) => Error::InvalidAuthorizationMessageException(inner),
                crate::error::DecodeAuthorizationMessageErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetAccessKeyInfoError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetAccessKeyInfoError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetAccessKeyInfoErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetCallerIdentityError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetCallerIdentityError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetCallerIdentityErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetFederationTokenError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetFederationTokenError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetFederationTokenErrorKind::MalformedPolicyDocumentException(
                    inner,
                ) => Error::MalformedPolicyDocumentException(inner),
                crate::error::GetFederationTokenErrorKind::PackedPolicyTooLargeException(inner) => {
                    Error::PackedPolicyTooLargeException(inner)
                }
                crate::error::GetFederationTokenErrorKind::RegionDisabledException(inner) => {
                    Error::RegionDisabledException(inner)
                }
                crate::error::GetFederationTokenErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSessionTokenError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSessionTokenError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetSessionTokenErrorKind::RegionDisabledException(inner) => {
                    Error::RegionDisabledException(inner)
                }
                crate::error::GetSessionTokenErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
