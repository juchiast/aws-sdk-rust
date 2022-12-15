// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) client: aws_smithy_client::Client<
        aws_smithy_client::erase::DynConnector,
        aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
    >,
    pub(crate) conf: crate::Config,
}

/// Client for Amazon Augmented AI Runtime
///
/// Client for invoking operations on Amazon Augmented AI Runtime. Each operation on Amazon Augmented AI Runtime is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_sagemakera2iruntime::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::retry::RetryConfig;
/// # async fn docs() {
/// let shared_config = aws_config::load_from_env().await;
/// let config = aws_sdk_sagemakera2iruntime::config::Builder::from(&shared_config)
///   .retry_config(RetryConfig::disabled())
///   .build();
/// let client = aws_sdk_sagemakera2iruntime::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client {
    handle: std::sync::Arc<Handle>,
}

impl std::clone::Clone for Client {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use aws_smithy_client::Builder;

impl
    From<
        aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    > for Client
{
    fn from(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    ) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl Client {
    /// Creates a client with the given service configuration.
    pub fn with_config(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
        conf: crate::Config,
    ) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    /// Constructs a fluent builder for the [`DeleteHumanLoop`](crate::client::fluent_builders::DeleteHumanLoop) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`human_loop_name(impl Into<String>)`](crate::client::fluent_builders::DeleteHumanLoop::human_loop_name) / [`set_human_loop_name(Option<String>)`](crate::client::fluent_builders::DeleteHumanLoop::set_human_loop_name): <p>The name of the human loop that you want to delete.</p>
    /// - On success, responds with [`DeleteHumanLoopOutput`](crate::output::DeleteHumanLoopOutput)

    /// - On failure, responds with [`SdkError<DeleteHumanLoopError>`](crate::error::DeleteHumanLoopError)
    pub fn delete_human_loop(&self) -> fluent_builders::DeleteHumanLoop {
        fluent_builders::DeleteHumanLoop::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`DescribeHumanLoop`](crate::client::fluent_builders::DescribeHumanLoop) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`human_loop_name(impl Into<String>)`](crate::client::fluent_builders::DescribeHumanLoop::human_loop_name) / [`set_human_loop_name(Option<String>)`](crate::client::fluent_builders::DescribeHumanLoop::set_human_loop_name): <p>The name of the human loop that you want information about.</p>
    /// - On success, responds with [`DescribeHumanLoopOutput`](crate::output::DescribeHumanLoopOutput) with field(s):
    ///   - [`creation_time(Option<DateTime>)`](crate::output::DescribeHumanLoopOutput::creation_time): <p>The creation time when Amazon Augmented AI created the human loop.</p>
    ///   - [`failure_reason(Option<String>)`](crate::output::DescribeHumanLoopOutput::failure_reason): <p>The reason why a human loop failed. The failure reason is returned when the status of the human loop is <code>Failed</code>.</p>
    ///   - [`failure_code(Option<String>)`](crate::output::DescribeHumanLoopOutput::failure_code): <p>A failure code that identifies the type of failure.</p>  <p>Possible values: <code>ValidationError</code>, <code>Expired</code>, <code>InternalError</code> </p>
    ///   - [`human_loop_status(Option<HumanLoopStatus>)`](crate::output::DescribeHumanLoopOutput::human_loop_status): <p>The status of the human loop. </p>
    ///   - [`human_loop_name(Option<String>)`](crate::output::DescribeHumanLoopOutput::human_loop_name): <p>The name of the human loop. The name must be lowercase, unique within the Region in your account, and can have up to 63 characters. Valid characters: a-z, 0-9, and - (hyphen).</p>
    ///   - [`human_loop_arn(Option<String>)`](crate::output::DescribeHumanLoopOutput::human_loop_arn): <p>The Amazon Resource Name (ARN) of the human loop.</p>
    ///   - [`flow_definition_arn(Option<String>)`](crate::output::DescribeHumanLoopOutput::flow_definition_arn): <p>The Amazon Resource Name (ARN) of the flow definition.</p>
    ///   - [`human_loop_output(Option<HumanLoopOutput>)`](crate::output::DescribeHumanLoopOutput::human_loop_output): <p>An object that contains information about the output of the human loop.</p>
    /// - On failure, responds with [`SdkError<DescribeHumanLoopError>`](crate::error::DescribeHumanLoopError)
    pub fn describe_human_loop(&self) -> fluent_builders::DescribeHumanLoop {
        fluent_builders::DescribeHumanLoop::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`ListHumanLoops`](crate::client::fluent_builders::ListHumanLoops) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListHumanLoops::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`creation_time_after(DateTime)`](crate::client::fluent_builders::ListHumanLoops::creation_time_after) / [`set_creation_time_after(Option<DateTime>)`](crate::client::fluent_builders::ListHumanLoops::set_creation_time_after): <p>(Optional) The timestamp of the date when you want the human loops to begin in ISO 8601 format. For example, <code>2020-02-24</code>.</p>
    ///   - [`creation_time_before(DateTime)`](crate::client::fluent_builders::ListHumanLoops::creation_time_before) / [`set_creation_time_before(Option<DateTime>)`](crate::client::fluent_builders::ListHumanLoops::set_creation_time_before): <p>(Optional) The timestamp of the date before which you want the human loops to begin in ISO 8601 format. For example, <code>2020-02-24</code>.</p>
    ///   - [`flow_definition_arn(impl Into<String>)`](crate::client::fluent_builders::ListHumanLoops::flow_definition_arn) / [`set_flow_definition_arn(Option<String>)`](crate::client::fluent_builders::ListHumanLoops::set_flow_definition_arn): <p>The Amazon Resource Name (ARN) of a flow definition.</p>
    ///   - [`sort_order(SortOrder)`](crate::client::fluent_builders::ListHumanLoops::sort_order) / [`set_sort_order(Option<SortOrder>)`](crate::client::fluent_builders::ListHumanLoops::set_sort_order): <p>Optional. The order for displaying results. Valid values: <code>Ascending</code> and <code>Descending</code>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListHumanLoops::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListHumanLoops::set_next_token): <p>A token to display the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListHumanLoops::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListHumanLoops::set_max_results): <p>The total number of items to return. If the total number of available items is more than the value specified in <code>MaxResults</code>, then a <code>NextToken</code> is returned in the output. You can use this token to display the next page of results. </p>
    /// - On success, responds with [`ListHumanLoopsOutput`](crate::output::ListHumanLoopsOutput) with field(s):
    ///   - [`human_loop_summaries(Option<Vec<HumanLoopSummary>>)`](crate::output::ListHumanLoopsOutput::human_loop_summaries): <p>An array of objects that contain information about the human loops.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListHumanLoopsOutput::next_token): <p>A token to display the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListHumanLoopsError>`](crate::error::ListHumanLoopsError)
    pub fn list_human_loops(&self) -> fluent_builders::ListHumanLoops {
        fluent_builders::ListHumanLoops::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`StartHumanLoop`](crate::client::fluent_builders::StartHumanLoop) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`human_loop_name(impl Into<String>)`](crate::client::fluent_builders::StartHumanLoop::human_loop_name) / [`set_human_loop_name(Option<String>)`](crate::client::fluent_builders::StartHumanLoop::set_human_loop_name): <p>The name of the human loop.</p>
    ///   - [`flow_definition_arn(impl Into<String>)`](crate::client::fluent_builders::StartHumanLoop::flow_definition_arn) / [`set_flow_definition_arn(Option<String>)`](crate::client::fluent_builders::StartHumanLoop::set_flow_definition_arn): <p>The Amazon Resource Name (ARN) of the flow definition associated with this human loop.</p>
    ///   - [`human_loop_input(HumanLoopInput)`](crate::client::fluent_builders::StartHumanLoop::human_loop_input) / [`set_human_loop_input(Option<HumanLoopInput>)`](crate::client::fluent_builders::StartHumanLoop::set_human_loop_input): <p>An object that contains information about the human loop.</p>
    ///   - [`data_attributes(HumanLoopDataAttributes)`](crate::client::fluent_builders::StartHumanLoop::data_attributes) / [`set_data_attributes(Option<HumanLoopDataAttributes>)`](crate::client::fluent_builders::StartHumanLoop::set_data_attributes): <p>Attributes of the specified data. Use <code>DataAttributes</code> to specify if your data is free of personally identifiable information and/or free of adult content.</p>
    /// - On success, responds with [`StartHumanLoopOutput`](crate::output::StartHumanLoopOutput) with field(s):
    ///   - [`human_loop_arn(Option<String>)`](crate::output::StartHumanLoopOutput::human_loop_arn): <p>The Amazon Resource Name (ARN) of the human loop.</p>
    /// - On failure, responds with [`SdkError<StartHumanLoopError>`](crate::error::StartHumanLoopError)
    pub fn start_human_loop(&self) -> fluent_builders::StartHumanLoop {
        fluent_builders::StartHumanLoop::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`StopHumanLoop`](crate::client::fluent_builders::StopHumanLoop) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`human_loop_name(impl Into<String>)`](crate::client::fluent_builders::StopHumanLoop::human_loop_name) / [`set_human_loop_name(Option<String>)`](crate::client::fluent_builders::StopHumanLoop::set_human_loop_name): <p>The name of the human loop that you want to stop.</p>
    /// - On success, responds with [`StopHumanLoopOutput`](crate::output::StopHumanLoopOutput)

    /// - On failure, responds with [`SdkError<StopHumanLoopError>`](crate::error::StopHumanLoopError)
    pub fn stop_human_loop(&self) -> fluent_builders::StopHumanLoop {
        fluent_builders::StopHumanLoop::new(self.handle.clone())
    }
}
pub mod fluent_builders {

    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `DeleteHumanLoop`.
    ///
    /// <p>Deletes the specified human loop for a flow definition.</p>
    /// <p>If the human loop was deleted, this operation will return a <code>ResourceNotFoundException</code>. </p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DeleteHumanLoop {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::delete_human_loop_input::Builder,
    }
    impl DeleteHumanLoop {
        /// Creates a new `DeleteHumanLoop`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
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
            crate::operation::customize::CustomizableOperation<
                crate::operation::DeleteHumanLoop,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::DeleteHumanLoopError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            Ok(crate::operation::customize::CustomizableOperation { handle, operation })
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
            crate::output::DeleteHumanLoopOutput,
            aws_smithy_http::result::SdkError<crate::error::DeleteHumanLoopError>,
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
        /// <p>The name of the human loop that you want to delete.</p>
        pub fn human_loop_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.human_loop_name(input.into());
            self
        }
        /// <p>The name of the human loop that you want to delete.</p>
        pub fn set_human_loop_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_human_loop_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DescribeHumanLoop`.
    ///
    /// <p>Returns information about the specified human loop. If the human loop was deleted, this operation will return a <code>ResourceNotFoundException</code> error. </p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DescribeHumanLoop {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::describe_human_loop_input::Builder,
    }
    impl DescribeHumanLoop {
        /// Creates a new `DescribeHumanLoop`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
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
            crate::operation::customize::CustomizableOperation<
                crate::operation::DescribeHumanLoop,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::DescribeHumanLoopError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            Ok(crate::operation::customize::CustomizableOperation { handle, operation })
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
            crate::output::DescribeHumanLoopOutput,
            aws_smithy_http::result::SdkError<crate::error::DescribeHumanLoopError>,
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
        /// <p>The name of the human loop that you want information about.</p>
        pub fn human_loop_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.human_loop_name(input.into());
            self
        }
        /// <p>The name of the human loop that you want information about.</p>
        pub fn set_human_loop_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_human_loop_name(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListHumanLoops`.
    ///
    /// <p>Returns information about human loops, given the specified parameters. If a human loop was deleted, it will not be included.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListHumanLoops {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::list_human_loops_input::Builder,
    }
    impl ListHumanLoops {
        /// Creates a new `ListHumanLoops`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
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
            crate::operation::customize::CustomizableOperation<
                crate::operation::ListHumanLoops,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::ListHumanLoopsError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            Ok(crate::operation::customize::CustomizableOperation { handle, operation })
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
            crate::output::ListHumanLoopsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListHumanLoopsError>,
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
        /// Paginators are used by calling [`send().await`](crate::paginator::ListHumanLoopsPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
        pub fn into_paginator(self) -> crate::paginator::ListHumanLoopsPaginator {
            crate::paginator::ListHumanLoopsPaginator::new(self.handle, self.inner)
        }
        /// <p>(Optional) The timestamp of the date when you want the human loops to begin in ISO 8601 format. For example, <code>2020-02-24</code>.</p>
        pub fn creation_time_after(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.inner = self.inner.creation_time_after(input);
            self
        }
        /// <p>(Optional) The timestamp of the date when you want the human loops to begin in ISO 8601 format. For example, <code>2020-02-24</code>.</p>
        pub fn set_creation_time_after(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.inner = self.inner.set_creation_time_after(input);
            self
        }
        /// <p>(Optional) The timestamp of the date before which you want the human loops to begin in ISO 8601 format. For example, <code>2020-02-24</code>.</p>
        pub fn creation_time_before(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.inner = self.inner.creation_time_before(input);
            self
        }
        /// <p>(Optional) The timestamp of the date before which you want the human loops to begin in ISO 8601 format. For example, <code>2020-02-24</code>.</p>
        pub fn set_creation_time_before(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.inner = self.inner.set_creation_time_before(input);
            self
        }
        /// <p>The Amazon Resource Name (ARN) of a flow definition.</p>
        pub fn flow_definition_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.flow_definition_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of a flow definition.</p>
        pub fn set_flow_definition_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_flow_definition_arn(input);
            self
        }
        /// <p>Optional. The order for displaying results. Valid values: <code>Ascending</code> and <code>Descending</code>.</p>
        pub fn sort_order(mut self, input: crate::model::SortOrder) -> Self {
            self.inner = self.inner.sort_order(input);
            self
        }
        /// <p>Optional. The order for displaying results. Valid values: <code>Ascending</code> and <code>Descending</code>.</p>
        pub fn set_sort_order(
            mut self,
            input: std::option::Option<crate::model::SortOrder>,
        ) -> Self {
            self.inner = self.inner.set_sort_order(input);
            self
        }
        /// <p>A token to display the next page of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>A token to display the next page of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The total number of items to return. If the total number of available items is more than the value specified in <code>MaxResults</code>, then a <code>NextToken</code> is returned in the output. You can use this token to display the next page of results. </p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The total number of items to return. If the total number of available items is more than the value specified in <code>MaxResults</code>, then a <code>NextToken</code> is returned in the output. You can use this token to display the next page of results. </p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    /// Fluent builder constructing a request to `StartHumanLoop`.
    ///
    /// <p>Starts a human loop, provided that at least one activation condition is met.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct StartHumanLoop {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::start_human_loop_input::Builder,
    }
    impl StartHumanLoop {
        /// Creates a new `StartHumanLoop`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
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
            crate::operation::customize::CustomizableOperation<
                crate::operation::StartHumanLoop,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::StartHumanLoopError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            Ok(crate::operation::customize::CustomizableOperation { handle, operation })
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
            crate::output::StartHumanLoopOutput,
            aws_smithy_http::result::SdkError<crate::error::StartHumanLoopError>,
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
        /// <p>The name of the human loop.</p>
        pub fn human_loop_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.human_loop_name(input.into());
            self
        }
        /// <p>The name of the human loop.</p>
        pub fn set_human_loop_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_human_loop_name(input);
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the flow definition associated with this human loop.</p>
        pub fn flow_definition_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.flow_definition_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the flow definition associated with this human loop.</p>
        pub fn set_flow_definition_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_flow_definition_arn(input);
            self
        }
        /// <p>An object that contains information about the human loop.</p>
        pub fn human_loop_input(mut self, input: crate::model::HumanLoopInput) -> Self {
            self.inner = self.inner.human_loop_input(input);
            self
        }
        /// <p>An object that contains information about the human loop.</p>
        pub fn set_human_loop_input(
            mut self,
            input: std::option::Option<crate::model::HumanLoopInput>,
        ) -> Self {
            self.inner = self.inner.set_human_loop_input(input);
            self
        }
        /// <p>Attributes of the specified data. Use <code>DataAttributes</code> to specify if your data is free of personally identifiable information and/or free of adult content.</p>
        pub fn data_attributes(mut self, input: crate::model::HumanLoopDataAttributes) -> Self {
            self.inner = self.inner.data_attributes(input);
            self
        }
        /// <p>Attributes of the specified data. Use <code>DataAttributes</code> to specify if your data is free of personally identifiable information and/or free of adult content.</p>
        pub fn set_data_attributes(
            mut self,
            input: std::option::Option<crate::model::HumanLoopDataAttributes>,
        ) -> Self {
            self.inner = self.inner.set_data_attributes(input);
            self
        }
    }
    /// Fluent builder constructing a request to `StopHumanLoop`.
    ///
    /// <p>Stops the specified human loop.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct StopHumanLoop {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::stop_human_loop_input::Builder,
    }
    impl StopHumanLoop {
        /// Creates a new `StopHumanLoop`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
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
            crate::operation::customize::CustomizableOperation<
                crate::operation::StopHumanLoop,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::StopHumanLoopError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            Ok(crate::operation::customize::CustomizableOperation { handle, operation })
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
            crate::output::StopHumanLoopOutput,
            aws_smithy_http::result::SdkError<crate::error::StopHumanLoopError>,
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
        /// <p>The name of the human loop that you want to stop.</p>
        pub fn human_loop_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.human_loop_name(input.into());
            self
        }
        /// <p>The name of the human loop that you want to stop.</p>
        pub fn set_human_loop_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_human_loop_name(input);
            self
        }
    }
}

impl Client {
    /// Creates a new client from an [SDK Config](aws_types::sdk_config::SdkConfig).
    ///
    /// # Panics
    ///
    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
    ///     the `sleep_impl` on the Config passed into this function to fix it.
    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
    ///     `http_connector` on the Config passed into this function to fix it.
    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    ///
    /// # Panics
    ///
    /// - This method will panic if the `conf` is missing an async sleep implementation. If you experience this panic, set
    ///     the `sleep_impl` on the Config passed into this function to fix it.
    /// - This method will panic if the `conf` is missing an HTTP connector. If you experience this panic, set the
    ///     `http_connector` on the Config passed into this function to fix it.
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf
            .retry_config()
            .cloned()
            .unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
        let timeout_config = conf
            .timeout_config()
            .cloned()
            .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
        let sleep_impl = conf.sleep_impl();
        if (retry_config.has_retry() || timeout_config.has_timeouts()) && sleep_impl.is_none() {
            panic!("An async sleep implementation is required for retries or timeouts to work. \
                                    Set the `sleep_impl` on the Config passed into this function to fix this panic.");
        }

        let connector = conf.http_connector().and_then(|c| {
            let timeout_config = conf
                .timeout_config()
                .cloned()
                .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
            let connector_settings =
                aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                    &timeout_config,
                );
            c.connector(&connector_settings, conf.sleep_impl())
        });

        let builder = aws_smithy_client::Builder::new();

        let builder = match connector {
            // Use provided connector
            Some(c) => builder.connector(c),
            None => {
                #[cfg(any(feature = "rustls", feature = "native-tls"))]
                {
                    // Use default connector based on enabled features
                    builder.dyn_https_connector(
                        aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                            &timeout_config,
                        ),
                    )
                }
                #[cfg(not(any(feature = "rustls", feature = "native-tls")))]
                {
                    panic!("No HTTP connector was available. Enable the `rustls` or `native-tls` crate feature or set a connector to fix this.");
                }
            }
        };
        let mut builder = builder
            .middleware(aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ))
            .retry_config(retry_config.into())
            .operation_timeout_config(timeout_config.into());
        builder.set_sleep_impl(sleep_impl);
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
