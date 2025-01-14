// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_virtual_gateway_payload(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::types::VirtualGatewayData>,
    crate::operation::update_virtual_gateway::UpdateVirtualGatewayError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_virtual_gateway_data::de_virtual_gateway_data_payload(body)
                .map_err(
                    crate::operation::update_virtual_gateway::UpdateVirtualGatewayError::unhandled,
                )
        })
        .transpose()
}
