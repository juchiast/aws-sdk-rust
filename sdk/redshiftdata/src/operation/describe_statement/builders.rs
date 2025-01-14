// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_statement::_describe_statement_output::DescribeStatementOutputBuilder;

pub use crate::operation::describe_statement::_describe_statement_input::DescribeStatementInputBuilder;

/// Fluent builder constructing a request to `DescribeStatement`.
///
/// <p>Describes the details about a specific instance when a query was run by the Amazon Redshift Data API. The information includes when the query started, when it finished, the query status, the number of rows returned, and the SQL statement. </p>
/// <p>For more information about the Amazon Redshift Data API and CLI usage examples, see <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/data-api.html">Using the Amazon Redshift Data API</a> in the <i>Amazon Redshift Management Guide</i>. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeStatementFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_statement::builders::DescribeStatementInputBuilder,
}
impl DescribeStatementFluentBuilder {
    /// Creates a new `DescribeStatement`.
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
            crate::operation::describe_statement::DescribeStatement,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_statement::DescribeStatementError,
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
        crate::operation::describe_statement::DescribeStatementOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_statement::DescribeStatementError,
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
    /// <p>The identifier of the SQL statement to describe. This value is a universally unique identifier (UUID) generated by Amazon Redshift Data API. A suffix indicates the number of the SQL statement. For example, <code>d9b6c0c9-0747-4bf4-b142-e8883122f766:2</code> has a suffix of <code>:2</code> that indicates the second SQL statement of a batch query. This identifier is returned by <code>BatchExecuteStatment</code>, <code>ExecuteStatement</code>, and <code>ListStatements</code>. </p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The identifier of the SQL statement to describe. This value is a universally unique identifier (UUID) generated by Amazon Redshift Data API. A suffix indicates the number of the SQL statement. For example, <code>d9b6c0c9-0747-4bf4-b142-e8883122f766:2</code> has a suffix of <code>:2</code> that indicates the second SQL statement of a batch query. This identifier is returned by <code>BatchExecuteStatment</code>, <code>ExecuteStatement</code>, and <code>ListStatements</code>. </p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
}
