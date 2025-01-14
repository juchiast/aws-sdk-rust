// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_connector_entities::_list_connector_entities_output::ListConnectorEntitiesOutputBuilder;

pub use crate::operation::list_connector_entities::_list_connector_entities_input::ListConnectorEntitiesInputBuilder;

/// Fluent builder constructing a request to `ListConnectorEntities`.
///
/// <p> Returns the list of available connector entities supported by Amazon AppFlow. For example, you can query Salesforce for <i>Account</i> and <i>Opportunity</i> entities, or query ServiceNow for the <i>Incident</i> entity. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListConnectorEntitiesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_connector_entities::builders::ListConnectorEntitiesInputBuilder,
}
impl ListConnectorEntitiesFluentBuilder {
    /// Creates a new `ListConnectorEntities`.
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
            crate::operation::list_connector_entities::ListConnectorEntities,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_connector_entities::ListConnectorEntitiesError,
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
        crate::operation::list_connector_entities::ListConnectorEntitiesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_connector_entities::ListConnectorEntitiesError,
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
    /// <p> The name of the connector profile. The name is unique for each <code>ConnectorProfile</code> in the Amazon Web Services account, and is used to query the downstream connector. </p>
    pub fn connector_profile_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.connector_profile_name(input.into());
        self
    }
    /// <p> The name of the connector profile. The name is unique for each <code>ConnectorProfile</code> in the Amazon Web Services account, and is used to query the downstream connector. </p>
    pub fn set_connector_profile_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_connector_profile_name(input);
        self
    }
    /// <p> The type of connector, such as Salesforce, Amplitude, and so on. </p>
    pub fn connector_type(mut self, input: crate::types::ConnectorType) -> Self {
        self.inner = self.inner.connector_type(input);
        self
    }
    /// <p> The type of connector, such as Salesforce, Amplitude, and so on. </p>
    pub fn set_connector_type(
        mut self,
        input: std::option::Option<crate::types::ConnectorType>,
    ) -> Self {
        self.inner = self.inner.set_connector_type(input);
        self
    }
    /// <p> This optional parameter is specific to connector implementation. Some connectors support multiple levels or categories of entities. You can find out the list of roots for such providers by sending a request without the <code>entitiesPath</code> parameter. If the connector supports entities at different roots, this initial request returns the list of roots. Otherwise, this request returns all entities supported by the provider. </p>
    pub fn entities_path(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.entities_path(input.into());
        self
    }
    /// <p> This optional parameter is specific to connector implementation. Some connectors support multiple levels or categories of entities. You can find out the list of roots for such providers by sending a request without the <code>entitiesPath</code> parameter. If the connector supports entities at different roots, this initial request returns the list of roots. Otherwise, this request returns all entities supported by the provider. </p>
    pub fn set_entities_path(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_entities_path(input);
        self
    }
    /// <p>The version of the API that's used by the connector.</p>
    pub fn api_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.api_version(input.into());
        self
    }
    /// <p>The version of the API that's used by the connector.</p>
    pub fn set_api_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_api_version(input);
        self
    }
    /// <p>The maximum number of items that the operation returns in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items that the operation returns in the response.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>A token that was provided by your prior <code>ListConnectorEntities</code> operation if the response was too big for the page size. You specify this token to get the next page of results in paginated response.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token that was provided by your prior <code>ListConnectorEntities</code> operation if the response was too big for the page size. You specify this token to get the next page of results in paginated response.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
