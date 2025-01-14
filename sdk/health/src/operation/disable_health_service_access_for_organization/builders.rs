// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_health_service_access_for_organization::_disable_health_service_access_for_organization_output::DisableHealthServiceAccessForOrganizationOutputBuilder;

pub use crate::operation::disable_health_service_access_for_organization::_disable_health_service_access_for_organization_input::DisableHealthServiceAccessForOrganizationInputBuilder;

/// Fluent builder constructing a request to `DisableHealthServiceAccessForOrganization`.
///
/// <p>Disables Health from working with Organizations. To call this operation, you must sign in as an Identity and Access Management (IAM) user, assume an IAM role, or sign in as the root user (not recommended) in the organization's management account. For more information, see <a href="https://docs.aws.amazon.com/health/latest/ug/aggregate-events.html">Aggregating Health events</a> in the <i>Health User Guide</i>.</p>
/// <p>This operation doesn't remove the service-linked role from the management account in your organization. You must use the IAM console, API, or Command Line Interface (CLI) to remove the service-linked role. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/using-service-linked-roles.html#delete-service-linked-role">Deleting a Service-Linked Role</a> in the <i>IAM User Guide</i>.</p> <note>
/// <p>You can also disable the organizational feature by using the Organizations <a href="https://docs.aws.amazon.com/organizations/latest/APIReference/API_DisableAWSServiceAccess.html">DisableAWSServiceAccess</a> API operation. After you call this operation, Health stops aggregating events for all other Amazon Web Services accounts in your organization. If you call the Health API operations for organizational view, Health returns an error. Health continues to aggregate health events for your Amazon Web Services account.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DisableHealthServiceAccessForOrganizationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::disable_health_service_access_for_organization::builders::DisableHealthServiceAccessForOrganizationInputBuilder
            }
impl DisableHealthServiceAccessForOrganizationFluentBuilder {
    /// Creates a new `DisableHealthServiceAccessForOrganization`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::disable_health_service_access_for_organization::DisableHealthServiceAccessForOrganization, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::disable_health_service_access_for_organization::DisableHealthServiceAccessForOrganizationError>
    >{
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
                    pub async fn send(self) -> std::result::Result<crate::operation::disable_health_service_access_for_organization::DisableHealthServiceAccessForOrganizationOutput, aws_smithy_http::result::SdkError<crate::operation::disable_health_service_access_for_organization::DisableHealthServiceAccessForOrganizationError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
}
