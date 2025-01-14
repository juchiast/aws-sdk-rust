// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_job_run::_start_job_run_output::StartJobRunOutputBuilder;

pub use crate::operation::start_job_run::_start_job_run_input::StartJobRunInputBuilder;

/// Fluent builder constructing a request to `StartJobRun`.
///
/// <p>Starts a job run.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct StartJobRunFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_job_run::builders::StartJobRunInputBuilder,
}
impl StartJobRunFluentBuilder {
    /// Creates a new `StartJobRun`.
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
            crate::operation::start_job_run::StartJobRun,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::start_job_run::StartJobRunError>,
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
        crate::operation::start_job_run::StartJobRunOutput,
        aws_smithy_http::result::SdkError<crate::operation::start_job_run::StartJobRunError>,
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
    /// <p>The ID of the application on which to run the job.</p>
    pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The ID of the application on which to run the job.</p>
    pub fn set_application_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The client idempotency token of the job run to start. Its value must be unique for each request.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The client idempotency token of the job run to start. Its value must be unique for each request.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The execution role ARN for the job run.</p>
    pub fn execution_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.execution_role_arn(input.into());
        self
    }
    /// <p>The execution role ARN for the job run.</p>
    pub fn set_execution_role_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_execution_role_arn(input);
        self
    }
    /// <p>The job driver for the job run.</p>
    pub fn job_driver(mut self, input: crate::types::JobDriver) -> Self {
        self.inner = self.inner.job_driver(input);
        self
    }
    /// <p>The job driver for the job run.</p>
    pub fn set_job_driver(mut self, input: std::option::Option<crate::types::JobDriver>) -> Self {
        self.inner = self.inner.set_job_driver(input);
        self
    }
    /// <p>The configuration overrides for the job run.</p>
    pub fn configuration_overrides(mut self, input: crate::types::ConfigurationOverrides) -> Self {
        self.inner = self.inner.configuration_overrides(input);
        self
    }
    /// <p>The configuration overrides for the job run.</p>
    pub fn set_configuration_overrides(
        mut self,
        input: std::option::Option<crate::types::ConfigurationOverrides>,
    ) -> Self {
        self.inner = self.inner.set_configuration_overrides(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags assigned to the job run.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags assigned to the job run.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The maximum duration for the job run to run. If the job run runs beyond this duration, it will be automatically cancelled.</p>
    pub fn execution_timeout_minutes(mut self, input: i64) -> Self {
        self.inner = self.inner.execution_timeout_minutes(input);
        self
    }
    /// <p>The maximum duration for the job run to run. If the job run runs beyond this duration, it will be automatically cancelled.</p>
    pub fn set_execution_timeout_minutes(mut self, input: std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_execution_timeout_minutes(input);
        self
    }
    /// <p>The optional job run name. This doesn't have to be unique.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The optional job run name. This doesn't have to be unique.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
}
