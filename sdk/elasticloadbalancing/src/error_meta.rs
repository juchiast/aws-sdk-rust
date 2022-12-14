// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The specified load balancer does not exist.</p>
    AccessPointNotFoundException(crate::error::AccessPointNotFoundException),
    /// <p>The specified ARN does not refer to a valid SSL certificate in AWS Identity and Access Management (IAM) or AWS Certificate Manager (ACM). Note that if you recently uploaded the certificate to IAM, this error might indicate that the certificate is not fully available yet.</p>
    CertificateNotFoundException(crate::error::CertificateNotFoundException),
    /// <p>A request made by Elastic Load Balancing to another service exceeds the maximum request rate permitted for your account.</p>
    DependencyThrottleException(crate::error::DependencyThrottleException),
    /// <p>The specified load balancer name already exists for this account.</p>
    DuplicateAccessPointNameException(crate::error::DuplicateAccessPointNameException),
    /// <p>A listener already exists for the specified load balancer name and port, but with a different instance port, protocol, or SSL certificate.</p>
    DuplicateListenerException(crate::error::DuplicateListenerException),
    /// <p>A policy with the specified name already exists for this load balancer.</p>
    DuplicatePolicyNameException(crate::error::DuplicatePolicyNameException),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeysException(crate::error::DuplicateTagKeysException),
    /// <p>The requested configuration change is not valid.</p>
    InvalidConfigurationRequestException(crate::error::InvalidConfigurationRequestException),
    /// <p>The specified endpoint is not valid.</p>
    InvalidEndPointException(crate::error::InvalidEndPointException),
    /// <p>The specified value for the schema is not valid. You can only specify a scheme for load balancers in a VPC.</p>
    InvalidSchemeException(crate::error::InvalidSchemeException),
    /// <p>One or more of the specified security groups do not exist.</p>
    InvalidSecurityGroupException(crate::error::InvalidSecurityGroupException),
    /// <p>The specified VPC has no associated Internet gateway.</p>
    InvalidSubnetException(crate::error::InvalidSubnetException),
    /// <p>The load balancer does not have a listener configured at the specified port.</p>
    ListenerNotFoundException(crate::error::ListenerNotFoundException),
    /// <p>The specified load balancer attribute does not exist.</p>
    LoadBalancerAttributeNotFoundException(crate::error::LoadBalancerAttributeNotFoundException),
    /// <p>This operation is not allowed.</p>
    OperationNotPermittedException(crate::error::OperationNotPermittedException),
    /// <p>One or more of the specified policies do not exist.</p>
    PolicyNotFoundException(crate::error::PolicyNotFoundException),
    /// <p>One or more of the specified policy types do not exist.</p>
    PolicyTypeNotFoundException(crate::error::PolicyTypeNotFoundException),
    /// <p>One or more of the specified subnets do not exist.</p>
    SubnetNotFoundException(crate::error::SubnetNotFoundException),
    /// <p>The quota for the number of load balancers has been reached.</p>
    TooManyAccessPointsException(crate::error::TooManyAccessPointsException),
    /// <p>The quota for the number of policies for this load balancer has been reached.</p>
    TooManyPoliciesException(crate::error::TooManyPoliciesException),
    /// <p>The quota for the number of tags that can be assigned to a load balancer has been reached.</p>
    TooManyTagsException(crate::error::TooManyTagsException),
    /// <p>The specified protocol or signature version is not supported.</p>
    UnsupportedProtocolException(crate::error::UnsupportedProtocolException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessPointNotFoundException(inner) => inner.fmt(f),
            Error::CertificateNotFoundException(inner) => inner.fmt(f),
            Error::DependencyThrottleException(inner) => inner.fmt(f),
            Error::DuplicateAccessPointNameException(inner) => inner.fmt(f),
            Error::DuplicateListenerException(inner) => inner.fmt(f),
            Error::DuplicatePolicyNameException(inner) => inner.fmt(f),
            Error::DuplicateTagKeysException(inner) => inner.fmt(f),
            Error::InvalidConfigurationRequestException(inner) => inner.fmt(f),
            Error::InvalidEndPointException(inner) => inner.fmt(f),
            Error::InvalidSchemeException(inner) => inner.fmt(f),
            Error::InvalidSecurityGroupException(inner) => inner.fmt(f),
            Error::InvalidSubnetException(inner) => inner.fmt(f),
            Error::ListenerNotFoundException(inner) => inner.fmt(f),
            Error::LoadBalancerAttributeNotFoundException(inner) => inner.fmt(f),
            Error::OperationNotPermittedException(inner) => inner.fmt(f),
            Error::PolicyNotFoundException(inner) => inner.fmt(f),
            Error::PolicyTypeNotFoundException(inner) => inner.fmt(f),
            Error::SubnetNotFoundException(inner) => inner.fmt(f),
            Error::TooManyAccessPointsException(inner) => inner.fmt(f),
            Error::TooManyPoliciesException(inner) => inner.fmt(f),
            Error::TooManyTagsException(inner) => inner.fmt(f),
            Error::UnsupportedProtocolException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AddTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::AddTagsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::AddTagsErrorKind::AccessPointNotFoundException(inner) => {
                    Error::AccessPointNotFoundException(inner)
                }
                crate::error::AddTagsErrorKind::DuplicateTagKeysException(inner) => {
                    Error::DuplicateTagKeysException(inner)
                }
                crate::error::AddTagsErrorKind::TooManyTagsException(inner) => {
                    Error::TooManyTagsException(inner)
                }
                crate::error::AddTagsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::ApplySecurityGroupsToLoadBalancerError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::ApplySecurityGroupsToLoadBalancerError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ApplySecurityGroupsToLoadBalancerErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::ApplySecurityGroupsToLoadBalancerErrorKind::InvalidConfigurationRequestException(inner) => Error::InvalidConfigurationRequestException(inner),
                crate::error::ApplySecurityGroupsToLoadBalancerErrorKind::InvalidSecurityGroupException(inner) => Error::InvalidSecurityGroupException(inner),
                crate::error::ApplySecurityGroupsToLoadBalancerErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AttachLoadBalancerToSubnetsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::AttachLoadBalancerToSubnetsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::AttachLoadBalancerToSubnetsErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::AttachLoadBalancerToSubnetsErrorKind::InvalidConfigurationRequestException(inner) => Error::InvalidConfigurationRequestException(inner),
                crate::error::AttachLoadBalancerToSubnetsErrorKind::InvalidSubnetException(inner) => Error::InvalidSubnetException(inner),
                crate::error::AttachLoadBalancerToSubnetsErrorKind::SubnetNotFoundException(inner) => Error::SubnetNotFoundException(inner),
                crate::error::AttachLoadBalancerToSubnetsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ConfigureHealthCheckError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ConfigureHealthCheckError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ConfigureHealthCheckErrorKind::AccessPointNotFoundException(
                    inner,
                ) => Error::AccessPointNotFoundException(inner),
                crate::error::ConfigureHealthCheckErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::CreateAppCookieStickinessPolicyError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::CreateAppCookieStickinessPolicyError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateAppCookieStickinessPolicyErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::CreateAppCookieStickinessPolicyErrorKind::DuplicatePolicyNameException(inner) => Error::DuplicatePolicyNameException(inner),
                crate::error::CreateAppCookieStickinessPolicyErrorKind::InvalidConfigurationRequestException(inner) => Error::InvalidConfigurationRequestException(inner),
                crate::error::CreateAppCookieStickinessPolicyErrorKind::TooManyPoliciesException(inner) => Error::TooManyPoliciesException(inner),
                crate::error::CreateAppCookieStickinessPolicyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::CreateLBCookieStickinessPolicyError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::CreateLBCookieStickinessPolicyError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateLBCookieStickinessPolicyErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::CreateLBCookieStickinessPolicyErrorKind::DuplicatePolicyNameException(inner) => Error::DuplicatePolicyNameException(inner),
                crate::error::CreateLBCookieStickinessPolicyErrorKind::InvalidConfigurationRequestException(inner) => Error::InvalidConfigurationRequestException(inner),
                crate::error::CreateLBCookieStickinessPolicyErrorKind::TooManyPoliciesException(inner) => Error::TooManyPoliciesException(inner),
                crate::error::CreateLBCookieStickinessPolicyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateLoadBalancerError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateLoadBalancerError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateLoadBalancerErrorKind::CertificateNotFoundException(inner) => {
                    Error::CertificateNotFoundException(inner)
                }
                crate::error::CreateLoadBalancerErrorKind::DuplicateAccessPointNameException(
                    inner,
                ) => Error::DuplicateAccessPointNameException(inner),
                crate::error::CreateLoadBalancerErrorKind::DuplicateTagKeysException(inner) => {
                    Error::DuplicateTagKeysException(inner)
                }
                crate::error::CreateLoadBalancerErrorKind::InvalidConfigurationRequestException(
                    inner,
                ) => Error::InvalidConfigurationRequestException(inner),
                crate::error::CreateLoadBalancerErrorKind::InvalidSchemeException(inner) => {
                    Error::InvalidSchemeException(inner)
                }
                crate::error::CreateLoadBalancerErrorKind::InvalidSecurityGroupException(inner) => {
                    Error::InvalidSecurityGroupException(inner)
                }
                crate::error::CreateLoadBalancerErrorKind::InvalidSubnetException(inner) => {
                    Error::InvalidSubnetException(inner)
                }
                crate::error::CreateLoadBalancerErrorKind::OperationNotPermittedException(
                    inner,
                ) => Error::OperationNotPermittedException(inner),
                crate::error::CreateLoadBalancerErrorKind::SubnetNotFoundException(inner) => {
                    Error::SubnetNotFoundException(inner)
                }
                crate::error::CreateLoadBalancerErrorKind::TooManyAccessPointsException(inner) => {
                    Error::TooManyAccessPointsException(inner)
                }
                crate::error::CreateLoadBalancerErrorKind::TooManyTagsException(inner) => {
                    Error::TooManyTagsException(inner)
                }
                crate::error::CreateLoadBalancerErrorKind::UnsupportedProtocolException(inner) => {
                    Error::UnsupportedProtocolException(inner)
                }
                crate::error::CreateLoadBalancerErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateLoadBalancerListenersError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateLoadBalancerListenersError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateLoadBalancerListenersErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::CreateLoadBalancerListenersErrorKind::CertificateNotFoundException(inner) => Error::CertificateNotFoundException(inner),
                crate::error::CreateLoadBalancerListenersErrorKind::DuplicateListenerException(inner) => Error::DuplicateListenerException(inner),
                crate::error::CreateLoadBalancerListenersErrorKind::InvalidConfigurationRequestException(inner) => Error::InvalidConfigurationRequestException(inner),
                crate::error::CreateLoadBalancerListenersErrorKind::UnsupportedProtocolException(inner) => Error::UnsupportedProtocolException(inner),
                crate::error::CreateLoadBalancerListenersErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateLoadBalancerPolicyError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateLoadBalancerPolicyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateLoadBalancerPolicyErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::CreateLoadBalancerPolicyErrorKind::DuplicatePolicyNameException(inner) => Error::DuplicatePolicyNameException(inner),
                crate::error::CreateLoadBalancerPolicyErrorKind::InvalidConfigurationRequestException(inner) => Error::InvalidConfigurationRequestException(inner),
                crate::error::CreateLoadBalancerPolicyErrorKind::PolicyTypeNotFoundException(inner) => Error::PolicyTypeNotFoundException(inner),
                crate::error::CreateLoadBalancerPolicyErrorKind::TooManyPoliciesException(inner) => Error::TooManyPoliciesException(inner),
                crate::error::CreateLoadBalancerPolicyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteLoadBalancerError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteLoadBalancerError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteLoadBalancerErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteLoadBalancerListenersError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteLoadBalancerListenersError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteLoadBalancerListenersErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::DeleteLoadBalancerListenersErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteLoadBalancerPolicyError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteLoadBalancerPolicyError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteLoadBalancerPolicyErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::DeleteLoadBalancerPolicyErrorKind::InvalidConfigurationRequestException(inner) => Error::InvalidConfigurationRequestException(inner),
                crate::error::DeleteLoadBalancerPolicyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::DeregisterInstancesFromLoadBalancerError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DeregisterInstancesFromLoadBalancerError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeregisterInstancesFromLoadBalancerErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::DeregisterInstancesFromLoadBalancerErrorKind::InvalidEndPointException(inner) => Error::InvalidEndPointException(inner),
                crate::error::DeregisterInstancesFromLoadBalancerErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeAccountLimitsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeAccountLimitsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeAccountLimitsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeInstanceHealthError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeInstanceHealthError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeInstanceHealthErrorKind::AccessPointNotFoundException(
                    inner,
                ) => Error::AccessPointNotFoundException(inner),
                crate::error::DescribeInstanceHealthErrorKind::InvalidEndPointException(inner) => {
                    Error::InvalidEndPointException(inner)
                }
                crate::error::DescribeInstanceHealthErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::DescribeLoadBalancerAttributesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DescribeLoadBalancerAttributesError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeLoadBalancerAttributesErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::DescribeLoadBalancerAttributesErrorKind::LoadBalancerAttributeNotFoundException(inner) => Error::LoadBalancerAttributeNotFoundException(inner),
                crate::error::DescribeLoadBalancerAttributesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeLoadBalancerPoliciesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeLoadBalancerPoliciesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeLoadBalancerPoliciesErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::DescribeLoadBalancerPoliciesErrorKind::PolicyNotFoundException(inner) => Error::PolicyNotFoundException(inner),
                crate::error::DescribeLoadBalancerPoliciesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::DescribeLoadBalancerPolicyTypesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DescribeLoadBalancerPolicyTypesError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeLoadBalancerPolicyTypesErrorKind::PolicyTypeNotFoundException(inner) => Error::PolicyTypeNotFoundException(inner),
                crate::error::DescribeLoadBalancerPolicyTypesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeLoadBalancersError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeLoadBalancersError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeLoadBalancersErrorKind::AccessPointNotFoundException(
                    inner,
                ) => Error::AccessPointNotFoundException(inner),
                crate::error::DescribeLoadBalancersErrorKind::DependencyThrottleException(
                    inner,
                ) => Error::DependencyThrottleException(inner),
                crate::error::DescribeLoadBalancersErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeTagsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeTagsErrorKind::AccessPointNotFoundException(inner) => {
                    Error::AccessPointNotFoundException(inner)
                }
                crate::error::DescribeTagsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DetachLoadBalancerFromSubnetsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DetachLoadBalancerFromSubnetsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DetachLoadBalancerFromSubnetsErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::DetachLoadBalancerFromSubnetsErrorKind::InvalidConfigurationRequestException(inner) => Error::InvalidConfigurationRequestException(inner),
                crate::error::DetachLoadBalancerFromSubnetsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::DisableAvailabilityZonesForLoadBalancerError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::DisableAvailabilityZonesForLoadBalancerError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DisableAvailabilityZonesForLoadBalancerErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::DisableAvailabilityZonesForLoadBalancerErrorKind::InvalidConfigurationRequestException(inner) => Error::InvalidConfigurationRequestException(inner),
                crate::error::DisableAvailabilityZonesForLoadBalancerErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::EnableAvailabilityZonesForLoadBalancerError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::EnableAvailabilityZonesForLoadBalancerError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::EnableAvailabilityZonesForLoadBalancerErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::EnableAvailabilityZonesForLoadBalancerErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ModifyLoadBalancerAttributesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ModifyLoadBalancerAttributesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ModifyLoadBalancerAttributesErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::ModifyLoadBalancerAttributesErrorKind::InvalidConfigurationRequestException(inner) => Error::InvalidConfigurationRequestException(inner),
                crate::error::ModifyLoadBalancerAttributesErrorKind::LoadBalancerAttributeNotFoundException(inner) => Error::LoadBalancerAttributeNotFoundException(inner),
                crate::error::ModifyLoadBalancerAttributesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::RegisterInstancesWithLoadBalancerError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::RegisterInstancesWithLoadBalancerError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::RegisterInstancesWithLoadBalancerErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::RegisterInstancesWithLoadBalancerErrorKind::InvalidEndPointException(inner) => Error::InvalidEndPointException(inner),
                crate::error::RegisterInstancesWithLoadBalancerErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RemoveTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::RemoveTagsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RemoveTagsErrorKind::AccessPointNotFoundException(inner) => {
                    Error::AccessPointNotFoundException(inner)
                }
                crate::error::RemoveTagsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::SetLoadBalancerListenerSSLCertificateError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::SetLoadBalancerListenerSSLCertificateError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::SetLoadBalancerListenerSSLCertificateErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::SetLoadBalancerListenerSSLCertificateErrorKind::CertificateNotFoundException(inner) => Error::CertificateNotFoundException(inner),
                crate::error::SetLoadBalancerListenerSSLCertificateErrorKind::InvalidConfigurationRequestException(inner) => Error::InvalidConfigurationRequestException(inner),
                crate::error::SetLoadBalancerListenerSSLCertificateErrorKind::ListenerNotFoundException(inner) => Error::ListenerNotFoundException(inner),
                crate::error::SetLoadBalancerListenerSSLCertificateErrorKind::UnsupportedProtocolException(inner) => Error::UnsupportedProtocolException(inner),
                crate::error::SetLoadBalancerListenerSSLCertificateErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::error::SetLoadBalancerPoliciesForBackendServerError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::SetLoadBalancerPoliciesForBackendServerError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::SetLoadBalancerPoliciesForBackendServerErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::SetLoadBalancerPoliciesForBackendServerErrorKind::InvalidConfigurationRequestException(inner) => Error::InvalidConfigurationRequestException(inner),
                crate::error::SetLoadBalancerPoliciesForBackendServerErrorKind::PolicyNotFoundException(inner) => Error::PolicyNotFoundException(inner),
                crate::error::SetLoadBalancerPoliciesForBackendServerErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::SetLoadBalancerPoliciesOfListenerError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::SetLoadBalancerPoliciesOfListenerError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::SetLoadBalancerPoliciesOfListenerErrorKind::AccessPointNotFoundException(inner) => Error::AccessPointNotFoundException(inner),
                crate::error::SetLoadBalancerPoliciesOfListenerErrorKind::InvalidConfigurationRequestException(inner) => Error::InvalidConfigurationRequestException(inner),
                crate::error::SetLoadBalancerPoliciesOfListenerErrorKind::ListenerNotFoundException(inner) => Error::ListenerNotFoundException(inner),
                crate::error::SetLoadBalancerPoliciesOfListenerErrorKind::PolicyNotFoundException(inner) => Error::PolicyNotFoundException(inner),
                crate::error::SetLoadBalancerPoliciesOfListenerErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
