// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::resolve_component_candidates::_resolve_component_candidates_output::ResolveComponentCandidatesOutputBuilder;

pub use crate::operation::resolve_component_candidates::_resolve_component_candidates_input::ResolveComponentCandidatesInputBuilder;

/// Fluent builder constructing a request to `ResolveComponentCandidates`.
///
/// <p>Retrieves a list of components that meet the component, version, and platform requirements of a deployment. Greengrass core devices call this operation when they receive a deployment to identify the components to install.</p>
/// <p>This operation identifies components that meet all dependency requirements for a deployment. If the requirements conflict, then this operation returns an error and the deployment fails. For example, this occurs if component <code>A</code> requires version <code>&gt;2.0.0</code> and component <code>B</code> requires version <code>&lt;2.0.0</code> of a component dependency.</p>
/// <p>When you specify the component candidates to resolve, IoT Greengrass compares each component's digest from the core device with the component's digest in the Amazon Web Services Cloud. If the digests don't match, then IoT Greengrass specifies to use the version from the Amazon Web Services Cloud.</p> <important>
/// <p>To use this operation, you must use the data plane API endpoint and authenticate with an IoT device certificate. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/greengrass.html">IoT Greengrass endpoints and quotas</a>.</p>
/// </important>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ResolveComponentCandidatesFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::resolve_component_candidates::builders::ResolveComponentCandidatesInputBuilder
            }
impl ResolveComponentCandidatesFluentBuilder {
    /// Creates a new `ResolveComponentCandidates`.
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
            crate::operation::resolve_component_candidates::ResolveComponentCandidates,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::resolve_component_candidates::ResolveComponentCandidatesError,
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
        crate::operation::resolve_component_candidates::ResolveComponentCandidatesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::resolve_component_candidates::ResolveComponentCandidatesError,
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
    /// <p>The platform to use to resolve compatible components.</p>
    pub fn platform(mut self, input: crate::types::ComponentPlatform) -> Self {
        self.inner = self.inner.platform(input);
        self
    }
    /// <p>The platform to use to resolve compatible components.</p>
    pub fn set_platform(
        mut self,
        input: std::option::Option<crate::types::ComponentPlatform>,
    ) -> Self {
        self.inner = self.inner.set_platform(input);
        self
    }
    /// Appends an item to `componentCandidates`.
    ///
    /// To override the contents of this collection use [`set_component_candidates`](Self::set_component_candidates).
    ///
    /// <p>The list of components to resolve.</p>
    pub fn component_candidates(mut self, input: crate::types::ComponentCandidate) -> Self {
        self.inner = self.inner.component_candidates(input);
        self
    }
    /// <p>The list of components to resolve.</p>
    pub fn set_component_candidates(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ComponentCandidate>>,
    ) -> Self {
        self.inner = self.inner.set_component_candidates(input);
        self
    }
}
