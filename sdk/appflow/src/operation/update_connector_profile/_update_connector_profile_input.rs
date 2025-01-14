// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateConnectorProfileInput {
    /// <p> The name of the connector profile and is unique for each <code>ConnectorProfile</code> in the Amazon Web Services account. </p>
    #[doc(hidden)]
    pub connector_profile_name: std::option::Option<std::string::String>,
    /// <p> Indicates the connection mode and if it is public or private. </p>
    #[doc(hidden)]
    pub connection_mode: std::option::Option<crate::types::ConnectionMode>,
    /// <p> Defines the connector-specific profile configuration and credentials. </p>
    #[doc(hidden)]
    pub connector_profile_config: std::option::Option<crate::types::ConnectorProfileConfig>,
}
impl UpdateConnectorProfileInput {
    /// <p> The name of the connector profile and is unique for each <code>ConnectorProfile</code> in the Amazon Web Services account. </p>
    pub fn connector_profile_name(&self) -> std::option::Option<&str> {
        self.connector_profile_name.as_deref()
    }
    /// <p> Indicates the connection mode and if it is public or private. </p>
    pub fn connection_mode(&self) -> std::option::Option<&crate::types::ConnectionMode> {
        self.connection_mode.as_ref()
    }
    /// <p> Defines the connector-specific profile configuration and credentials. </p>
    pub fn connector_profile_config(
        &self,
    ) -> std::option::Option<&crate::types::ConnectorProfileConfig> {
        self.connector_profile_config.as_ref()
    }
}
impl UpdateConnectorProfileInput {
    /// Creates a new builder-style object to manufacture [`UpdateConnectorProfileInput`](crate::operation::update_connector_profile::UpdateConnectorProfileInput).
    pub fn builder(
    ) -> crate::operation::update_connector_profile::builders::UpdateConnectorProfileInputBuilder
    {
        crate::operation::update_connector_profile::builders::UpdateConnectorProfileInputBuilder::default()
    }
}

/// A builder for [`UpdateConnectorProfileInput`](crate::operation::update_connector_profile::UpdateConnectorProfileInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct UpdateConnectorProfileInputBuilder {
    pub(crate) connector_profile_name: std::option::Option<std::string::String>,
    pub(crate) connection_mode: std::option::Option<crate::types::ConnectionMode>,
    pub(crate) connector_profile_config: std::option::Option<crate::types::ConnectorProfileConfig>,
}
impl UpdateConnectorProfileInputBuilder {
    /// <p> The name of the connector profile and is unique for each <code>ConnectorProfile</code> in the Amazon Web Services account. </p>
    pub fn connector_profile_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.connector_profile_name = Some(input.into());
        self
    }
    /// <p> The name of the connector profile and is unique for each <code>ConnectorProfile</code> in the Amazon Web Services account. </p>
    pub fn set_connector_profile_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.connector_profile_name = input;
        self
    }
    /// <p> Indicates the connection mode and if it is public or private. </p>
    pub fn connection_mode(mut self, input: crate::types::ConnectionMode) -> Self {
        self.connection_mode = Some(input);
        self
    }
    /// <p> Indicates the connection mode and if it is public or private. </p>
    pub fn set_connection_mode(
        mut self,
        input: std::option::Option<crate::types::ConnectionMode>,
    ) -> Self {
        self.connection_mode = input;
        self
    }
    /// <p> Defines the connector-specific profile configuration and credentials. </p>
    pub fn connector_profile_config(mut self, input: crate::types::ConnectorProfileConfig) -> Self {
        self.connector_profile_config = Some(input);
        self
    }
    /// <p> Defines the connector-specific profile configuration and credentials. </p>
    pub fn set_connector_profile_config(
        mut self,
        input: std::option::Option<crate::types::ConnectorProfileConfig>,
    ) -> Self {
        self.connector_profile_config = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateConnectorProfileInput`](crate::operation::update_connector_profile::UpdateConnectorProfileInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::update_connector_profile::UpdateConnectorProfileInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::update_connector_profile::UpdateConnectorProfileInput {
                connector_profile_name: self.connector_profile_name,
                connection_mode: self.connection_mode,
                connector_profile_config: self.connector_profile_config,
            },
        )
    }
}
