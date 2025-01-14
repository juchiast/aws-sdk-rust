// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_distributions_by_web_acl_id::_list_distributions_by_web_acl_id_output::ListDistributionsByWebAclIdOutputBuilder;

pub use crate::operation::list_distributions_by_web_acl_id::_list_distributions_by_web_acl_id_input::ListDistributionsByWebAclIdInputBuilder;

/// Fluent builder constructing a request to `ListDistributionsByWebACLId`.
///
/// <p>List the distributions that are associated with a specified WAF web ACL.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListDistributionsByWebACLIdFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::list_distributions_by_web_acl_id::builders::ListDistributionsByWebAclIdInputBuilder
            }
impl ListDistributionsByWebACLIdFluentBuilder {
    /// Creates a new `ListDistributionsByWebACLId`.
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
            crate::operation::list_distributions_by_web_acl_id::ListDistributionsByWebACLId,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_distributions_by_web_acl_id::ListDistributionsByWebACLIdError,
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
        crate::operation::list_distributions_by_web_acl_id::ListDistributionsByWebAclIdOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_distributions_by_web_acl_id::ListDistributionsByWebACLIdError,
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
    /// <p>Use <code>Marker</code> and <code>MaxItems</code> to control pagination of results. If you have more than <code>MaxItems</code> distributions that satisfy the request, the response includes a <code>NextMarker</code> element. To get the next page of results, submit another request. For the value of <code>Marker</code>, specify the value of <code>NextMarker</code> from the last response. (For the first request, omit <code>Marker</code>.)</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>Use <code>Marker</code> and <code>MaxItems</code> to control pagination of results. If you have more than <code>MaxItems</code> distributions that satisfy the request, the response includes a <code>NextMarker</code> element. To get the next page of results, submit another request. For the value of <code>Marker</code>, specify the value of <code>NextMarker</code> from the last response. (For the first request, omit <code>Marker</code>.)</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>The maximum number of distributions that you want CloudFront to return in the response body. The maximum and default values are both 100.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>The maximum number of distributions that you want CloudFront to return in the response body. The maximum and default values are both 100.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
    /// <p>The ID of the WAF web ACL that you want to list the associated distributions. If you specify "null" for the ID, the request returns a list of the distributions that aren't associated with a web ACL.</p>
    pub fn web_acl_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.web_acl_id(input.into());
        self
    }
    /// <p>The ID of the WAF web ACL that you want to list the associated distributions. If you specify "null" for the ID, the request returns a list of the distributions that aren't associated with a web ACL.</p>
    pub fn set_web_acl_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_web_acl_id(input);
        self
    }
}
