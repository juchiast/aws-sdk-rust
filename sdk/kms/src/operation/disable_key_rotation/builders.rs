// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_key_rotation::_disable_key_rotation_output::DisableKeyRotationOutputBuilder;

pub use crate::operation::disable_key_rotation::_disable_key_rotation_input::DisableKeyRotationInputBuilder;

/// Fluent builder constructing a request to `DisableKeyRotation`.
///
/// <p>Disables <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> of the specified symmetric encryption KMS key.</p>
/// <p>Automatic key rotation is supported only on symmetric encryption KMS keys. You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">asymmetric KMS keys</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC KMS keys</a>, KMS keys with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or KMS keys in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. To enable or disable automatic rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-manage.html#multi-region-rotate">multi-Region keys</a>, set the property on the primary key.</p>
/// <p>You can enable (<code>EnableKeyRotation</code>) and disable automatic rotation of the key material in <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed KMS keys</a>. Key material rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">Amazon Web Services managed KMS keys</a> is not configurable. KMS always rotates the key material for every year. Rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-owned-cmk">Amazon Web Services owned KMS keys</a> varies.</p> <note>
/// <p>In May 2022, KMS changed the rotation schedule for Amazon Web Services managed keys from every three years to every year. For details, see <code>EnableKeyRotation</code>.</p>
/// </note>
/// <p>The KMS key that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key states of KMS keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
/// <p> <b>Cross-account use</b>: No. You cannot perform this operation on a KMS key in a different Amazon Web Services account.</p>
/// <p> <b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DisableKeyRotation</a> (key policy)</p>
/// <p> <b>Related operations:</b> </p>
/// <ul>
/// <li> <p> <code>EnableKeyRotation</code> </p> </li>
/// <li> <p> <code>GetKeyRotationStatus</code> </p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DisableKeyRotationFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_key_rotation::builders::DisableKeyRotationInputBuilder,
}
impl DisableKeyRotationFluentBuilder {
    /// Creates a new `DisableKeyRotation`.
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
            crate::operation::disable_key_rotation::DisableKeyRotation,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::disable_key_rotation::DisableKeyRotationError,
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
        crate::operation::disable_key_rotation::DisableKeyRotationOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::disable_key_rotation::DisableKeyRotationError,
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
    /// <p>Identifies a symmetric encryption KMS key. You cannot enable or disable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html#asymmetric-cmks">asymmetric KMS keys</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC KMS keys</a>, KMS keys with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or KMS keys in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.key_id(input.into());
        self
    }
    /// <p>Identifies a symmetric encryption KMS key. You cannot enable or disable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html#asymmetric-cmks">asymmetric KMS keys</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC KMS keys</a>, KMS keys with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or KMS keys in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn set_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_key_id(input);
        self
    }
}
