// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_role_from_db_instance_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::remove_role_from_db_instance::RemoveRoleFromDbInstanceOutput,
    crate::operation::remove_role_from_db_instance::RemoveRoleFromDBInstanceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::remove_role_from_db_instance::RemoveRoleFromDBInstanceError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::remove_role_from_db_instance::RemoveRoleFromDBInstanceError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DBInstanceNotFound" => crate::operation::remove_role_from_db_instance::RemoveRoleFromDBInstanceError::DbInstanceNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DbInstanceNotFoundFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_instance_not_found_fault::de_db_instance_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::operation::remove_role_from_db_instance::RemoveRoleFromDBInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBInstanceRoleNotFound" => crate::operation::remove_role_from_db_instance::RemoveRoleFromDBInstanceError::DbInstanceRoleNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DbInstanceRoleNotFoundFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_instance_role_not_found_fault::de_db_instance_role_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::operation::remove_role_from_db_instance::RemoveRoleFromDBInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDBInstanceState" => crate::operation::remove_role_from_db_instance::RemoveRoleFromDBInstanceError::InvalidDbInstanceStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidDbInstanceStateFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_db_instance_state_fault::de_invalid_db_instance_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::operation::remove_role_from_db_instance::RemoveRoleFromDBInstanceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::remove_role_from_db_instance::RemoveRoleFromDBInstanceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_role_from_db_instance_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::remove_role_from_db_instance::RemoveRoleFromDbInstanceOutput,
    crate::operation::remove_role_from_db_instance::RemoveRoleFromDBInstanceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::remove_role_from_db_instance::builders::RemoveRoleFromDbInstanceOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
