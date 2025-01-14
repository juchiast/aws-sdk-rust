// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartFlow`](crate::operation::start_flow::builders::StartFlowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`flow_name(impl Into<String>)`](crate::operation::start_flow::builders::StartFlowFluentBuilder::flow_name) / [`set_flow_name(Option<String>)`](crate::operation::start_flow::builders::StartFlowFluentBuilder::set_flow_name): <p> The specified name of the flow. Spaces are not allowed. Use underscores (_) or hyphens (-) only. </p>
    /// - On success, responds with [`StartFlowOutput`](crate::operation::start_flow::StartFlowOutput) with field(s):
    ///   - [`flow_arn(Option<String>)`](crate::operation::start_flow::StartFlowOutput::flow_arn): <p> The flow's Amazon Resource Name (ARN). </p>
    ///   - [`flow_status(Option<FlowStatus>)`](crate::operation::start_flow::StartFlowOutput::flow_status): <p> Indicates the current status of the flow. </p>
    ///   - [`execution_id(Option<String>)`](crate::operation::start_flow::StartFlowOutput::execution_id): <p> Returns the internal execution ID of an on-demand flow when the flow is started. For scheduled or event-triggered flows, this value is null. </p>
    /// - On failure, responds with [`SdkError<StartFlowError>`](crate::operation::start_flow::StartFlowError)
    pub fn start_flow(&self) -> crate::operation::start_flow::builders::StartFlowFluentBuilder {
        crate::operation::start_flow::builders::StartFlowFluentBuilder::new(self.handle.clone())
    }
}
