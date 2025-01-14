// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_inference_events::_list_inference_events_output::ListInferenceEventsOutputBuilder;

pub use crate::operation::list_inference_events::_list_inference_events_input::ListInferenceEventsInputBuilder;

/// Fluent builder constructing a request to `ListInferenceEvents`.
///
/// <p> Lists all inference events that have been found for the specified inference scheduler. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListInferenceEventsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_inference_events::builders::ListInferenceEventsInputBuilder,
}
impl ListInferenceEventsFluentBuilder {
    /// Creates a new `ListInferenceEvents`.
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
            crate::operation::list_inference_events::ListInferenceEvents,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_inference_events::ListInferenceEventsError,
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
        crate::operation::list_inference_events::ListInferenceEventsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_inference_events::ListInferenceEventsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_inference_events::paginator::ListInferenceEventsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_inference_events::paginator::ListInferenceEventsPaginator {
        crate::operation::list_inference_events::paginator::ListInferenceEventsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>An opaque pagination token indicating where to continue the listing of inference events.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An opaque pagination token indicating where to continue the listing of inference events.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Specifies the maximum number of inference events to list. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Specifies the maximum number of inference events to list. </p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The name of the inference scheduler for the inference events listed. </p>
    pub fn inference_scheduler_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.inference_scheduler_name(input.into());
        self
    }
    /// <p>The name of the inference scheduler for the inference events listed. </p>
    pub fn set_inference_scheduler_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_inference_scheduler_name(input);
        self
    }
    /// <p> Lookout for Equipment will return all the inference events with an end time equal to or greater than the start time given.</p>
    pub fn interval_start_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.interval_start_time(input);
        self
    }
    /// <p> Lookout for Equipment will return all the inference events with an end time equal to or greater than the start time given.</p>
    pub fn set_interval_start_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_interval_start_time(input);
        self
    }
    /// <p>Returns all the inference events with an end start time equal to or greater than less than the end time given</p>
    pub fn interval_end_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.interval_end_time(input);
        self
    }
    /// <p>Returns all the inference events with an end start time equal to or greater than less than the end time given</p>
    pub fn set_interval_end_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_interval_end_time(input);
        self
    }
}
