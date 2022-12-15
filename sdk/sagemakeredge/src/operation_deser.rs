// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_deployments_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetDeploymentsOutput, crate::error::GetDeploymentsError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetDeploymentsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetDeploymentsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => crate::error::GetDeploymentsError {
            meta: generic,
            kind: crate::error::GetDeploymentsErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDeploymentsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetDeploymentsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_deployments_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetDeploymentsOutput, crate::error::GetDeploymentsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_deployments_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_deployments(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetDeploymentsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_device_registration_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetDeviceRegistrationOutput,
    crate::error::GetDeviceRegistrationError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetDeviceRegistrationError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetDeviceRegistrationError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => crate::error::GetDeviceRegistrationError {
            meta: generic,
            kind: crate::error::GetDeviceRegistrationErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDeviceRegistrationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetDeviceRegistrationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_device_registration_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetDeviceRegistrationOutput,
    crate::error::GetDeviceRegistrationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_device_registration_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_device_registration(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetDeviceRegistrationError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_send_heartbeat_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::SendHeartbeatOutput, crate::error::SendHeartbeatError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::SendHeartbeatError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::SendHeartbeatError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => crate::error::SendHeartbeatError {
            meta: generic,
            kind: crate::error::SendHeartbeatErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::SendHeartbeatError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::SendHeartbeatError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_send_heartbeat_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::SendHeartbeatOutput, crate::error::SendHeartbeatError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::send_heartbeat_output::Builder::default();
        let _ = response;
        output.build()
    })
}
