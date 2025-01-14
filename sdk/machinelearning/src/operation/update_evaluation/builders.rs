// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_evaluation::_update_evaluation_output::UpdateEvaluationOutputBuilder;

pub use crate::operation::update_evaluation::_update_evaluation_input::UpdateEvaluationInputBuilder;

/// Fluent builder constructing a request to `UpdateEvaluation`.
///
/// <p>Updates the <code>EvaluationName</code> of an <code>Evaluation</code>.</p>
/// <p>You can use the <code>GetEvaluation</code> operation to view the contents of the updated data element.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateEvaluationFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_evaluation::builders::UpdateEvaluationInputBuilder,
}
impl UpdateEvaluationFluentBuilder {
    /// Creates a new `UpdateEvaluation`.
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
            crate::operation::update_evaluation::UpdateEvaluation,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_evaluation::UpdateEvaluationError,
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
        crate::operation::update_evaluation::UpdateEvaluationOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_evaluation::UpdateEvaluationError,
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
    /// <p>The ID assigned to the <code>Evaluation</code> during creation.</p>
    pub fn evaluation_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.evaluation_id(input.into());
        self
    }
    /// <p>The ID assigned to the <code>Evaluation</code> during creation.</p>
    pub fn set_evaluation_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_evaluation_id(input);
        self
    }
    /// <p>A new user-supplied name or description of the <code>Evaluation</code> that will replace the current content. </p>
    pub fn evaluation_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.evaluation_name(input.into());
        self
    }
    /// <p>A new user-supplied name or description of the <code>Evaluation</code> that will replace the current content. </p>
    pub fn set_evaluation_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_evaluation_name(input);
        self
    }
}
