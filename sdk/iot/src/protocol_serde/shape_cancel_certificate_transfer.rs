// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_certificate_transfer_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::cancel_certificate_transfer::CancelCertificateTransferOutput,
    crate::operation::cancel_certificate_transfer::CancelCertificateTransferError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalFailureException" => crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TransferAlreadyCompletedException" => crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::TransferAlreadyCompletedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TransferAlreadyCompletedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_transfer_already_completed_exception::de_transfer_already_completed_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthorizedException" => crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::UnauthorizedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::cancel_certificate_transfer::CancelCertificateTransferError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_certificate_transfer_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::cancel_certificate_transfer::CancelCertificateTransferOutput,
    crate::operation::cancel_certificate_transfer::CancelCertificateTransferError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::cancel_certificate_transfer::builders::CancelCertificateTransferOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
