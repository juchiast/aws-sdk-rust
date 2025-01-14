// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_disable_standards::_batch_disable_standards_output::BatchDisableStandardsOutputBuilder;

pub use crate::operation::batch_disable_standards::_batch_disable_standards_input::BatchDisableStandardsInputBuilder;

/// Fluent builder constructing a request to `BatchDisableStandards`.
///
/// <p>Disables the standards specified by the provided <code>StandardsSubscriptionArns</code>.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-standards.html">Security Standards</a> section of the <i>Security Hub User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct BatchDisableStandardsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_disable_standards::builders::BatchDisableStandardsInputBuilder,
}
impl BatchDisableStandardsFluentBuilder {
    /// Creates a new `BatchDisableStandards`.
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
            crate::operation::batch_disable_standards::BatchDisableStandards,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_disable_standards::BatchDisableStandardsError,
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
        crate::operation::batch_disable_standards::BatchDisableStandardsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_disable_standards::BatchDisableStandardsError,
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
    /// Appends an item to `StandardsSubscriptionArns`.
    ///
    /// To override the contents of this collection use [`set_standards_subscription_arns`](Self::set_standards_subscription_arns).
    ///
    /// <p>The ARNs of the standards subscriptions to disable.</p>
    pub fn standards_subscription_arns(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.standards_subscription_arns(input.into());
        self
    }
    /// <p>The ARNs of the standards subscriptions to disable.</p>
    pub fn set_standards_subscription_arns(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_standards_subscription_arns(input);
        self
    }
}
