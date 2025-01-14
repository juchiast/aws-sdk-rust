// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_fleet_advisor_collector::_create_fleet_advisor_collector_output::CreateFleetAdvisorCollectorOutputBuilder;

pub use crate::operation::create_fleet_advisor_collector::_create_fleet_advisor_collector_input::CreateFleetAdvisorCollectorInputBuilder;

/// Fluent builder constructing a request to `CreateFleetAdvisorCollector`.
///
/// <p>Creates a Fleet Advisor collector using the specified parameters.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateFleetAdvisorCollectorFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_fleet_advisor_collector::builders::CreateFleetAdvisorCollectorInputBuilder
            }
impl CreateFleetAdvisorCollectorFluentBuilder {
    /// Creates a new `CreateFleetAdvisorCollector`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_fleet_advisor_collector::CreateFleetAdvisorCollector,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_fleet_advisor_collector::CreateFleetAdvisorCollectorError,
        >,
    > {
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
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::create_fleet_advisor_collector::CreateFleetAdvisorCollectorOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_fleet_advisor_collector::CreateFleetAdvisorCollectorError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The name of your Fleet Advisor collector (for example, <code>sample-collector</code>).</p>
    pub fn collector_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.collector_name(input.into());
        self
    }
    /// <p>The name of your Fleet Advisor collector (for example, <code>sample-collector</code>).</p>
    pub fn set_collector_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_collector_name(input);
        self
    }
    /// <p>A summary description of your Fleet Advisor collector.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A summary description of your Fleet Advisor collector.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The IAM role that grants permissions to access the specified Amazon S3 bucket.</p>
    pub fn service_access_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.service_access_role_arn(input.into());
        self
    }
    /// <p>The IAM role that grants permissions to access the specified Amazon S3 bucket.</p>
    pub fn set_service_access_role_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_service_access_role_arn(input);
        self
    }
    /// <p>The Amazon S3 bucket that the Fleet Advisor collector uses to store inventory metadata.</p>
    pub fn s3_bucket_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.s3_bucket_name(input.into());
        self
    }
    /// <p>The Amazon S3 bucket that the Fleet Advisor collector uses to store inventory metadata.</p>
    pub fn set_s3_bucket_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_s3_bucket_name(input);
        self
    }
}
