// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_retention_configuration::_put_retention_configuration_output::PutRetentionConfigurationOutputBuilder;

pub use crate::operation::put_retention_configuration::_put_retention_configuration_input::PutRetentionConfigurationInputBuilder;

/// Fluent builder constructing a request to `PutRetentionConfiguration`.
///
/// <p>Creates and updates the retention configuration with details about retention period (number of days) that Config stores your historical information. The API creates the <code>RetentionConfiguration</code> object and names the object as <b>default</b>. When you have a <code>RetentionConfiguration</code> object named <b>default</b>, calling the API modifies the default object. </p> <note>
/// <p>Currently, Config supports only one retention configuration per region in your account.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutRetentionConfigurationFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::put_retention_configuration::builders::PutRetentionConfigurationInputBuilder
            }
impl PutRetentionConfigurationFluentBuilder {
    /// Creates a new `PutRetentionConfiguration`.
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
            crate::operation::put_retention_configuration::PutRetentionConfiguration,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_retention_configuration::PutRetentionConfigurationError,
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
        crate::operation::put_retention_configuration::PutRetentionConfigurationOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_retention_configuration::PutRetentionConfigurationError,
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
    /// <p>Number of days Config stores your historical information.</p> <note>
    /// <p>Currently, only applicable to the configuration item history.</p>
    /// </note>
    pub fn retention_period_in_days(mut self, input: i32) -> Self {
        self.inner = self.inner.retention_period_in_days(input);
        self
    }
    /// <p>Number of days Config stores your historical information.</p> <note>
    /// <p>Currently, only applicable to the configuration item history.</p>
    /// </note>
    pub fn set_retention_period_in_days(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_retention_period_in_days(input);
        self
    }
}
