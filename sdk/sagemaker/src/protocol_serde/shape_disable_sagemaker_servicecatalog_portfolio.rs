// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disable_sagemaker_servicecatalog_portfolio_input(
    _input: &crate::operation::disable_sagemaker_servicecatalog_portfolio::DisableSagemakerServicecatalogPortfolioInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disable_sagemaker_servicecatalog_portfolio_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::disable_sagemaker_servicecatalog_portfolio::DisableSagemakerServicecatalogPortfolioOutput, crate::operation::disable_sagemaker_servicecatalog_portfolio::DisableSagemakerServicecatalogPortfolioError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::disable_sagemaker_servicecatalog_portfolio::DisableSagemakerServicecatalogPortfolioError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::disable_sagemaker_servicecatalog_portfolio::DisableSagemakerServicecatalogPortfolioError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disable_sagemaker_servicecatalog_portfolio_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::disable_sagemaker_servicecatalog_portfolio::DisableSagemakerServicecatalogPortfolioOutput, crate::operation::disable_sagemaker_servicecatalog_portfolio::DisableSagemakerServicecatalogPortfolioError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::disable_sagemaker_servicecatalog_portfolio::builders::DisableSagemakerServicecatalogPortfolioOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
