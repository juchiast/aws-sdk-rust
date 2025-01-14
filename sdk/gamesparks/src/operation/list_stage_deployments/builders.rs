// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_stage_deployments::_list_stage_deployments_output::ListStageDeploymentsOutputBuilder;

pub use crate::operation::list_stage_deployments::_list_stage_deployments_input::ListStageDeploymentsInputBuilder;

/// Fluent builder constructing a request to `ListStageDeployments`.
///
/// <p>Gets a paginated list of stage deployment summaries from the game.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListStageDeploymentsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_stage_deployments::builders::ListStageDeploymentsInputBuilder,
}
impl ListStageDeploymentsFluentBuilder {
    /// Creates a new `ListStageDeployments`.
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
            crate::operation::list_stage_deployments::ListStageDeployments,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_stage_deployments::ListStageDeploymentsError,
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
        crate::operation::list_stage_deployments::ListStageDeploymentsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_stage_deployments::ListStageDeploymentsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_stage_deployments::paginator::ListStageDeploymentsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_stage_deployments::paginator::ListStageDeploymentsPaginator {
        crate::operation::list_stage_deployments::paginator::ListStageDeploymentsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The name of the game.</p>
    pub fn game_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.game_name(input.into());
        self
    }
    /// <p>The name of the game.</p>
    pub fn set_game_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_game_name(input);
        self
    }
    /// <p>The name of the stage.</p>
    pub fn stage_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.stage_name(input.into());
        self
    }
    /// <p>The name of the stage.</p>
    pub fn set_stage_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_stage_name(input);
        self
    }
    /// <p>The token that indicates the start of the next sequential page of results.</p>
    /// <p> Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token that indicates the start of the next sequential page of results.</p>
    /// <p> Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value. </p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    /// <p> Use this parameter with NextToken to get results as a set of sequential pages. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    /// <p> Use this parameter with NextToken to get results as a set of sequential pages. </p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
