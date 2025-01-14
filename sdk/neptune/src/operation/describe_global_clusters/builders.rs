// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_global_clusters::_describe_global_clusters_output::DescribeGlobalClustersOutputBuilder;

pub use crate::operation::describe_global_clusters::_describe_global_clusters_input::DescribeGlobalClustersInputBuilder;

/// Fluent builder constructing a request to `DescribeGlobalClusters`.
///
/// <p>Returns information about Neptune global database clusters. This API supports pagination.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeGlobalClustersFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_global_clusters::builders::DescribeGlobalClustersInputBuilder,
}
impl DescribeGlobalClustersFluentBuilder {
    /// Creates a new `DescribeGlobalClusters`.
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
            crate::operation::describe_global_clusters::DescribeGlobalClusters,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_global_clusters::DescribeGlobalClustersError,
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
        crate::operation::describe_global_clusters::DescribeGlobalClustersOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_global_clusters::DescribeGlobalClustersError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_global_clusters::paginator::DescribeGlobalClustersPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_global_clusters::paginator::DescribeGlobalClustersPaginator
    {
        crate::operation::describe_global_clusters::paginator::DescribeGlobalClustersPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The user-supplied DB cluster identifier. If this parameter is specified, only information about the specified DB cluster is returned. This parameter is not case-sensitive.</p>
    /// <p>Constraints: If supplied, must match an existing DB cluster identifier.</p>
    pub fn global_cluster_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.global_cluster_identifier(input.into());
        self
    }
    /// <p>The user-supplied DB cluster identifier. If this parameter is specified, only information about the specified DB cluster is returned. This parameter is not case-sensitive.</p>
    /// <p>Constraints: If supplied, must match an existing DB cluster identifier.</p>
    pub fn set_global_cluster_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_global_cluster_identifier(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination marker token is included in the response that you can use to retrieve the remaining results.</p>
    /// <p>Default: <code>100</code> </p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination marker token is included in the response that you can use to retrieve the remaining results.</p>
    /// <p>Default: <code>100</code> </p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn set_max_records(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p>(<i>Optional</i>) A pagination token returned by a previous call to <code>DescribeGlobalClusters</code>. If this parameter is specified, the response will only include records beyond the marker, up to the number specified by <code>MaxRecords</code>.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>(<i>Optional</i>) A pagination token returned by a previous call to <code>DescribeGlobalClusters</code>. If this parameter is specified, the response will only include records beyond the marker, up to the number specified by <code>MaxRecords</code>.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
}
