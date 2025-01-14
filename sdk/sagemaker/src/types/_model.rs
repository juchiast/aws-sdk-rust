// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The properties of a model as returned by the <code>Search</code> API.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Model {
    /// <p>The name of the model.</p>
    #[doc(hidden)]
    pub model_name: std::option::Option<std::string::String>,
    /// <p>Describes the container, as part of model definition.</p>
    #[doc(hidden)]
    pub primary_container: std::option::Option<crate::types::ContainerDefinition>,
    /// <p>The containers in the inference pipeline.</p>
    #[doc(hidden)]
    pub containers: std::option::Option<std::vec::Vec<crate::types::ContainerDefinition>>,
    /// <p>Specifies details about how containers in a multi-container endpoint are run.</p>
    #[doc(hidden)]
    pub inference_execution_config: std::option::Option<crate::types::InferenceExecutionConfig>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that you specified for the model.</p>
    #[doc(hidden)]
    pub execution_role_arn: std::option::Option<std::string::String>,
    /// <p>Specifies a VPC that your training jobs and hosted models have access to. Control access to and from your training and model containers by configuring the VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/host-vpc.html">Protect Endpoints by Using an Amazon Virtual Private Cloud</a> and <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-vpc.html">Protect Training Jobs by Using an Amazon Virtual Private Cloud</a>. </p>
    #[doc(hidden)]
    pub vpc_config: std::option::Option<crate::types::VpcConfig>,
    /// <p>A timestamp that indicates when the model was created.</p>
    #[doc(hidden)]
    pub creation_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The Amazon Resource Name (ARN) of the model.</p>
    #[doc(hidden)]
    pub model_arn: std::option::Option<std::string::String>,
    /// <p>Isolates the model container. No inbound or outbound network calls can be made to or from the model container.</p>
    #[doc(hidden)]
    pub enable_network_isolation: bool,
    /// <p>A list of key-value pairs associated with the model. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl Model {
    /// <p>The name of the model.</p>
    pub fn model_name(&self) -> std::option::Option<&str> {
        self.model_name.as_deref()
    }
    /// <p>Describes the container, as part of model definition.</p>
    pub fn primary_container(&self) -> std::option::Option<&crate::types::ContainerDefinition> {
        self.primary_container.as_ref()
    }
    /// <p>The containers in the inference pipeline.</p>
    pub fn containers(&self) -> std::option::Option<&[crate::types::ContainerDefinition]> {
        self.containers.as_deref()
    }
    /// <p>Specifies details about how containers in a multi-container endpoint are run.</p>
    pub fn inference_execution_config(
        &self,
    ) -> std::option::Option<&crate::types::InferenceExecutionConfig> {
        self.inference_execution_config.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role that you specified for the model.</p>
    pub fn execution_role_arn(&self) -> std::option::Option<&str> {
        self.execution_role_arn.as_deref()
    }
    /// <p>Specifies a VPC that your training jobs and hosted models have access to. Control access to and from your training and model containers by configuring the VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/host-vpc.html">Protect Endpoints by Using an Amazon Virtual Private Cloud</a> and <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-vpc.html">Protect Training Jobs by Using an Amazon Virtual Private Cloud</a>. </p>
    pub fn vpc_config(&self) -> std::option::Option<&crate::types::VpcConfig> {
        self.vpc_config.as_ref()
    }
    /// <p>A timestamp that indicates when the model was created.</p>
    pub fn creation_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the model.</p>
    pub fn model_arn(&self) -> std::option::Option<&str> {
        self.model_arn.as_deref()
    }
    /// <p>Isolates the model container. No inbound or outbound network calls can be made to or from the model container.</p>
    pub fn enable_network_isolation(&self) -> bool {
        self.enable_network_isolation
    }
    /// <p>A list of key-value pairs associated with the model. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl Model {
    /// Creates a new builder-style object to manufacture [`Model`](crate::types::Model).
    pub fn builder() -> crate::types::builders::ModelBuilder {
        crate::types::builders::ModelBuilder::default()
    }
}

/// A builder for [`Model`](crate::types::Model).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ModelBuilder {
    pub(crate) model_name: std::option::Option<std::string::String>,
    pub(crate) primary_container: std::option::Option<crate::types::ContainerDefinition>,
    pub(crate) containers: std::option::Option<std::vec::Vec<crate::types::ContainerDefinition>>,
    pub(crate) inference_execution_config:
        std::option::Option<crate::types::InferenceExecutionConfig>,
    pub(crate) execution_role_arn: std::option::Option<std::string::String>,
    pub(crate) vpc_config: std::option::Option<crate::types::VpcConfig>,
    pub(crate) creation_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) model_arn: std::option::Option<std::string::String>,
    pub(crate) enable_network_isolation: std::option::Option<bool>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl ModelBuilder {
    /// <p>The name of the model.</p>
    pub fn model_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.model_name = Some(input.into());
        self
    }
    /// <p>The name of the model.</p>
    pub fn set_model_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.model_name = input;
        self
    }
    /// <p>Describes the container, as part of model definition.</p>
    pub fn primary_container(mut self, input: crate::types::ContainerDefinition) -> Self {
        self.primary_container = Some(input);
        self
    }
    /// <p>Describes the container, as part of model definition.</p>
    pub fn set_primary_container(
        mut self,
        input: std::option::Option<crate::types::ContainerDefinition>,
    ) -> Self {
        self.primary_container = input;
        self
    }
    /// Appends an item to `containers`.
    ///
    /// To override the contents of this collection use [`set_containers`](Self::set_containers).
    ///
    /// <p>The containers in the inference pipeline.</p>
    pub fn containers(mut self, input: crate::types::ContainerDefinition) -> Self {
        let mut v = self.containers.unwrap_or_default();
        v.push(input);
        self.containers = Some(v);
        self
    }
    /// <p>The containers in the inference pipeline.</p>
    pub fn set_containers(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ContainerDefinition>>,
    ) -> Self {
        self.containers = input;
        self
    }
    /// <p>Specifies details about how containers in a multi-container endpoint are run.</p>
    pub fn inference_execution_config(
        mut self,
        input: crate::types::InferenceExecutionConfig,
    ) -> Self {
        self.inference_execution_config = Some(input);
        self
    }
    /// <p>Specifies details about how containers in a multi-container endpoint are run.</p>
    pub fn set_inference_execution_config(
        mut self,
        input: std::option::Option<crate::types::InferenceExecutionConfig>,
    ) -> Self {
        self.inference_execution_config = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role that you specified for the model.</p>
    pub fn execution_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.execution_role_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role that you specified for the model.</p>
    pub fn set_execution_role_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.execution_role_arn = input;
        self
    }
    /// <p>Specifies a VPC that your training jobs and hosted models have access to. Control access to and from your training and model containers by configuring the VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/host-vpc.html">Protect Endpoints by Using an Amazon Virtual Private Cloud</a> and <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-vpc.html">Protect Training Jobs by Using an Amazon Virtual Private Cloud</a>. </p>
    pub fn vpc_config(mut self, input: crate::types::VpcConfig) -> Self {
        self.vpc_config = Some(input);
        self
    }
    /// <p>Specifies a VPC that your training jobs and hosted models have access to. Control access to and from your training and model containers by configuring the VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/host-vpc.html">Protect Endpoints by Using an Amazon Virtual Private Cloud</a> and <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/train-vpc.html">Protect Training Jobs by Using an Amazon Virtual Private Cloud</a>. </p>
    pub fn set_vpc_config(mut self, input: std::option::Option<crate::types::VpcConfig>) -> Self {
        self.vpc_config = input;
        self
    }
    /// <p>A timestamp that indicates when the model was created.</p>
    pub fn creation_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.creation_time = Some(input);
        self
    }
    /// <p>A timestamp that indicates when the model was created.</p>
    pub fn set_creation_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the model.</p>
    pub fn model_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.model_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the model.</p>
    pub fn set_model_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.model_arn = input;
        self
    }
    /// <p>Isolates the model container. No inbound or outbound network calls can be made to or from the model container.</p>
    pub fn enable_network_isolation(mut self, input: bool) -> Self {
        self.enable_network_isolation = Some(input);
        self
    }
    /// <p>Isolates the model container. No inbound or outbound network calls can be made to or from the model container.</p>
    pub fn set_enable_network_isolation(mut self, input: std::option::Option<bool>) -> Self {
        self.enable_network_isolation = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of key-value pairs associated with the model. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>A list of key-value pairs associated with the model. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`Model`](crate::types::Model).
    pub fn build(self) -> crate::types::Model {
        crate::types::Model {
            model_name: self.model_name,
            primary_container: self.primary_container,
            containers: self.containers,
            inference_execution_config: self.inference_execution_config,
            execution_role_arn: self.execution_role_arn,
            vpc_config: self.vpc_config,
            creation_time: self.creation_time,
            model_arn: self.model_arn,
            enable_network_isolation: self.enable_network_isolation.unwrap_or_default(),
            tags: self.tags,
        }
    }
}
