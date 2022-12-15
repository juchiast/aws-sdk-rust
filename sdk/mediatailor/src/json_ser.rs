// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_configure_logs_for_playback_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ConfigureLogsForPlaybackConfigurationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    {
        object.key("PercentEnabled").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.percent_enabled).into()),
        );
    }
    if let Some(var_1) = &input.playback_configuration_name {
        object
            .key("PlaybackConfigurationName")
            .string(var_1.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateChannelInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_2) = &input.filler_slate {
        #[allow(unused_mut)]
        let mut object_3 = object.key("FillerSlate").start_object();
        crate::json_ser::serialize_structure_crate_model_slate_source(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.outputs {
        let mut array_5 = object.key("Outputs").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::json_ser::serialize_structure_crate_model_request_output_item(
                    &mut object_7,
                    item_6,
                )?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.playback_mode {
        object.key("PlaybackMode").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        #[allow(unused_mut)]
        let mut object_10 = object.key("tags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    if let Some(var_13) = &input.tier {
        object.key("Tier").string(var_13.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_live_source_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateLiveSourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_14) = &input.http_package_configurations {
        let mut array_15 = object.key("HttpPackageConfigurations").start_array();
        for item_16 in var_14 {
            {
                #[allow(unused_mut)]
                let mut object_17 = array_15.value().start_object();
                crate::json_ser::serialize_structure_crate_model_http_package_configuration(
                    &mut object_17,
                    item_16,
                )?;
                object_17.finish();
            }
        }
        array_15.finish();
    }
    if let Some(var_18) = &input.tags {
        #[allow(unused_mut)]
        let mut object_19 = object.key("tags").start_object();
        for (key_20, value_21) in var_18 {
            {
                object_19.key(key_20.as_str()).string(value_21.as_str());
            }
        }
        object_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_prefetch_schedule_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreatePrefetchScheduleInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.consumption {
        #[allow(unused_mut)]
        let mut object_23 = object.key("Consumption").start_object();
        crate::json_ser::serialize_structure_crate_model_prefetch_consumption(
            &mut object_23,
            var_22,
        )?;
        object_23.finish();
    }
    if let Some(var_24) = &input.retrieval {
        #[allow(unused_mut)]
        let mut object_25 = object.key("Retrieval").start_object();
        crate::json_ser::serialize_structure_crate_model_prefetch_retrieval(
            &mut object_25,
            var_24,
        )?;
        object_25.finish();
    }
    if let Some(var_26) = &input.stream_id {
        object.key("StreamId").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_program_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateProgramInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_27) = &input.ad_breaks {
        let mut array_28 = object.key("AdBreaks").start_array();
        for item_29 in var_27 {
            {
                #[allow(unused_mut)]
                let mut object_30 = array_28.value().start_object();
                crate::json_ser::serialize_structure_crate_model_ad_break(&mut object_30, item_29)?;
                object_30.finish();
            }
        }
        array_28.finish();
    }
    if let Some(var_31) = &input.live_source_name {
        object.key("LiveSourceName").string(var_31.as_str());
    }
    if let Some(var_32) = &input.schedule_configuration {
        #[allow(unused_mut)]
        let mut object_33 = object.key("ScheduleConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_schedule_configuration(
            &mut object_33,
            var_32,
        )?;
        object_33.finish();
    }
    if let Some(var_34) = &input.source_location_name {
        object.key("SourceLocationName").string(var_34.as_str());
    }
    if let Some(var_35) = &input.vod_source_name {
        object.key("VodSourceName").string(var_35.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_source_location_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSourceLocationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.access_configuration {
        #[allow(unused_mut)]
        let mut object_37 = object.key("AccessConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_access_configuration(
            &mut object_37,
            var_36,
        )?;
        object_37.finish();
    }
    if let Some(var_38) = &input.default_segment_delivery_configuration {
        #[allow(unused_mut)]
        let mut object_39 = object
            .key("DefaultSegmentDeliveryConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_default_segment_delivery_configuration(
            &mut object_39,
            var_38,
        )?;
        object_39.finish();
    }
    if let Some(var_40) = &input.http_configuration {
        #[allow(unused_mut)]
        let mut object_41 = object.key("HttpConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_http_configuration(
            &mut object_41,
            var_40,
        )?;
        object_41.finish();
    }
    if let Some(var_42) = &input.segment_delivery_configurations {
        let mut array_43 = object.key("SegmentDeliveryConfigurations").start_array();
        for item_44 in var_42 {
            {
                #[allow(unused_mut)]
                let mut object_45 = array_43.value().start_object();
                crate::json_ser::serialize_structure_crate_model_segment_delivery_configuration(
                    &mut object_45,
                    item_44,
                )?;
                object_45.finish();
            }
        }
        array_43.finish();
    }
    if let Some(var_46) = &input.tags {
        #[allow(unused_mut)]
        let mut object_47 = object.key("tags").start_object();
        for (key_48, value_49) in var_46 {
            {
                object_47.key(key_48.as_str()).string(value_49.as_str());
            }
        }
        object_47.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_vod_source_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateVodSourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.http_package_configurations {
        let mut array_51 = object.key("HttpPackageConfigurations").start_array();
        for item_52 in var_50 {
            {
                #[allow(unused_mut)]
                let mut object_53 = array_51.value().start_object();
                crate::json_ser::serialize_structure_crate_model_http_package_configuration(
                    &mut object_53,
                    item_52,
                )?;
                object_53.finish();
            }
        }
        array_51.finish();
    }
    if let Some(var_54) = &input.tags {
        #[allow(unused_mut)]
        let mut object_55 = object.key("tags").start_object();
        for (key_56, value_57) in var_54 {
            {
                object_55.key(key_56.as_str()).string(value_57.as_str());
            }
        }
        object_55.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_prefetch_schedules_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListPrefetchSchedulesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.max_results != 0 {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_58) = &input.next_token {
        object.key("NextToken").string(var_58.as_str());
    }
    if let Some(var_59) = &input.stream_id {
        object.key("StreamId").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_channel_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutChannelPolicyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_60) = &input.policy {
        object.key("Policy").string(var_60.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_playback_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutPlaybackConfigurationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_61) = &input.ad_decision_server_url {
        object.key("AdDecisionServerUrl").string(var_61.as_str());
    }
    if let Some(var_62) = &input.avail_suppression {
        #[allow(unused_mut)]
        let mut object_63 = object.key("AvailSuppression").start_object();
        crate::json_ser::serialize_structure_crate_model_avail_suppression(&mut object_63, var_62)?;
        object_63.finish();
    }
    if let Some(var_64) = &input.bumper {
        #[allow(unused_mut)]
        let mut object_65 = object.key("Bumper").start_object();
        crate::json_ser::serialize_structure_crate_model_bumper(&mut object_65, var_64)?;
        object_65.finish();
    }
    if let Some(var_66) = &input.cdn_configuration {
        #[allow(unused_mut)]
        let mut object_67 = object.key("CdnConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_cdn_configuration(&mut object_67, var_66)?;
        object_67.finish();
    }
    if let Some(var_68) = &input.configuration_aliases {
        #[allow(unused_mut)]
        let mut object_69 = object.key("ConfigurationAliases").start_object();
        for (key_70, value_71) in var_68 {
            {
                #[allow(unused_mut)]
                let mut object_72 = object_69.key(key_70.as_str()).start_object();
                for (key_73, value_74) in value_71 {
                    {
                        object_72.key(key_73.as_str()).string(value_74.as_str());
                    }
                }
                object_72.finish();
            }
        }
        object_69.finish();
    }
    if let Some(var_75) = &input.dash_configuration {
        #[allow(unused_mut)]
        let mut object_76 = object.key("DashConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_configuration_for_put(
            &mut object_76,
            var_75,
        )?;
        object_76.finish();
    }
    if let Some(var_77) = &input.live_pre_roll_configuration {
        #[allow(unused_mut)]
        let mut object_78 = object.key("LivePreRollConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_live_pre_roll_configuration(
            &mut object_78,
            var_77,
        )?;
        object_78.finish();
    }
    if let Some(var_79) = &input.manifest_processing_rules {
        #[allow(unused_mut)]
        let mut object_80 = object.key("ManifestProcessingRules").start_object();
        crate::json_ser::serialize_structure_crate_model_manifest_processing_rules(
            &mut object_80,
            var_79,
        )?;
        object_80.finish();
    }
    if let Some(var_81) = &input.name {
        object.key("Name").string(var_81.as_str());
    }
    if input.personalization_threshold_seconds != 0 {
        object.key("PersonalizationThresholdSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.personalization_threshold_seconds).into()),
        );
    }
    if let Some(var_82) = &input.slate_ad_url {
        object.key("SlateAdUrl").string(var_82.as_str());
    }
    if let Some(var_83) = &input.tags {
        #[allow(unused_mut)]
        let mut object_84 = object.key("tags").start_object();
        for (key_85, value_86) in var_83 {
            {
                object_84.key(key_85.as_str()).string(value_86.as_str());
            }
        }
        object_84.finish();
    }
    if let Some(var_87) = &input.transcode_profile_name {
        object.key("TranscodeProfileName").string(var_87.as_str());
    }
    if let Some(var_88) = &input.video_content_source_url {
        object.key("VideoContentSourceUrl").string(var_88.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_89) = &input.tags {
        #[allow(unused_mut)]
        let mut object_90 = object.key("tags").start_object();
        for (key_91, value_92) in var_89 {
            {
                object_90.key(key_91.as_str()).string(value_92.as_str());
            }
        }
        object_90.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_channel_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateChannelInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_93) = &input.filler_slate {
        #[allow(unused_mut)]
        let mut object_94 = object.key("FillerSlate").start_object();
        crate::json_ser::serialize_structure_crate_model_slate_source(&mut object_94, var_93)?;
        object_94.finish();
    }
    if let Some(var_95) = &input.outputs {
        let mut array_96 = object.key("Outputs").start_array();
        for item_97 in var_95 {
            {
                #[allow(unused_mut)]
                let mut object_98 = array_96.value().start_object();
                crate::json_ser::serialize_structure_crate_model_request_output_item(
                    &mut object_98,
                    item_97,
                )?;
                object_98.finish();
            }
        }
        array_96.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_live_source_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLiveSourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_99) = &input.http_package_configurations {
        let mut array_100 = object.key("HttpPackageConfigurations").start_array();
        for item_101 in var_99 {
            {
                #[allow(unused_mut)]
                let mut object_102 = array_100.value().start_object();
                crate::json_ser::serialize_structure_crate_model_http_package_configuration(
                    &mut object_102,
                    item_101,
                )?;
                object_102.finish();
            }
        }
        array_100.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_source_location_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSourceLocationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_103) = &input.access_configuration {
        #[allow(unused_mut)]
        let mut object_104 = object.key("AccessConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_access_configuration(
            &mut object_104,
            var_103,
        )?;
        object_104.finish();
    }
    if let Some(var_105) = &input.default_segment_delivery_configuration {
        #[allow(unused_mut)]
        let mut object_106 = object
            .key("DefaultSegmentDeliveryConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_default_segment_delivery_configuration(
            &mut object_106,
            var_105,
        )?;
        object_106.finish();
    }
    if let Some(var_107) = &input.http_configuration {
        #[allow(unused_mut)]
        let mut object_108 = object.key("HttpConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_http_configuration(
            &mut object_108,
            var_107,
        )?;
        object_108.finish();
    }
    if let Some(var_109) = &input.segment_delivery_configurations {
        let mut array_110 = object.key("SegmentDeliveryConfigurations").start_array();
        for item_111 in var_109 {
            {
                #[allow(unused_mut)]
                let mut object_112 = array_110.value().start_object();
                crate::json_ser::serialize_structure_crate_model_segment_delivery_configuration(
                    &mut object_112,
                    item_111,
                )?;
                object_112.finish();
            }
        }
        array_110.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_vod_source_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateVodSourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_113) = &input.http_package_configurations {
        let mut array_114 = object.key("HttpPackageConfigurations").start_array();
        for item_115 in var_113 {
            {
                #[allow(unused_mut)]
                let mut object_116 = array_114.value().start_object();
                crate::json_ser::serialize_structure_crate_model_http_package_configuration(
                    &mut object_116,
                    item_115,
                )?;
                object_116.finish();
            }
        }
        array_114.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_slate_source(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SlateSource,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_117) = &input.source_location_name {
        object.key("SourceLocationName").string(var_117.as_str());
    }
    if let Some(var_118) = &input.vod_source_name {
        object.key("VodSourceName").string(var_118.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_request_output_item(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RequestOutputItem,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_119) = &input.dash_playlist_settings {
        #[allow(unused_mut)]
        let mut object_120 = object.key("DashPlaylistSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_playlist_settings(
            &mut object_120,
            var_119,
        )?;
        object_120.finish();
    }
    if let Some(var_121) = &input.hls_playlist_settings {
        #[allow(unused_mut)]
        let mut object_122 = object.key("HlsPlaylistSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_hls_playlist_settings(
            &mut object_122,
            var_121,
        )?;
        object_122.finish();
    }
    if let Some(var_123) = &input.manifest_name {
        object.key("ManifestName").string(var_123.as_str());
    }
    if let Some(var_124) = &input.source_group {
        object.key("SourceGroup").string(var_124.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_http_package_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HttpPackageConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_125) = &input.path {
        object.key("Path").string(var_125.as_str());
    }
    if let Some(var_126) = &input.source_group {
        object.key("SourceGroup").string(var_126.as_str());
    }
    if let Some(var_127) = &input.r#type {
        object.key("Type").string(var_127.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_prefetch_consumption(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PrefetchConsumption,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_128) = &input.avail_matching_criteria {
        let mut array_129 = object.key("AvailMatchingCriteria").start_array();
        for item_130 in var_128 {
            {
                #[allow(unused_mut)]
                let mut object_131 = array_129.value().start_object();
                crate::json_ser::serialize_structure_crate_model_avail_matching_criteria(
                    &mut object_131,
                    item_130,
                )?;
                object_131.finish();
            }
        }
        array_129.finish();
    }
    if let Some(var_132) = &input.end_time {
        object
            .key("EndTime")
            .date_time(var_132, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_133) = &input.start_time {
        object
            .key("StartTime")
            .date_time(var_133, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_model_prefetch_retrieval(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PrefetchRetrieval,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_134) = &input.dynamic_variables {
        #[allow(unused_mut)]
        let mut object_135 = object.key("DynamicVariables").start_object();
        for (key_136, value_137) in var_134 {
            {
                object_135.key(key_136.as_str()).string(value_137.as_str());
            }
        }
        object_135.finish();
    }
    if let Some(var_138) = &input.end_time {
        object
            .key("EndTime")
            .date_time(var_138, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_139) = &input.start_time {
        object
            .key("StartTime")
            .date_time(var_139, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ad_break(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdBreak,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_140) = &input.message_type {
        object.key("MessageType").string(var_140.as_str());
    }
    if input.offset_millis != 0 {
        object.key("OffsetMillis").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.offset_millis).into()),
        );
    }
    if let Some(var_141) = &input.slate {
        #[allow(unused_mut)]
        let mut object_142 = object.key("Slate").start_object();
        crate::json_ser::serialize_structure_crate_model_slate_source(&mut object_142, var_141)?;
        object_142.finish();
    }
    if let Some(var_143) = &input.splice_insert_message {
        #[allow(unused_mut)]
        let mut object_144 = object.key("SpliceInsertMessage").start_object();
        crate::json_ser::serialize_structure_crate_model_splice_insert_message(
            &mut object_144,
            var_143,
        )?;
        object_144.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_schedule_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ScheduleConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_145) = &input.transition {
        #[allow(unused_mut)]
        let mut object_146 = object.key("Transition").start_object();
        crate::json_ser::serialize_structure_crate_model_transition(&mut object_146, var_145)?;
        object_146.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_access_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AccessConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_147) = &input.access_type {
        object.key("AccessType").string(var_147.as_str());
    }
    if let Some(var_148) = &input.secrets_manager_access_token_configuration {
        #[allow(unused_mut)]
        let mut object_149 = object
            .key("SecretsManagerAccessTokenConfiguration")
            .start_object();
        crate::json_ser::serialize_structure_crate_model_secrets_manager_access_token_configuration(&mut object_149, var_148)?;
        object_149.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_default_segment_delivery_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DefaultSegmentDeliveryConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_150) = &input.base_url {
        object.key("BaseUrl").string(var_150.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_http_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HttpConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_151) = &input.base_url {
        object.key("BaseUrl").string(var_151.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_segment_delivery_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SegmentDeliveryConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_152) = &input.base_url {
        object.key("BaseUrl").string(var_152.as_str());
    }
    if let Some(var_153) = &input.name {
        object.key("Name").string(var_153.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_avail_suppression(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AvailSuppression,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_154) = &input.mode {
        object.key("Mode").string(var_154.as_str());
    }
    if let Some(var_155) = &input.value {
        object.key("Value").string(var_155.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_bumper(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Bumper,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_156) = &input.end_url {
        object.key("EndUrl").string(var_156.as_str());
    }
    if let Some(var_157) = &input.start_url {
        object.key("StartUrl").string(var_157.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cdn_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CdnConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_158) = &input.ad_segment_url_prefix {
        object.key("AdSegmentUrlPrefix").string(var_158.as_str());
    }
    if let Some(var_159) = &input.content_segment_url_prefix {
        object
            .key("ContentSegmentUrlPrefix")
            .string(var_159.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dash_configuration_for_put(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DashConfigurationForPut,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_160) = &input.mpd_location {
        object.key("MpdLocation").string(var_160.as_str());
    }
    if let Some(var_161) = &input.origin_manifest_type {
        object.key("OriginManifestType").string(var_161.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_live_pre_roll_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LivePreRollConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_162) = &input.ad_decision_server_url {
        object.key("AdDecisionServerUrl").string(var_162.as_str());
    }
    if input.max_duration_seconds != 0 {
        object.key("MaxDurationSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_duration_seconds).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_manifest_processing_rules(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ManifestProcessingRules,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_163) = &input.ad_marker_passthrough {
        #[allow(unused_mut)]
        let mut object_164 = object.key("AdMarkerPassthrough").start_object();
        crate::json_ser::serialize_structure_crate_model_ad_marker_passthrough(
            &mut object_164,
            var_163,
        )?;
        object_164.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dash_playlist_settings(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DashPlaylistSettings,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.manifest_window_seconds != 0 {
        object.key("ManifestWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.manifest_window_seconds).into()),
        );
    }
    if input.min_buffer_time_seconds != 0 {
        object.key("MinBufferTimeSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.min_buffer_time_seconds).into()),
        );
    }
    if input.min_update_period_seconds != 0 {
        object.key("MinUpdatePeriodSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.min_update_period_seconds).into()),
        );
    }
    if input.suggested_presentation_delay_seconds != 0 {
        object.key("SuggestedPresentationDelaySeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.suggested_presentation_delay_seconds).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_hls_playlist_settings(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::HlsPlaylistSettings,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.manifest_window_seconds != 0 {
        object.key("ManifestWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.manifest_window_seconds).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_avail_matching_criteria(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AvailMatchingCriteria,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_165) = &input.dynamic_variable {
        object.key("DynamicVariable").string(var_165.as_str());
    }
    if let Some(var_166) = &input.operator {
        object.key("Operator").string(var_166.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_splice_insert_message(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SpliceInsertMessage,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.avail_num != 0 {
        object.key("AvailNum").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.avail_num).into()),
        );
    }
    if input.avails_expected != 0 {
        object.key("AvailsExpected").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.avails_expected).into()),
        );
    }
    if input.splice_event_id != 0 {
        object.key("SpliceEventId").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.splice_event_id).into()),
        );
    }
    if input.unique_program_id != 0 {
        object.key("UniqueProgramId").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.unique_program_id).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_transition(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Transition,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.duration_millis != 0 {
        object.key("DurationMillis").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.duration_millis).into()),
        );
    }
    if let Some(var_167) = &input.relative_position {
        object.key("RelativePosition").string(var_167.as_str());
    }
    if let Some(var_168) = &input.relative_program {
        object.key("RelativeProgram").string(var_168.as_str());
    }
    if input.scheduled_start_time_millis != 0 {
        object.key("ScheduledStartTimeMillis").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.scheduled_start_time_millis).into()),
        );
    }
    if let Some(var_169) = &input.r#type {
        object.key("Type").string(var_169.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_secrets_manager_access_token_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SecretsManagerAccessTokenConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_170) = &input.header_name {
        object.key("HeaderName").string(var_170.as_str());
    }
    if let Some(var_171) = &input.secret_arn {
        object.key("SecretArn").string(var_171.as_str());
    }
    if let Some(var_172) = &input.secret_string_key {
        object.key("SecretStringKey").string(var_172.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ad_marker_passthrough(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AdMarkerPassthrough,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.enabled {
        object.key("Enabled").boolean(input.enabled);
    }
    Ok(())
}
