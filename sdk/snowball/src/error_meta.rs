// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Job creation failed. Currently, clusters support five nodes. If you have fewer than five nodes for your cluster and you have more nodes to create for this cluster, try again and create jobs until your cluster has exactly five nodes.</p>
    ClusterLimitExceededException(crate::error::ClusterLimitExceededException),
    /// <p>You get this exception when you call <code>CreateReturnShippingLabel</code> more than once when other requests are not completed.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>Your IAM user lacks the necessary Amazon EC2 permissions to perform the attempted action.</p>
    Ec2RequestFailedException(crate::error::Ec2RequestFailedException),
    /// <p>The address provided was invalid. Check the address with your region's carrier, and try again.</p>
    InvalidAddressException(crate::error::InvalidAddressException),
    /// <p>Job or cluster creation failed. One or more inputs were invalid. Confirm that the <code>CreateClusterRequest$SnowballType</code> value supports your <code>CreateJobRequest$JobType</code>, and try again.</p>
    InvalidInputCombinationException(crate::error::InvalidInputCombinationException),
    /// <p>The action can't be performed because the job's current state doesn't allow that action to be performed.</p>
    InvalidJobStateException(crate::error::InvalidJobStateException),
    /// <p>The <code>NextToken</code> string was altered unexpectedly, and the operation has stopped. Run the operation without changing the <code>NextToken</code> string, and try again.</p>
    InvalidNextTokenException(crate::error::InvalidNextTokenException),
    /// <p>The specified resource can't be found. Check the information you provided in your last request, and try again.</p>
    InvalidResourceException(crate::error::InvalidResourceException),
    /// <p>The provided Key Management Service key lacks the permissions to perform the specified <code>CreateJob</code> or <code>UpdateJob</code> action.</p>
    KmsRequestFailedException(crate::error::KmsRequestFailedException),
    /// <p>You get this exception if you call <code>CreateReturnShippingLabel</code> and a valid return shipping label already exists. In this case, use <code>DescribeReturnShippingLabel</code> to get the URL.</p>
    ReturnShippingLabelAlreadyExistsException(
        crate::error::ReturnShippingLabelAlreadyExistsException,
    ),
    /// <p>The address is either outside the serviceable area for your region, or an error occurred. Check the address with your region's carrier and try again. If the issue persists, contact Amazon Web Services Support.</p>
    UnsupportedAddressException(crate::error::UnsupportedAddressException),
    /// An unhandled error occurred.
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ClusterLimitExceededException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::Ec2RequestFailedException(inner) => inner.fmt(f),
            Error::InvalidAddressException(inner) => inner.fmt(f),
            Error::InvalidInputCombinationException(inner) => inner.fmt(f),
            Error::InvalidJobStateException(inner) => inner.fmt(f),
            Error::InvalidNextTokenException(inner) => inner.fmt(f),
            Error::InvalidResourceException(inner) => inner.fmt(f),
            Error::KmsRequestFailedException(inner) => inner.fmt(f),
            Error::ReturnShippingLabelAlreadyExistsException(inner) => inner.fmt(f),
            Error::UnsupportedAddressException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CancelClusterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CancelClusterError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CancelClusterErrorKind::InvalidJobStateException(inner) => {
                    Error::InvalidJobStateException(inner)
                }
                crate::error::CancelClusterErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::CancelClusterErrorKind::KmsRequestFailedException(inner) => {
                    Error::KmsRequestFailedException(inner)
                }
                crate::error::CancelClusterErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CancelJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CancelJobError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CancelJobErrorKind::InvalidJobStateException(inner) => {
                    Error::InvalidJobStateException(inner)
                }
                crate::error::CancelJobErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::CancelJobErrorKind::KmsRequestFailedException(inner) => {
                    Error::KmsRequestFailedException(inner)
                }
                crate::error::CancelJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateAddressError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateAddressError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateAddressErrorKind::InvalidAddressException(inner) => {
                    Error::InvalidAddressException(inner)
                }
                crate::error::CreateAddressErrorKind::UnsupportedAddressException(inner) => {
                    Error::UnsupportedAddressException(inner)
                }
                crate::error::CreateAddressErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateClusterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateClusterError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateClusterErrorKind::Ec2RequestFailedException(inner) => {
                    Error::Ec2RequestFailedException(inner)
                }
                crate::error::CreateClusterErrorKind::InvalidInputCombinationException(inner) => {
                    Error::InvalidInputCombinationException(inner)
                }
                crate::error::CreateClusterErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::CreateClusterErrorKind::KmsRequestFailedException(inner) => {
                    Error::KmsRequestFailedException(inner)
                }
                crate::error::CreateClusterErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateJobError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateJobErrorKind::ClusterLimitExceededException(inner) => {
                    Error::ClusterLimitExceededException(inner)
                }
                crate::error::CreateJobErrorKind::Ec2RequestFailedException(inner) => {
                    Error::Ec2RequestFailedException(inner)
                }
                crate::error::CreateJobErrorKind::InvalidInputCombinationException(inner) => {
                    Error::InvalidInputCombinationException(inner)
                }
                crate::error::CreateJobErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::CreateJobErrorKind::KmsRequestFailedException(inner) => {
                    Error::KmsRequestFailedException(inner)
                }
                crate::error::CreateJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateLongTermPricingError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateLongTermPricingError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateLongTermPricingErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::CreateLongTermPricingErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateReturnShippingLabelError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateReturnShippingLabelError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateReturnShippingLabelErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::CreateReturnShippingLabelErrorKind::InvalidInputCombinationException(inner) => Error::InvalidInputCombinationException(inner),
                crate::error::CreateReturnShippingLabelErrorKind::InvalidJobStateException(inner) => Error::InvalidJobStateException(inner),
                crate::error::CreateReturnShippingLabelErrorKind::InvalidResourceException(inner) => Error::InvalidResourceException(inner),
                crate::error::CreateReturnShippingLabelErrorKind::ReturnShippingLabelAlreadyExistsException(inner) => Error::ReturnShippingLabelAlreadyExistsException(inner),
                crate::error::CreateReturnShippingLabelErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeAddressError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeAddressError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeAddressErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::DescribeAddressErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeAddressesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeAddressesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeAddressesErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::DescribeAddressesErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::DescribeAddressesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeClusterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeClusterError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeClusterErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::DescribeClusterErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeJobError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeJobErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::DescribeJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeReturnShippingLabelError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeReturnShippingLabelError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeReturnShippingLabelErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::DescribeReturnShippingLabelErrorKind::InvalidJobStateException(
                    inner,
                ) => Error::InvalidJobStateException(inner),
                crate::error::DescribeReturnShippingLabelErrorKind::InvalidResourceException(
                    inner,
                ) => Error::InvalidResourceException(inner),
                crate::error::DescribeReturnShippingLabelErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetJobManifestError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetJobManifestError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetJobManifestErrorKind::InvalidJobStateException(inner) => {
                    Error::InvalidJobStateException(inner)
                }
                crate::error::GetJobManifestErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::GetJobManifestErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetJobUnlockCodeError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetJobUnlockCodeError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetJobUnlockCodeErrorKind::InvalidJobStateException(inner) => {
                    Error::InvalidJobStateException(inner)
                }
                crate::error::GetJobUnlockCodeErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::GetJobUnlockCodeErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSnowballUsageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetSnowballUsageError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetSnowballUsageErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSoftwareUpdatesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetSoftwareUpdatesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetSoftwareUpdatesErrorKind::InvalidJobStateException(inner) => {
                    Error::InvalidJobStateException(inner)
                }
                crate::error::GetSoftwareUpdatesErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::GetSoftwareUpdatesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListClusterJobsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListClusterJobsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListClusterJobsErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::ListClusterJobsErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::ListClusterJobsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListClustersError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListClustersError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListClustersErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::ListClustersErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListCompatibleImagesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListCompatibleImagesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListCompatibleImagesErrorKind::Ec2RequestFailedException(inner) => {
                    Error::Ec2RequestFailedException(inner)
                }
                crate::error::ListCompatibleImagesErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::ListCompatibleImagesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListJobsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListJobsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListJobsErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::ListJobsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListLongTermPricingError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListLongTermPricingError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListLongTermPricingErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::ListLongTermPricingErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::ListLongTermPricingErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateClusterError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateClusterError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateClusterErrorKind::Ec2RequestFailedException(inner) => {
                    Error::Ec2RequestFailedException(inner)
                }
                crate::error::UpdateClusterErrorKind::InvalidInputCombinationException(inner) => {
                    Error::InvalidInputCombinationException(inner)
                }
                crate::error::UpdateClusterErrorKind::InvalidJobStateException(inner) => {
                    Error::InvalidJobStateException(inner)
                }
                crate::error::UpdateClusterErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::UpdateClusterErrorKind::KmsRequestFailedException(inner) => {
                    Error::KmsRequestFailedException(inner)
                }
                crate::error::UpdateClusterErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateJobError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateJobError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateJobErrorKind::ClusterLimitExceededException(inner) => {
                    Error::ClusterLimitExceededException(inner)
                }
                crate::error::UpdateJobErrorKind::Ec2RequestFailedException(inner) => {
                    Error::Ec2RequestFailedException(inner)
                }
                crate::error::UpdateJobErrorKind::InvalidInputCombinationException(inner) => {
                    Error::InvalidInputCombinationException(inner)
                }
                crate::error::UpdateJobErrorKind::InvalidJobStateException(inner) => {
                    Error::InvalidJobStateException(inner)
                }
                crate::error::UpdateJobErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::UpdateJobErrorKind::KmsRequestFailedException(inner) => {
                    Error::KmsRequestFailedException(inner)
                }
                crate::error::UpdateJobErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateJobShipmentStateError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateJobShipmentStateError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateJobShipmentStateErrorKind::InvalidJobStateException(inner) => {
                    Error::InvalidJobStateException(inner)
                }
                crate::error::UpdateJobShipmentStateErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::UpdateJobShipmentStateErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateLongTermPricingError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateLongTermPricingError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateLongTermPricingErrorKind::InvalidResourceException(inner) => {
                    Error::InvalidResourceException(inner)
                }
                crate::error::UpdateLongTermPricingErrorKind::Unhandled(inner) => {
                    Error::Unhandled(crate::error::Unhandled::new(inner.into()))
                }
            },
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl std::error::Error for Error {}
