// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_regional_buckets::_list_regional_buckets_output::ListRegionalBucketsOutputBuilder;

pub use crate::operation::list_regional_buckets::_list_regional_buckets_input::ListRegionalBucketsInputBuilder;

/// Fluent builder constructing a request to `ListRegionalBuckets`.
///
/// <p>Returns a list of all Outposts buckets in an Outpost that are owned by the authenticated sender of the request. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
/// <p>For an example of the request syntax for Amazon S3 on Outposts that uses the S3 on Outposts endpoint hostname prefix and <code>x-amz-outpost-id</code> in your request, see the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_ListRegionalBuckets.html#API_control_ListRegionalBuckets_Examples">Examples</a> section.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListRegionalBucketsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_regional_buckets::builders::ListRegionalBucketsInputBuilder,
}
impl ListRegionalBucketsFluentBuilder {
    /// Creates a new `ListRegionalBuckets`.
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
            crate::operation::list_regional_buckets::ListRegionalBuckets,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_regional_buckets::ListRegionalBucketsError,
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
        crate::operation::list_regional_buckets::ListRegionalBucketsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_regional_buckets::ListRegionalBucketsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_regional_buckets::paginator::ListRegionalBucketsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_regional_buckets::paginator::ListRegionalBucketsPaginator {
        crate::operation::list_regional_buckets::paginator::ListRegionalBucketsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The Amazon Web Services account ID of the Outposts bucket.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID of the Outposts bucket.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p></p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p></p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p></p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p></p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The ID of the Outposts resource.</p> <note>
    /// <p>This ID is required by Amazon S3 on Outposts buckets.</p>
    /// </note>
    pub fn outpost_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.outpost_id(input.into());
        self
    }
    /// <p>The ID of the Outposts resource.</p> <note>
    /// <p>This ID is required by Amazon S3 on Outposts buckets.</p>
    /// </note>
    pub fn set_outpost_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_outpost_id(input);
        self
    }
}
