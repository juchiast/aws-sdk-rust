// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_schema_version_metadata::_remove_schema_version_metadata_output::RemoveSchemaVersionMetadataOutputBuilder;

pub use crate::operation::remove_schema_version_metadata::_remove_schema_version_metadata_input::RemoveSchemaVersionMetadataInputBuilder;

/// Fluent builder constructing a request to `RemoveSchemaVersionMetadata`.
///
/// <p>Removes a key value pair from the schema version metadata for the specified schema version ID.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RemoveSchemaVersionMetadataFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::remove_schema_version_metadata::builders::RemoveSchemaVersionMetadataInputBuilder
            }
impl RemoveSchemaVersionMetadataFluentBuilder {
    /// Creates a new `RemoveSchemaVersionMetadata`.
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
            crate::operation::remove_schema_version_metadata::RemoveSchemaVersionMetadata,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::remove_schema_version_metadata::RemoveSchemaVersionMetadataError,
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
        crate::operation::remove_schema_version_metadata::RemoveSchemaVersionMetadataOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::remove_schema_version_metadata::RemoveSchemaVersionMetadataError,
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
    /// <p>A wrapper structure that may contain the schema name and Amazon Resource Name (ARN).</p>
    pub fn schema_id(mut self, input: crate::types::SchemaId) -> Self {
        self.inner = self.inner.schema_id(input);
        self
    }
    /// <p>A wrapper structure that may contain the schema name and Amazon Resource Name (ARN).</p>
    pub fn set_schema_id(mut self, input: std::option::Option<crate::types::SchemaId>) -> Self {
        self.inner = self.inner.set_schema_id(input);
        self
    }
    /// <p>The version number of the schema.</p>
    pub fn schema_version_number(mut self, input: crate::types::SchemaVersionNumber) -> Self {
        self.inner = self.inner.schema_version_number(input);
        self
    }
    /// <p>The version number of the schema.</p>
    pub fn set_schema_version_number(
        mut self,
        input: std::option::Option<crate::types::SchemaVersionNumber>,
    ) -> Self {
        self.inner = self.inner.set_schema_version_number(input);
        self
    }
    /// <p>The unique version ID of the schema version.</p>
    pub fn schema_version_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.schema_version_id(input.into());
        self
    }
    /// <p>The unique version ID of the schema version.</p>
    pub fn set_schema_version_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_schema_version_id(input);
        self
    }
    /// <p>The value of the metadata key.</p>
    pub fn metadata_key_value(mut self, input: crate::types::MetadataKeyValuePair) -> Self {
        self.inner = self.inner.metadata_key_value(input);
        self
    }
    /// <p>The value of the metadata key.</p>
    pub fn set_metadata_key_value(
        mut self,
        input: std::option::Option<crate::types::MetadataKeyValuePair>,
    ) -> Self {
        self.inner = self.inner.set_metadata_key_value(input);
        self
    }
}
