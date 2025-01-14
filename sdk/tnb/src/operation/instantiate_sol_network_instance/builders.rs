// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::instantiate_sol_network_instance::_instantiate_sol_network_instance_output::InstantiateSolNetworkInstanceOutputBuilder;

pub use crate::operation::instantiate_sol_network_instance::_instantiate_sol_network_instance_input::InstantiateSolNetworkInstanceInputBuilder;

/// Fluent builder constructing a request to `InstantiateSolNetworkInstance`.
///
/// <p>Instantiates a network instance.</p>
/// <p>A network instance is a single network created in Amazon Web Services TNB that can be deployed and on which life-cycle operations (like terminate, update, and delete) can be performed.</p>
/// <p>Before you can instantiate a network instance, you have to create a network instance. For more information, see <a href="https://docs.aws.amazon.com/tnb/latest/APIReference/API_CreateSolNetworkInstance.html">CreateSolNetworkInstance</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct InstantiateSolNetworkInstanceFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::instantiate_sol_network_instance::builders::InstantiateSolNetworkInstanceInputBuilder
            }
impl InstantiateSolNetworkInstanceFluentBuilder {
    /// Creates a new `InstantiateSolNetworkInstance`.
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
            crate::operation::instantiate_sol_network_instance::InstantiateSolNetworkInstance,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::instantiate_sol_network_instance::InstantiateSolNetworkInstanceError,
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
        crate::operation::instantiate_sol_network_instance::InstantiateSolNetworkInstanceOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::instantiate_sol_network_instance::InstantiateSolNetworkInstanceError,
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
    /// <p>ID of the network instance.</p>
    pub fn ns_instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.ns_instance_id(input.into());
        self
    }
    /// <p>ID of the network instance.</p>
    pub fn set_ns_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_ns_instance_id(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Provides values for the configurable properties.</p>
    pub fn additional_params_for_ns(mut self, input: aws_smithy_types::Document) -> Self {
        self.inner = self.inner.additional_params_for_ns(input);
        self
    }
    /// <p>Provides values for the configurable properties.</p>
    pub fn set_additional_params_for_ns(
        mut self,
        input: std::option::Option<aws_smithy_types::Document>,
    ) -> Self {
        self.inner = self.inner.set_additional_params_for_ns(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. When you use this API, the tags are transferred to the network operation that is created. Use tags to search and filter your resources or track your Amazon Web Services costs.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. When you use this API, the tags are transferred to the network operation that is created. Use tags to search and filter your resources or track your Amazon Web Services costs.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
