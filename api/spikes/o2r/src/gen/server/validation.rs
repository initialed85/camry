//! Offline request-schema validators and normalized public rejections.
#![allow(dead_code)]
use super::errors::{InvalidParameter, ProblemDetails, RequestValidationRejection};
const VALIDATION_SCHEMA: &str = "{\"$defs\":{\"components\":{\"component_ArrayOfNullableAudioEvent\":{\"items\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_AudioEvent\",\"nullable\":true},\"nullable\":true,\"type\":\"array\"},\"component_ArrayOfNullableCamera\":{\"items\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Camera\",\"nullable\":true},\"nullable\":true,\"type\":\"array\"},\"component_ArrayOfNullableDetection\":{\"items\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Detection\",\"nullable\":true},\"nullable\":true,\"type\":\"array\"},\"component_ArrayOfNullableTrack\":{\"items\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Track\",\"nullable\":true},\"nullable\":true,\"type\":\"array\"},\"component_ArrayOfNullableVideo\":{\"items\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Video\",\"nullable\":true},\"nullable\":true,\"type\":\"array\"},\"component_AudioEvent\":{\"properties\":{\"camera_id\":{\"format\":\"uuid\",\"type\":\"string\"},\"camera_id_object\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Camera\",\"nullable\":true},\"created_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"crest_db\":{\"format\":\"double\",\"type\":\"number\"},\"deleted_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"duration_ms\":{\"format\":\"int64\",\"type\":\"integer\"},\"ended_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"floor_dbfs\":{\"format\":\"double\",\"type\":\"number\"},\"id\":{\"format\":\"uuid\",\"type\":\"string\"},\"kind\":{\"type\":\"string\"},\"offset_ms\":{\"format\":\"int64\",\"type\":\"integer\"},\"peak_dbfs\":{\"format\":\"double\",\"type\":\"number\"},\"prominence_db\":{\"format\":\"double\",\"type\":\"number\"},\"rms_dbfs\":{\"format\":\"double\",\"type\":\"number\"},\"score\":{\"format\":\"double\",\"type\":\"number\"},\"seen_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"spectral\":{\"nullable\":true},\"updated_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"video_id\":{\"format\":\"uuid\",\"type\":\"string\"},\"video_id_object\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Video\",\"nullable\":true}},\"type\":\"object\"},\"component_Camera\":{\"properties\":{\"created_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"deleted_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"id\":{\"format\":\"uuid\",\"type\":\"string\"},\"last_seen\":{\"format\":\"date-time\",\"type\":\"string\"},\"low_res_stream_url\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"overlay_offset_ms\":{\"format\":\"int64\",\"type\":\"integer\"},\"referenced_by_audio_event_camera_id_objects\":{\"items\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_AudioEvent\",\"nullable\":true},\"nullable\":true,\"type\":\"array\"},\"referenced_by_detection_camera_id_objects\":{\"items\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Detection\",\"nullable\":true},\"nullable\":true,\"type\":\"array\"},\"referenced_by_track_camera_id_objects\":{\"items\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Track\",\"nullable\":true},\"nullable\":true,\"type\":\"array\"},\"referenced_by_video_camera_id_objects\":{\"items\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Video\",\"nullable\":true},\"nullable\":true,\"type\":\"array\"},\"segment_producer_claimed_until\":{\"format\":\"date-time\",\"type\":\"string\"},\"stream_producer_claimed_until\":{\"format\":\"date-time\",\"type\":\"string\"},\"stream_url\":{\"type\":\"string\"},\"updated_at\":{\"format\":\"date-time\",\"type\":\"string\"}},\"type\":\"object\"},\"component_CameraSegmentProducerClaimRequest\":{\"properties\":{\"timeout_seconds\":{\"format\":\"double\",\"type\":\"number\"},\"until\":{\"format\":\"date-time\",\"type\":\"string\"}},\"type\":\"object\"},\"component_CameraStreamProducerClaimRequest\":{\"properties\":{\"timeout_seconds\":{\"format\":\"double\",\"type\":\"number\"},\"until\":{\"format\":\"date-time\",\"type\":\"string\"}},\"type\":\"object\"},\"component_Detection\":{\"properties\":{\"bounding_box\":{\"items\":{\"properties\":{\"X\":{\"format\":\"double\",\"type\":\"number\"},\"Y\":{\"format\":\"double\",\"type\":\"number\"}},\"type\":\"object\"},\"nullable\":true,\"type\":\"array\"},\"camera_id\":{\"format\":\"uuid\",\"type\":\"string\"},\"camera_id_object\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Camera\",\"nullable\":true},\"centroid\":{\"properties\":{\"X\":{\"format\":\"double\",\"type\":\"number\"},\"Y\":{\"format\":\"double\",\"type\":\"number\"}},\"type\":\"object\"},\"class_id\":{\"format\":\"int64\",\"type\":\"integer\"},\"class_name\":{\"type\":\"string\"},\"created_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"deleted_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"id\":{\"format\":\"uuid\",\"type\":\"string\"},\"score\":{\"format\":\"double\",\"type\":\"number\"},\"seen_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"track_id\":{\"format\":\"uuid\",\"type\":\"string\"},\"track_id_object\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Track\",\"nullable\":true},\"updated_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"video_id\":{\"format\":\"uuid\",\"type\":\"string\"},\"video_id_object\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Video\",\"nullable\":true}},\"type\":\"object\"},\"component_Track\":{\"properties\":{\"bounding_box\":{\"items\":{\"properties\":{\"X\":{\"format\":\"double\",\"type\":\"number\"},\"Y\":{\"format\":\"double\",\"type\":\"number\"}},\"type\":\"object\"},\"nullable\":true,\"type\":\"array\"},\"camera_id\":{\"format\":\"uuid\",\"type\":\"string\"},\"camera_id_object\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Camera\",\"nullable\":true},\"centroid\":{\"properties\":{\"X\":{\"format\":\"double\",\"type\":\"number\"},\"Y\":{\"format\":\"double\",\"type\":\"number\"}},\"type\":\"object\"},\"class_id\":{\"format\":\"int64\",\"type\":\"integer\"},\"class_name\":{\"type\":\"string\"},\"created_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"deleted_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"ended_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"frame_count\":{\"format\":\"int64\",\"type\":\"integer\"},\"id\":{\"format\":\"uuid\",\"type\":\"string\"},\"is_static\":{\"type\":\"boolean\"},\"key\":{\"format\":\"uuid\",\"type\":\"string\"},\"max_score\":{\"format\":\"double\",\"type\":\"number\"},\"mean_score\":{\"format\":\"double\",\"type\":\"number\"},\"referenced_by_detection_track_id_objects\":{\"items\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Detection\",\"nullable\":true},\"nullable\":true,\"type\":\"array\"},\"started_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"status\":{\"type\":\"string\"},\"trajectory\":{\"nullable\":true},\"updated_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"video_id\":{\"format\":\"uuid\",\"type\":\"string\"},\"video_id_object\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Video\",\"nullable\":true}},\"type\":\"object\"},\"component_Video\":{\"properties\":{\"audio_detected_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"audio_detector_claimed_until\":{\"format\":\"date-time\",\"type\":\"string\"},\"audio_profile\":{\"nullable\":true},\"audio_summary\":{\"nullable\":true},\"camera_id\":{\"format\":\"uuid\",\"type\":\"string\"},\"camera_id_object\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Camera\",\"nullable\":true},\"created_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"deleted_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"detection_summary\":{\"nullable\":true},\"duration\":{\"format\":\"int64\",\"type\":\"integer\"},\"ended_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"file_name\":{\"type\":\"string\"},\"file_size\":{\"format\":\"double\",\"type\":\"number\"},\"id\":{\"format\":\"uuid\",\"type\":\"string\"},\"is_low_res\":{\"type\":\"boolean\"},\"object_detector_claimed_until\":{\"format\":\"date-time\",\"type\":\"string\"},\"object_tracker_claimed_until\":{\"format\":\"date-time\",\"type\":\"string\"},\"overlay_offset_ms\":{\"format\":\"int64\",\"type\":\"integer\"},\"referenced_by_audio_event_video_id_objects\":{\"items\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_AudioEvent\",\"nullable\":true},\"nullable\":true,\"type\":\"array\"},\"referenced_by_detection_video_id_objects\":{\"items\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Detection\",\"nullable\":true},\"nullable\":true,\"type\":\"array\"},\"referenced_by_track_video_id_objects\":{\"items\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Track\",\"nullable\":true},\"nullable\":true,\"type\":\"array\"},\"started_at\":{\"format\":\"date-time\",\"type\":\"string\"},\"status\":{\"type\":\"string\"},\"thumbnail_name\":{\"type\":\"string\"},\"track_summary\":{\"nullable\":true},\"updated_at\":{\"format\":\"date-time\",\"type\":\"string\"}},\"type\":\"object\"},\"component_VideoAudioDetectorClaimRequest\":{\"properties\":{\"timeout_seconds\":{\"format\":\"double\",\"type\":\"number\"},\"until\":{\"format\":\"date-time\",\"type\":\"string\"}},\"type\":\"object\"},\"component_VideoObjectDetectorClaimRequest\":{\"properties\":{\"timeout_seconds\":{\"format\":\"double\",\"type\":\"number\"},\"until\":{\"format\":\"date-time\",\"type\":\"string\"}},\"type\":\"object\"},\"component_VideoObjectTrackerClaimRequest\":{\"properties\":{\"timeout_seconds\":{\"format\":\"double\",\"type\":\"number\"},\"until\":{\"format\":\"date-time\",\"type\":\"string\"}},\"type\":\"object\"}},\"targets\":{\"v0\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_VideoAudioDetectorClaimRequest\"},\"v1\":{\"format\":\"int32\",\"type\":\"integer\"},\"v10\":{\"format\":\"date-time\",\"type\":\"string\"},\"v100\":{\"nullable\":true,\"type\":\"string\"},\"v1000\":{\"type\":\"string\"},\"v1001\":{\"nullable\":true,\"type\":\"integer\"},\"v1002\":{\"nullable\":true,\"type\":\"integer\"},\"v1003\":{\"nullable\":true,\"type\":\"integer\"},\"v1004\":{\"nullable\":true,\"type\":\"integer\"},\"v1005\":{\"nullable\":true,\"type\":\"integer\"},\"v1006\":{\"nullable\":true,\"type\":\"integer\"},\"v1007\":{\"nullable\":true,\"type\":\"integer\"},\"v1008\":{\"nullable\":true,\"type\":\"integer\"},\"v1009\":{\"type\":\"string\"},\"v101\":{\"nullable\":true,\"type\":\"string\"},\"v1010\":{\"type\":\"string\"},\"v1011\":{\"type\":\"string\"},\"v1012\":{\"type\":\"string\"},\"v1013\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1014\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1015\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1016\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1017\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1018\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1019\":{\"format\":\"date-time\",\"type\":\"string\"},\"v102\":{\"nullable\":true,\"type\":\"string\"},\"v1020\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1021\":{\"type\":\"string\"},\"v1022\":{\"type\":\"string\"},\"v1023\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1024\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1025\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1026\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1027\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1028\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1029\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v103\":{\"nullable\":true,\"type\":\"string\"},\"v1030\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1031\":{\"type\":\"string\"},\"v1032\":{\"type\":\"string\"},\"v1033\":{\"type\":\"string\"},\"v1034\":{\"type\":\"string\"},\"v1035\":{\"nullable\":true},\"v1036\":{\"nullable\":true},\"v1037\":{\"type\":\"string\"},\"v1038\":{\"type\":\"string\"},\"v1039\":{\"nullable\":true},\"v104\":{\"type\":\"string\"},\"v1040\":{\"nullable\":true},\"v1041\":{\"type\":\"string\"},\"v1042\":{\"type\":\"string\"},\"v1043\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_CameraSegmentProducerClaimRequest\"},\"v1044\":{\"format\":\"int32\",\"type\":\"integer\"},\"v1045\":{\"format\":\"int32\",\"type\":\"integer\"},\"v1046\":{\"type\":\"integer\"},\"v1047\":{\"type\":\"string\"},\"v1048\":{\"type\":\"string\"},\"v1049\":{\"type\":\"string\"},\"v105\":{\"type\":\"string\"},\"v1050\":{\"type\":\"string\"},\"v1051\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1052\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1053\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1054\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1055\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1056\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1057\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1058\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1059\":{\"type\":\"string\"},\"v106\":{\"type\":\"string\"},\"v1060\":{\"type\":\"string\"},\"v1061\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1062\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1063\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1064\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1065\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1066\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1067\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1068\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1069\":{\"type\":\"string\"},\"v107\":{\"type\":\"string\"},\"v1070\":{\"type\":\"string\"},\"v1071\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1072\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1073\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1074\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1075\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1076\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1077\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1078\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1079\":{\"type\":\"string\"},\"v108\":{\"nullable\":true,\"type\":\"string\"},\"v1080\":{\"type\":\"string\"},\"v1081\":{\"type\":\"string\"},\"v1082\":{\"type\":\"string\"},\"v1083\":{\"type\":\"string\"},\"v1084\":{\"type\":\"string\"},\"v1085\":{\"type\":\"string\"},\"v1086\":{\"type\":\"string\"},\"v1087\":{\"type\":\"string\"},\"v1088\":{\"type\":\"string\"},\"v1089\":{\"type\":\"string\"},\"v109\":{\"nullable\":true,\"type\":\"string\"},\"v1090\":{\"type\":\"string\"},\"v1091\":{\"type\":\"string\"},\"v1092\":{\"type\":\"string\"},\"v1093\":{\"type\":\"string\"},\"v1094\":{\"type\":\"string\"},\"v1095\":{\"type\":\"string\"},\"v1096\":{\"type\":\"string\"},\"v1097\":{\"type\":\"string\"},\"v1098\":{\"type\":\"string\"},\"v1099\":{\"type\":\"string\"},\"v11\":{\"format\":\"date-time\",\"type\":\"string\"},\"v110\":{\"nullable\":true,\"type\":\"string\"},\"v1100\":{\"type\":\"string\"},\"v1101\":{\"type\":\"string\"},\"v1102\":{\"type\":\"string\"},\"v1103\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1104\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1105\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1106\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1107\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1108\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1109\":{\"format\":\"date-time\",\"type\":\"string\"},\"v111\":{\"nullable\":true,\"type\":\"string\"},\"v1110\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1111\":{\"type\":\"string\"},\"v1112\":{\"type\":\"string\"},\"v1113\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1114\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1115\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1116\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1117\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1118\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1119\":{\"format\":\"date-time\",\"type\":\"string\"},\"v112\":{\"nullable\":true,\"type\":\"string\"},\"v1120\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1121\":{\"type\":\"string\"},\"v1122\":{\"type\":\"string\"},\"v1123\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1124\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1125\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1126\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1127\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1128\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1129\":{\"format\":\"date-time\",\"type\":\"string\"},\"v113\":{\"nullable\":true,\"type\":\"string\"},\"v1130\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1131\":{\"type\":\"string\"},\"v1132\":{\"type\":\"string\"},\"v1133\":{\"nullable\":true,\"type\":\"string\"},\"v1134\":{\"nullable\":true,\"type\":\"string\"},\"v1135\":{\"nullable\":true,\"type\":\"string\"},\"v1136\":{\"nullable\":true,\"type\":\"string\"},\"v1137\":{\"nullable\":true,\"type\":\"string\"},\"v1138\":{\"nullable\":true,\"type\":\"string\"},\"v1139\":{\"nullable\":true,\"type\":\"string\"},\"v114\":{\"nullable\":true,\"type\":\"string\"},\"v1140\":{\"nullable\":true,\"type\":\"string\"},\"v1141\":{\"type\":\"string\"},\"v1142\":{\"type\":\"string\"},\"v1143\":{\"type\":\"string\"},\"v1144\":{\"type\":\"string\"},\"v1145\":{\"type\":\"integer\"},\"v1146\":{\"type\":\"integer\"},\"v1147\":{\"type\":\"integer\"},\"v1148\":{\"type\":\"integer\"},\"v1149\":{\"type\":\"integer\"},\"v115\":{\"nullable\":true,\"type\":\"string\"},\"v1150\":{\"type\":\"integer\"},\"v1151\":{\"type\":\"integer\"},\"v1152\":{\"type\":\"integer\"},\"v1153\":{\"type\":\"string\"},\"v1154\":{\"type\":\"string\"},\"v1155\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_CameraStreamProducerClaimRequest\"},\"v1156\":{\"format\":\"int32\",\"type\":\"integer\"},\"v1157\":{\"format\":\"int32\",\"type\":\"integer\"},\"v1158\":{\"type\":\"integer\"},\"v1159\":{\"type\":\"string\"},\"v116\":{\"type\":\"string\"},\"v1160\":{\"type\":\"string\"},\"v1161\":{\"type\":\"string\"},\"v1162\":{\"type\":\"string\"},\"v1163\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1164\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1165\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1166\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1167\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1168\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1169\":{\"format\":\"date-time\",\"type\":\"string\"},\"v117\":{\"type\":\"string\"},\"v1170\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1171\":{\"type\":\"string\"},\"v1172\":{\"type\":\"string\"},\"v1173\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1174\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1175\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1176\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1177\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1178\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1179\":{\"format\":\"date-time\",\"type\":\"string\"},\"v118\":{\"type\":\"string\"},\"v1180\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1181\":{\"type\":\"string\"},\"v1182\":{\"type\":\"string\"},\"v1183\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1184\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1185\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1186\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1187\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1188\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1189\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v119\":{\"type\":\"string\"},\"v1190\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1191\":{\"type\":\"string\"},\"v1192\":{\"type\":\"string\"},\"v1193\":{\"type\":\"string\"},\"v1194\":{\"type\":\"string\"},\"v1195\":{\"type\":\"string\"},\"v1196\":{\"type\":\"string\"},\"v1197\":{\"type\":\"string\"},\"v1198\":{\"type\":\"string\"},\"v1199\":{\"type\":\"string\"},\"v12\":{\"format\":\"date-time\",\"type\":\"string\"},\"v120\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1200\":{\"type\":\"string\"},\"v1201\":{\"type\":\"string\"},\"v1202\":{\"type\":\"string\"},\"v1203\":{\"type\":\"string\"},\"v1204\":{\"type\":\"string\"},\"v1205\":{\"type\":\"string\"},\"v1206\":{\"type\":\"string\"},\"v1207\":{\"type\":\"string\"},\"v1208\":{\"type\":\"string\"},\"v1209\":{\"type\":\"string\"},\"v121\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1210\":{\"type\":\"string\"},\"v1211\":{\"type\":\"string\"},\"v1212\":{\"type\":\"string\"},\"v1213\":{\"type\":\"string\"},\"v1214\":{\"type\":\"string\"},\"v1215\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1216\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1217\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1218\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1219\":{\"format\":\"date-time\",\"type\":\"string\"},\"v122\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1220\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1221\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1222\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1223\":{\"type\":\"string\"},\"v1224\":{\"type\":\"string\"},\"v1225\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1226\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1227\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1228\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1229\":{\"format\":\"date-time\",\"type\":\"string\"},\"v123\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1230\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1231\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1232\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1233\":{\"type\":\"string\"},\"v1234\":{\"type\":\"string\"},\"v1235\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1236\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1237\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1238\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1239\":{\"format\":\"date-time\",\"type\":\"string\"},\"v124\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1240\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1241\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1242\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1243\":{\"type\":\"string\"},\"v1244\":{\"type\":\"string\"},\"v1245\":{\"nullable\":true,\"type\":\"string\"},\"v1246\":{\"nullable\":true,\"type\":\"string\"},\"v1247\":{\"nullable\":true,\"type\":\"string\"},\"v1248\":{\"nullable\":true,\"type\":\"string\"},\"v1249\":{\"nullable\":true,\"type\":\"string\"},\"v125\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1250\":{\"nullable\":true,\"type\":\"string\"},\"v1251\":{\"nullable\":true,\"type\":\"string\"},\"v1252\":{\"nullable\":true,\"type\":\"string\"},\"v1253\":{\"type\":\"string\"},\"v1254\":{\"type\":\"string\"},\"v1255\":{\"type\":\"string\"},\"v1256\":{\"type\":\"string\"},\"v1257\":{\"type\":\"integer\"},\"v1258\":{\"type\":\"integer\"},\"v1259\":{\"type\":\"integer\"},\"v126\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1260\":{\"type\":\"integer\"},\"v1261\":{\"type\":\"integer\"},\"v1262\":{\"type\":\"integer\"},\"v1263\":{\"type\":\"integer\"},\"v1264\":{\"type\":\"integer\"},\"v1265\":{\"type\":\"string\"},\"v1266\":{\"type\":\"string\"},\"v1267\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1268\":{\"format\":\"int64\",\"type\":\"integer\"},\"v1269\":{\"format\":\"uuid\",\"type\":\"string\"},\"v127\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1270\":{\"format\":\"int64\",\"type\":\"integer\"},\"v1271\":{\"format\":\"int32\",\"type\":\"integer\"},\"v1272\":{\"format\":\"int32\",\"type\":\"integer\"},\"v1273\":{\"type\":\"integer\"},\"v1274\":{\"type\":\"string\"},\"v1275\":{\"type\":\"string\"},\"v1276\":{\"type\":\"string\"},\"v1277\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1278\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1279\":{\"format\":\"uuid\",\"type\":\"string\"},\"v128\":{\"type\":\"string\"},\"v1280\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1281\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1282\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1283\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1284\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1285\":{\"type\":\"string\"},\"v1286\":{\"type\":\"string\"},\"v1287\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1288\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1289\":{\"format\":\"date-time\",\"type\":\"string\"},\"v129\":{\"type\":\"string\"},\"v1290\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1291\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1292\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1293\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1294\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1295\":{\"type\":\"string\"},\"v1296\":{\"type\":\"string\"},\"v1297\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1298\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1299\":{\"format\":\"date-time\",\"type\":\"string\"},\"v13\":{\"format\":\"date-time\",\"type\":\"string\"},\"v130\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1300\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1301\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1302\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1303\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1304\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1305\":{\"type\":\"string\"},\"v1306\":{\"type\":\"string\"},\"v1307\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1308\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1309\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v131\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1310\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1311\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1312\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1313\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1314\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1315\":{\"type\":\"string\"},\"v1316\":{\"type\":\"string\"},\"v1317\":{\"type\":\"string\"},\"v1318\":{\"type\":\"string\"},\"v1319\":{\"format\":\"uuid\",\"type\":\"string\"},\"v132\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1320\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1321\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1322\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1323\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1324\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1325\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1326\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1327\":{\"type\":\"string\"},\"v1328\":{\"type\":\"string\"},\"v1329\":{\"format\":\"uuid\",\"type\":\"string\"},\"v133\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1330\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1331\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1332\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1333\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1334\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1335\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1336\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1337\":{\"type\":\"string\"},\"v1338\":{\"type\":\"string\"},\"v1339\":{\"type\":\"integer\"},\"v134\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1340\":{\"type\":\"integer\"},\"v1341\":{\"type\":\"integer\"},\"v1342\":{\"type\":\"integer\"},\"v1343\":{\"type\":\"integer\"},\"v1344\":{\"type\":\"integer\"},\"v1345\":{\"type\":\"integer\"},\"v1346\":{\"type\":\"integer\"},\"v1347\":{\"type\":\"string\"},\"v1348\":{\"type\":\"string\"},\"v1349\":{\"type\":\"string\"},\"v135\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1350\":{\"type\":\"string\"},\"v1351\":{\"type\":\"string\"},\"v1352\":{\"type\":\"string\"},\"v1353\":{\"type\":\"string\"},\"v1354\":{\"type\":\"string\"},\"v1355\":{\"type\":\"string\"},\"v1356\":{\"type\":\"string\"},\"v1357\":{\"type\":\"string\"},\"v1358\":{\"type\":\"string\"},\"v1359\":{\"format\":\"date-time\",\"type\":\"string\"},\"v136\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1360\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1361\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1362\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1363\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1364\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1365\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1366\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1367\":{\"type\":\"string\"},\"v1368\":{\"type\":\"string\"},\"v1369\":{\"format\":\"date-time\",\"type\":\"string\"},\"v137\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1370\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1371\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1372\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1373\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1374\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1375\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1376\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1377\":{\"type\":\"string\"},\"v1378\":{\"type\":\"string\"},\"v1379\":{\"type\":\"integer\"},\"v138\":{\"type\":\"string\"},\"v1380\":{\"type\":\"integer\"},\"v1381\":{\"type\":\"integer\"},\"v1382\":{\"type\":\"integer\"},\"v1383\":{\"type\":\"integer\"},\"v1384\":{\"type\":\"integer\"},\"v1385\":{\"type\":\"integer\"},\"v1386\":{\"type\":\"integer\"},\"v1387\":{\"type\":\"string\"},\"v1388\":{\"type\":\"string\"},\"v1389\":{\"type\":\"number\"},\"v139\":{\"type\":\"string\"},\"v1390\":{\"type\":\"number\"},\"v1391\":{\"type\":\"number\"},\"v1392\":{\"type\":\"number\"},\"v1393\":{\"type\":\"number\"},\"v1394\":{\"type\":\"number\"},\"v1395\":{\"type\":\"number\"},\"v1396\":{\"type\":\"number\"},\"v1397\":{\"type\":\"string\"},\"v1398\":{\"type\":\"string\"},\"v1399\":{\"type\":\"number\"},\"v14\":{\"format\":\"date-time\",\"type\":\"string\"},\"v140\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1400\":{\"type\":\"number\"},\"v1401\":{\"type\":\"number\"},\"v1402\":{\"type\":\"number\"},\"v1403\":{\"type\":\"number\"},\"v1404\":{\"type\":\"number\"},\"v1405\":{\"type\":\"number\"},\"v1406\":{\"type\":\"number\"},\"v1407\":{\"type\":\"string\"},\"v1408\":{\"type\":\"string\"},\"v1409\":{\"nullable\":true},\"v141\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1410\":{\"nullable\":true},\"v1411\":{\"type\":\"string\"},\"v1412\":{\"type\":\"string\"},\"v1413\":{\"type\":\"boolean\"},\"v1414\":{\"type\":\"boolean\"},\"v1415\":{\"type\":\"string\"},\"v1416\":{\"type\":\"string\"},\"v1417\":{\"type\":\"string\"},\"v1418\":{\"type\":\"string\"},\"v1419\":{\"type\":\"string\"},\"v142\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1420\":{\"type\":\"string\"},\"v1421\":{\"type\":\"string\"},\"v1422\":{\"type\":\"string\"},\"v1423\":{\"type\":\"string\"},\"v1424\":{\"type\":\"string\"},\"v1425\":{\"type\":\"string\"},\"v1426\":{\"type\":\"string\"},\"v1427\":{\"type\":\"string\"},\"v1428\":{\"type\":\"string\"},\"v1429\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Track\"},\"v143\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1430\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1431\":{\"format\":\"int64\",\"type\":\"integer\"},\"v1432\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_ArrayOfNullableTrack\"},\"v1433\":{\"format\":\"int64\",\"type\":\"integer\"},\"v1434\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1435\":{\"format\":\"int64\",\"type\":\"integer\"},\"v1436\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1437\":{\"format\":\"int64\",\"type\":\"integer\"},\"v1438\":{\"format\":\"int32\",\"type\":\"integer\"},\"v1439\":{\"format\":\"int32\",\"type\":\"integer\"},\"v144\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1440\":{\"type\":\"integer\"},\"v1441\":{\"type\":\"string\"},\"v1442\":{\"type\":\"string\"},\"v1443\":{\"type\":\"string\"},\"v1444\":{\"type\":\"string\"},\"v1445\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1446\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1447\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1448\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1449\":{\"format\":\"date-time\",\"type\":\"string\"},\"v145\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1450\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1451\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1452\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1453\":{\"type\":\"string\"},\"v1454\":{\"type\":\"string\"},\"v1455\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1456\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1457\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1458\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1459\":{\"format\":\"date-time\",\"type\":\"string\"},\"v146\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1460\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1461\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1462\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1463\":{\"type\":\"string\"},\"v1464\":{\"type\":\"string\"},\"v1465\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1466\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1467\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1468\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1469\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v147\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1470\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1471\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1472\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1473\":{\"type\":\"string\"},\"v1474\":{\"type\":\"string\"},\"v1475\":{\"type\":\"string\"},\"v1476\":{\"type\":\"string\"},\"v1477\":{\"type\":\"string\"},\"v1478\":{\"type\":\"string\"},\"v1479\":{\"type\":\"string\"},\"v148\":{\"type\":\"string\"},\"v1480\":{\"type\":\"string\"},\"v1481\":{\"type\":\"string\"},\"v1482\":{\"type\":\"string\"},\"v1483\":{\"type\":\"string\"},\"v1484\":{\"type\":\"string\"},\"v1485\":{\"type\":\"string\"},\"v1486\":{\"type\":\"string\"},\"v1487\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1488\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1489\":{\"format\":\"date-time\",\"type\":\"string\"},\"v149\":{\"type\":\"string\"},\"v1490\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1491\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1492\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1493\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1494\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1495\":{\"type\":\"string\"},\"v1496\":{\"type\":\"string\"},\"v1497\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1498\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1499\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v15\":{\"format\":\"date-time\",\"type\":\"string\"},\"v150\":{\"nullable\":true},\"v1500\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1501\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1502\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1503\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1504\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1505\":{\"type\":\"string\"},\"v1506\":{\"type\":\"string\"},\"v1507\":{\"type\":\"string\"},\"v1508\":{\"type\":\"string\"},\"v1509\":{\"nullable\":true,\"type\":\"string\"},\"v151\":{\"nullable\":true},\"v1510\":{\"nullable\":true,\"type\":\"string\"},\"v1511\":{\"nullable\":true,\"type\":\"string\"},\"v1512\":{\"nullable\":true,\"type\":\"string\"},\"v1513\":{\"nullable\":true,\"type\":\"string\"},\"v1514\":{\"nullable\":true,\"type\":\"string\"},\"v1515\":{\"nullable\":true,\"type\":\"string\"},\"v1516\":{\"nullable\":true,\"type\":\"string\"},\"v1517\":{\"type\":\"string\"},\"v1518\":{\"type\":\"string\"},\"v1519\":{\"type\":\"string\"},\"v152\":{\"type\":\"string\"},\"v1520\":{\"type\":\"string\"},\"v1521\":{\"nullable\":true,\"type\":\"number\"},\"v1522\":{\"nullable\":true,\"type\":\"number\"},\"v1523\":{\"nullable\":true,\"type\":\"number\"},\"v1524\":{\"nullable\":true,\"type\":\"number\"},\"v1525\":{\"nullable\":true,\"type\":\"number\"},\"v1526\":{\"nullable\":true,\"type\":\"number\"},\"v1527\":{\"nullable\":true,\"type\":\"number\"},\"v1528\":{\"nullable\":true,\"type\":\"number\"},\"v1529\":{\"type\":\"string\"},\"v153\":{\"type\":\"string\"},\"v1530\":{\"type\":\"string\"},\"v1531\":{\"type\":\"string\"},\"v1532\":{\"type\":\"string\"},\"v1533\":{\"nullable\":true,\"type\":\"string\"},\"v1534\":{\"nullable\":true,\"type\":\"string\"},\"v1535\":{\"nullable\":true,\"type\":\"string\"},\"v1536\":{\"nullable\":true,\"type\":\"string\"},\"v1537\":{\"nullable\":true,\"type\":\"string\"},\"v1538\":{\"nullable\":true,\"type\":\"string\"},\"v1539\":{\"nullable\":true,\"type\":\"string\"},\"v154\":{\"type\":\"boolean\"},\"v1540\":{\"nullable\":true,\"type\":\"string\"},\"v1541\":{\"type\":\"string\"},\"v1542\":{\"type\":\"string\"},\"v1543\":{\"type\":\"string\"},\"v1544\":{\"type\":\"string\"},\"v1545\":{\"nullable\":true,\"type\":\"string\"},\"v1546\":{\"nullable\":true,\"type\":\"string\"},\"v1547\":{\"nullable\":true,\"type\":\"string\"},\"v1548\":{\"nullable\":true,\"type\":\"string\"},\"v1549\":{\"nullable\":true,\"type\":\"string\"},\"v155\":{\"type\":\"boolean\"},\"v1550\":{\"nullable\":true,\"type\":\"string\"},\"v1551\":{\"nullable\":true,\"type\":\"string\"},\"v1552\":{\"nullable\":true,\"type\":\"string\"},\"v1553\":{\"type\":\"string\"},\"v1554\":{\"type\":\"string\"},\"v1555\":{\"type\":\"string\"},\"v1556\":{\"type\":\"string\"},\"v1557\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1558\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1559\":{\"format\":\"date-time\",\"type\":\"string\"},\"v156\":{\"type\":\"string\"},\"v1560\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1561\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1562\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1563\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1564\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1565\":{\"type\":\"string\"},\"v1566\":{\"type\":\"string\"},\"v1567\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1568\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1569\":{\"format\":\"date-time\",\"type\":\"string\"},\"v157\":{\"type\":\"string\"},\"v1570\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1571\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1572\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1573\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1574\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1575\":{\"type\":\"string\"},\"v1576\":{\"type\":\"string\"},\"v1577\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1578\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1579\":{\"format\":\"uuid\",\"type\":\"string\"},\"v158\":{\"type\":\"string\"},\"v1580\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1581\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1582\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1583\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1584\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1585\":{\"type\":\"string\"},\"v1586\":{\"type\":\"string\"},\"v1587\":{\"nullable\":true},\"v1588\":{\"nullable\":true},\"v1589\":{\"type\":\"string\"},\"v159\":{\"type\":\"string\"},\"v1590\":{\"type\":\"string\"},\"v1591\":{\"type\":\"boolean\"},\"v1592\":{\"type\":\"boolean\"},\"v1593\":{\"type\":\"string\"},\"v1594\":{\"type\":\"string\"},\"v1595\":{\"type\":\"string\"},\"v1596\":{\"type\":\"string\"},\"v1597\":{\"nullable\":true},\"v1598\":{\"nullable\":true},\"v1599\":{\"type\":\"string\"},\"v16\":{\"type\":\"string\"},\"v160\":{\"nullable\":true},\"v1600\":{\"type\":\"string\"},\"v1601\":{\"nullable\":true,\"type\":\"integer\"},\"v1602\":{\"nullable\":true,\"type\":\"integer\"},\"v1603\":{\"nullable\":true,\"type\":\"integer\"},\"v1604\":{\"nullable\":true,\"type\":\"integer\"},\"v1605\":{\"nullable\":true,\"type\":\"integer\"},\"v1606\":{\"nullable\":true,\"type\":\"integer\"},\"v1607\":{\"nullable\":true,\"type\":\"integer\"},\"v1608\":{\"nullable\":true,\"type\":\"integer\"},\"v1609\":{\"type\":\"string\"},\"v161\":{\"nullable\":true},\"v1610\":{\"type\":\"string\"},\"v1611\":{\"type\":\"string\"},\"v1612\":{\"type\":\"string\"},\"v1613\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1614\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1615\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1616\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1617\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1618\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1619\":{\"format\":\"date-time\",\"type\":\"string\"},\"v162\":{\"type\":\"string\"},\"v1620\":{\"format\":\"date-time\",\"type\":\"string\"},\"v1621\":{\"type\":\"string\"},\"v1622\":{\"type\":\"string\"},\"v1623\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1624\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1625\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1626\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1627\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1628\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1629\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v163\":{\"type\":\"string\"},\"v1630\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v1631\":{\"type\":\"string\"},\"v1632\":{\"type\":\"string\"},\"v1633\":{\"type\":\"string\"},\"v1634\":{\"type\":\"string\"},\"v1635\":{\"nullable\":true},\"v1636\":{\"nullable\":true},\"v1637\":{\"type\":\"string\"},\"v1638\":{\"type\":\"string\"},\"v1639\":{\"nullable\":true},\"v164\":{\"nullable\":true,\"type\":\"integer\"},\"v1640\":{\"nullable\":true},\"v1641\":{\"type\":\"string\"},\"v1642\":{\"type\":\"string\"},\"v1643\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Video\"},\"v1644\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1645\":{\"format\":\"int64\",\"type\":\"integer\"},\"v1646\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_ArrayOfNullableVideo\"},\"v1647\":{\"format\":\"int64\",\"type\":\"integer\"},\"v1648\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_VideoAudioDetectorClaimRequest\"},\"v1649\":{\"format\":\"uuid\",\"type\":\"string\"},\"v165\":{\"nullable\":true,\"type\":\"integer\"},\"v1650\":{\"format\":\"int64\",\"type\":\"integer\"},\"v1651\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_VideoObjectDetectorClaimRequest\"},\"v1652\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1653\":{\"format\":\"int64\",\"type\":\"integer\"},\"v1654\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_VideoObjectTrackerClaimRequest\"},\"v1655\":{\"format\":\"uuid\",\"type\":\"string\"},\"v1656\":{\"format\":\"int64\",\"type\":\"integer\"},\"v166\":{\"nullable\":true,\"type\":\"integer\"},\"v167\":{\"nullable\":true,\"type\":\"integer\"},\"v168\":{\"nullable\":true,\"type\":\"integer\"},\"v169\":{\"nullable\":true,\"type\":\"integer\"},\"v17\":{\"type\":\"string\"},\"v170\":{\"nullable\":true,\"type\":\"integer\"},\"v171\":{\"nullable\":true,\"type\":\"integer\"},\"v172\":{\"type\":\"string\"},\"v173\":{\"type\":\"string\"},\"v174\":{\"type\":\"string\"},\"v175\":{\"type\":\"string\"},\"v176\":{\"format\":\"date-time\",\"type\":\"string\"},\"v177\":{\"format\":\"date-time\",\"type\":\"string\"},\"v178\":{\"format\":\"date-time\",\"type\":\"string\"},\"v179\":{\"format\":\"date-time\",\"type\":\"string\"},\"v18\":{\"format\":\"date-time\",\"type\":\"string\"},\"v180\":{\"format\":\"date-time\",\"type\":\"string\"},\"v181\":{\"format\":\"date-time\",\"type\":\"string\"},\"v182\":{\"format\":\"date-time\",\"type\":\"string\"},\"v183\":{\"format\":\"date-time\",\"type\":\"string\"},\"v184\":{\"type\":\"string\"},\"v185\":{\"type\":\"string\"},\"v186\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v187\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v188\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v189\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v19\":{\"format\":\"date-time\",\"type\":\"string\"},\"v190\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v191\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v192\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v193\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v194\":{\"type\":\"string\"},\"v195\":{\"type\":\"string\"},\"v196\":{\"type\":\"string\"},\"v197\":{\"type\":\"string\"},\"v198\":{\"nullable\":true},\"v199\":{\"nullable\":true},\"v2\":{\"format\":\"int32\",\"type\":\"integer\"},\"v20\":{\"format\":\"date-time\",\"type\":\"string\"},\"v200\":{\"type\":\"string\"},\"v201\":{\"type\":\"string\"},\"v202\":{\"nullable\":true},\"v203\":{\"nullable\":true},\"v204\":{\"type\":\"string\"},\"v205\":{\"type\":\"string\"},\"v206\":{\"format\":\"uuid\",\"type\":\"string\"},\"v207\":{\"format\":\"int64\",\"type\":\"integer\"},\"v208\":{\"format\":\"uuid\",\"type\":\"string\"},\"v209\":{\"format\":\"int64\",\"type\":\"integer\"},\"v21\":{\"format\":\"date-time\",\"type\":\"string\"},\"v210\":{\"format\":\"int32\",\"type\":\"integer\"},\"v211\":{\"format\":\"int32\",\"type\":\"integer\"},\"v212\":{\"type\":\"integer\"},\"v213\":{\"type\":\"string\"},\"v214\":{\"type\":\"string\"},\"v215\":{\"format\":\"date-time\",\"type\":\"string\"},\"v216\":{\"format\":\"date-time\",\"type\":\"string\"},\"v217\":{\"format\":\"date-time\",\"type\":\"string\"},\"v218\":{\"format\":\"date-time\",\"type\":\"string\"},\"v219\":{\"format\":\"date-time\",\"type\":\"string\"},\"v22\":{\"format\":\"date-time\",\"type\":\"string\"},\"v220\":{\"format\":\"date-time\",\"type\":\"string\"},\"v221\":{\"format\":\"date-time\",\"type\":\"string\"},\"v222\":{\"format\":\"date-time\",\"type\":\"string\"},\"v223\":{\"type\":\"string\"},\"v224\":{\"type\":\"string\"},\"v225\":{\"format\":\"date-time\",\"type\":\"string\"},\"v226\":{\"format\":\"date-time\",\"type\":\"string\"},\"v227\":{\"format\":\"date-time\",\"type\":\"string\"},\"v228\":{\"format\":\"date-time\",\"type\":\"string\"},\"v229\":{\"format\":\"date-time\",\"type\":\"string\"},\"v23\":{\"format\":\"date-time\",\"type\":\"string\"},\"v230\":{\"format\":\"date-time\",\"type\":\"string\"},\"v231\":{\"format\":\"date-time\",\"type\":\"string\"},\"v232\":{\"format\":\"date-time\",\"type\":\"string\"},\"v233\":{\"type\":\"string\"},\"v234\":{\"type\":\"string\"},\"v235\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v236\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v237\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v238\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v239\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v24\":{\"format\":\"date-time\",\"type\":\"string\"},\"v240\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v241\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v242\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v243\":{\"type\":\"string\"},\"v244\":{\"type\":\"string\"},\"v245\":{\"type\":\"string\"},\"v246\":{\"type\":\"string\"},\"v247\":{\"format\":\"uuid\",\"type\":\"string\"},\"v248\":{\"format\":\"uuid\",\"type\":\"string\"},\"v249\":{\"format\":\"uuid\",\"type\":\"string\"},\"v25\":{\"format\":\"date-time\",\"type\":\"string\"},\"v250\":{\"format\":\"uuid\",\"type\":\"string\"},\"v251\":{\"format\":\"uuid\",\"type\":\"string\"},\"v252\":{\"format\":\"uuid\",\"type\":\"string\"},\"v253\":{\"format\":\"uuid\",\"type\":\"string\"},\"v254\":{\"format\":\"uuid\",\"type\":\"string\"},\"v255\":{\"type\":\"string\"},\"v256\":{\"type\":\"string\"},\"v257\":{\"format\":\"uuid\",\"type\":\"string\"},\"v258\":{\"format\":\"uuid\",\"type\":\"string\"},\"v259\":{\"format\":\"uuid\",\"type\":\"string\"},\"v26\":{\"type\":\"string\"},\"v260\":{\"format\":\"uuid\",\"type\":\"string\"},\"v261\":{\"format\":\"uuid\",\"type\":\"string\"},\"v262\":{\"format\":\"uuid\",\"type\":\"string\"},\"v263\":{\"format\":\"uuid\",\"type\":\"string\"},\"v264\":{\"format\":\"uuid\",\"type\":\"string\"},\"v265\":{\"type\":\"string\"},\"v266\":{\"type\":\"string\"},\"v267\":{\"format\":\"date-time\",\"type\":\"string\"},\"v268\":{\"format\":\"date-time\",\"type\":\"string\"},\"v269\":{\"format\":\"date-time\",\"type\":\"string\"},\"v27\":{\"type\":\"string\"},\"v270\":{\"format\":\"date-time\",\"type\":\"string\"},\"v271\":{\"format\":\"date-time\",\"type\":\"string\"},\"v272\":{\"format\":\"date-time\",\"type\":\"string\"},\"v273\":{\"format\":\"date-time\",\"type\":\"string\"},\"v274\":{\"format\":\"date-time\",\"type\":\"string\"},\"v275\":{\"type\":\"string\"},\"v276\":{\"type\":\"string\"},\"v277\":{\"type\":\"integer\"},\"v278\":{\"type\":\"integer\"},\"v279\":{\"type\":\"integer\"},\"v28\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v280\":{\"type\":\"integer\"},\"v281\":{\"type\":\"integer\"},\"v282\":{\"type\":\"integer\"},\"v283\":{\"type\":\"integer\"},\"v284\":{\"type\":\"integer\"},\"v285\":{\"type\":\"string\"},\"v286\":{\"type\":\"string\"},\"v287\":{\"format\":\"date-time\",\"type\":\"string\"},\"v288\":{\"format\":\"date-time\",\"type\":\"string\"},\"v289\":{\"format\":\"date-time\",\"type\":\"string\"},\"v29\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v290\":{\"format\":\"date-time\",\"type\":\"string\"},\"v291\":{\"format\":\"date-time\",\"type\":\"string\"},\"v292\":{\"format\":\"date-time\",\"type\":\"string\"},\"v293\":{\"format\":\"date-time\",\"type\":\"string\"},\"v294\":{\"format\":\"date-time\",\"type\":\"string\"},\"v295\":{\"type\":\"string\"},\"v296\":{\"type\":\"string\"},\"v297\":{\"type\":\"integer\"},\"v298\":{\"type\":\"integer\"},\"v299\":{\"type\":\"integer\"},\"v3\":{\"type\":\"integer\"},\"v30\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v300\":{\"type\":\"integer\"},\"v301\":{\"type\":\"integer\"},\"v302\":{\"type\":\"integer\"},\"v303\":{\"type\":\"integer\"},\"v304\":{\"type\":\"integer\"},\"v305\":{\"type\":\"string\"},\"v306\":{\"type\":\"string\"},\"v307\":{\"type\":\"string\"},\"v308\":{\"type\":\"string\"},\"v309\":{\"type\":\"string\"},\"v31\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v310\":{\"type\":\"string\"},\"v311\":{\"type\":\"string\"},\"v312\":{\"type\":\"string\"},\"v313\":{\"type\":\"string\"},\"v314\":{\"type\":\"string\"},\"v315\":{\"type\":\"string\"},\"v316\":{\"type\":\"string\"},\"v317\":{\"type\":\"number\"},\"v318\":{\"type\":\"number\"},\"v319\":{\"type\":\"number\"},\"v32\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v320\":{\"type\":\"number\"},\"v321\":{\"type\":\"number\"},\"v322\":{\"type\":\"number\"},\"v323\":{\"type\":\"number\"},\"v324\":{\"type\":\"number\"},\"v325\":{\"type\":\"string\"},\"v326\":{\"type\":\"string\"},\"v327\":{\"type\":\"number\"},\"v328\":{\"type\":\"number\"},\"v329\":{\"type\":\"number\"},\"v33\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v330\":{\"type\":\"number\"},\"v331\":{\"type\":\"number\"},\"v332\":{\"type\":\"number\"},\"v333\":{\"type\":\"number\"},\"v334\":{\"type\":\"number\"},\"v335\":{\"type\":\"string\"},\"v336\":{\"type\":\"string\"},\"v337\":{\"type\":\"number\"},\"v338\":{\"type\":\"number\"},\"v339\":{\"type\":\"number\"},\"v34\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v340\":{\"type\":\"number\"},\"v341\":{\"type\":\"number\"},\"v342\":{\"type\":\"number\"},\"v343\":{\"type\":\"number\"},\"v344\":{\"type\":\"number\"},\"v345\":{\"type\":\"string\"},\"v346\":{\"type\":\"string\"},\"v347\":{\"type\":\"number\"},\"v348\":{\"type\":\"number\"},\"v349\":{\"type\":\"number\"},\"v35\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v350\":{\"type\":\"number\"},\"v351\":{\"type\":\"number\"},\"v352\":{\"type\":\"number\"},\"v353\":{\"type\":\"number\"},\"v354\":{\"type\":\"number\"},\"v355\":{\"type\":\"string\"},\"v356\":{\"type\":\"string\"},\"v357\":{\"type\":\"number\"},\"v358\":{\"type\":\"number\"},\"v359\":{\"type\":\"number\"},\"v36\":{\"type\":\"string\"},\"v360\":{\"type\":\"number\"},\"v361\":{\"type\":\"number\"},\"v362\":{\"type\":\"number\"},\"v363\":{\"type\":\"number\"},\"v364\":{\"type\":\"number\"},\"v365\":{\"type\":\"string\"},\"v366\":{\"type\":\"string\"},\"v367\":{\"type\":\"number\"},\"v368\":{\"type\":\"number\"},\"v369\":{\"type\":\"number\"},\"v37\":{\"type\":\"string\"},\"v370\":{\"type\":\"number\"},\"v371\":{\"type\":\"number\"},\"v372\":{\"type\":\"number\"},\"v373\":{\"type\":\"number\"},\"v374\":{\"type\":\"number\"},\"v375\":{\"type\":\"string\"},\"v376\":{\"type\":\"string\"},\"v377\":{\"nullable\":true},\"v378\":{\"nullable\":true},\"v379\":{\"type\":\"string\"},\"v38\":{\"type\":\"string\"},\"v380\":{\"type\":\"string\"},\"v381\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_AudioEvent\"},\"v382\":{\"format\":\"uuid\",\"type\":\"string\"},\"v383\":{\"format\":\"int64\",\"type\":\"integer\"},\"v384\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_ArrayOfNullableAudioEvent\"},\"v385\":{\"format\":\"int64\",\"type\":\"integer\"},\"v386\":{\"format\":\"uuid\",\"type\":\"string\"},\"v387\":{\"format\":\"int64\",\"type\":\"integer\"},\"v388\":{\"format\":\"uuid\",\"type\":\"string\"},\"v389\":{\"format\":\"int64\",\"type\":\"integer\"},\"v39\":{\"type\":\"string\"},\"v390\":{\"format\":\"int32\",\"type\":\"integer\"},\"v391\":{\"format\":\"int32\",\"type\":\"integer\"},\"v392\":{\"type\":\"integer\"},\"v393\":{\"type\":\"string\"},\"v394\":{\"type\":\"string\"},\"v395\":{\"type\":\"string\"},\"v396\":{\"type\":\"string\"},\"v397\":{\"format\":\"date-time\",\"type\":\"string\"},\"v398\":{\"format\":\"date-time\",\"type\":\"string\"},\"v399\":{\"format\":\"date-time\",\"type\":\"string\"},\"v4\":{\"type\":\"string\"},\"v40\":{\"type\":\"string\"},\"v400\":{\"format\":\"date-time\",\"type\":\"string\"},\"v401\":{\"format\":\"date-time\",\"type\":\"string\"},\"v402\":{\"format\":\"date-time\",\"type\":\"string\"},\"v403\":{\"format\":\"date-time\",\"type\":\"string\"},\"v404\":{\"format\":\"date-time\",\"type\":\"string\"},\"v405\":{\"type\":\"string\"},\"v406\":{\"type\":\"string\"},\"v407\":{\"format\":\"date-time\",\"type\":\"string\"},\"v408\":{\"format\":\"date-time\",\"type\":\"string\"},\"v409\":{\"format\":\"date-time\",\"type\":\"string\"},\"v41\":{\"type\":\"string\"},\"v410\":{\"format\":\"date-time\",\"type\":\"string\"},\"v411\":{\"format\":\"date-time\",\"type\":\"string\"},\"v412\":{\"format\":\"date-time\",\"type\":\"string\"},\"v413\":{\"format\":\"date-time\",\"type\":\"string\"},\"v414\":{\"format\":\"date-time\",\"type\":\"string\"},\"v415\":{\"type\":\"string\"},\"v416\":{\"type\":\"string\"},\"v417\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v418\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v419\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v42\":{\"type\":\"string\"},\"v420\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v421\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v422\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v423\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v424\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v425\":{\"type\":\"string\"},\"v426\":{\"type\":\"string\"},\"v427\":{\"type\":\"string\"},\"v428\":{\"type\":\"string\"},\"v429\":{\"type\":\"string\"},\"v43\":{\"type\":\"string\"},\"v430\":{\"type\":\"string\"},\"v431\":{\"type\":\"string\"},\"v432\":{\"type\":\"string\"},\"v433\":{\"type\":\"string\"},\"v434\":{\"type\":\"string\"},\"v435\":{\"type\":\"string\"},\"v436\":{\"type\":\"string\"},\"v437\":{\"type\":\"string\"},\"v438\":{\"type\":\"string\"},\"v439\":{\"type\":\"string\"},\"v44\":{\"type\":\"string\"},\"v440\":{\"type\":\"string\"},\"v441\":{\"type\":\"string\"},\"v442\":{\"type\":\"string\"},\"v443\":{\"type\":\"string\"},\"v444\":{\"type\":\"string\"},\"v445\":{\"type\":\"string\"},\"v446\":{\"type\":\"string\"},\"v447\":{\"type\":\"string\"},\"v448\":{\"type\":\"string\"},\"v449\":{\"format\":\"date-time\",\"type\":\"string\"},\"v45\":{\"type\":\"string\"},\"v450\":{\"format\":\"date-time\",\"type\":\"string\"},\"v451\":{\"format\":\"date-time\",\"type\":\"string\"},\"v452\":{\"format\":\"date-time\",\"type\":\"string\"},\"v453\":{\"format\":\"date-time\",\"type\":\"string\"},\"v454\":{\"format\":\"date-time\",\"type\":\"string\"},\"v455\":{\"format\":\"date-time\",\"type\":\"string\"},\"v456\":{\"format\":\"date-time\",\"type\":\"string\"},\"v457\":{\"type\":\"string\"},\"v458\":{\"type\":\"string\"},\"v459\":{\"format\":\"date-time\",\"type\":\"string\"},\"v46\":{\"type\":\"string\"},\"v460\":{\"format\":\"date-time\",\"type\":\"string\"},\"v461\":{\"format\":\"date-time\",\"type\":\"string\"},\"v462\":{\"format\":\"date-time\",\"type\":\"string\"},\"v463\":{\"format\":\"date-time\",\"type\":\"string\"},\"v464\":{\"format\":\"date-time\",\"type\":\"string\"},\"v465\":{\"format\":\"date-time\",\"type\":\"string\"},\"v466\":{\"format\":\"date-time\",\"type\":\"string\"},\"v467\":{\"type\":\"string\"},\"v468\":{\"type\":\"string\"},\"v469\":{\"format\":\"date-time\",\"type\":\"string\"},\"v47\":{\"type\":\"string\"},\"v470\":{\"format\":\"date-time\",\"type\":\"string\"},\"v471\":{\"format\":\"date-time\",\"type\":\"string\"},\"v472\":{\"format\":\"date-time\",\"type\":\"string\"},\"v473\":{\"format\":\"date-time\",\"type\":\"string\"},\"v474\":{\"format\":\"date-time\",\"type\":\"string\"},\"v475\":{\"format\":\"date-time\",\"type\":\"string\"},\"v476\":{\"format\":\"date-time\",\"type\":\"string\"},\"v477\":{\"type\":\"string\"},\"v478\":{\"type\":\"string\"},\"v479\":{\"nullable\":true,\"type\":\"string\"},\"v48\":{\"type\":\"string\"},\"v480\":{\"nullable\":true,\"type\":\"string\"},\"v481\":{\"nullable\":true,\"type\":\"string\"},\"v482\":{\"nullable\":true,\"type\":\"string\"},\"v483\":{\"nullable\":true,\"type\":\"string\"},\"v484\":{\"nullable\":true,\"type\":\"string\"},\"v485\":{\"nullable\":true,\"type\":\"string\"},\"v486\":{\"nullable\":true,\"type\":\"string\"},\"v487\":{\"type\":\"string\"},\"v488\":{\"type\":\"string\"},\"v489\":{\"type\":\"string\"},\"v49\":{\"type\":\"string\"},\"v490\":{\"type\":\"string\"},\"v491\":{\"type\":\"integer\"},\"v492\":{\"type\":\"integer\"},\"v493\":{\"type\":\"integer\"},\"v494\":{\"type\":\"integer\"},\"v495\":{\"type\":\"integer\"},\"v496\":{\"type\":\"integer\"},\"v497\":{\"type\":\"integer\"},\"v498\":{\"type\":\"integer\"},\"v499\":{\"type\":\"string\"},\"v5\":{\"type\":\"string\"},\"v50\":{\"format\":\"date-time\",\"type\":\"string\"},\"v500\":{\"type\":\"string\"},\"v501\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Camera\"},\"v502\":{\"format\":\"uuid\",\"type\":\"string\"},\"v503\":{\"format\":\"int64\",\"type\":\"integer\"},\"v504\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_ArrayOfNullableCamera\"},\"v505\":{\"format\":\"int64\",\"type\":\"integer\"},\"v506\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_CameraSegmentProducerClaimRequest\"},\"v507\":{\"format\":\"uuid\",\"type\":\"string\"},\"v508\":{\"format\":\"int64\",\"type\":\"integer\"},\"v509\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_CameraStreamProducerClaimRequest\"},\"v51\":{\"format\":\"date-time\",\"type\":\"string\"},\"v510\":{\"format\":\"uuid\",\"type\":\"string\"},\"v511\":{\"format\":\"int64\",\"type\":\"integer\"},\"v512\":{\"format\":\"uuid\",\"type\":\"string\"},\"v513\":{\"format\":\"int64\",\"type\":\"integer\"},\"v514\":{\"format\":\"uuid\",\"type\":\"string\"},\"v515\":{\"format\":\"int64\",\"type\":\"integer\"},\"v516\":{\"format\":\"int32\",\"type\":\"integer\"},\"v517\":{\"format\":\"int32\",\"type\":\"integer\"},\"v518\":{\"type\":\"integer\"},\"v519\":{\"type\":\"string\"},\"v52\":{\"format\":\"date-time\",\"type\":\"string\"},\"v520\":{\"type\":\"string\"},\"v521\":{\"type\":\"string\"},\"v522\":{\"format\":\"date-time\",\"type\":\"string\"},\"v523\":{\"format\":\"date-time\",\"type\":\"string\"},\"v524\":{\"format\":\"date-time\",\"type\":\"string\"},\"v525\":{\"format\":\"date-time\",\"type\":\"string\"},\"v526\":{\"format\":\"date-time\",\"type\":\"string\"},\"v527\":{\"format\":\"date-time\",\"type\":\"string\"},\"v528\":{\"format\":\"date-time\",\"type\":\"string\"},\"v529\":{\"format\":\"date-time\",\"type\":\"string\"},\"v53\":{\"format\":\"date-time\",\"type\":\"string\"},\"v530\":{\"type\":\"string\"},\"v531\":{\"type\":\"string\"},\"v532\":{\"format\":\"date-time\",\"type\":\"string\"},\"v533\":{\"format\":\"date-time\",\"type\":\"string\"},\"v534\":{\"format\":\"date-time\",\"type\":\"string\"},\"v535\":{\"format\":\"date-time\",\"type\":\"string\"},\"v536\":{\"format\":\"date-time\",\"type\":\"string\"},\"v537\":{\"format\":\"date-time\",\"type\":\"string\"},\"v538\":{\"format\":\"date-time\",\"type\":\"string\"},\"v539\":{\"format\":\"date-time\",\"type\":\"string\"},\"v54\":{\"format\":\"date-time\",\"type\":\"string\"},\"v540\":{\"type\":\"string\"},\"v541\":{\"type\":\"string\"},\"v542\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v543\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v544\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v545\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v546\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v547\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v548\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v549\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v55\":{\"format\":\"date-time\",\"type\":\"string\"},\"v550\":{\"type\":\"string\"},\"v551\":{\"type\":\"string\"},\"v552\":{\"type\":\"string\"},\"v553\":{\"type\":\"string\"},\"v554\":{\"format\":\"date-time\",\"type\":\"string\"},\"v555\":{\"format\":\"date-time\",\"type\":\"string\"},\"v556\":{\"format\":\"date-time\",\"type\":\"string\"},\"v557\":{\"format\":\"date-time\",\"type\":\"string\"},\"v558\":{\"format\":\"date-time\",\"type\":\"string\"},\"v559\":{\"format\":\"date-time\",\"type\":\"string\"},\"v56\":{\"format\":\"date-time\",\"type\":\"string\"},\"v560\":{\"format\":\"date-time\",\"type\":\"string\"},\"v561\":{\"format\":\"date-time\",\"type\":\"string\"},\"v562\":{\"type\":\"string\"},\"v563\":{\"type\":\"string\"},\"v564\":{\"type\":\"integer\"},\"v565\":{\"type\":\"integer\"},\"v566\":{\"type\":\"integer\"},\"v567\":{\"type\":\"integer\"},\"v568\":{\"type\":\"integer\"},\"v569\":{\"type\":\"integer\"},\"v57\":{\"format\":\"date-time\",\"type\":\"string\"},\"v570\":{\"type\":\"integer\"},\"v571\":{\"type\":\"integer\"},\"v572\":{\"type\":\"string\"},\"v573\":{\"type\":\"string\"},\"v574\":{\"type\":\"string\"},\"v575\":{\"type\":\"string\"},\"v576\":{\"type\":\"string\"},\"v577\":{\"type\":\"string\"},\"v578\":{\"type\":\"string\"},\"v579\":{\"type\":\"string\"},\"v58\":{\"type\":\"string\"},\"v580\":{\"type\":\"string\"},\"v581\":{\"type\":\"string\"},\"v582\":{\"type\":\"string\"},\"v583\":{\"type\":\"string\"},\"v584\":{\"type\":\"number\"},\"v585\":{\"type\":\"number\"},\"v586\":{\"type\":\"number\"},\"v587\":{\"type\":\"number\"},\"v588\":{\"type\":\"number\"},\"v589\":{\"type\":\"number\"},\"v59\":{\"type\":\"string\"},\"v590\":{\"type\":\"number\"},\"v591\":{\"type\":\"number\"},\"v592\":{\"type\":\"string\"},\"v593\":{\"type\":\"string\"},\"v594\":{\"format\":\"uuid\",\"type\":\"string\"},\"v595\":{\"format\":\"uuid\",\"type\":\"string\"},\"v596\":{\"format\":\"uuid\",\"type\":\"string\"},\"v597\":{\"format\":\"uuid\",\"type\":\"string\"},\"v598\":{\"format\":\"uuid\",\"type\":\"string\"},\"v599\":{\"format\":\"uuid\",\"type\":\"string\"},\"v6\":{\"type\":\"string\"},\"v60\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v600\":{\"format\":\"uuid\",\"type\":\"string\"},\"v601\":{\"format\":\"uuid\",\"type\":\"string\"},\"v602\":{\"type\":\"string\"},\"v603\":{\"type\":\"string\"},\"v604\":{\"format\":\"uuid\",\"type\":\"string\"},\"v605\":{\"format\":\"uuid\",\"type\":\"string\"},\"v606\":{\"format\":\"uuid\",\"type\":\"string\"},\"v607\":{\"format\":\"uuid\",\"type\":\"string\"},\"v608\":{\"format\":\"uuid\",\"type\":\"string\"},\"v609\":{\"format\":\"uuid\",\"type\":\"string\"},\"v61\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v610\":{\"format\":\"uuid\",\"type\":\"string\"},\"v611\":{\"format\":\"uuid\",\"type\":\"string\"},\"v612\":{\"type\":\"string\"},\"v613\":{\"type\":\"string\"},\"v614\":{\"format\":\"uuid\",\"nullable\":true,\"type\":\"string\"},\"v615\":{\"format\":\"uuid\",\"nullable\":true,\"type\":\"string\"},\"v616\":{\"format\":\"uuid\",\"nullable\":true,\"type\":\"string\"},\"v617\":{\"format\":\"uuid\",\"nullable\":true,\"type\":\"string\"},\"v618\":{\"format\":\"uuid\",\"nullable\":true,\"type\":\"string\"},\"v619\":{\"format\":\"uuid\",\"nullable\":true,\"type\":\"string\"},\"v62\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v620\":{\"format\":\"uuid\",\"nullable\":true,\"type\":\"string\"},\"v621\":{\"format\":\"uuid\",\"nullable\":true,\"type\":\"string\"},\"v622\":{\"type\":\"string\"},\"v623\":{\"type\":\"string\"},\"v624\":{\"type\":\"string\"},\"v625\":{\"type\":\"string\"},\"v626\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_Detection\"},\"v627\":{\"format\":\"uuid\",\"type\":\"string\"},\"v628\":{\"format\":\"int64\",\"type\":\"integer\"},\"v629\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_ArrayOfNullableDetection\"},\"v63\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v630\":{\"format\":\"int64\",\"type\":\"integer\"},\"v631\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_VideoObjectDetectorClaimRequest\"},\"v632\":{\"format\":\"int32\",\"type\":\"integer\"},\"v633\":{\"format\":\"int32\",\"type\":\"integer\"},\"v634\":{\"type\":\"integer\"},\"v635\":{\"type\":\"string\"},\"v636\":{\"type\":\"string\"},\"v637\":{\"type\":\"string\"},\"v638\":{\"type\":\"string\"},\"v639\":{\"format\":\"date-time\",\"type\":\"string\"},\"v64\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v640\":{\"format\":\"date-time\",\"type\":\"string\"},\"v641\":{\"format\":\"date-time\",\"type\":\"string\"},\"v642\":{\"format\":\"date-time\",\"type\":\"string\"},\"v643\":{\"format\":\"date-time\",\"type\":\"string\"},\"v644\":{\"format\":\"date-time\",\"type\":\"string\"},\"v645\":{\"format\":\"date-time\",\"type\":\"string\"},\"v646\":{\"format\":\"date-time\",\"type\":\"string\"},\"v647\":{\"type\":\"string\"},\"v648\":{\"type\":\"string\"},\"v649\":{\"format\":\"date-time\",\"type\":\"string\"},\"v65\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v650\":{\"format\":\"date-time\",\"type\":\"string\"},\"v651\":{\"format\":\"date-time\",\"type\":\"string\"},\"v652\":{\"format\":\"date-time\",\"type\":\"string\"},\"v653\":{\"format\":\"date-time\",\"type\":\"string\"},\"v654\":{\"format\":\"date-time\",\"type\":\"string\"},\"v655\":{\"format\":\"date-time\",\"type\":\"string\"},\"v656\":{\"format\":\"date-time\",\"type\":\"string\"},\"v657\":{\"type\":\"string\"},\"v658\":{\"type\":\"string\"},\"v659\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v66\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v660\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v661\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v662\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v663\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v664\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v665\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v666\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v667\":{\"type\":\"string\"},\"v668\":{\"type\":\"string\"},\"v669\":{\"type\":\"string\"},\"v67\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v670\":{\"type\":\"string\"},\"v671\":{\"type\":\"string\"},\"v672\":{\"type\":\"string\"},\"v673\":{\"type\":\"string\"},\"v674\":{\"type\":\"string\"},\"v675\":{\"type\":\"string\"},\"v676\":{\"type\":\"string\"},\"v677\":{\"type\":\"string\"},\"v678\":{\"type\":\"string\"},\"v679\":{\"type\":\"string\"},\"v68\":{\"type\":\"string\"},\"v680\":{\"type\":\"string\"},\"v681\":{\"format\":\"date-time\",\"type\":\"string\"},\"v682\":{\"format\":\"date-time\",\"type\":\"string\"},\"v683\":{\"format\":\"date-time\",\"type\":\"string\"},\"v684\":{\"format\":\"date-time\",\"type\":\"string\"},\"v685\":{\"format\":\"date-time\",\"type\":\"string\"},\"v686\":{\"format\":\"date-time\",\"type\":\"string\"},\"v687\":{\"format\":\"date-time\",\"type\":\"string\"},\"v688\":{\"format\":\"date-time\",\"type\":\"string\"},\"v689\":{\"type\":\"string\"},\"v69\":{\"type\":\"string\"},\"v690\":{\"type\":\"string\"},\"v691\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v692\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v693\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v694\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v695\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v696\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v697\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v698\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v699\":{\"type\":\"string\"},\"v7\":{\"type\":\"string\"},\"v70\":{\"type\":\"string\"},\"v700\":{\"type\":\"string\"},\"v701\":{\"type\":\"string\"},\"v702\":{\"type\":\"string\"},\"v703\":{\"nullable\":true,\"type\":\"string\"},\"v704\":{\"nullable\":true,\"type\":\"string\"},\"v705\":{\"nullable\":true,\"type\":\"string\"},\"v706\":{\"nullable\":true,\"type\":\"string\"},\"v707\":{\"nullable\":true,\"type\":\"string\"},\"v708\":{\"nullable\":true,\"type\":\"string\"},\"v709\":{\"nullable\":true,\"type\":\"string\"},\"v71\":{\"type\":\"string\"},\"v710\":{\"nullable\":true,\"type\":\"string\"},\"v711\":{\"type\":\"string\"},\"v712\":{\"type\":\"string\"},\"v713\":{\"type\":\"string\"},\"v714\":{\"type\":\"string\"},\"v715\":{\"nullable\":true,\"type\":\"number\"},\"v716\":{\"nullable\":true,\"type\":\"number\"},\"v717\":{\"nullable\":true,\"type\":\"number\"},\"v718\":{\"nullable\":true,\"type\":\"number\"},\"v719\":{\"nullable\":true,\"type\":\"number\"},\"v72\":{\"nullable\":true,\"type\":\"string\"},\"v720\":{\"nullable\":true,\"type\":\"number\"},\"v721\":{\"nullable\":true,\"type\":\"number\"},\"v722\":{\"nullable\":true,\"type\":\"number\"},\"v723\":{\"type\":\"string\"},\"v724\":{\"type\":\"string\"},\"v725\":{\"type\":\"string\"},\"v726\":{\"type\":\"string\"},\"v727\":{\"nullable\":true,\"type\":\"string\"},\"v728\":{\"nullable\":true,\"type\":\"string\"},\"v729\":{\"nullable\":true,\"type\":\"string\"},\"v73\":{\"nullable\":true,\"type\":\"string\"},\"v730\":{\"nullable\":true,\"type\":\"string\"},\"v731\":{\"nullable\":true,\"type\":\"string\"},\"v732\":{\"nullable\":true,\"type\":\"string\"},\"v733\":{\"nullable\":true,\"type\":\"string\"},\"v734\":{\"nullable\":true,\"type\":\"string\"},\"v735\":{\"type\":\"string\"},\"v736\":{\"type\":\"string\"},\"v737\":{\"type\":\"string\"},\"v738\":{\"type\":\"string\"},\"v739\":{\"nullable\":true,\"type\":\"string\"},\"v74\":{\"nullable\":true,\"type\":\"string\"},\"v740\":{\"nullable\":true,\"type\":\"string\"},\"v741\":{\"nullable\":true,\"type\":\"string\"},\"v742\":{\"nullable\":true,\"type\":\"string\"},\"v743\":{\"nullable\":true,\"type\":\"string\"},\"v744\":{\"nullable\":true,\"type\":\"string\"},\"v745\":{\"nullable\":true,\"type\":\"string\"},\"v746\":{\"nullable\":true,\"type\":\"string\"},\"v747\":{\"type\":\"string\"},\"v748\":{\"type\":\"string\"},\"v749\":{\"type\":\"string\"},\"v75\":{\"nullable\":true,\"type\":\"string\"},\"v750\":{\"type\":\"string\"},\"v751\":{\"format\":\"date-time\",\"type\":\"string\"},\"v752\":{\"format\":\"date-time\",\"type\":\"string\"},\"v753\":{\"format\":\"date-time\",\"type\":\"string\"},\"v754\":{\"format\":\"date-time\",\"type\":\"string\"},\"v755\":{\"format\":\"date-time\",\"type\":\"string\"},\"v756\":{\"format\":\"date-time\",\"type\":\"string\"},\"v757\":{\"format\":\"date-time\",\"type\":\"string\"},\"v758\":{\"format\":\"date-time\",\"type\":\"string\"},\"v759\":{\"type\":\"string\"},\"v76\":{\"nullable\":true,\"type\":\"string\"},\"v760\":{\"type\":\"string\"},\"v761\":{\"format\":\"date-time\",\"type\":\"string\"},\"v762\":{\"format\":\"date-time\",\"type\":\"string\"},\"v763\":{\"format\":\"date-time\",\"type\":\"string\"},\"v764\":{\"format\":\"date-time\",\"type\":\"string\"},\"v765\":{\"format\":\"date-time\",\"type\":\"string\"},\"v766\":{\"format\":\"date-time\",\"type\":\"string\"},\"v767\":{\"format\":\"date-time\",\"type\":\"string\"},\"v768\":{\"format\":\"date-time\",\"type\":\"string\"},\"v769\":{\"type\":\"string\"},\"v77\":{\"nullable\":true,\"type\":\"string\"},\"v770\":{\"type\":\"string\"},\"v771\":{\"format\":\"uuid\",\"type\":\"string\"},\"v772\":{\"format\":\"uuid\",\"type\":\"string\"},\"v773\":{\"format\":\"uuid\",\"type\":\"string\"},\"v774\":{\"format\":\"uuid\",\"type\":\"string\"},\"v775\":{\"format\":\"uuid\",\"type\":\"string\"},\"v776\":{\"format\":\"uuid\",\"type\":\"string\"},\"v777\":{\"format\":\"uuid\",\"type\":\"string\"},\"v778\":{\"format\":\"uuid\",\"type\":\"string\"},\"v779\":{\"type\":\"string\"},\"v78\":{\"nullable\":true,\"type\":\"string\"},\"v780\":{\"type\":\"string\"},\"v781\":{\"nullable\":true},\"v782\":{\"nullable\":true},\"v783\":{\"type\":\"string\"},\"v784\":{\"type\":\"string\"},\"v785\":{\"type\":\"boolean\"},\"v786\":{\"type\":\"boolean\"},\"v787\":{\"type\":\"string\"},\"v788\":{\"type\":\"string\"},\"v789\":{\"type\":\"string\"},\"v79\":{\"nullable\":true,\"type\":\"string\"},\"v790\":{\"type\":\"string\"},\"v791\":{\"nullable\":true},\"v792\":{\"nullable\":true},\"v793\":{\"type\":\"string\"},\"v794\":{\"type\":\"string\"},\"v795\":{\"nullable\":true,\"type\":\"integer\"},\"v796\":{\"nullable\":true,\"type\":\"integer\"},\"v797\":{\"nullable\":true,\"type\":\"integer\"},\"v798\":{\"nullable\":true,\"type\":\"integer\"},\"v799\":{\"nullable\":true,\"type\":\"integer\"},\"v8\":{\"format\":\"date-time\",\"type\":\"string\"},\"v80\":{\"type\":\"string\"},\"v800\":{\"nullable\":true,\"type\":\"integer\"},\"v801\":{\"nullable\":true,\"type\":\"integer\"},\"v802\":{\"nullable\":true,\"type\":\"integer\"},\"v803\":{\"type\":\"string\"},\"v804\":{\"type\":\"string\"},\"v805\":{\"type\":\"string\"},\"v806\":{\"type\":\"string\"},\"v807\":{\"format\":\"date-time\",\"type\":\"string\"},\"v808\":{\"format\":\"date-time\",\"type\":\"string\"},\"v809\":{\"format\":\"date-time\",\"type\":\"string\"},\"v81\":{\"type\":\"string\"},\"v810\":{\"format\":\"date-time\",\"type\":\"string\"},\"v811\":{\"format\":\"date-time\",\"type\":\"string\"},\"v812\":{\"format\":\"date-time\",\"type\":\"string\"},\"v813\":{\"format\":\"date-time\",\"type\":\"string\"},\"v814\":{\"format\":\"date-time\",\"type\":\"string\"},\"v815\":{\"type\":\"string\"},\"v816\":{\"type\":\"string\"},\"v817\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v818\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v819\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v82\":{\"type\":\"string\"},\"v820\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v821\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v822\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v823\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v824\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v825\":{\"type\":\"string\"},\"v826\":{\"type\":\"string\"},\"v827\":{\"type\":\"string\"},\"v828\":{\"type\":\"string\"},\"v829\":{\"nullable\":true},\"v83\":{\"type\":\"string\"},\"v830\":{\"nullable\":true},\"v831\":{\"type\":\"string\"},\"v832\":{\"type\":\"string\"},\"v833\":{\"nullable\":true},\"v834\":{\"nullable\":true},\"v835\":{\"type\":\"string\"},\"v836\":{\"type\":\"string\"},\"v837\":{\"$ref\":\"urn:openapi-to-rust:request-validation#/$defs/components/component_VideoObjectTrackerClaimRequest\"},\"v838\":{\"format\":\"int32\",\"type\":\"integer\"},\"v839\":{\"format\":\"int32\",\"type\":\"integer\"},\"v84\":{\"nullable\":true,\"type\":\"number\"},\"v840\":{\"type\":\"integer\"},\"v841\":{\"type\":\"string\"},\"v842\":{\"type\":\"string\"},\"v843\":{\"type\":\"string\"},\"v844\":{\"type\":\"string\"},\"v845\":{\"format\":\"date-time\",\"type\":\"string\"},\"v846\":{\"format\":\"date-time\",\"type\":\"string\"},\"v847\":{\"format\":\"date-time\",\"type\":\"string\"},\"v848\":{\"format\":\"date-time\",\"type\":\"string\"},\"v849\":{\"format\":\"date-time\",\"type\":\"string\"},\"v85\":{\"nullable\":true,\"type\":\"number\"},\"v850\":{\"format\":\"date-time\",\"type\":\"string\"},\"v851\":{\"format\":\"date-time\",\"type\":\"string\"},\"v852\":{\"format\":\"date-time\",\"type\":\"string\"},\"v853\":{\"type\":\"string\"},\"v854\":{\"type\":\"string\"},\"v855\":{\"format\":\"date-time\",\"type\":\"string\"},\"v856\":{\"format\":\"date-time\",\"type\":\"string\"},\"v857\":{\"format\":\"date-time\",\"type\":\"string\"},\"v858\":{\"format\":\"date-time\",\"type\":\"string\"},\"v859\":{\"format\":\"date-time\",\"type\":\"string\"},\"v86\":{\"nullable\":true,\"type\":\"number\"},\"v860\":{\"format\":\"date-time\",\"type\":\"string\"},\"v861\":{\"format\":\"date-time\",\"type\":\"string\"},\"v862\":{\"format\":\"date-time\",\"type\":\"string\"},\"v863\":{\"type\":\"string\"},\"v864\":{\"type\":\"string\"},\"v865\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v866\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v867\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v868\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v869\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v87\":{\"nullable\":true,\"type\":\"number\"},\"v870\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v871\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v872\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v873\":{\"type\":\"string\"},\"v874\":{\"type\":\"string\"},\"v875\":{\"type\":\"string\"},\"v876\":{\"type\":\"string\"},\"v877\":{\"type\":\"string\"},\"v878\":{\"type\":\"string\"},\"v879\":{\"type\":\"string\"},\"v88\":{\"nullable\":true,\"type\":\"number\"},\"v880\":{\"type\":\"string\"},\"v881\":{\"type\":\"string\"},\"v882\":{\"type\":\"string\"},\"v883\":{\"type\":\"string\"},\"v884\":{\"type\":\"string\"},\"v885\":{\"type\":\"string\"},\"v886\":{\"type\":\"string\"},\"v887\":{\"format\":\"date-time\",\"type\":\"string\"},\"v888\":{\"format\":\"date-time\",\"type\":\"string\"},\"v889\":{\"format\":\"date-time\",\"type\":\"string\"},\"v89\":{\"nullable\":true,\"type\":\"number\"},\"v890\":{\"format\":\"date-time\",\"type\":\"string\"},\"v891\":{\"format\":\"date-time\",\"type\":\"string\"},\"v892\":{\"format\":\"date-time\",\"type\":\"string\"},\"v893\":{\"format\":\"date-time\",\"type\":\"string\"},\"v894\":{\"format\":\"date-time\",\"type\":\"string\"},\"v895\":{\"type\":\"string\"},\"v896\":{\"type\":\"string\"},\"v897\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v898\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v899\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v9\":{\"format\":\"date-time\",\"type\":\"string\"},\"v90\":{\"nullable\":true,\"type\":\"number\"},\"v900\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v901\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v902\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v903\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v904\":{\"format\":\"date-time\",\"nullable\":true,\"type\":\"string\"},\"v905\":{\"type\":\"string\"},\"v906\":{\"type\":\"string\"},\"v907\":{\"type\":\"string\"},\"v908\":{\"type\":\"string\"},\"v909\":{\"nullable\":true,\"type\":\"string\"},\"v91\":{\"nullable\":true,\"type\":\"number\"},\"v910\":{\"nullable\":true,\"type\":\"string\"},\"v911\":{\"nullable\":true,\"type\":\"string\"},\"v912\":{\"nullable\":true,\"type\":\"string\"},\"v913\":{\"nullable\":true,\"type\":\"string\"},\"v914\":{\"nullable\":true,\"type\":\"string\"},\"v915\":{\"nullable\":true,\"type\":\"string\"},\"v916\":{\"nullable\":true,\"type\":\"string\"},\"v917\":{\"type\":\"string\"},\"v918\":{\"type\":\"string\"},\"v919\":{\"type\":\"string\"},\"v92\":{\"type\":\"string\"},\"v920\":{\"type\":\"string\"},\"v921\":{\"nullable\":true,\"type\":\"number\"},\"v922\":{\"nullable\":true,\"type\":\"number\"},\"v923\":{\"nullable\":true,\"type\":\"number\"},\"v924\":{\"nullable\":true,\"type\":\"number\"},\"v925\":{\"nullable\":true,\"type\":\"number\"},\"v926\":{\"nullable\":true,\"type\":\"number\"},\"v927\":{\"nullable\":true,\"type\":\"number\"},\"v928\":{\"nullable\":true,\"type\":\"number\"},\"v929\":{\"type\":\"string\"},\"v93\":{\"type\":\"string\"},\"v930\":{\"type\":\"string\"},\"v931\":{\"type\":\"string\"},\"v932\":{\"type\":\"string\"},\"v933\":{\"nullable\":true,\"type\":\"string\"},\"v934\":{\"nullable\":true,\"type\":\"string\"},\"v935\":{\"nullable\":true,\"type\":\"string\"},\"v936\":{\"nullable\":true,\"type\":\"string\"},\"v937\":{\"nullable\":true,\"type\":\"string\"},\"v938\":{\"nullable\":true,\"type\":\"string\"},\"v939\":{\"nullable\":true,\"type\":\"string\"},\"v94\":{\"type\":\"string\"},\"v940\":{\"nullable\":true,\"type\":\"string\"},\"v941\":{\"type\":\"string\"},\"v942\":{\"type\":\"string\"},\"v943\":{\"type\":\"string\"},\"v944\":{\"type\":\"string\"},\"v945\":{\"nullable\":true,\"type\":\"string\"},\"v946\":{\"nullable\":true,\"type\":\"string\"},\"v947\":{\"nullable\":true,\"type\":\"string\"},\"v948\":{\"nullable\":true,\"type\":\"string\"},\"v949\":{\"nullable\":true,\"type\":\"string\"},\"v95\":{\"type\":\"string\"},\"v950\":{\"nullable\":true,\"type\":\"string\"},\"v951\":{\"nullable\":true,\"type\":\"string\"},\"v952\":{\"nullable\":true,\"type\":\"string\"},\"v953\":{\"type\":\"string\"},\"v954\":{\"type\":\"string\"},\"v955\":{\"type\":\"string\"},\"v956\":{\"type\":\"string\"},\"v957\":{\"format\":\"date-time\",\"type\":\"string\"},\"v958\":{\"format\":\"date-time\",\"type\":\"string\"},\"v959\":{\"format\":\"date-time\",\"type\":\"string\"},\"v96\":{\"nullable\":true,\"type\":\"string\"},\"v960\":{\"format\":\"date-time\",\"type\":\"string\"},\"v961\":{\"format\":\"date-time\",\"type\":\"string\"},\"v962\":{\"format\":\"date-time\",\"type\":\"string\"},\"v963\":{\"format\":\"date-time\",\"type\":\"string\"},\"v964\":{\"format\":\"date-time\",\"type\":\"string\"},\"v965\":{\"type\":\"string\"},\"v966\":{\"type\":\"string\"},\"v967\":{\"format\":\"date-time\",\"type\":\"string\"},\"v968\":{\"format\":\"date-time\",\"type\":\"string\"},\"v969\":{\"format\":\"date-time\",\"type\":\"string\"},\"v97\":{\"nullable\":true,\"type\":\"string\"},\"v970\":{\"format\":\"date-time\",\"type\":\"string\"},\"v971\":{\"format\":\"date-time\",\"type\":\"string\"},\"v972\":{\"format\":\"date-time\",\"type\":\"string\"},\"v973\":{\"format\":\"date-time\",\"type\":\"string\"},\"v974\":{\"format\":\"date-time\",\"type\":\"string\"},\"v975\":{\"type\":\"string\"},\"v976\":{\"type\":\"string\"},\"v977\":{\"format\":\"uuid\",\"type\":\"string\"},\"v978\":{\"format\":\"uuid\",\"type\":\"string\"},\"v979\":{\"format\":\"uuid\",\"type\":\"string\"},\"v98\":{\"nullable\":true,\"type\":\"string\"},\"v980\":{\"format\":\"uuid\",\"type\":\"string\"},\"v981\":{\"format\":\"uuid\",\"type\":\"string\"},\"v982\":{\"format\":\"uuid\",\"type\":\"string\"},\"v983\":{\"format\":\"uuid\",\"type\":\"string\"},\"v984\":{\"format\":\"uuid\",\"type\":\"string\"},\"v985\":{\"type\":\"string\"},\"v986\":{\"type\":\"string\"},\"v987\":{\"nullable\":true},\"v988\":{\"nullable\":true},\"v989\":{\"type\":\"string\"},\"v99\":{\"nullable\":true,\"type\":\"string\"},\"v990\":{\"type\":\"string\"},\"v991\":{\"type\":\"boolean\"},\"v992\":{\"type\":\"boolean\"},\"v993\":{\"type\":\"string\"},\"v994\":{\"type\":\"string\"},\"v995\":{\"type\":\"string\"},\"v996\":{\"type\":\"string\"},\"v997\":{\"nullable\":true},\"v998\":{\"nullable\":true},\"v999\":{\"type\":\"string\"}}},\"$id\":\"urn:openapi-to-rust:request-validation\",\"$schema\":\"https://json-schema.org/draft/2020-12/schema\"}";
const MAX_VALIDATION_ERRORS: usize = 16usize;
pub(crate) const VALIDATION_TARGET_0_BODY: &str = "#/$defs/targets/v0";
pub(crate) const VALIDATION_TARGET_1_QUERY_0: &str = "#/$defs/targets/v1";
pub(crate) const VALIDATION_TARGET_2_QUERY_1: &str = "#/$defs/targets/v2";
pub(crate) const VALIDATION_TARGET_3_QUERY_2: &str = "#/$defs/targets/v3";
pub(crate) const VALIDATION_TARGET_4_QUERY_3: &str = "#/$defs/targets/v4";
pub(crate) const VALIDATION_TARGET_5_QUERY_4: &str = "#/$defs/targets/v5";
pub(crate) const VALIDATION_TARGET_6_QUERY_5: &str = "#/$defs/targets/v6";
pub(crate) const VALIDATION_TARGET_7_QUERY_6: &str = "#/$defs/targets/v7";
pub(crate) const VALIDATION_TARGET_8_QUERY_7: &str = "#/$defs/targets/v8";
pub(crate) const VALIDATION_TARGET_9_QUERY_8: &str = "#/$defs/targets/v9";
pub(crate) const VALIDATION_TARGET_10_QUERY_9: &str = "#/$defs/targets/v10";
pub(crate) const VALIDATION_TARGET_11_QUERY_10: &str = "#/$defs/targets/v11";
pub(crate) const VALIDATION_TARGET_12_QUERY_11: &str = "#/$defs/targets/v12";
pub(crate) const VALIDATION_TARGET_13_QUERY_12: &str = "#/$defs/targets/v13";
pub(crate) const VALIDATION_TARGET_14_QUERY_13: &str = "#/$defs/targets/v14";
pub(crate) const VALIDATION_TARGET_15_QUERY_14: &str = "#/$defs/targets/v15";
pub(crate) const VALIDATION_TARGET_16_QUERY_15: &str = "#/$defs/targets/v16";
pub(crate) const VALIDATION_TARGET_17_QUERY_16: &str = "#/$defs/targets/v17";
pub(crate) const VALIDATION_TARGET_18_QUERY_17: &str = "#/$defs/targets/v18";
pub(crate) const VALIDATION_TARGET_19_QUERY_18: &str = "#/$defs/targets/v19";
pub(crate) const VALIDATION_TARGET_20_QUERY_19: &str = "#/$defs/targets/v20";
pub(crate) const VALIDATION_TARGET_21_QUERY_20: &str = "#/$defs/targets/v21";
pub(crate) const VALIDATION_TARGET_22_QUERY_21: &str = "#/$defs/targets/v22";
pub(crate) const VALIDATION_TARGET_23_QUERY_22: &str = "#/$defs/targets/v23";
pub(crate) const VALIDATION_TARGET_24_QUERY_23: &str = "#/$defs/targets/v24";
pub(crate) const VALIDATION_TARGET_25_QUERY_24: &str = "#/$defs/targets/v25";
pub(crate) const VALIDATION_TARGET_26_QUERY_25: &str = "#/$defs/targets/v26";
pub(crate) const VALIDATION_TARGET_27_QUERY_26: &str = "#/$defs/targets/v27";
pub(crate) const VALIDATION_TARGET_28_QUERY_27: &str = "#/$defs/targets/v28";
pub(crate) const VALIDATION_TARGET_29_QUERY_28: &str = "#/$defs/targets/v29";
pub(crate) const VALIDATION_TARGET_30_QUERY_29: &str = "#/$defs/targets/v30";
pub(crate) const VALIDATION_TARGET_31_QUERY_30: &str = "#/$defs/targets/v31";
pub(crate) const VALIDATION_TARGET_32_QUERY_31: &str = "#/$defs/targets/v32";
pub(crate) const VALIDATION_TARGET_33_QUERY_32: &str = "#/$defs/targets/v33";
pub(crate) const VALIDATION_TARGET_34_QUERY_33: &str = "#/$defs/targets/v34";
pub(crate) const VALIDATION_TARGET_35_QUERY_34: &str = "#/$defs/targets/v35";
pub(crate) const VALIDATION_TARGET_36_QUERY_35: &str = "#/$defs/targets/v36";
pub(crate) const VALIDATION_TARGET_37_QUERY_36: &str = "#/$defs/targets/v37";
pub(crate) const VALIDATION_TARGET_38_QUERY_37: &str = "#/$defs/targets/v38";
pub(crate) const VALIDATION_TARGET_39_QUERY_38: &str = "#/$defs/targets/v39";
pub(crate) const VALIDATION_TARGET_40_QUERY_39: &str = "#/$defs/targets/v40";
pub(crate) const VALIDATION_TARGET_41_QUERY_40: &str = "#/$defs/targets/v41";
pub(crate) const VALIDATION_TARGET_42_QUERY_41: &str = "#/$defs/targets/v42";
pub(crate) const VALIDATION_TARGET_43_QUERY_42: &str = "#/$defs/targets/v43";
pub(crate) const VALIDATION_TARGET_44_QUERY_43: &str = "#/$defs/targets/v44";
pub(crate) const VALIDATION_TARGET_45_QUERY_44: &str = "#/$defs/targets/v45";
pub(crate) const VALIDATION_TARGET_46_QUERY_45: &str = "#/$defs/targets/v46";
pub(crate) const VALIDATION_TARGET_47_QUERY_46: &str = "#/$defs/targets/v47";
pub(crate) const VALIDATION_TARGET_48_QUERY_47: &str = "#/$defs/targets/v48";
pub(crate) const VALIDATION_TARGET_49_QUERY_48: &str = "#/$defs/targets/v49";
pub(crate) const VALIDATION_TARGET_50_QUERY_49: &str = "#/$defs/targets/v50";
pub(crate) const VALIDATION_TARGET_51_QUERY_50: &str = "#/$defs/targets/v51";
pub(crate) const VALIDATION_TARGET_52_QUERY_51: &str = "#/$defs/targets/v52";
pub(crate) const VALIDATION_TARGET_53_QUERY_52: &str = "#/$defs/targets/v53";
pub(crate) const VALIDATION_TARGET_54_QUERY_53: &str = "#/$defs/targets/v54";
pub(crate) const VALIDATION_TARGET_55_QUERY_54: &str = "#/$defs/targets/v55";
pub(crate) const VALIDATION_TARGET_56_QUERY_55: &str = "#/$defs/targets/v56";
pub(crate) const VALIDATION_TARGET_57_QUERY_56: &str = "#/$defs/targets/v57";
pub(crate) const VALIDATION_TARGET_58_QUERY_57: &str = "#/$defs/targets/v58";
pub(crate) const VALIDATION_TARGET_59_QUERY_58: &str = "#/$defs/targets/v59";
pub(crate) const VALIDATION_TARGET_60_QUERY_59: &str = "#/$defs/targets/v60";
pub(crate) const VALIDATION_TARGET_61_QUERY_60: &str = "#/$defs/targets/v61";
pub(crate) const VALIDATION_TARGET_62_QUERY_61: &str = "#/$defs/targets/v62";
pub(crate) const VALIDATION_TARGET_63_QUERY_62: &str = "#/$defs/targets/v63";
pub(crate) const VALIDATION_TARGET_64_QUERY_63: &str = "#/$defs/targets/v64";
pub(crate) const VALIDATION_TARGET_65_QUERY_64: &str = "#/$defs/targets/v65";
pub(crate) const VALIDATION_TARGET_66_QUERY_65: &str = "#/$defs/targets/v66";
pub(crate) const VALIDATION_TARGET_67_QUERY_66: &str = "#/$defs/targets/v67";
pub(crate) const VALIDATION_TARGET_68_QUERY_67: &str = "#/$defs/targets/v68";
pub(crate) const VALIDATION_TARGET_69_QUERY_68: &str = "#/$defs/targets/v69";
pub(crate) const VALIDATION_TARGET_70_QUERY_69: &str = "#/$defs/targets/v70";
pub(crate) const VALIDATION_TARGET_71_QUERY_70: &str = "#/$defs/targets/v71";
pub(crate) const VALIDATION_TARGET_72_QUERY_71: &str = "#/$defs/targets/v72";
pub(crate) const VALIDATION_TARGET_73_QUERY_72: &str = "#/$defs/targets/v73";
pub(crate) const VALIDATION_TARGET_74_QUERY_73: &str = "#/$defs/targets/v74";
pub(crate) const VALIDATION_TARGET_75_QUERY_74: &str = "#/$defs/targets/v75";
pub(crate) const VALIDATION_TARGET_76_QUERY_75: &str = "#/$defs/targets/v76";
pub(crate) const VALIDATION_TARGET_77_QUERY_76: &str = "#/$defs/targets/v77";
pub(crate) const VALIDATION_TARGET_78_QUERY_77: &str = "#/$defs/targets/v78";
pub(crate) const VALIDATION_TARGET_79_QUERY_78: &str = "#/$defs/targets/v79";
pub(crate) const VALIDATION_TARGET_80_QUERY_79: &str = "#/$defs/targets/v80";
pub(crate) const VALIDATION_TARGET_81_QUERY_80: &str = "#/$defs/targets/v81";
pub(crate) const VALIDATION_TARGET_82_QUERY_81: &str = "#/$defs/targets/v82";
pub(crate) const VALIDATION_TARGET_83_QUERY_82: &str = "#/$defs/targets/v83";
pub(crate) const VALIDATION_TARGET_84_QUERY_83: &str = "#/$defs/targets/v84";
pub(crate) const VALIDATION_TARGET_85_QUERY_84: &str = "#/$defs/targets/v85";
pub(crate) const VALIDATION_TARGET_86_QUERY_85: &str = "#/$defs/targets/v86";
pub(crate) const VALIDATION_TARGET_87_QUERY_86: &str = "#/$defs/targets/v87";
pub(crate) const VALIDATION_TARGET_88_QUERY_87: &str = "#/$defs/targets/v88";
pub(crate) const VALIDATION_TARGET_89_QUERY_88: &str = "#/$defs/targets/v89";
pub(crate) const VALIDATION_TARGET_90_QUERY_89: &str = "#/$defs/targets/v90";
pub(crate) const VALIDATION_TARGET_91_QUERY_90: &str = "#/$defs/targets/v91";
pub(crate) const VALIDATION_TARGET_92_QUERY_91: &str = "#/$defs/targets/v92";
pub(crate) const VALIDATION_TARGET_93_QUERY_92: &str = "#/$defs/targets/v93";
pub(crate) const VALIDATION_TARGET_94_QUERY_93: &str = "#/$defs/targets/v94";
pub(crate) const VALIDATION_TARGET_95_QUERY_94: &str = "#/$defs/targets/v95";
pub(crate) const VALIDATION_TARGET_96_QUERY_95: &str = "#/$defs/targets/v96";
pub(crate) const VALIDATION_TARGET_97_QUERY_96: &str = "#/$defs/targets/v97";
pub(crate) const VALIDATION_TARGET_98_QUERY_97: &str = "#/$defs/targets/v98";
pub(crate) const VALIDATION_TARGET_99_QUERY_98: &str = "#/$defs/targets/v99";
pub(crate) const VALIDATION_TARGET_100_QUERY_99: &str = "#/$defs/targets/v100";
pub(crate) const VALIDATION_TARGET_101_QUERY_100: &str = "#/$defs/targets/v101";
pub(crate) const VALIDATION_TARGET_102_QUERY_101: &str = "#/$defs/targets/v102";
pub(crate) const VALIDATION_TARGET_103_QUERY_102: &str = "#/$defs/targets/v103";
pub(crate) const VALIDATION_TARGET_104_QUERY_103: &str = "#/$defs/targets/v104";
pub(crate) const VALIDATION_TARGET_105_QUERY_104: &str = "#/$defs/targets/v105";
pub(crate) const VALIDATION_TARGET_106_QUERY_105: &str = "#/$defs/targets/v106";
pub(crate) const VALIDATION_TARGET_107_QUERY_106: &str = "#/$defs/targets/v107";
pub(crate) const VALIDATION_TARGET_108_QUERY_107: &str = "#/$defs/targets/v108";
pub(crate) const VALIDATION_TARGET_109_QUERY_108: &str = "#/$defs/targets/v109";
pub(crate) const VALIDATION_TARGET_110_QUERY_109: &str = "#/$defs/targets/v110";
pub(crate) const VALIDATION_TARGET_111_QUERY_110: &str = "#/$defs/targets/v111";
pub(crate) const VALIDATION_TARGET_112_QUERY_111: &str = "#/$defs/targets/v112";
pub(crate) const VALIDATION_TARGET_113_QUERY_112: &str = "#/$defs/targets/v113";
pub(crate) const VALIDATION_TARGET_114_QUERY_113: &str = "#/$defs/targets/v114";
pub(crate) const VALIDATION_TARGET_115_QUERY_114: &str = "#/$defs/targets/v115";
pub(crate) const VALIDATION_TARGET_116_QUERY_115: &str = "#/$defs/targets/v116";
pub(crate) const VALIDATION_TARGET_117_QUERY_116: &str = "#/$defs/targets/v117";
pub(crate) const VALIDATION_TARGET_118_QUERY_117: &str = "#/$defs/targets/v118";
pub(crate) const VALIDATION_TARGET_119_QUERY_118: &str = "#/$defs/targets/v119";
pub(crate) const VALIDATION_TARGET_120_QUERY_119: &str = "#/$defs/targets/v120";
pub(crate) const VALIDATION_TARGET_121_QUERY_120: &str = "#/$defs/targets/v121";
pub(crate) const VALIDATION_TARGET_122_QUERY_121: &str = "#/$defs/targets/v122";
pub(crate) const VALIDATION_TARGET_123_QUERY_122: &str = "#/$defs/targets/v123";
pub(crate) const VALIDATION_TARGET_124_QUERY_123: &str = "#/$defs/targets/v124";
pub(crate) const VALIDATION_TARGET_125_QUERY_124: &str = "#/$defs/targets/v125";
pub(crate) const VALIDATION_TARGET_126_QUERY_125: &str = "#/$defs/targets/v126";
pub(crate) const VALIDATION_TARGET_127_QUERY_126: &str = "#/$defs/targets/v127";
pub(crate) const VALIDATION_TARGET_128_QUERY_127: &str = "#/$defs/targets/v128";
pub(crate) const VALIDATION_TARGET_129_QUERY_128: &str = "#/$defs/targets/v129";
pub(crate) const VALIDATION_TARGET_130_QUERY_129: &str = "#/$defs/targets/v130";
pub(crate) const VALIDATION_TARGET_131_QUERY_130: &str = "#/$defs/targets/v131";
pub(crate) const VALIDATION_TARGET_132_QUERY_131: &str = "#/$defs/targets/v132";
pub(crate) const VALIDATION_TARGET_133_QUERY_132: &str = "#/$defs/targets/v133";
pub(crate) const VALIDATION_TARGET_134_QUERY_133: &str = "#/$defs/targets/v134";
pub(crate) const VALIDATION_TARGET_135_QUERY_134: &str = "#/$defs/targets/v135";
pub(crate) const VALIDATION_TARGET_136_QUERY_135: &str = "#/$defs/targets/v136";
pub(crate) const VALIDATION_TARGET_137_QUERY_136: &str = "#/$defs/targets/v137";
pub(crate) const VALIDATION_TARGET_138_QUERY_137: &str = "#/$defs/targets/v138";
pub(crate) const VALIDATION_TARGET_139_QUERY_138: &str = "#/$defs/targets/v139";
pub(crate) const VALIDATION_TARGET_140_QUERY_139: &str = "#/$defs/targets/v140";
pub(crate) const VALIDATION_TARGET_141_QUERY_140: &str = "#/$defs/targets/v141";
pub(crate) const VALIDATION_TARGET_142_QUERY_141: &str = "#/$defs/targets/v142";
pub(crate) const VALIDATION_TARGET_143_QUERY_142: &str = "#/$defs/targets/v143";
pub(crate) const VALIDATION_TARGET_144_QUERY_143: &str = "#/$defs/targets/v144";
pub(crate) const VALIDATION_TARGET_145_QUERY_144: &str = "#/$defs/targets/v145";
pub(crate) const VALIDATION_TARGET_146_QUERY_145: &str = "#/$defs/targets/v146";
pub(crate) const VALIDATION_TARGET_147_QUERY_146: &str = "#/$defs/targets/v147";
pub(crate) const VALIDATION_TARGET_148_QUERY_147: &str = "#/$defs/targets/v148";
pub(crate) const VALIDATION_TARGET_149_QUERY_148: &str = "#/$defs/targets/v149";
pub(crate) const VALIDATION_TARGET_150_QUERY_149: &str = "#/$defs/targets/v150";
pub(crate) const VALIDATION_TARGET_151_QUERY_150: &str = "#/$defs/targets/v151";
pub(crate) const VALIDATION_TARGET_152_QUERY_151: &str = "#/$defs/targets/v152";
pub(crate) const VALIDATION_TARGET_153_QUERY_152: &str = "#/$defs/targets/v153";
pub(crate) const VALIDATION_TARGET_154_QUERY_153: &str = "#/$defs/targets/v154";
pub(crate) const VALIDATION_TARGET_155_QUERY_154: &str = "#/$defs/targets/v155";
pub(crate) const VALIDATION_TARGET_156_QUERY_155: &str = "#/$defs/targets/v156";
pub(crate) const VALIDATION_TARGET_157_QUERY_156: &str = "#/$defs/targets/v157";
pub(crate) const VALIDATION_TARGET_158_QUERY_157: &str = "#/$defs/targets/v158";
pub(crate) const VALIDATION_TARGET_159_QUERY_158: &str = "#/$defs/targets/v159";
pub(crate) const VALIDATION_TARGET_160_QUERY_159: &str = "#/$defs/targets/v160";
pub(crate) const VALIDATION_TARGET_161_QUERY_160: &str = "#/$defs/targets/v161";
pub(crate) const VALIDATION_TARGET_162_QUERY_161: &str = "#/$defs/targets/v162";
pub(crate) const VALIDATION_TARGET_163_QUERY_162: &str = "#/$defs/targets/v163";
pub(crate) const VALIDATION_TARGET_164_QUERY_163: &str = "#/$defs/targets/v164";
pub(crate) const VALIDATION_TARGET_165_QUERY_164: &str = "#/$defs/targets/v165";
pub(crate) const VALIDATION_TARGET_166_QUERY_165: &str = "#/$defs/targets/v166";
pub(crate) const VALIDATION_TARGET_167_QUERY_166: &str = "#/$defs/targets/v167";
pub(crate) const VALIDATION_TARGET_168_QUERY_167: &str = "#/$defs/targets/v168";
pub(crate) const VALIDATION_TARGET_169_QUERY_168: &str = "#/$defs/targets/v169";
pub(crate) const VALIDATION_TARGET_170_QUERY_169: &str = "#/$defs/targets/v170";
pub(crate) const VALIDATION_TARGET_171_QUERY_170: &str = "#/$defs/targets/v171";
pub(crate) const VALIDATION_TARGET_172_QUERY_171: &str = "#/$defs/targets/v172";
pub(crate) const VALIDATION_TARGET_173_QUERY_172: &str = "#/$defs/targets/v173";
pub(crate) const VALIDATION_TARGET_174_QUERY_173: &str = "#/$defs/targets/v174";
pub(crate) const VALIDATION_TARGET_175_QUERY_174: &str = "#/$defs/targets/v175";
pub(crate) const VALIDATION_TARGET_176_QUERY_175: &str = "#/$defs/targets/v176";
pub(crate) const VALIDATION_TARGET_177_QUERY_176: &str = "#/$defs/targets/v177";
pub(crate) const VALIDATION_TARGET_178_QUERY_177: &str = "#/$defs/targets/v178";
pub(crate) const VALIDATION_TARGET_179_QUERY_178: &str = "#/$defs/targets/v179";
pub(crate) const VALIDATION_TARGET_180_QUERY_179: &str = "#/$defs/targets/v180";
pub(crate) const VALIDATION_TARGET_181_QUERY_180: &str = "#/$defs/targets/v181";
pub(crate) const VALIDATION_TARGET_182_QUERY_181: &str = "#/$defs/targets/v182";
pub(crate) const VALIDATION_TARGET_183_QUERY_182: &str = "#/$defs/targets/v183";
pub(crate) const VALIDATION_TARGET_184_QUERY_183: &str = "#/$defs/targets/v184";
pub(crate) const VALIDATION_TARGET_185_QUERY_184: &str = "#/$defs/targets/v185";
pub(crate) const VALIDATION_TARGET_186_QUERY_185: &str = "#/$defs/targets/v186";
pub(crate) const VALIDATION_TARGET_187_QUERY_186: &str = "#/$defs/targets/v187";
pub(crate) const VALIDATION_TARGET_188_QUERY_187: &str = "#/$defs/targets/v188";
pub(crate) const VALIDATION_TARGET_189_QUERY_188: &str = "#/$defs/targets/v189";
pub(crate) const VALIDATION_TARGET_190_QUERY_189: &str = "#/$defs/targets/v190";
pub(crate) const VALIDATION_TARGET_191_QUERY_190: &str = "#/$defs/targets/v191";
pub(crate) const VALIDATION_TARGET_192_QUERY_191: &str = "#/$defs/targets/v192";
pub(crate) const VALIDATION_TARGET_193_QUERY_192: &str = "#/$defs/targets/v193";
pub(crate) const VALIDATION_TARGET_194_QUERY_193: &str = "#/$defs/targets/v194";
pub(crate) const VALIDATION_TARGET_195_QUERY_194: &str = "#/$defs/targets/v195";
pub(crate) const VALIDATION_TARGET_196_QUERY_195: &str = "#/$defs/targets/v196";
pub(crate) const VALIDATION_TARGET_197_QUERY_196: &str = "#/$defs/targets/v197";
pub(crate) const VALIDATION_TARGET_198_QUERY_197: &str = "#/$defs/targets/v198";
pub(crate) const VALIDATION_TARGET_199_QUERY_198: &str = "#/$defs/targets/v199";
pub(crate) const VALIDATION_TARGET_200_QUERY_199: &str = "#/$defs/targets/v200";
pub(crate) const VALIDATION_TARGET_201_QUERY_200: &str = "#/$defs/targets/v201";
pub(crate) const VALIDATION_TARGET_202_QUERY_201: &str = "#/$defs/targets/v202";
pub(crate) const VALIDATION_TARGET_203_QUERY_202: &str = "#/$defs/targets/v203";
pub(crate) const VALIDATION_TARGET_204_QUERY_203: &str = "#/$defs/targets/v204";
pub(crate) const VALIDATION_TARGET_205_QUERY_204: &str = "#/$defs/targets/v205";
pub(crate) const VALIDATION_TARGET_206_PATH_0: &str = "#/$defs/targets/v206";
pub(crate) const VALIDATION_TARGET_207_QUERY_1: &str = "#/$defs/targets/v207";
pub(crate) const VALIDATION_TARGET_208_PATH_0: &str = "#/$defs/targets/v208";
pub(crate) const VALIDATION_TARGET_209_QUERY_1: &str = "#/$defs/targets/v209";
pub(crate) const VALIDATION_TARGET_210_QUERY_0: &str = "#/$defs/targets/v210";
pub(crate) const VALIDATION_TARGET_211_QUERY_1: &str = "#/$defs/targets/v211";
pub(crate) const VALIDATION_TARGET_212_QUERY_2: &str = "#/$defs/targets/v212";
pub(crate) const VALIDATION_TARGET_213_QUERY_3: &str = "#/$defs/targets/v213";
pub(crate) const VALIDATION_TARGET_214_QUERY_4: &str = "#/$defs/targets/v214";
pub(crate) const VALIDATION_TARGET_215_QUERY_5: &str = "#/$defs/targets/v215";
pub(crate) const VALIDATION_TARGET_216_QUERY_6: &str = "#/$defs/targets/v216";
pub(crate) const VALIDATION_TARGET_217_QUERY_7: &str = "#/$defs/targets/v217";
pub(crate) const VALIDATION_TARGET_218_QUERY_8: &str = "#/$defs/targets/v218";
pub(crate) const VALIDATION_TARGET_219_QUERY_9: &str = "#/$defs/targets/v219";
pub(crate) const VALIDATION_TARGET_220_QUERY_10: &str = "#/$defs/targets/v220";
pub(crate) const VALIDATION_TARGET_221_QUERY_11: &str = "#/$defs/targets/v221";
pub(crate) const VALIDATION_TARGET_222_QUERY_12: &str = "#/$defs/targets/v222";
pub(crate) const VALIDATION_TARGET_223_QUERY_13: &str = "#/$defs/targets/v223";
pub(crate) const VALIDATION_TARGET_224_QUERY_14: &str = "#/$defs/targets/v224";
pub(crate) const VALIDATION_TARGET_225_QUERY_15: &str = "#/$defs/targets/v225";
pub(crate) const VALIDATION_TARGET_226_QUERY_16: &str = "#/$defs/targets/v226";
pub(crate) const VALIDATION_TARGET_227_QUERY_17: &str = "#/$defs/targets/v227";
pub(crate) const VALIDATION_TARGET_228_QUERY_18: &str = "#/$defs/targets/v228";
pub(crate) const VALIDATION_TARGET_229_QUERY_19: &str = "#/$defs/targets/v229";
pub(crate) const VALIDATION_TARGET_230_QUERY_20: &str = "#/$defs/targets/v230";
pub(crate) const VALIDATION_TARGET_231_QUERY_21: &str = "#/$defs/targets/v231";
pub(crate) const VALIDATION_TARGET_232_QUERY_22: &str = "#/$defs/targets/v232";
pub(crate) const VALIDATION_TARGET_233_QUERY_23: &str = "#/$defs/targets/v233";
pub(crate) const VALIDATION_TARGET_234_QUERY_24: &str = "#/$defs/targets/v234";
pub(crate) const VALIDATION_TARGET_235_QUERY_25: &str = "#/$defs/targets/v235";
pub(crate) const VALIDATION_TARGET_236_QUERY_26: &str = "#/$defs/targets/v236";
pub(crate) const VALIDATION_TARGET_237_QUERY_27: &str = "#/$defs/targets/v237";
pub(crate) const VALIDATION_TARGET_238_QUERY_28: &str = "#/$defs/targets/v238";
pub(crate) const VALIDATION_TARGET_239_QUERY_29: &str = "#/$defs/targets/v239";
pub(crate) const VALIDATION_TARGET_240_QUERY_30: &str = "#/$defs/targets/v240";
pub(crate) const VALIDATION_TARGET_241_QUERY_31: &str = "#/$defs/targets/v241";
pub(crate) const VALIDATION_TARGET_242_QUERY_32: &str = "#/$defs/targets/v242";
pub(crate) const VALIDATION_TARGET_243_QUERY_33: &str = "#/$defs/targets/v243";
pub(crate) const VALIDATION_TARGET_244_QUERY_34: &str = "#/$defs/targets/v244";
pub(crate) const VALIDATION_TARGET_245_QUERY_35: &str = "#/$defs/targets/v245";
pub(crate) const VALIDATION_TARGET_246_QUERY_36: &str = "#/$defs/targets/v246";
pub(crate) const VALIDATION_TARGET_247_QUERY_37: &str = "#/$defs/targets/v247";
pub(crate) const VALIDATION_TARGET_248_QUERY_38: &str = "#/$defs/targets/v248";
pub(crate) const VALIDATION_TARGET_249_QUERY_39: &str = "#/$defs/targets/v249";
pub(crate) const VALIDATION_TARGET_250_QUERY_40: &str = "#/$defs/targets/v250";
pub(crate) const VALIDATION_TARGET_251_QUERY_41: &str = "#/$defs/targets/v251";
pub(crate) const VALIDATION_TARGET_252_QUERY_42: &str = "#/$defs/targets/v252";
pub(crate) const VALIDATION_TARGET_253_QUERY_43: &str = "#/$defs/targets/v253";
pub(crate) const VALIDATION_TARGET_254_QUERY_44: &str = "#/$defs/targets/v254";
pub(crate) const VALIDATION_TARGET_255_QUERY_45: &str = "#/$defs/targets/v255";
pub(crate) const VALIDATION_TARGET_256_QUERY_46: &str = "#/$defs/targets/v256";
pub(crate) const VALIDATION_TARGET_257_QUERY_47: &str = "#/$defs/targets/v257";
pub(crate) const VALIDATION_TARGET_258_QUERY_48: &str = "#/$defs/targets/v258";
pub(crate) const VALIDATION_TARGET_259_QUERY_49: &str = "#/$defs/targets/v259";
pub(crate) const VALIDATION_TARGET_260_QUERY_50: &str = "#/$defs/targets/v260";
pub(crate) const VALIDATION_TARGET_261_QUERY_51: &str = "#/$defs/targets/v261";
pub(crate) const VALIDATION_TARGET_262_QUERY_52: &str = "#/$defs/targets/v262";
pub(crate) const VALIDATION_TARGET_263_QUERY_53: &str = "#/$defs/targets/v263";
pub(crate) const VALIDATION_TARGET_264_QUERY_54: &str = "#/$defs/targets/v264";
pub(crate) const VALIDATION_TARGET_265_QUERY_55: &str = "#/$defs/targets/v265";
pub(crate) const VALIDATION_TARGET_266_QUERY_56: &str = "#/$defs/targets/v266";
pub(crate) const VALIDATION_TARGET_267_QUERY_57: &str = "#/$defs/targets/v267";
pub(crate) const VALIDATION_TARGET_268_QUERY_58: &str = "#/$defs/targets/v268";
pub(crate) const VALIDATION_TARGET_269_QUERY_59: &str = "#/$defs/targets/v269";
pub(crate) const VALIDATION_TARGET_270_QUERY_60: &str = "#/$defs/targets/v270";
pub(crate) const VALIDATION_TARGET_271_QUERY_61: &str = "#/$defs/targets/v271";
pub(crate) const VALIDATION_TARGET_272_QUERY_62: &str = "#/$defs/targets/v272";
pub(crate) const VALIDATION_TARGET_273_QUERY_63: &str = "#/$defs/targets/v273";
pub(crate) const VALIDATION_TARGET_274_QUERY_64: &str = "#/$defs/targets/v274";
pub(crate) const VALIDATION_TARGET_275_QUERY_65: &str = "#/$defs/targets/v275";
pub(crate) const VALIDATION_TARGET_276_QUERY_66: &str = "#/$defs/targets/v276";
pub(crate) const VALIDATION_TARGET_277_QUERY_67: &str = "#/$defs/targets/v277";
pub(crate) const VALIDATION_TARGET_278_QUERY_68: &str = "#/$defs/targets/v278";
pub(crate) const VALIDATION_TARGET_279_QUERY_69: &str = "#/$defs/targets/v279";
pub(crate) const VALIDATION_TARGET_280_QUERY_70: &str = "#/$defs/targets/v280";
pub(crate) const VALIDATION_TARGET_281_QUERY_71: &str = "#/$defs/targets/v281";
pub(crate) const VALIDATION_TARGET_282_QUERY_72: &str = "#/$defs/targets/v282";
pub(crate) const VALIDATION_TARGET_283_QUERY_73: &str = "#/$defs/targets/v283";
pub(crate) const VALIDATION_TARGET_284_QUERY_74: &str = "#/$defs/targets/v284";
pub(crate) const VALIDATION_TARGET_285_QUERY_75: &str = "#/$defs/targets/v285";
pub(crate) const VALIDATION_TARGET_286_QUERY_76: &str = "#/$defs/targets/v286";
pub(crate) const VALIDATION_TARGET_287_QUERY_77: &str = "#/$defs/targets/v287";
pub(crate) const VALIDATION_TARGET_288_QUERY_78: &str = "#/$defs/targets/v288";
pub(crate) const VALIDATION_TARGET_289_QUERY_79: &str = "#/$defs/targets/v289";
pub(crate) const VALIDATION_TARGET_290_QUERY_80: &str = "#/$defs/targets/v290";
pub(crate) const VALIDATION_TARGET_291_QUERY_81: &str = "#/$defs/targets/v291";
pub(crate) const VALIDATION_TARGET_292_QUERY_82: &str = "#/$defs/targets/v292";
pub(crate) const VALIDATION_TARGET_293_QUERY_83: &str = "#/$defs/targets/v293";
pub(crate) const VALIDATION_TARGET_294_QUERY_84: &str = "#/$defs/targets/v294";
pub(crate) const VALIDATION_TARGET_295_QUERY_85: &str = "#/$defs/targets/v295";
pub(crate) const VALIDATION_TARGET_296_QUERY_86: &str = "#/$defs/targets/v296";
pub(crate) const VALIDATION_TARGET_297_QUERY_87: &str = "#/$defs/targets/v297";
pub(crate) const VALIDATION_TARGET_298_QUERY_88: &str = "#/$defs/targets/v298";
pub(crate) const VALIDATION_TARGET_299_QUERY_89: &str = "#/$defs/targets/v299";
pub(crate) const VALIDATION_TARGET_300_QUERY_90: &str = "#/$defs/targets/v300";
pub(crate) const VALIDATION_TARGET_301_QUERY_91: &str = "#/$defs/targets/v301";
pub(crate) const VALIDATION_TARGET_302_QUERY_92: &str = "#/$defs/targets/v302";
pub(crate) const VALIDATION_TARGET_303_QUERY_93: &str = "#/$defs/targets/v303";
pub(crate) const VALIDATION_TARGET_304_QUERY_94: &str = "#/$defs/targets/v304";
pub(crate) const VALIDATION_TARGET_305_QUERY_95: &str = "#/$defs/targets/v305";
pub(crate) const VALIDATION_TARGET_306_QUERY_96: &str = "#/$defs/targets/v306";
pub(crate) const VALIDATION_TARGET_307_QUERY_97: &str = "#/$defs/targets/v307";
pub(crate) const VALIDATION_TARGET_308_QUERY_98: &str = "#/$defs/targets/v308";
pub(crate) const VALIDATION_TARGET_309_QUERY_99: &str = "#/$defs/targets/v309";
pub(crate) const VALIDATION_TARGET_310_QUERY_100: &str = "#/$defs/targets/v310";
pub(crate) const VALIDATION_TARGET_311_QUERY_101: &str = "#/$defs/targets/v311";
pub(crate) const VALIDATION_TARGET_312_QUERY_102: &str = "#/$defs/targets/v312";
pub(crate) const VALIDATION_TARGET_313_QUERY_103: &str = "#/$defs/targets/v313";
pub(crate) const VALIDATION_TARGET_314_QUERY_104: &str = "#/$defs/targets/v314";
pub(crate) const VALIDATION_TARGET_315_QUERY_105: &str = "#/$defs/targets/v315";
pub(crate) const VALIDATION_TARGET_316_QUERY_106: &str = "#/$defs/targets/v316";
pub(crate) const VALIDATION_TARGET_317_QUERY_107: &str = "#/$defs/targets/v317";
pub(crate) const VALIDATION_TARGET_318_QUERY_108: &str = "#/$defs/targets/v318";
pub(crate) const VALIDATION_TARGET_319_QUERY_109: &str = "#/$defs/targets/v319";
pub(crate) const VALIDATION_TARGET_320_QUERY_110: &str = "#/$defs/targets/v320";
pub(crate) const VALIDATION_TARGET_321_QUERY_111: &str = "#/$defs/targets/v321";
pub(crate) const VALIDATION_TARGET_322_QUERY_112: &str = "#/$defs/targets/v322";
pub(crate) const VALIDATION_TARGET_323_QUERY_113: &str = "#/$defs/targets/v323";
pub(crate) const VALIDATION_TARGET_324_QUERY_114: &str = "#/$defs/targets/v324";
pub(crate) const VALIDATION_TARGET_325_QUERY_115: &str = "#/$defs/targets/v325";
pub(crate) const VALIDATION_TARGET_326_QUERY_116: &str = "#/$defs/targets/v326";
pub(crate) const VALIDATION_TARGET_327_QUERY_117: &str = "#/$defs/targets/v327";
pub(crate) const VALIDATION_TARGET_328_QUERY_118: &str = "#/$defs/targets/v328";
pub(crate) const VALIDATION_TARGET_329_QUERY_119: &str = "#/$defs/targets/v329";
pub(crate) const VALIDATION_TARGET_330_QUERY_120: &str = "#/$defs/targets/v330";
pub(crate) const VALIDATION_TARGET_331_QUERY_121: &str = "#/$defs/targets/v331";
pub(crate) const VALIDATION_TARGET_332_QUERY_122: &str = "#/$defs/targets/v332";
pub(crate) const VALIDATION_TARGET_333_QUERY_123: &str = "#/$defs/targets/v333";
pub(crate) const VALIDATION_TARGET_334_QUERY_124: &str = "#/$defs/targets/v334";
pub(crate) const VALIDATION_TARGET_335_QUERY_125: &str = "#/$defs/targets/v335";
pub(crate) const VALIDATION_TARGET_336_QUERY_126: &str = "#/$defs/targets/v336";
pub(crate) const VALIDATION_TARGET_337_QUERY_127: &str = "#/$defs/targets/v337";
pub(crate) const VALIDATION_TARGET_338_QUERY_128: &str = "#/$defs/targets/v338";
pub(crate) const VALIDATION_TARGET_339_QUERY_129: &str = "#/$defs/targets/v339";
pub(crate) const VALIDATION_TARGET_340_QUERY_130: &str = "#/$defs/targets/v340";
pub(crate) const VALIDATION_TARGET_341_QUERY_131: &str = "#/$defs/targets/v341";
pub(crate) const VALIDATION_TARGET_342_QUERY_132: &str = "#/$defs/targets/v342";
pub(crate) const VALIDATION_TARGET_343_QUERY_133: &str = "#/$defs/targets/v343";
pub(crate) const VALIDATION_TARGET_344_QUERY_134: &str = "#/$defs/targets/v344";
pub(crate) const VALIDATION_TARGET_345_QUERY_135: &str = "#/$defs/targets/v345";
pub(crate) const VALIDATION_TARGET_346_QUERY_136: &str = "#/$defs/targets/v346";
pub(crate) const VALIDATION_TARGET_347_QUERY_137: &str = "#/$defs/targets/v347";
pub(crate) const VALIDATION_TARGET_348_QUERY_138: &str = "#/$defs/targets/v348";
pub(crate) const VALIDATION_TARGET_349_QUERY_139: &str = "#/$defs/targets/v349";
pub(crate) const VALIDATION_TARGET_350_QUERY_140: &str = "#/$defs/targets/v350";
pub(crate) const VALIDATION_TARGET_351_QUERY_141: &str = "#/$defs/targets/v351";
pub(crate) const VALIDATION_TARGET_352_QUERY_142: &str = "#/$defs/targets/v352";
pub(crate) const VALIDATION_TARGET_353_QUERY_143: &str = "#/$defs/targets/v353";
pub(crate) const VALIDATION_TARGET_354_QUERY_144: &str = "#/$defs/targets/v354";
pub(crate) const VALIDATION_TARGET_355_QUERY_145: &str = "#/$defs/targets/v355";
pub(crate) const VALIDATION_TARGET_356_QUERY_146: &str = "#/$defs/targets/v356";
pub(crate) const VALIDATION_TARGET_357_QUERY_147: &str = "#/$defs/targets/v357";
pub(crate) const VALIDATION_TARGET_358_QUERY_148: &str = "#/$defs/targets/v358";
pub(crate) const VALIDATION_TARGET_359_QUERY_149: &str = "#/$defs/targets/v359";
pub(crate) const VALIDATION_TARGET_360_QUERY_150: &str = "#/$defs/targets/v360";
pub(crate) const VALIDATION_TARGET_361_QUERY_151: &str = "#/$defs/targets/v361";
pub(crate) const VALIDATION_TARGET_362_QUERY_152: &str = "#/$defs/targets/v362";
pub(crate) const VALIDATION_TARGET_363_QUERY_153: &str = "#/$defs/targets/v363";
pub(crate) const VALIDATION_TARGET_364_QUERY_154: &str = "#/$defs/targets/v364";
pub(crate) const VALIDATION_TARGET_365_QUERY_155: &str = "#/$defs/targets/v365";
pub(crate) const VALIDATION_TARGET_366_QUERY_156: &str = "#/$defs/targets/v366";
pub(crate) const VALIDATION_TARGET_367_QUERY_157: &str = "#/$defs/targets/v367";
pub(crate) const VALIDATION_TARGET_368_QUERY_158: &str = "#/$defs/targets/v368";
pub(crate) const VALIDATION_TARGET_369_QUERY_159: &str = "#/$defs/targets/v369";
pub(crate) const VALIDATION_TARGET_370_QUERY_160: &str = "#/$defs/targets/v370";
pub(crate) const VALIDATION_TARGET_371_QUERY_161: &str = "#/$defs/targets/v371";
pub(crate) const VALIDATION_TARGET_372_QUERY_162: &str = "#/$defs/targets/v372";
pub(crate) const VALIDATION_TARGET_373_QUERY_163: &str = "#/$defs/targets/v373";
pub(crate) const VALIDATION_TARGET_374_QUERY_164: &str = "#/$defs/targets/v374";
pub(crate) const VALIDATION_TARGET_375_QUERY_165: &str = "#/$defs/targets/v375";
pub(crate) const VALIDATION_TARGET_376_QUERY_166: &str = "#/$defs/targets/v376";
pub(crate) const VALIDATION_TARGET_377_QUERY_167: &str = "#/$defs/targets/v377";
pub(crate) const VALIDATION_TARGET_378_QUERY_168: &str = "#/$defs/targets/v378";
pub(crate) const VALIDATION_TARGET_379_QUERY_169: &str = "#/$defs/targets/v379";
pub(crate) const VALIDATION_TARGET_380_QUERY_170: &str = "#/$defs/targets/v380";
pub(crate) const VALIDATION_TARGET_381_BODY: &str = "#/$defs/targets/v381";
pub(crate) const VALIDATION_TARGET_382_PATH_0: &str = "#/$defs/targets/v382";
pub(crate) const VALIDATION_TARGET_383_QUERY_1: &str = "#/$defs/targets/v383";
pub(crate) const VALIDATION_TARGET_384_BODY: &str = "#/$defs/targets/v384";
pub(crate) const VALIDATION_TARGET_385_QUERY_0: &str = "#/$defs/targets/v385";
pub(crate) const VALIDATION_TARGET_386_PATH_0: &str = "#/$defs/targets/v386";
pub(crate) const VALIDATION_TARGET_387_QUERY_1: &str = "#/$defs/targets/v387";
pub(crate) const VALIDATION_TARGET_388_PATH_0: &str = "#/$defs/targets/v388";
pub(crate) const VALIDATION_TARGET_389_QUERY_1: &str = "#/$defs/targets/v389";
pub(crate) const VALIDATION_TARGET_390_QUERY_0: &str = "#/$defs/targets/v390";
pub(crate) const VALIDATION_TARGET_391_QUERY_1: &str = "#/$defs/targets/v391";
pub(crate) const VALIDATION_TARGET_392_QUERY_2: &str = "#/$defs/targets/v392";
pub(crate) const VALIDATION_TARGET_393_QUERY_3: &str = "#/$defs/targets/v393";
pub(crate) const VALIDATION_TARGET_394_QUERY_4: &str = "#/$defs/targets/v394";
pub(crate) const VALIDATION_TARGET_395_QUERY_5: &str = "#/$defs/targets/v395";
pub(crate) const VALIDATION_TARGET_396_QUERY_6: &str = "#/$defs/targets/v396";
pub(crate) const VALIDATION_TARGET_397_QUERY_7: &str = "#/$defs/targets/v397";
pub(crate) const VALIDATION_TARGET_398_QUERY_8: &str = "#/$defs/targets/v398";
pub(crate) const VALIDATION_TARGET_399_QUERY_9: &str = "#/$defs/targets/v399";
pub(crate) const VALIDATION_TARGET_400_QUERY_10: &str = "#/$defs/targets/v400";
pub(crate) const VALIDATION_TARGET_401_QUERY_11: &str = "#/$defs/targets/v401";
pub(crate) const VALIDATION_TARGET_402_QUERY_12: &str = "#/$defs/targets/v402";
pub(crate) const VALIDATION_TARGET_403_QUERY_13: &str = "#/$defs/targets/v403";
pub(crate) const VALIDATION_TARGET_404_QUERY_14: &str = "#/$defs/targets/v404";
pub(crate) const VALIDATION_TARGET_405_QUERY_15: &str = "#/$defs/targets/v405";
pub(crate) const VALIDATION_TARGET_406_QUERY_16: &str = "#/$defs/targets/v406";
pub(crate) const VALIDATION_TARGET_407_QUERY_17: &str = "#/$defs/targets/v407";
pub(crate) const VALIDATION_TARGET_408_QUERY_18: &str = "#/$defs/targets/v408";
pub(crate) const VALIDATION_TARGET_409_QUERY_19: &str = "#/$defs/targets/v409";
pub(crate) const VALIDATION_TARGET_410_QUERY_20: &str = "#/$defs/targets/v410";
pub(crate) const VALIDATION_TARGET_411_QUERY_21: &str = "#/$defs/targets/v411";
pub(crate) const VALIDATION_TARGET_412_QUERY_22: &str = "#/$defs/targets/v412";
pub(crate) const VALIDATION_TARGET_413_QUERY_23: &str = "#/$defs/targets/v413";
pub(crate) const VALIDATION_TARGET_414_QUERY_24: &str = "#/$defs/targets/v414";
pub(crate) const VALIDATION_TARGET_415_QUERY_25: &str = "#/$defs/targets/v415";
pub(crate) const VALIDATION_TARGET_416_QUERY_26: &str = "#/$defs/targets/v416";
pub(crate) const VALIDATION_TARGET_417_QUERY_27: &str = "#/$defs/targets/v417";
pub(crate) const VALIDATION_TARGET_418_QUERY_28: &str = "#/$defs/targets/v418";
pub(crate) const VALIDATION_TARGET_419_QUERY_29: &str = "#/$defs/targets/v419";
pub(crate) const VALIDATION_TARGET_420_QUERY_30: &str = "#/$defs/targets/v420";
pub(crate) const VALIDATION_TARGET_421_QUERY_31: &str = "#/$defs/targets/v421";
pub(crate) const VALIDATION_TARGET_422_QUERY_32: &str = "#/$defs/targets/v422";
pub(crate) const VALIDATION_TARGET_423_QUERY_33: &str = "#/$defs/targets/v423";
pub(crate) const VALIDATION_TARGET_424_QUERY_34: &str = "#/$defs/targets/v424";
pub(crate) const VALIDATION_TARGET_425_QUERY_35: &str = "#/$defs/targets/v425";
pub(crate) const VALIDATION_TARGET_426_QUERY_36: &str = "#/$defs/targets/v426";
pub(crate) const VALIDATION_TARGET_427_QUERY_37: &str = "#/$defs/targets/v427";
pub(crate) const VALIDATION_TARGET_428_QUERY_38: &str = "#/$defs/targets/v428";
pub(crate) const VALIDATION_TARGET_429_QUERY_39: &str = "#/$defs/targets/v429";
pub(crate) const VALIDATION_TARGET_430_QUERY_40: &str = "#/$defs/targets/v430";
pub(crate) const VALIDATION_TARGET_431_QUERY_41: &str = "#/$defs/targets/v431";
pub(crate) const VALIDATION_TARGET_432_QUERY_42: &str = "#/$defs/targets/v432";
pub(crate) const VALIDATION_TARGET_433_QUERY_43: &str = "#/$defs/targets/v433";
pub(crate) const VALIDATION_TARGET_434_QUERY_44: &str = "#/$defs/targets/v434";
pub(crate) const VALIDATION_TARGET_435_QUERY_45: &str = "#/$defs/targets/v435";
pub(crate) const VALIDATION_TARGET_436_QUERY_46: &str = "#/$defs/targets/v436";
pub(crate) const VALIDATION_TARGET_437_QUERY_47: &str = "#/$defs/targets/v437";
pub(crate) const VALIDATION_TARGET_438_QUERY_48: &str = "#/$defs/targets/v438";
pub(crate) const VALIDATION_TARGET_439_QUERY_49: &str = "#/$defs/targets/v439";
pub(crate) const VALIDATION_TARGET_440_QUERY_50: &str = "#/$defs/targets/v440";
pub(crate) const VALIDATION_TARGET_441_QUERY_51: &str = "#/$defs/targets/v441";
pub(crate) const VALIDATION_TARGET_442_QUERY_52: &str = "#/$defs/targets/v442";
pub(crate) const VALIDATION_TARGET_443_QUERY_53: &str = "#/$defs/targets/v443";
pub(crate) const VALIDATION_TARGET_444_QUERY_54: &str = "#/$defs/targets/v444";
pub(crate) const VALIDATION_TARGET_445_QUERY_55: &str = "#/$defs/targets/v445";
pub(crate) const VALIDATION_TARGET_446_QUERY_56: &str = "#/$defs/targets/v446";
pub(crate) const VALIDATION_TARGET_447_QUERY_57: &str = "#/$defs/targets/v447";
pub(crate) const VALIDATION_TARGET_448_QUERY_58: &str = "#/$defs/targets/v448";
pub(crate) const VALIDATION_TARGET_449_QUERY_59: &str = "#/$defs/targets/v449";
pub(crate) const VALIDATION_TARGET_450_QUERY_60: &str = "#/$defs/targets/v450";
pub(crate) const VALIDATION_TARGET_451_QUERY_61: &str = "#/$defs/targets/v451";
pub(crate) const VALIDATION_TARGET_452_QUERY_62: &str = "#/$defs/targets/v452";
pub(crate) const VALIDATION_TARGET_453_QUERY_63: &str = "#/$defs/targets/v453";
pub(crate) const VALIDATION_TARGET_454_QUERY_64: &str = "#/$defs/targets/v454";
pub(crate) const VALIDATION_TARGET_455_QUERY_65: &str = "#/$defs/targets/v455";
pub(crate) const VALIDATION_TARGET_456_QUERY_66: &str = "#/$defs/targets/v456";
pub(crate) const VALIDATION_TARGET_457_QUERY_67: &str = "#/$defs/targets/v457";
pub(crate) const VALIDATION_TARGET_458_QUERY_68: &str = "#/$defs/targets/v458";
pub(crate) const VALIDATION_TARGET_459_QUERY_69: &str = "#/$defs/targets/v459";
pub(crate) const VALIDATION_TARGET_460_QUERY_70: &str = "#/$defs/targets/v460";
pub(crate) const VALIDATION_TARGET_461_QUERY_71: &str = "#/$defs/targets/v461";
pub(crate) const VALIDATION_TARGET_462_QUERY_72: &str = "#/$defs/targets/v462";
pub(crate) const VALIDATION_TARGET_463_QUERY_73: &str = "#/$defs/targets/v463";
pub(crate) const VALIDATION_TARGET_464_QUERY_74: &str = "#/$defs/targets/v464";
pub(crate) const VALIDATION_TARGET_465_QUERY_75: &str = "#/$defs/targets/v465";
pub(crate) const VALIDATION_TARGET_466_QUERY_76: &str = "#/$defs/targets/v466";
pub(crate) const VALIDATION_TARGET_467_QUERY_77: &str = "#/$defs/targets/v467";
pub(crate) const VALIDATION_TARGET_468_QUERY_78: &str = "#/$defs/targets/v468";
pub(crate) const VALIDATION_TARGET_469_QUERY_79: &str = "#/$defs/targets/v469";
pub(crate) const VALIDATION_TARGET_470_QUERY_80: &str = "#/$defs/targets/v470";
pub(crate) const VALIDATION_TARGET_471_QUERY_81: &str = "#/$defs/targets/v471";
pub(crate) const VALIDATION_TARGET_472_QUERY_82: &str = "#/$defs/targets/v472";
pub(crate) const VALIDATION_TARGET_473_QUERY_83: &str = "#/$defs/targets/v473";
pub(crate) const VALIDATION_TARGET_474_QUERY_84: &str = "#/$defs/targets/v474";
pub(crate) const VALIDATION_TARGET_475_QUERY_85: &str = "#/$defs/targets/v475";
pub(crate) const VALIDATION_TARGET_476_QUERY_86: &str = "#/$defs/targets/v476";
pub(crate) const VALIDATION_TARGET_477_QUERY_87: &str = "#/$defs/targets/v477";
pub(crate) const VALIDATION_TARGET_478_QUERY_88: &str = "#/$defs/targets/v478";
pub(crate) const VALIDATION_TARGET_479_QUERY_89: &str = "#/$defs/targets/v479";
pub(crate) const VALIDATION_TARGET_480_QUERY_90: &str = "#/$defs/targets/v480";
pub(crate) const VALIDATION_TARGET_481_QUERY_91: &str = "#/$defs/targets/v481";
pub(crate) const VALIDATION_TARGET_482_QUERY_92: &str = "#/$defs/targets/v482";
pub(crate) const VALIDATION_TARGET_483_QUERY_93: &str = "#/$defs/targets/v483";
pub(crate) const VALIDATION_TARGET_484_QUERY_94: &str = "#/$defs/targets/v484";
pub(crate) const VALIDATION_TARGET_485_QUERY_95: &str = "#/$defs/targets/v485";
pub(crate) const VALIDATION_TARGET_486_QUERY_96: &str = "#/$defs/targets/v486";
pub(crate) const VALIDATION_TARGET_487_QUERY_97: &str = "#/$defs/targets/v487";
pub(crate) const VALIDATION_TARGET_488_QUERY_98: &str = "#/$defs/targets/v488";
pub(crate) const VALIDATION_TARGET_489_QUERY_99: &str = "#/$defs/targets/v489";
pub(crate) const VALIDATION_TARGET_490_QUERY_100: &str = "#/$defs/targets/v490";
pub(crate) const VALIDATION_TARGET_491_QUERY_101: &str = "#/$defs/targets/v491";
pub(crate) const VALIDATION_TARGET_492_QUERY_102: &str = "#/$defs/targets/v492";
pub(crate) const VALIDATION_TARGET_493_QUERY_103: &str = "#/$defs/targets/v493";
pub(crate) const VALIDATION_TARGET_494_QUERY_104: &str = "#/$defs/targets/v494";
pub(crate) const VALIDATION_TARGET_495_QUERY_105: &str = "#/$defs/targets/v495";
pub(crate) const VALIDATION_TARGET_496_QUERY_106: &str = "#/$defs/targets/v496";
pub(crate) const VALIDATION_TARGET_497_QUERY_107: &str = "#/$defs/targets/v497";
pub(crate) const VALIDATION_TARGET_498_QUERY_108: &str = "#/$defs/targets/v498";
pub(crate) const VALIDATION_TARGET_499_QUERY_109: &str = "#/$defs/targets/v499";
pub(crate) const VALIDATION_TARGET_500_QUERY_110: &str = "#/$defs/targets/v500";
pub(crate) const VALIDATION_TARGET_501_BODY: &str = "#/$defs/targets/v501";
pub(crate) const VALIDATION_TARGET_502_PATH_0: &str = "#/$defs/targets/v502";
pub(crate) const VALIDATION_TARGET_503_QUERY_1: &str = "#/$defs/targets/v503";
pub(crate) const VALIDATION_TARGET_504_BODY: &str = "#/$defs/targets/v504";
pub(crate) const VALIDATION_TARGET_505_QUERY_0: &str = "#/$defs/targets/v505";
pub(crate) const VALIDATION_TARGET_506_BODY: &str = "#/$defs/targets/v506";
pub(crate) const VALIDATION_TARGET_507_PATH_0: &str = "#/$defs/targets/v507";
pub(crate) const VALIDATION_TARGET_508_QUERY_1: &str = "#/$defs/targets/v508";
pub(crate) const VALIDATION_TARGET_509_BODY: &str = "#/$defs/targets/v509";
pub(crate) const VALIDATION_TARGET_510_PATH_0: &str = "#/$defs/targets/v510";
pub(crate) const VALIDATION_TARGET_511_QUERY_1: &str = "#/$defs/targets/v511";
pub(crate) const VALIDATION_TARGET_512_PATH_0: &str = "#/$defs/targets/v512";
pub(crate) const VALIDATION_TARGET_513_QUERY_1: &str = "#/$defs/targets/v513";
pub(crate) const VALIDATION_TARGET_514_PATH_0: &str = "#/$defs/targets/v514";
pub(crate) const VALIDATION_TARGET_515_QUERY_1: &str = "#/$defs/targets/v515";
pub(crate) const VALIDATION_TARGET_516_QUERY_0: &str = "#/$defs/targets/v516";
pub(crate) const VALIDATION_TARGET_517_QUERY_1: &str = "#/$defs/targets/v517";
pub(crate) const VALIDATION_TARGET_518_QUERY_2: &str = "#/$defs/targets/v518";
pub(crate) const VALIDATION_TARGET_519_QUERY_3: &str = "#/$defs/targets/v519";
pub(crate) const VALIDATION_TARGET_520_QUERY_4: &str = "#/$defs/targets/v520";
pub(crate) const VALIDATION_TARGET_521_QUERY_5: &str = "#/$defs/targets/v521";
pub(crate) const VALIDATION_TARGET_522_QUERY_6: &str = "#/$defs/targets/v522";
pub(crate) const VALIDATION_TARGET_523_QUERY_7: &str = "#/$defs/targets/v523";
pub(crate) const VALIDATION_TARGET_524_QUERY_8: &str = "#/$defs/targets/v524";
pub(crate) const VALIDATION_TARGET_525_QUERY_9: &str = "#/$defs/targets/v525";
pub(crate) const VALIDATION_TARGET_526_QUERY_10: &str = "#/$defs/targets/v526";
pub(crate) const VALIDATION_TARGET_527_QUERY_11: &str = "#/$defs/targets/v527";
pub(crate) const VALIDATION_TARGET_528_QUERY_12: &str = "#/$defs/targets/v528";
pub(crate) const VALIDATION_TARGET_529_QUERY_13: &str = "#/$defs/targets/v529";
pub(crate) const VALIDATION_TARGET_530_QUERY_14: &str = "#/$defs/targets/v530";
pub(crate) const VALIDATION_TARGET_531_QUERY_15: &str = "#/$defs/targets/v531";
pub(crate) const VALIDATION_TARGET_532_QUERY_16: &str = "#/$defs/targets/v532";
pub(crate) const VALIDATION_TARGET_533_QUERY_17: &str = "#/$defs/targets/v533";
pub(crate) const VALIDATION_TARGET_534_QUERY_18: &str = "#/$defs/targets/v534";
pub(crate) const VALIDATION_TARGET_535_QUERY_19: &str = "#/$defs/targets/v535";
pub(crate) const VALIDATION_TARGET_536_QUERY_20: &str = "#/$defs/targets/v536";
pub(crate) const VALIDATION_TARGET_537_QUERY_21: &str = "#/$defs/targets/v537";
pub(crate) const VALIDATION_TARGET_538_QUERY_22: &str = "#/$defs/targets/v538";
pub(crate) const VALIDATION_TARGET_539_QUERY_23: &str = "#/$defs/targets/v539";
pub(crate) const VALIDATION_TARGET_540_QUERY_24: &str = "#/$defs/targets/v540";
pub(crate) const VALIDATION_TARGET_541_QUERY_25: &str = "#/$defs/targets/v541";
pub(crate) const VALIDATION_TARGET_542_QUERY_26: &str = "#/$defs/targets/v542";
pub(crate) const VALIDATION_TARGET_543_QUERY_27: &str = "#/$defs/targets/v543";
pub(crate) const VALIDATION_TARGET_544_QUERY_28: &str = "#/$defs/targets/v544";
pub(crate) const VALIDATION_TARGET_545_QUERY_29: &str = "#/$defs/targets/v545";
pub(crate) const VALIDATION_TARGET_546_QUERY_30: &str = "#/$defs/targets/v546";
pub(crate) const VALIDATION_TARGET_547_QUERY_31: &str = "#/$defs/targets/v547";
pub(crate) const VALIDATION_TARGET_548_QUERY_32: &str = "#/$defs/targets/v548";
pub(crate) const VALIDATION_TARGET_549_QUERY_33: &str = "#/$defs/targets/v549";
pub(crate) const VALIDATION_TARGET_550_QUERY_34: &str = "#/$defs/targets/v550";
pub(crate) const VALIDATION_TARGET_551_QUERY_35: &str = "#/$defs/targets/v551";
pub(crate) const VALIDATION_TARGET_552_QUERY_36: &str = "#/$defs/targets/v552";
pub(crate) const VALIDATION_TARGET_553_QUERY_37: &str = "#/$defs/targets/v553";
pub(crate) const VALIDATION_TARGET_554_QUERY_38: &str = "#/$defs/targets/v554";
pub(crate) const VALIDATION_TARGET_555_QUERY_39: &str = "#/$defs/targets/v555";
pub(crate) const VALIDATION_TARGET_556_QUERY_40: &str = "#/$defs/targets/v556";
pub(crate) const VALIDATION_TARGET_557_QUERY_41: &str = "#/$defs/targets/v557";
pub(crate) const VALIDATION_TARGET_558_QUERY_42: &str = "#/$defs/targets/v558";
pub(crate) const VALIDATION_TARGET_559_QUERY_43: &str = "#/$defs/targets/v559";
pub(crate) const VALIDATION_TARGET_560_QUERY_44: &str = "#/$defs/targets/v560";
pub(crate) const VALIDATION_TARGET_561_QUERY_45: &str = "#/$defs/targets/v561";
pub(crate) const VALIDATION_TARGET_562_QUERY_46: &str = "#/$defs/targets/v562";
pub(crate) const VALIDATION_TARGET_563_QUERY_47: &str = "#/$defs/targets/v563";
pub(crate) const VALIDATION_TARGET_564_QUERY_48: &str = "#/$defs/targets/v564";
pub(crate) const VALIDATION_TARGET_565_QUERY_49: &str = "#/$defs/targets/v565";
pub(crate) const VALIDATION_TARGET_566_QUERY_50: &str = "#/$defs/targets/v566";
pub(crate) const VALIDATION_TARGET_567_QUERY_51: &str = "#/$defs/targets/v567";
pub(crate) const VALIDATION_TARGET_568_QUERY_52: &str = "#/$defs/targets/v568";
pub(crate) const VALIDATION_TARGET_569_QUERY_53: &str = "#/$defs/targets/v569";
pub(crate) const VALIDATION_TARGET_570_QUERY_54: &str = "#/$defs/targets/v570";
pub(crate) const VALIDATION_TARGET_571_QUERY_55: &str = "#/$defs/targets/v571";
pub(crate) const VALIDATION_TARGET_572_QUERY_56: &str = "#/$defs/targets/v572";
pub(crate) const VALIDATION_TARGET_573_QUERY_57: &str = "#/$defs/targets/v573";
pub(crate) const VALIDATION_TARGET_574_QUERY_58: &str = "#/$defs/targets/v574";
pub(crate) const VALIDATION_TARGET_575_QUERY_59: &str = "#/$defs/targets/v575";
pub(crate) const VALIDATION_TARGET_576_QUERY_60: &str = "#/$defs/targets/v576";
pub(crate) const VALIDATION_TARGET_577_QUERY_61: &str = "#/$defs/targets/v577";
pub(crate) const VALIDATION_TARGET_578_QUERY_62: &str = "#/$defs/targets/v578";
pub(crate) const VALIDATION_TARGET_579_QUERY_63: &str = "#/$defs/targets/v579";
pub(crate) const VALIDATION_TARGET_580_QUERY_64: &str = "#/$defs/targets/v580";
pub(crate) const VALIDATION_TARGET_581_QUERY_65: &str = "#/$defs/targets/v581";
pub(crate) const VALIDATION_TARGET_582_QUERY_66: &str = "#/$defs/targets/v582";
pub(crate) const VALIDATION_TARGET_583_QUERY_67: &str = "#/$defs/targets/v583";
pub(crate) const VALIDATION_TARGET_584_QUERY_68: &str = "#/$defs/targets/v584";
pub(crate) const VALIDATION_TARGET_585_QUERY_69: &str = "#/$defs/targets/v585";
pub(crate) const VALIDATION_TARGET_586_QUERY_70: &str = "#/$defs/targets/v586";
pub(crate) const VALIDATION_TARGET_587_QUERY_71: &str = "#/$defs/targets/v587";
pub(crate) const VALIDATION_TARGET_588_QUERY_72: &str = "#/$defs/targets/v588";
pub(crate) const VALIDATION_TARGET_589_QUERY_73: &str = "#/$defs/targets/v589";
pub(crate) const VALIDATION_TARGET_590_QUERY_74: &str = "#/$defs/targets/v590";
pub(crate) const VALIDATION_TARGET_591_QUERY_75: &str = "#/$defs/targets/v591";
pub(crate) const VALIDATION_TARGET_592_QUERY_76: &str = "#/$defs/targets/v592";
pub(crate) const VALIDATION_TARGET_593_QUERY_77: &str = "#/$defs/targets/v593";
pub(crate) const VALIDATION_TARGET_594_QUERY_78: &str = "#/$defs/targets/v594";
pub(crate) const VALIDATION_TARGET_595_QUERY_79: &str = "#/$defs/targets/v595";
pub(crate) const VALIDATION_TARGET_596_QUERY_80: &str = "#/$defs/targets/v596";
pub(crate) const VALIDATION_TARGET_597_QUERY_81: &str = "#/$defs/targets/v597";
pub(crate) const VALIDATION_TARGET_598_QUERY_82: &str = "#/$defs/targets/v598";
pub(crate) const VALIDATION_TARGET_599_QUERY_83: &str = "#/$defs/targets/v599";
pub(crate) const VALIDATION_TARGET_600_QUERY_84: &str = "#/$defs/targets/v600";
pub(crate) const VALIDATION_TARGET_601_QUERY_85: &str = "#/$defs/targets/v601";
pub(crate) const VALIDATION_TARGET_602_QUERY_86: &str = "#/$defs/targets/v602";
pub(crate) const VALIDATION_TARGET_603_QUERY_87: &str = "#/$defs/targets/v603";
pub(crate) const VALIDATION_TARGET_604_QUERY_88: &str = "#/$defs/targets/v604";
pub(crate) const VALIDATION_TARGET_605_QUERY_89: &str = "#/$defs/targets/v605";
pub(crate) const VALIDATION_TARGET_606_QUERY_90: &str = "#/$defs/targets/v606";
pub(crate) const VALIDATION_TARGET_607_QUERY_91: &str = "#/$defs/targets/v607";
pub(crate) const VALIDATION_TARGET_608_QUERY_92: &str = "#/$defs/targets/v608";
pub(crate) const VALIDATION_TARGET_609_QUERY_93: &str = "#/$defs/targets/v609";
pub(crate) const VALIDATION_TARGET_610_QUERY_94: &str = "#/$defs/targets/v610";
pub(crate) const VALIDATION_TARGET_611_QUERY_95: &str = "#/$defs/targets/v611";
pub(crate) const VALIDATION_TARGET_612_QUERY_96: &str = "#/$defs/targets/v612";
pub(crate) const VALIDATION_TARGET_613_QUERY_97: &str = "#/$defs/targets/v613";
pub(crate) const VALIDATION_TARGET_614_QUERY_98: &str = "#/$defs/targets/v614";
pub(crate) const VALIDATION_TARGET_615_QUERY_99: &str = "#/$defs/targets/v615";
pub(crate) const VALIDATION_TARGET_616_QUERY_100: &str = "#/$defs/targets/v616";
pub(crate) const VALIDATION_TARGET_617_QUERY_101: &str = "#/$defs/targets/v617";
pub(crate) const VALIDATION_TARGET_618_QUERY_102: &str = "#/$defs/targets/v618";
pub(crate) const VALIDATION_TARGET_619_QUERY_103: &str = "#/$defs/targets/v619";
pub(crate) const VALIDATION_TARGET_620_QUERY_104: &str = "#/$defs/targets/v620";
pub(crate) const VALIDATION_TARGET_621_QUERY_105: &str = "#/$defs/targets/v621";
pub(crate) const VALIDATION_TARGET_622_QUERY_106: &str = "#/$defs/targets/v622";
pub(crate) const VALIDATION_TARGET_623_QUERY_107: &str = "#/$defs/targets/v623";
pub(crate) const VALIDATION_TARGET_624_QUERY_108: &str = "#/$defs/targets/v624";
pub(crate) const VALIDATION_TARGET_625_QUERY_109: &str = "#/$defs/targets/v625";
pub(crate) const VALIDATION_TARGET_626_BODY: &str = "#/$defs/targets/v626";
pub(crate) const VALIDATION_TARGET_627_PATH_0: &str = "#/$defs/targets/v627";
pub(crate) const VALIDATION_TARGET_628_QUERY_1: &str = "#/$defs/targets/v628";
pub(crate) const VALIDATION_TARGET_629_BODY: &str = "#/$defs/targets/v629";
pub(crate) const VALIDATION_TARGET_630_QUERY_0: &str = "#/$defs/targets/v630";
pub(crate) const VALIDATION_TARGET_631_BODY: &str = "#/$defs/targets/v631";
pub(crate) const VALIDATION_TARGET_632_QUERY_0: &str = "#/$defs/targets/v632";
pub(crate) const VALIDATION_TARGET_633_QUERY_1: &str = "#/$defs/targets/v633";
pub(crate) const VALIDATION_TARGET_634_QUERY_2: &str = "#/$defs/targets/v634";
pub(crate) const VALIDATION_TARGET_635_QUERY_3: &str = "#/$defs/targets/v635";
pub(crate) const VALIDATION_TARGET_636_QUERY_4: &str = "#/$defs/targets/v636";
pub(crate) const VALIDATION_TARGET_637_QUERY_5: &str = "#/$defs/targets/v637";
pub(crate) const VALIDATION_TARGET_638_QUERY_6: &str = "#/$defs/targets/v638";
pub(crate) const VALIDATION_TARGET_639_QUERY_7: &str = "#/$defs/targets/v639";
pub(crate) const VALIDATION_TARGET_640_QUERY_8: &str = "#/$defs/targets/v640";
pub(crate) const VALIDATION_TARGET_641_QUERY_9: &str = "#/$defs/targets/v641";
pub(crate) const VALIDATION_TARGET_642_QUERY_10: &str = "#/$defs/targets/v642";
pub(crate) const VALIDATION_TARGET_643_QUERY_11: &str = "#/$defs/targets/v643";
pub(crate) const VALIDATION_TARGET_644_QUERY_12: &str = "#/$defs/targets/v644";
pub(crate) const VALIDATION_TARGET_645_QUERY_13: &str = "#/$defs/targets/v645";
pub(crate) const VALIDATION_TARGET_646_QUERY_14: &str = "#/$defs/targets/v646";
pub(crate) const VALIDATION_TARGET_647_QUERY_15: &str = "#/$defs/targets/v647";
pub(crate) const VALIDATION_TARGET_648_QUERY_16: &str = "#/$defs/targets/v648";
pub(crate) const VALIDATION_TARGET_649_QUERY_17: &str = "#/$defs/targets/v649";
pub(crate) const VALIDATION_TARGET_650_QUERY_18: &str = "#/$defs/targets/v650";
pub(crate) const VALIDATION_TARGET_651_QUERY_19: &str = "#/$defs/targets/v651";
pub(crate) const VALIDATION_TARGET_652_QUERY_20: &str = "#/$defs/targets/v652";
pub(crate) const VALIDATION_TARGET_653_QUERY_21: &str = "#/$defs/targets/v653";
pub(crate) const VALIDATION_TARGET_654_QUERY_22: &str = "#/$defs/targets/v654";
pub(crate) const VALIDATION_TARGET_655_QUERY_23: &str = "#/$defs/targets/v655";
pub(crate) const VALIDATION_TARGET_656_QUERY_24: &str = "#/$defs/targets/v656";
pub(crate) const VALIDATION_TARGET_657_QUERY_25: &str = "#/$defs/targets/v657";
pub(crate) const VALIDATION_TARGET_658_QUERY_26: &str = "#/$defs/targets/v658";
pub(crate) const VALIDATION_TARGET_659_QUERY_27: &str = "#/$defs/targets/v659";
pub(crate) const VALIDATION_TARGET_660_QUERY_28: &str = "#/$defs/targets/v660";
pub(crate) const VALIDATION_TARGET_661_QUERY_29: &str = "#/$defs/targets/v661";
pub(crate) const VALIDATION_TARGET_662_QUERY_30: &str = "#/$defs/targets/v662";
pub(crate) const VALIDATION_TARGET_663_QUERY_31: &str = "#/$defs/targets/v663";
pub(crate) const VALIDATION_TARGET_664_QUERY_32: &str = "#/$defs/targets/v664";
pub(crate) const VALIDATION_TARGET_665_QUERY_33: &str = "#/$defs/targets/v665";
pub(crate) const VALIDATION_TARGET_666_QUERY_34: &str = "#/$defs/targets/v666";
pub(crate) const VALIDATION_TARGET_667_QUERY_35: &str = "#/$defs/targets/v667";
pub(crate) const VALIDATION_TARGET_668_QUERY_36: &str = "#/$defs/targets/v668";
pub(crate) const VALIDATION_TARGET_669_QUERY_37: &str = "#/$defs/targets/v669";
pub(crate) const VALIDATION_TARGET_670_QUERY_38: &str = "#/$defs/targets/v670";
pub(crate) const VALIDATION_TARGET_671_QUERY_39: &str = "#/$defs/targets/v671";
pub(crate) const VALIDATION_TARGET_672_QUERY_40: &str = "#/$defs/targets/v672";
pub(crate) const VALIDATION_TARGET_673_QUERY_41: &str = "#/$defs/targets/v673";
pub(crate) const VALIDATION_TARGET_674_QUERY_42: &str = "#/$defs/targets/v674";
pub(crate) const VALIDATION_TARGET_675_QUERY_43: &str = "#/$defs/targets/v675";
pub(crate) const VALIDATION_TARGET_676_QUERY_44: &str = "#/$defs/targets/v676";
pub(crate) const VALIDATION_TARGET_677_QUERY_45: &str = "#/$defs/targets/v677";
pub(crate) const VALIDATION_TARGET_678_QUERY_46: &str = "#/$defs/targets/v678";
pub(crate) const VALIDATION_TARGET_679_QUERY_47: &str = "#/$defs/targets/v679";
pub(crate) const VALIDATION_TARGET_680_QUERY_48: &str = "#/$defs/targets/v680";
pub(crate) const VALIDATION_TARGET_681_QUERY_49: &str = "#/$defs/targets/v681";
pub(crate) const VALIDATION_TARGET_682_QUERY_50: &str = "#/$defs/targets/v682";
pub(crate) const VALIDATION_TARGET_683_QUERY_51: &str = "#/$defs/targets/v683";
pub(crate) const VALIDATION_TARGET_684_QUERY_52: &str = "#/$defs/targets/v684";
pub(crate) const VALIDATION_TARGET_685_QUERY_53: &str = "#/$defs/targets/v685";
pub(crate) const VALIDATION_TARGET_686_QUERY_54: &str = "#/$defs/targets/v686";
pub(crate) const VALIDATION_TARGET_687_QUERY_55: &str = "#/$defs/targets/v687";
pub(crate) const VALIDATION_TARGET_688_QUERY_56: &str = "#/$defs/targets/v688";
pub(crate) const VALIDATION_TARGET_689_QUERY_57: &str = "#/$defs/targets/v689";
pub(crate) const VALIDATION_TARGET_690_QUERY_58: &str = "#/$defs/targets/v690";
pub(crate) const VALIDATION_TARGET_691_QUERY_59: &str = "#/$defs/targets/v691";
pub(crate) const VALIDATION_TARGET_692_QUERY_60: &str = "#/$defs/targets/v692";
pub(crate) const VALIDATION_TARGET_693_QUERY_61: &str = "#/$defs/targets/v693";
pub(crate) const VALIDATION_TARGET_694_QUERY_62: &str = "#/$defs/targets/v694";
pub(crate) const VALIDATION_TARGET_695_QUERY_63: &str = "#/$defs/targets/v695";
pub(crate) const VALIDATION_TARGET_696_QUERY_64: &str = "#/$defs/targets/v696";
pub(crate) const VALIDATION_TARGET_697_QUERY_65: &str = "#/$defs/targets/v697";
pub(crate) const VALIDATION_TARGET_698_QUERY_66: &str = "#/$defs/targets/v698";
pub(crate) const VALIDATION_TARGET_699_QUERY_67: &str = "#/$defs/targets/v699";
pub(crate) const VALIDATION_TARGET_700_QUERY_68: &str = "#/$defs/targets/v700";
pub(crate) const VALIDATION_TARGET_701_QUERY_69: &str = "#/$defs/targets/v701";
pub(crate) const VALIDATION_TARGET_702_QUERY_70: &str = "#/$defs/targets/v702";
pub(crate) const VALIDATION_TARGET_703_QUERY_71: &str = "#/$defs/targets/v703";
pub(crate) const VALIDATION_TARGET_704_QUERY_72: &str = "#/$defs/targets/v704";
pub(crate) const VALIDATION_TARGET_705_QUERY_73: &str = "#/$defs/targets/v705";
pub(crate) const VALIDATION_TARGET_706_QUERY_74: &str = "#/$defs/targets/v706";
pub(crate) const VALIDATION_TARGET_707_QUERY_75: &str = "#/$defs/targets/v707";
pub(crate) const VALIDATION_TARGET_708_QUERY_76: &str = "#/$defs/targets/v708";
pub(crate) const VALIDATION_TARGET_709_QUERY_77: &str = "#/$defs/targets/v709";
pub(crate) const VALIDATION_TARGET_710_QUERY_78: &str = "#/$defs/targets/v710";
pub(crate) const VALIDATION_TARGET_711_QUERY_79: &str = "#/$defs/targets/v711";
pub(crate) const VALIDATION_TARGET_712_QUERY_80: &str = "#/$defs/targets/v712";
pub(crate) const VALIDATION_TARGET_713_QUERY_81: &str = "#/$defs/targets/v713";
pub(crate) const VALIDATION_TARGET_714_QUERY_82: &str = "#/$defs/targets/v714";
pub(crate) const VALIDATION_TARGET_715_QUERY_83: &str = "#/$defs/targets/v715";
pub(crate) const VALIDATION_TARGET_716_QUERY_84: &str = "#/$defs/targets/v716";
pub(crate) const VALIDATION_TARGET_717_QUERY_85: &str = "#/$defs/targets/v717";
pub(crate) const VALIDATION_TARGET_718_QUERY_86: &str = "#/$defs/targets/v718";
pub(crate) const VALIDATION_TARGET_719_QUERY_87: &str = "#/$defs/targets/v719";
pub(crate) const VALIDATION_TARGET_720_QUERY_88: &str = "#/$defs/targets/v720";
pub(crate) const VALIDATION_TARGET_721_QUERY_89: &str = "#/$defs/targets/v721";
pub(crate) const VALIDATION_TARGET_722_QUERY_90: &str = "#/$defs/targets/v722";
pub(crate) const VALIDATION_TARGET_723_QUERY_91: &str = "#/$defs/targets/v723";
pub(crate) const VALIDATION_TARGET_724_QUERY_92: &str = "#/$defs/targets/v724";
pub(crate) const VALIDATION_TARGET_725_QUERY_93: &str = "#/$defs/targets/v725";
pub(crate) const VALIDATION_TARGET_726_QUERY_94: &str = "#/$defs/targets/v726";
pub(crate) const VALIDATION_TARGET_727_QUERY_95: &str = "#/$defs/targets/v727";
pub(crate) const VALIDATION_TARGET_728_QUERY_96: &str = "#/$defs/targets/v728";
pub(crate) const VALIDATION_TARGET_729_QUERY_97: &str = "#/$defs/targets/v729";
pub(crate) const VALIDATION_TARGET_730_QUERY_98: &str = "#/$defs/targets/v730";
pub(crate) const VALIDATION_TARGET_731_QUERY_99: &str = "#/$defs/targets/v731";
pub(crate) const VALIDATION_TARGET_732_QUERY_100: &str = "#/$defs/targets/v732";
pub(crate) const VALIDATION_TARGET_733_QUERY_101: &str = "#/$defs/targets/v733";
pub(crate) const VALIDATION_TARGET_734_QUERY_102: &str = "#/$defs/targets/v734";
pub(crate) const VALIDATION_TARGET_735_QUERY_103: &str = "#/$defs/targets/v735";
pub(crate) const VALIDATION_TARGET_736_QUERY_104: &str = "#/$defs/targets/v736";
pub(crate) const VALIDATION_TARGET_737_QUERY_105: &str = "#/$defs/targets/v737";
pub(crate) const VALIDATION_TARGET_738_QUERY_106: &str = "#/$defs/targets/v738";
pub(crate) const VALIDATION_TARGET_739_QUERY_107: &str = "#/$defs/targets/v739";
pub(crate) const VALIDATION_TARGET_740_QUERY_108: &str = "#/$defs/targets/v740";
pub(crate) const VALIDATION_TARGET_741_QUERY_109: &str = "#/$defs/targets/v741";
pub(crate) const VALIDATION_TARGET_742_QUERY_110: &str = "#/$defs/targets/v742";
pub(crate) const VALIDATION_TARGET_743_QUERY_111: &str = "#/$defs/targets/v743";
pub(crate) const VALIDATION_TARGET_744_QUERY_112: &str = "#/$defs/targets/v744";
pub(crate) const VALIDATION_TARGET_745_QUERY_113: &str = "#/$defs/targets/v745";
pub(crate) const VALIDATION_TARGET_746_QUERY_114: &str = "#/$defs/targets/v746";
pub(crate) const VALIDATION_TARGET_747_QUERY_115: &str = "#/$defs/targets/v747";
pub(crate) const VALIDATION_TARGET_748_QUERY_116: &str = "#/$defs/targets/v748";
pub(crate) const VALIDATION_TARGET_749_QUERY_117: &str = "#/$defs/targets/v749";
pub(crate) const VALIDATION_TARGET_750_QUERY_118: &str = "#/$defs/targets/v750";
pub(crate) const VALIDATION_TARGET_751_QUERY_119: &str = "#/$defs/targets/v751";
pub(crate) const VALIDATION_TARGET_752_QUERY_120: &str = "#/$defs/targets/v752";
pub(crate) const VALIDATION_TARGET_753_QUERY_121: &str = "#/$defs/targets/v753";
pub(crate) const VALIDATION_TARGET_754_QUERY_122: &str = "#/$defs/targets/v754";
pub(crate) const VALIDATION_TARGET_755_QUERY_123: &str = "#/$defs/targets/v755";
pub(crate) const VALIDATION_TARGET_756_QUERY_124: &str = "#/$defs/targets/v756";
pub(crate) const VALIDATION_TARGET_757_QUERY_125: &str = "#/$defs/targets/v757";
pub(crate) const VALIDATION_TARGET_758_QUERY_126: &str = "#/$defs/targets/v758";
pub(crate) const VALIDATION_TARGET_759_QUERY_127: &str = "#/$defs/targets/v759";
pub(crate) const VALIDATION_TARGET_760_QUERY_128: &str = "#/$defs/targets/v760";
pub(crate) const VALIDATION_TARGET_761_QUERY_129: &str = "#/$defs/targets/v761";
pub(crate) const VALIDATION_TARGET_762_QUERY_130: &str = "#/$defs/targets/v762";
pub(crate) const VALIDATION_TARGET_763_QUERY_131: &str = "#/$defs/targets/v763";
pub(crate) const VALIDATION_TARGET_764_QUERY_132: &str = "#/$defs/targets/v764";
pub(crate) const VALIDATION_TARGET_765_QUERY_133: &str = "#/$defs/targets/v765";
pub(crate) const VALIDATION_TARGET_766_QUERY_134: &str = "#/$defs/targets/v766";
pub(crate) const VALIDATION_TARGET_767_QUERY_135: &str = "#/$defs/targets/v767";
pub(crate) const VALIDATION_TARGET_768_QUERY_136: &str = "#/$defs/targets/v768";
pub(crate) const VALIDATION_TARGET_769_QUERY_137: &str = "#/$defs/targets/v769";
pub(crate) const VALIDATION_TARGET_770_QUERY_138: &str = "#/$defs/targets/v770";
pub(crate) const VALIDATION_TARGET_771_QUERY_139: &str = "#/$defs/targets/v771";
pub(crate) const VALIDATION_TARGET_772_QUERY_140: &str = "#/$defs/targets/v772";
pub(crate) const VALIDATION_TARGET_773_QUERY_141: &str = "#/$defs/targets/v773";
pub(crate) const VALIDATION_TARGET_774_QUERY_142: &str = "#/$defs/targets/v774";
pub(crate) const VALIDATION_TARGET_775_QUERY_143: &str = "#/$defs/targets/v775";
pub(crate) const VALIDATION_TARGET_776_QUERY_144: &str = "#/$defs/targets/v776";
pub(crate) const VALIDATION_TARGET_777_QUERY_145: &str = "#/$defs/targets/v777";
pub(crate) const VALIDATION_TARGET_778_QUERY_146: &str = "#/$defs/targets/v778";
pub(crate) const VALIDATION_TARGET_779_QUERY_147: &str = "#/$defs/targets/v779";
pub(crate) const VALIDATION_TARGET_780_QUERY_148: &str = "#/$defs/targets/v780";
pub(crate) const VALIDATION_TARGET_781_QUERY_149: &str = "#/$defs/targets/v781";
pub(crate) const VALIDATION_TARGET_782_QUERY_150: &str = "#/$defs/targets/v782";
pub(crate) const VALIDATION_TARGET_783_QUERY_151: &str = "#/$defs/targets/v783";
pub(crate) const VALIDATION_TARGET_784_QUERY_152: &str = "#/$defs/targets/v784";
pub(crate) const VALIDATION_TARGET_785_QUERY_153: &str = "#/$defs/targets/v785";
pub(crate) const VALIDATION_TARGET_786_QUERY_154: &str = "#/$defs/targets/v786";
pub(crate) const VALIDATION_TARGET_787_QUERY_155: &str = "#/$defs/targets/v787";
pub(crate) const VALIDATION_TARGET_788_QUERY_156: &str = "#/$defs/targets/v788";
pub(crate) const VALIDATION_TARGET_789_QUERY_157: &str = "#/$defs/targets/v789";
pub(crate) const VALIDATION_TARGET_790_QUERY_158: &str = "#/$defs/targets/v790";
pub(crate) const VALIDATION_TARGET_791_QUERY_159: &str = "#/$defs/targets/v791";
pub(crate) const VALIDATION_TARGET_792_QUERY_160: &str = "#/$defs/targets/v792";
pub(crate) const VALIDATION_TARGET_793_QUERY_161: &str = "#/$defs/targets/v793";
pub(crate) const VALIDATION_TARGET_794_QUERY_162: &str = "#/$defs/targets/v794";
pub(crate) const VALIDATION_TARGET_795_QUERY_163: &str = "#/$defs/targets/v795";
pub(crate) const VALIDATION_TARGET_796_QUERY_164: &str = "#/$defs/targets/v796";
pub(crate) const VALIDATION_TARGET_797_QUERY_165: &str = "#/$defs/targets/v797";
pub(crate) const VALIDATION_TARGET_798_QUERY_166: &str = "#/$defs/targets/v798";
pub(crate) const VALIDATION_TARGET_799_QUERY_167: &str = "#/$defs/targets/v799";
pub(crate) const VALIDATION_TARGET_800_QUERY_168: &str = "#/$defs/targets/v800";
pub(crate) const VALIDATION_TARGET_801_QUERY_169: &str = "#/$defs/targets/v801";
pub(crate) const VALIDATION_TARGET_802_QUERY_170: &str = "#/$defs/targets/v802";
pub(crate) const VALIDATION_TARGET_803_QUERY_171: &str = "#/$defs/targets/v803";
pub(crate) const VALIDATION_TARGET_804_QUERY_172: &str = "#/$defs/targets/v804";
pub(crate) const VALIDATION_TARGET_805_QUERY_173: &str = "#/$defs/targets/v805";
pub(crate) const VALIDATION_TARGET_806_QUERY_174: &str = "#/$defs/targets/v806";
pub(crate) const VALIDATION_TARGET_807_QUERY_175: &str = "#/$defs/targets/v807";
pub(crate) const VALIDATION_TARGET_808_QUERY_176: &str = "#/$defs/targets/v808";
pub(crate) const VALIDATION_TARGET_809_QUERY_177: &str = "#/$defs/targets/v809";
pub(crate) const VALIDATION_TARGET_810_QUERY_178: &str = "#/$defs/targets/v810";
pub(crate) const VALIDATION_TARGET_811_QUERY_179: &str = "#/$defs/targets/v811";
pub(crate) const VALIDATION_TARGET_812_QUERY_180: &str = "#/$defs/targets/v812";
pub(crate) const VALIDATION_TARGET_813_QUERY_181: &str = "#/$defs/targets/v813";
pub(crate) const VALIDATION_TARGET_814_QUERY_182: &str = "#/$defs/targets/v814";
pub(crate) const VALIDATION_TARGET_815_QUERY_183: &str = "#/$defs/targets/v815";
pub(crate) const VALIDATION_TARGET_816_QUERY_184: &str = "#/$defs/targets/v816";
pub(crate) const VALIDATION_TARGET_817_QUERY_185: &str = "#/$defs/targets/v817";
pub(crate) const VALIDATION_TARGET_818_QUERY_186: &str = "#/$defs/targets/v818";
pub(crate) const VALIDATION_TARGET_819_QUERY_187: &str = "#/$defs/targets/v819";
pub(crate) const VALIDATION_TARGET_820_QUERY_188: &str = "#/$defs/targets/v820";
pub(crate) const VALIDATION_TARGET_821_QUERY_189: &str = "#/$defs/targets/v821";
pub(crate) const VALIDATION_TARGET_822_QUERY_190: &str = "#/$defs/targets/v822";
pub(crate) const VALIDATION_TARGET_823_QUERY_191: &str = "#/$defs/targets/v823";
pub(crate) const VALIDATION_TARGET_824_QUERY_192: &str = "#/$defs/targets/v824";
pub(crate) const VALIDATION_TARGET_825_QUERY_193: &str = "#/$defs/targets/v825";
pub(crate) const VALIDATION_TARGET_826_QUERY_194: &str = "#/$defs/targets/v826";
pub(crate) const VALIDATION_TARGET_827_QUERY_195: &str = "#/$defs/targets/v827";
pub(crate) const VALIDATION_TARGET_828_QUERY_196: &str = "#/$defs/targets/v828";
pub(crate) const VALIDATION_TARGET_829_QUERY_197: &str = "#/$defs/targets/v829";
pub(crate) const VALIDATION_TARGET_830_QUERY_198: &str = "#/$defs/targets/v830";
pub(crate) const VALIDATION_TARGET_831_QUERY_199: &str = "#/$defs/targets/v831";
pub(crate) const VALIDATION_TARGET_832_QUERY_200: &str = "#/$defs/targets/v832";
pub(crate) const VALIDATION_TARGET_833_QUERY_201: &str = "#/$defs/targets/v833";
pub(crate) const VALIDATION_TARGET_834_QUERY_202: &str = "#/$defs/targets/v834";
pub(crate) const VALIDATION_TARGET_835_QUERY_203: &str = "#/$defs/targets/v835";
pub(crate) const VALIDATION_TARGET_836_QUERY_204: &str = "#/$defs/targets/v836";
pub(crate) const VALIDATION_TARGET_837_BODY: &str = "#/$defs/targets/v837";
pub(crate) const VALIDATION_TARGET_838_QUERY_0: &str = "#/$defs/targets/v838";
pub(crate) const VALIDATION_TARGET_839_QUERY_1: &str = "#/$defs/targets/v839";
pub(crate) const VALIDATION_TARGET_840_QUERY_2: &str = "#/$defs/targets/v840";
pub(crate) const VALIDATION_TARGET_841_QUERY_3: &str = "#/$defs/targets/v841";
pub(crate) const VALIDATION_TARGET_842_QUERY_4: &str = "#/$defs/targets/v842";
pub(crate) const VALIDATION_TARGET_843_QUERY_5: &str = "#/$defs/targets/v843";
pub(crate) const VALIDATION_TARGET_844_QUERY_6: &str = "#/$defs/targets/v844";
pub(crate) const VALIDATION_TARGET_845_QUERY_7: &str = "#/$defs/targets/v845";
pub(crate) const VALIDATION_TARGET_846_QUERY_8: &str = "#/$defs/targets/v846";
pub(crate) const VALIDATION_TARGET_847_QUERY_9: &str = "#/$defs/targets/v847";
pub(crate) const VALIDATION_TARGET_848_QUERY_10: &str = "#/$defs/targets/v848";
pub(crate) const VALIDATION_TARGET_849_QUERY_11: &str = "#/$defs/targets/v849";
pub(crate) const VALIDATION_TARGET_850_QUERY_12: &str = "#/$defs/targets/v850";
pub(crate) const VALIDATION_TARGET_851_QUERY_13: &str = "#/$defs/targets/v851";
pub(crate) const VALIDATION_TARGET_852_QUERY_14: &str = "#/$defs/targets/v852";
pub(crate) const VALIDATION_TARGET_853_QUERY_15: &str = "#/$defs/targets/v853";
pub(crate) const VALIDATION_TARGET_854_QUERY_16: &str = "#/$defs/targets/v854";
pub(crate) const VALIDATION_TARGET_855_QUERY_17: &str = "#/$defs/targets/v855";
pub(crate) const VALIDATION_TARGET_856_QUERY_18: &str = "#/$defs/targets/v856";
pub(crate) const VALIDATION_TARGET_857_QUERY_19: &str = "#/$defs/targets/v857";
pub(crate) const VALIDATION_TARGET_858_QUERY_20: &str = "#/$defs/targets/v858";
pub(crate) const VALIDATION_TARGET_859_QUERY_21: &str = "#/$defs/targets/v859";
pub(crate) const VALIDATION_TARGET_860_QUERY_22: &str = "#/$defs/targets/v860";
pub(crate) const VALIDATION_TARGET_861_QUERY_23: &str = "#/$defs/targets/v861";
pub(crate) const VALIDATION_TARGET_862_QUERY_24: &str = "#/$defs/targets/v862";
pub(crate) const VALIDATION_TARGET_863_QUERY_25: &str = "#/$defs/targets/v863";
pub(crate) const VALIDATION_TARGET_864_QUERY_26: &str = "#/$defs/targets/v864";
pub(crate) const VALIDATION_TARGET_865_QUERY_27: &str = "#/$defs/targets/v865";
pub(crate) const VALIDATION_TARGET_866_QUERY_28: &str = "#/$defs/targets/v866";
pub(crate) const VALIDATION_TARGET_867_QUERY_29: &str = "#/$defs/targets/v867";
pub(crate) const VALIDATION_TARGET_868_QUERY_30: &str = "#/$defs/targets/v868";
pub(crate) const VALIDATION_TARGET_869_QUERY_31: &str = "#/$defs/targets/v869";
pub(crate) const VALIDATION_TARGET_870_QUERY_32: &str = "#/$defs/targets/v870";
pub(crate) const VALIDATION_TARGET_871_QUERY_33: &str = "#/$defs/targets/v871";
pub(crate) const VALIDATION_TARGET_872_QUERY_34: &str = "#/$defs/targets/v872";
pub(crate) const VALIDATION_TARGET_873_QUERY_35: &str = "#/$defs/targets/v873";
pub(crate) const VALIDATION_TARGET_874_QUERY_36: &str = "#/$defs/targets/v874";
pub(crate) const VALIDATION_TARGET_875_QUERY_37: &str = "#/$defs/targets/v875";
pub(crate) const VALIDATION_TARGET_876_QUERY_38: &str = "#/$defs/targets/v876";
pub(crate) const VALIDATION_TARGET_877_QUERY_39: &str = "#/$defs/targets/v877";
pub(crate) const VALIDATION_TARGET_878_QUERY_40: &str = "#/$defs/targets/v878";
pub(crate) const VALIDATION_TARGET_879_QUERY_41: &str = "#/$defs/targets/v879";
pub(crate) const VALIDATION_TARGET_880_QUERY_42: &str = "#/$defs/targets/v880";
pub(crate) const VALIDATION_TARGET_881_QUERY_43: &str = "#/$defs/targets/v881";
pub(crate) const VALIDATION_TARGET_882_QUERY_44: &str = "#/$defs/targets/v882";
pub(crate) const VALIDATION_TARGET_883_QUERY_45: &str = "#/$defs/targets/v883";
pub(crate) const VALIDATION_TARGET_884_QUERY_46: &str = "#/$defs/targets/v884";
pub(crate) const VALIDATION_TARGET_885_QUERY_47: &str = "#/$defs/targets/v885";
pub(crate) const VALIDATION_TARGET_886_QUERY_48: &str = "#/$defs/targets/v886";
pub(crate) const VALIDATION_TARGET_887_QUERY_49: &str = "#/$defs/targets/v887";
pub(crate) const VALIDATION_TARGET_888_QUERY_50: &str = "#/$defs/targets/v888";
pub(crate) const VALIDATION_TARGET_889_QUERY_51: &str = "#/$defs/targets/v889";
pub(crate) const VALIDATION_TARGET_890_QUERY_52: &str = "#/$defs/targets/v890";
pub(crate) const VALIDATION_TARGET_891_QUERY_53: &str = "#/$defs/targets/v891";
pub(crate) const VALIDATION_TARGET_892_QUERY_54: &str = "#/$defs/targets/v892";
pub(crate) const VALIDATION_TARGET_893_QUERY_55: &str = "#/$defs/targets/v893";
pub(crate) const VALIDATION_TARGET_894_QUERY_56: &str = "#/$defs/targets/v894";
pub(crate) const VALIDATION_TARGET_895_QUERY_57: &str = "#/$defs/targets/v895";
pub(crate) const VALIDATION_TARGET_896_QUERY_58: &str = "#/$defs/targets/v896";
pub(crate) const VALIDATION_TARGET_897_QUERY_59: &str = "#/$defs/targets/v897";
pub(crate) const VALIDATION_TARGET_898_QUERY_60: &str = "#/$defs/targets/v898";
pub(crate) const VALIDATION_TARGET_899_QUERY_61: &str = "#/$defs/targets/v899";
pub(crate) const VALIDATION_TARGET_900_QUERY_62: &str = "#/$defs/targets/v900";
pub(crate) const VALIDATION_TARGET_901_QUERY_63: &str = "#/$defs/targets/v901";
pub(crate) const VALIDATION_TARGET_902_QUERY_64: &str = "#/$defs/targets/v902";
pub(crate) const VALIDATION_TARGET_903_QUERY_65: &str = "#/$defs/targets/v903";
pub(crate) const VALIDATION_TARGET_904_QUERY_66: &str = "#/$defs/targets/v904";
pub(crate) const VALIDATION_TARGET_905_QUERY_67: &str = "#/$defs/targets/v905";
pub(crate) const VALIDATION_TARGET_906_QUERY_68: &str = "#/$defs/targets/v906";
pub(crate) const VALIDATION_TARGET_907_QUERY_69: &str = "#/$defs/targets/v907";
pub(crate) const VALIDATION_TARGET_908_QUERY_70: &str = "#/$defs/targets/v908";
pub(crate) const VALIDATION_TARGET_909_QUERY_71: &str = "#/$defs/targets/v909";
pub(crate) const VALIDATION_TARGET_910_QUERY_72: &str = "#/$defs/targets/v910";
pub(crate) const VALIDATION_TARGET_911_QUERY_73: &str = "#/$defs/targets/v911";
pub(crate) const VALIDATION_TARGET_912_QUERY_74: &str = "#/$defs/targets/v912";
pub(crate) const VALIDATION_TARGET_913_QUERY_75: &str = "#/$defs/targets/v913";
pub(crate) const VALIDATION_TARGET_914_QUERY_76: &str = "#/$defs/targets/v914";
pub(crate) const VALIDATION_TARGET_915_QUERY_77: &str = "#/$defs/targets/v915";
pub(crate) const VALIDATION_TARGET_916_QUERY_78: &str = "#/$defs/targets/v916";
pub(crate) const VALIDATION_TARGET_917_QUERY_79: &str = "#/$defs/targets/v917";
pub(crate) const VALIDATION_TARGET_918_QUERY_80: &str = "#/$defs/targets/v918";
pub(crate) const VALIDATION_TARGET_919_QUERY_81: &str = "#/$defs/targets/v919";
pub(crate) const VALIDATION_TARGET_920_QUERY_82: &str = "#/$defs/targets/v920";
pub(crate) const VALIDATION_TARGET_921_QUERY_83: &str = "#/$defs/targets/v921";
pub(crate) const VALIDATION_TARGET_922_QUERY_84: &str = "#/$defs/targets/v922";
pub(crate) const VALIDATION_TARGET_923_QUERY_85: &str = "#/$defs/targets/v923";
pub(crate) const VALIDATION_TARGET_924_QUERY_86: &str = "#/$defs/targets/v924";
pub(crate) const VALIDATION_TARGET_925_QUERY_87: &str = "#/$defs/targets/v925";
pub(crate) const VALIDATION_TARGET_926_QUERY_88: &str = "#/$defs/targets/v926";
pub(crate) const VALIDATION_TARGET_927_QUERY_89: &str = "#/$defs/targets/v927";
pub(crate) const VALIDATION_TARGET_928_QUERY_90: &str = "#/$defs/targets/v928";
pub(crate) const VALIDATION_TARGET_929_QUERY_91: &str = "#/$defs/targets/v929";
pub(crate) const VALIDATION_TARGET_930_QUERY_92: &str = "#/$defs/targets/v930";
pub(crate) const VALIDATION_TARGET_931_QUERY_93: &str = "#/$defs/targets/v931";
pub(crate) const VALIDATION_TARGET_932_QUERY_94: &str = "#/$defs/targets/v932";
pub(crate) const VALIDATION_TARGET_933_QUERY_95: &str = "#/$defs/targets/v933";
pub(crate) const VALIDATION_TARGET_934_QUERY_96: &str = "#/$defs/targets/v934";
pub(crate) const VALIDATION_TARGET_935_QUERY_97: &str = "#/$defs/targets/v935";
pub(crate) const VALIDATION_TARGET_936_QUERY_98: &str = "#/$defs/targets/v936";
pub(crate) const VALIDATION_TARGET_937_QUERY_99: &str = "#/$defs/targets/v937";
pub(crate) const VALIDATION_TARGET_938_QUERY_100: &str = "#/$defs/targets/v938";
pub(crate) const VALIDATION_TARGET_939_QUERY_101: &str = "#/$defs/targets/v939";
pub(crate) const VALIDATION_TARGET_940_QUERY_102: &str = "#/$defs/targets/v940";
pub(crate) const VALIDATION_TARGET_941_QUERY_103: &str = "#/$defs/targets/v941";
pub(crate) const VALIDATION_TARGET_942_QUERY_104: &str = "#/$defs/targets/v942";
pub(crate) const VALIDATION_TARGET_943_QUERY_105: &str = "#/$defs/targets/v943";
pub(crate) const VALIDATION_TARGET_944_QUERY_106: &str = "#/$defs/targets/v944";
pub(crate) const VALIDATION_TARGET_945_QUERY_107: &str = "#/$defs/targets/v945";
pub(crate) const VALIDATION_TARGET_946_QUERY_108: &str = "#/$defs/targets/v946";
pub(crate) const VALIDATION_TARGET_947_QUERY_109: &str = "#/$defs/targets/v947";
pub(crate) const VALIDATION_TARGET_948_QUERY_110: &str = "#/$defs/targets/v948";
pub(crate) const VALIDATION_TARGET_949_QUERY_111: &str = "#/$defs/targets/v949";
pub(crate) const VALIDATION_TARGET_950_QUERY_112: &str = "#/$defs/targets/v950";
pub(crate) const VALIDATION_TARGET_951_QUERY_113: &str = "#/$defs/targets/v951";
pub(crate) const VALIDATION_TARGET_952_QUERY_114: &str = "#/$defs/targets/v952";
pub(crate) const VALIDATION_TARGET_953_QUERY_115: &str = "#/$defs/targets/v953";
pub(crate) const VALIDATION_TARGET_954_QUERY_116: &str = "#/$defs/targets/v954";
pub(crate) const VALIDATION_TARGET_955_QUERY_117: &str = "#/$defs/targets/v955";
pub(crate) const VALIDATION_TARGET_956_QUERY_118: &str = "#/$defs/targets/v956";
pub(crate) const VALIDATION_TARGET_957_QUERY_119: &str = "#/$defs/targets/v957";
pub(crate) const VALIDATION_TARGET_958_QUERY_120: &str = "#/$defs/targets/v958";
pub(crate) const VALIDATION_TARGET_959_QUERY_121: &str = "#/$defs/targets/v959";
pub(crate) const VALIDATION_TARGET_960_QUERY_122: &str = "#/$defs/targets/v960";
pub(crate) const VALIDATION_TARGET_961_QUERY_123: &str = "#/$defs/targets/v961";
pub(crate) const VALIDATION_TARGET_962_QUERY_124: &str = "#/$defs/targets/v962";
pub(crate) const VALIDATION_TARGET_963_QUERY_125: &str = "#/$defs/targets/v963";
pub(crate) const VALIDATION_TARGET_964_QUERY_126: &str = "#/$defs/targets/v964";
pub(crate) const VALIDATION_TARGET_965_QUERY_127: &str = "#/$defs/targets/v965";
pub(crate) const VALIDATION_TARGET_966_QUERY_128: &str = "#/$defs/targets/v966";
pub(crate) const VALIDATION_TARGET_967_QUERY_129: &str = "#/$defs/targets/v967";
pub(crate) const VALIDATION_TARGET_968_QUERY_130: &str = "#/$defs/targets/v968";
pub(crate) const VALIDATION_TARGET_969_QUERY_131: &str = "#/$defs/targets/v969";
pub(crate) const VALIDATION_TARGET_970_QUERY_132: &str = "#/$defs/targets/v970";
pub(crate) const VALIDATION_TARGET_971_QUERY_133: &str = "#/$defs/targets/v971";
pub(crate) const VALIDATION_TARGET_972_QUERY_134: &str = "#/$defs/targets/v972";
pub(crate) const VALIDATION_TARGET_973_QUERY_135: &str = "#/$defs/targets/v973";
pub(crate) const VALIDATION_TARGET_974_QUERY_136: &str = "#/$defs/targets/v974";
pub(crate) const VALIDATION_TARGET_975_QUERY_137: &str = "#/$defs/targets/v975";
pub(crate) const VALIDATION_TARGET_976_QUERY_138: &str = "#/$defs/targets/v976";
pub(crate) const VALIDATION_TARGET_977_QUERY_139: &str = "#/$defs/targets/v977";
pub(crate) const VALIDATION_TARGET_978_QUERY_140: &str = "#/$defs/targets/v978";
pub(crate) const VALIDATION_TARGET_979_QUERY_141: &str = "#/$defs/targets/v979";
pub(crate) const VALIDATION_TARGET_980_QUERY_142: &str = "#/$defs/targets/v980";
pub(crate) const VALIDATION_TARGET_981_QUERY_143: &str = "#/$defs/targets/v981";
pub(crate) const VALIDATION_TARGET_982_QUERY_144: &str = "#/$defs/targets/v982";
pub(crate) const VALIDATION_TARGET_983_QUERY_145: &str = "#/$defs/targets/v983";
pub(crate) const VALIDATION_TARGET_984_QUERY_146: &str = "#/$defs/targets/v984";
pub(crate) const VALIDATION_TARGET_985_QUERY_147: &str = "#/$defs/targets/v985";
pub(crate) const VALIDATION_TARGET_986_QUERY_148: &str = "#/$defs/targets/v986";
pub(crate) const VALIDATION_TARGET_987_QUERY_149: &str = "#/$defs/targets/v987";
pub(crate) const VALIDATION_TARGET_988_QUERY_150: &str = "#/$defs/targets/v988";
pub(crate) const VALIDATION_TARGET_989_QUERY_151: &str = "#/$defs/targets/v989";
pub(crate) const VALIDATION_TARGET_990_QUERY_152: &str = "#/$defs/targets/v990";
pub(crate) const VALIDATION_TARGET_991_QUERY_153: &str = "#/$defs/targets/v991";
pub(crate) const VALIDATION_TARGET_992_QUERY_154: &str = "#/$defs/targets/v992";
pub(crate) const VALIDATION_TARGET_993_QUERY_155: &str = "#/$defs/targets/v993";
pub(crate) const VALIDATION_TARGET_994_QUERY_156: &str = "#/$defs/targets/v994";
pub(crate) const VALIDATION_TARGET_995_QUERY_157: &str = "#/$defs/targets/v995";
pub(crate) const VALIDATION_TARGET_996_QUERY_158: &str = "#/$defs/targets/v996";
pub(crate) const VALIDATION_TARGET_997_QUERY_159: &str = "#/$defs/targets/v997";
pub(crate) const VALIDATION_TARGET_998_QUERY_160: &str = "#/$defs/targets/v998";
pub(crate) const VALIDATION_TARGET_999_QUERY_161: &str = "#/$defs/targets/v999";
pub(crate) const VALIDATION_TARGET_1000_QUERY_162: &str = "#/$defs/targets/v1000";
pub(crate) const VALIDATION_TARGET_1001_QUERY_163: &str = "#/$defs/targets/v1001";
pub(crate) const VALIDATION_TARGET_1002_QUERY_164: &str = "#/$defs/targets/v1002";
pub(crate) const VALIDATION_TARGET_1003_QUERY_165: &str = "#/$defs/targets/v1003";
pub(crate) const VALIDATION_TARGET_1004_QUERY_166: &str = "#/$defs/targets/v1004";
pub(crate) const VALIDATION_TARGET_1005_QUERY_167: &str = "#/$defs/targets/v1005";
pub(crate) const VALIDATION_TARGET_1006_QUERY_168: &str = "#/$defs/targets/v1006";
pub(crate) const VALIDATION_TARGET_1007_QUERY_169: &str = "#/$defs/targets/v1007";
pub(crate) const VALIDATION_TARGET_1008_QUERY_170: &str = "#/$defs/targets/v1008";
pub(crate) const VALIDATION_TARGET_1009_QUERY_171: &str = "#/$defs/targets/v1009";
pub(crate) const VALIDATION_TARGET_1010_QUERY_172: &str = "#/$defs/targets/v1010";
pub(crate) const VALIDATION_TARGET_1011_QUERY_173: &str = "#/$defs/targets/v1011";
pub(crate) const VALIDATION_TARGET_1012_QUERY_174: &str = "#/$defs/targets/v1012";
pub(crate) const VALIDATION_TARGET_1013_QUERY_175: &str = "#/$defs/targets/v1013";
pub(crate) const VALIDATION_TARGET_1014_QUERY_176: &str = "#/$defs/targets/v1014";
pub(crate) const VALIDATION_TARGET_1015_QUERY_177: &str = "#/$defs/targets/v1015";
pub(crate) const VALIDATION_TARGET_1016_QUERY_178: &str = "#/$defs/targets/v1016";
pub(crate) const VALIDATION_TARGET_1017_QUERY_179: &str = "#/$defs/targets/v1017";
pub(crate) const VALIDATION_TARGET_1018_QUERY_180: &str = "#/$defs/targets/v1018";
pub(crate) const VALIDATION_TARGET_1019_QUERY_181: &str = "#/$defs/targets/v1019";
pub(crate) const VALIDATION_TARGET_1020_QUERY_182: &str = "#/$defs/targets/v1020";
pub(crate) const VALIDATION_TARGET_1021_QUERY_183: &str = "#/$defs/targets/v1021";
pub(crate) const VALIDATION_TARGET_1022_QUERY_184: &str = "#/$defs/targets/v1022";
pub(crate) const VALIDATION_TARGET_1023_QUERY_185: &str = "#/$defs/targets/v1023";
pub(crate) const VALIDATION_TARGET_1024_QUERY_186: &str = "#/$defs/targets/v1024";
pub(crate) const VALIDATION_TARGET_1025_QUERY_187: &str = "#/$defs/targets/v1025";
pub(crate) const VALIDATION_TARGET_1026_QUERY_188: &str = "#/$defs/targets/v1026";
pub(crate) const VALIDATION_TARGET_1027_QUERY_189: &str = "#/$defs/targets/v1027";
pub(crate) const VALIDATION_TARGET_1028_QUERY_190: &str = "#/$defs/targets/v1028";
pub(crate) const VALIDATION_TARGET_1029_QUERY_191: &str = "#/$defs/targets/v1029";
pub(crate) const VALIDATION_TARGET_1030_QUERY_192: &str = "#/$defs/targets/v1030";
pub(crate) const VALIDATION_TARGET_1031_QUERY_193: &str = "#/$defs/targets/v1031";
pub(crate) const VALIDATION_TARGET_1032_QUERY_194: &str = "#/$defs/targets/v1032";
pub(crate) const VALIDATION_TARGET_1033_QUERY_195: &str = "#/$defs/targets/v1033";
pub(crate) const VALIDATION_TARGET_1034_QUERY_196: &str = "#/$defs/targets/v1034";
pub(crate) const VALIDATION_TARGET_1035_QUERY_197: &str = "#/$defs/targets/v1035";
pub(crate) const VALIDATION_TARGET_1036_QUERY_198: &str = "#/$defs/targets/v1036";
pub(crate) const VALIDATION_TARGET_1037_QUERY_199: &str = "#/$defs/targets/v1037";
pub(crate) const VALIDATION_TARGET_1038_QUERY_200: &str = "#/$defs/targets/v1038";
pub(crate) const VALIDATION_TARGET_1039_QUERY_201: &str = "#/$defs/targets/v1039";
pub(crate) const VALIDATION_TARGET_1040_QUERY_202: &str = "#/$defs/targets/v1040";
pub(crate) const VALIDATION_TARGET_1041_QUERY_203: &str = "#/$defs/targets/v1041";
pub(crate) const VALIDATION_TARGET_1042_QUERY_204: &str = "#/$defs/targets/v1042";
pub(crate) const VALIDATION_TARGET_1043_BODY: &str = "#/$defs/targets/v1043";
pub(crate) const VALIDATION_TARGET_1044_QUERY_0: &str = "#/$defs/targets/v1044";
pub(crate) const VALIDATION_TARGET_1045_QUERY_1: &str = "#/$defs/targets/v1045";
pub(crate) const VALIDATION_TARGET_1046_QUERY_2: &str = "#/$defs/targets/v1046";
pub(crate) const VALIDATION_TARGET_1047_QUERY_3: &str = "#/$defs/targets/v1047";
pub(crate) const VALIDATION_TARGET_1048_QUERY_4: &str = "#/$defs/targets/v1048";
pub(crate) const VALIDATION_TARGET_1049_QUERY_5: &str = "#/$defs/targets/v1049";
pub(crate) const VALIDATION_TARGET_1050_QUERY_6: &str = "#/$defs/targets/v1050";
pub(crate) const VALIDATION_TARGET_1051_QUERY_7: &str = "#/$defs/targets/v1051";
pub(crate) const VALIDATION_TARGET_1052_QUERY_8: &str = "#/$defs/targets/v1052";
pub(crate) const VALIDATION_TARGET_1053_QUERY_9: &str = "#/$defs/targets/v1053";
pub(crate) const VALIDATION_TARGET_1054_QUERY_10: &str = "#/$defs/targets/v1054";
pub(crate) const VALIDATION_TARGET_1055_QUERY_11: &str = "#/$defs/targets/v1055";
pub(crate) const VALIDATION_TARGET_1056_QUERY_12: &str = "#/$defs/targets/v1056";
pub(crate) const VALIDATION_TARGET_1057_QUERY_13: &str = "#/$defs/targets/v1057";
pub(crate) const VALIDATION_TARGET_1058_QUERY_14: &str = "#/$defs/targets/v1058";
pub(crate) const VALIDATION_TARGET_1059_QUERY_15: &str = "#/$defs/targets/v1059";
pub(crate) const VALIDATION_TARGET_1060_QUERY_16: &str = "#/$defs/targets/v1060";
pub(crate) const VALIDATION_TARGET_1061_QUERY_17: &str = "#/$defs/targets/v1061";
pub(crate) const VALIDATION_TARGET_1062_QUERY_18: &str = "#/$defs/targets/v1062";
pub(crate) const VALIDATION_TARGET_1063_QUERY_19: &str = "#/$defs/targets/v1063";
pub(crate) const VALIDATION_TARGET_1064_QUERY_20: &str = "#/$defs/targets/v1064";
pub(crate) const VALIDATION_TARGET_1065_QUERY_21: &str = "#/$defs/targets/v1065";
pub(crate) const VALIDATION_TARGET_1066_QUERY_22: &str = "#/$defs/targets/v1066";
pub(crate) const VALIDATION_TARGET_1067_QUERY_23: &str = "#/$defs/targets/v1067";
pub(crate) const VALIDATION_TARGET_1068_QUERY_24: &str = "#/$defs/targets/v1068";
pub(crate) const VALIDATION_TARGET_1069_QUERY_25: &str = "#/$defs/targets/v1069";
pub(crate) const VALIDATION_TARGET_1070_QUERY_26: &str = "#/$defs/targets/v1070";
pub(crate) const VALIDATION_TARGET_1071_QUERY_27: &str = "#/$defs/targets/v1071";
pub(crate) const VALIDATION_TARGET_1072_QUERY_28: &str = "#/$defs/targets/v1072";
pub(crate) const VALIDATION_TARGET_1073_QUERY_29: &str = "#/$defs/targets/v1073";
pub(crate) const VALIDATION_TARGET_1074_QUERY_30: &str = "#/$defs/targets/v1074";
pub(crate) const VALIDATION_TARGET_1075_QUERY_31: &str = "#/$defs/targets/v1075";
pub(crate) const VALIDATION_TARGET_1076_QUERY_32: &str = "#/$defs/targets/v1076";
pub(crate) const VALIDATION_TARGET_1077_QUERY_33: &str = "#/$defs/targets/v1077";
pub(crate) const VALIDATION_TARGET_1078_QUERY_34: &str = "#/$defs/targets/v1078";
pub(crate) const VALIDATION_TARGET_1079_QUERY_35: &str = "#/$defs/targets/v1079";
pub(crate) const VALIDATION_TARGET_1080_QUERY_36: &str = "#/$defs/targets/v1080";
pub(crate) const VALIDATION_TARGET_1081_QUERY_37: &str = "#/$defs/targets/v1081";
pub(crate) const VALIDATION_TARGET_1082_QUERY_38: &str = "#/$defs/targets/v1082";
pub(crate) const VALIDATION_TARGET_1083_QUERY_39: &str = "#/$defs/targets/v1083";
pub(crate) const VALIDATION_TARGET_1084_QUERY_40: &str = "#/$defs/targets/v1084";
pub(crate) const VALIDATION_TARGET_1085_QUERY_41: &str = "#/$defs/targets/v1085";
pub(crate) const VALIDATION_TARGET_1086_QUERY_42: &str = "#/$defs/targets/v1086";
pub(crate) const VALIDATION_TARGET_1087_QUERY_43: &str = "#/$defs/targets/v1087";
pub(crate) const VALIDATION_TARGET_1088_QUERY_44: &str = "#/$defs/targets/v1088";
pub(crate) const VALIDATION_TARGET_1089_QUERY_45: &str = "#/$defs/targets/v1089";
pub(crate) const VALIDATION_TARGET_1090_QUERY_46: &str = "#/$defs/targets/v1090";
pub(crate) const VALIDATION_TARGET_1091_QUERY_47: &str = "#/$defs/targets/v1091";
pub(crate) const VALIDATION_TARGET_1092_QUERY_48: &str = "#/$defs/targets/v1092";
pub(crate) const VALIDATION_TARGET_1093_QUERY_49: &str = "#/$defs/targets/v1093";
pub(crate) const VALIDATION_TARGET_1094_QUERY_50: &str = "#/$defs/targets/v1094";
pub(crate) const VALIDATION_TARGET_1095_QUERY_51: &str = "#/$defs/targets/v1095";
pub(crate) const VALIDATION_TARGET_1096_QUERY_52: &str = "#/$defs/targets/v1096";
pub(crate) const VALIDATION_TARGET_1097_QUERY_53: &str = "#/$defs/targets/v1097";
pub(crate) const VALIDATION_TARGET_1098_QUERY_54: &str = "#/$defs/targets/v1098";
pub(crate) const VALIDATION_TARGET_1099_QUERY_55: &str = "#/$defs/targets/v1099";
pub(crate) const VALIDATION_TARGET_1100_QUERY_56: &str = "#/$defs/targets/v1100";
pub(crate) const VALIDATION_TARGET_1101_QUERY_57: &str = "#/$defs/targets/v1101";
pub(crate) const VALIDATION_TARGET_1102_QUERY_58: &str = "#/$defs/targets/v1102";
pub(crate) const VALIDATION_TARGET_1103_QUERY_59: &str = "#/$defs/targets/v1103";
pub(crate) const VALIDATION_TARGET_1104_QUERY_60: &str = "#/$defs/targets/v1104";
pub(crate) const VALIDATION_TARGET_1105_QUERY_61: &str = "#/$defs/targets/v1105";
pub(crate) const VALIDATION_TARGET_1106_QUERY_62: &str = "#/$defs/targets/v1106";
pub(crate) const VALIDATION_TARGET_1107_QUERY_63: &str = "#/$defs/targets/v1107";
pub(crate) const VALIDATION_TARGET_1108_QUERY_64: &str = "#/$defs/targets/v1108";
pub(crate) const VALIDATION_TARGET_1109_QUERY_65: &str = "#/$defs/targets/v1109";
pub(crate) const VALIDATION_TARGET_1110_QUERY_66: &str = "#/$defs/targets/v1110";
pub(crate) const VALIDATION_TARGET_1111_QUERY_67: &str = "#/$defs/targets/v1111";
pub(crate) const VALIDATION_TARGET_1112_QUERY_68: &str = "#/$defs/targets/v1112";
pub(crate) const VALIDATION_TARGET_1113_QUERY_69: &str = "#/$defs/targets/v1113";
pub(crate) const VALIDATION_TARGET_1114_QUERY_70: &str = "#/$defs/targets/v1114";
pub(crate) const VALIDATION_TARGET_1115_QUERY_71: &str = "#/$defs/targets/v1115";
pub(crate) const VALIDATION_TARGET_1116_QUERY_72: &str = "#/$defs/targets/v1116";
pub(crate) const VALIDATION_TARGET_1117_QUERY_73: &str = "#/$defs/targets/v1117";
pub(crate) const VALIDATION_TARGET_1118_QUERY_74: &str = "#/$defs/targets/v1118";
pub(crate) const VALIDATION_TARGET_1119_QUERY_75: &str = "#/$defs/targets/v1119";
pub(crate) const VALIDATION_TARGET_1120_QUERY_76: &str = "#/$defs/targets/v1120";
pub(crate) const VALIDATION_TARGET_1121_QUERY_77: &str = "#/$defs/targets/v1121";
pub(crate) const VALIDATION_TARGET_1122_QUERY_78: &str = "#/$defs/targets/v1122";
pub(crate) const VALIDATION_TARGET_1123_QUERY_79: &str = "#/$defs/targets/v1123";
pub(crate) const VALIDATION_TARGET_1124_QUERY_80: &str = "#/$defs/targets/v1124";
pub(crate) const VALIDATION_TARGET_1125_QUERY_81: &str = "#/$defs/targets/v1125";
pub(crate) const VALIDATION_TARGET_1126_QUERY_82: &str = "#/$defs/targets/v1126";
pub(crate) const VALIDATION_TARGET_1127_QUERY_83: &str = "#/$defs/targets/v1127";
pub(crate) const VALIDATION_TARGET_1128_QUERY_84: &str = "#/$defs/targets/v1128";
pub(crate) const VALIDATION_TARGET_1129_QUERY_85: &str = "#/$defs/targets/v1129";
pub(crate) const VALIDATION_TARGET_1130_QUERY_86: &str = "#/$defs/targets/v1130";
pub(crate) const VALIDATION_TARGET_1131_QUERY_87: &str = "#/$defs/targets/v1131";
pub(crate) const VALIDATION_TARGET_1132_QUERY_88: &str = "#/$defs/targets/v1132";
pub(crate) const VALIDATION_TARGET_1133_QUERY_89: &str = "#/$defs/targets/v1133";
pub(crate) const VALIDATION_TARGET_1134_QUERY_90: &str = "#/$defs/targets/v1134";
pub(crate) const VALIDATION_TARGET_1135_QUERY_91: &str = "#/$defs/targets/v1135";
pub(crate) const VALIDATION_TARGET_1136_QUERY_92: &str = "#/$defs/targets/v1136";
pub(crate) const VALIDATION_TARGET_1137_QUERY_93: &str = "#/$defs/targets/v1137";
pub(crate) const VALIDATION_TARGET_1138_QUERY_94: &str = "#/$defs/targets/v1138";
pub(crate) const VALIDATION_TARGET_1139_QUERY_95: &str = "#/$defs/targets/v1139";
pub(crate) const VALIDATION_TARGET_1140_QUERY_96: &str = "#/$defs/targets/v1140";
pub(crate) const VALIDATION_TARGET_1141_QUERY_97: &str = "#/$defs/targets/v1141";
pub(crate) const VALIDATION_TARGET_1142_QUERY_98: &str = "#/$defs/targets/v1142";
pub(crate) const VALIDATION_TARGET_1143_QUERY_99: &str = "#/$defs/targets/v1143";
pub(crate) const VALIDATION_TARGET_1144_QUERY_100: &str = "#/$defs/targets/v1144";
pub(crate) const VALIDATION_TARGET_1145_QUERY_101: &str = "#/$defs/targets/v1145";
pub(crate) const VALIDATION_TARGET_1146_QUERY_102: &str = "#/$defs/targets/v1146";
pub(crate) const VALIDATION_TARGET_1147_QUERY_103: &str = "#/$defs/targets/v1147";
pub(crate) const VALIDATION_TARGET_1148_QUERY_104: &str = "#/$defs/targets/v1148";
pub(crate) const VALIDATION_TARGET_1149_QUERY_105: &str = "#/$defs/targets/v1149";
pub(crate) const VALIDATION_TARGET_1150_QUERY_106: &str = "#/$defs/targets/v1150";
pub(crate) const VALIDATION_TARGET_1151_QUERY_107: &str = "#/$defs/targets/v1151";
pub(crate) const VALIDATION_TARGET_1152_QUERY_108: &str = "#/$defs/targets/v1152";
pub(crate) const VALIDATION_TARGET_1153_QUERY_109: &str = "#/$defs/targets/v1153";
pub(crate) const VALIDATION_TARGET_1154_QUERY_110: &str = "#/$defs/targets/v1154";
pub(crate) const VALIDATION_TARGET_1155_BODY: &str = "#/$defs/targets/v1155";
pub(crate) const VALIDATION_TARGET_1156_QUERY_0: &str = "#/$defs/targets/v1156";
pub(crate) const VALIDATION_TARGET_1157_QUERY_1: &str = "#/$defs/targets/v1157";
pub(crate) const VALIDATION_TARGET_1158_QUERY_2: &str = "#/$defs/targets/v1158";
pub(crate) const VALIDATION_TARGET_1159_QUERY_3: &str = "#/$defs/targets/v1159";
pub(crate) const VALIDATION_TARGET_1160_QUERY_4: &str = "#/$defs/targets/v1160";
pub(crate) const VALIDATION_TARGET_1161_QUERY_5: &str = "#/$defs/targets/v1161";
pub(crate) const VALIDATION_TARGET_1162_QUERY_6: &str = "#/$defs/targets/v1162";
pub(crate) const VALIDATION_TARGET_1163_QUERY_7: &str = "#/$defs/targets/v1163";
pub(crate) const VALIDATION_TARGET_1164_QUERY_8: &str = "#/$defs/targets/v1164";
pub(crate) const VALIDATION_TARGET_1165_QUERY_9: &str = "#/$defs/targets/v1165";
pub(crate) const VALIDATION_TARGET_1166_QUERY_10: &str = "#/$defs/targets/v1166";
pub(crate) const VALIDATION_TARGET_1167_QUERY_11: &str = "#/$defs/targets/v1167";
pub(crate) const VALIDATION_TARGET_1168_QUERY_12: &str = "#/$defs/targets/v1168";
pub(crate) const VALIDATION_TARGET_1169_QUERY_13: &str = "#/$defs/targets/v1169";
pub(crate) const VALIDATION_TARGET_1170_QUERY_14: &str = "#/$defs/targets/v1170";
pub(crate) const VALIDATION_TARGET_1171_QUERY_15: &str = "#/$defs/targets/v1171";
pub(crate) const VALIDATION_TARGET_1172_QUERY_16: &str = "#/$defs/targets/v1172";
pub(crate) const VALIDATION_TARGET_1173_QUERY_17: &str = "#/$defs/targets/v1173";
pub(crate) const VALIDATION_TARGET_1174_QUERY_18: &str = "#/$defs/targets/v1174";
pub(crate) const VALIDATION_TARGET_1175_QUERY_19: &str = "#/$defs/targets/v1175";
pub(crate) const VALIDATION_TARGET_1176_QUERY_20: &str = "#/$defs/targets/v1176";
pub(crate) const VALIDATION_TARGET_1177_QUERY_21: &str = "#/$defs/targets/v1177";
pub(crate) const VALIDATION_TARGET_1178_QUERY_22: &str = "#/$defs/targets/v1178";
pub(crate) const VALIDATION_TARGET_1179_QUERY_23: &str = "#/$defs/targets/v1179";
pub(crate) const VALIDATION_TARGET_1180_QUERY_24: &str = "#/$defs/targets/v1180";
pub(crate) const VALIDATION_TARGET_1181_QUERY_25: &str = "#/$defs/targets/v1181";
pub(crate) const VALIDATION_TARGET_1182_QUERY_26: &str = "#/$defs/targets/v1182";
pub(crate) const VALIDATION_TARGET_1183_QUERY_27: &str = "#/$defs/targets/v1183";
pub(crate) const VALIDATION_TARGET_1184_QUERY_28: &str = "#/$defs/targets/v1184";
pub(crate) const VALIDATION_TARGET_1185_QUERY_29: &str = "#/$defs/targets/v1185";
pub(crate) const VALIDATION_TARGET_1186_QUERY_30: &str = "#/$defs/targets/v1186";
pub(crate) const VALIDATION_TARGET_1187_QUERY_31: &str = "#/$defs/targets/v1187";
pub(crate) const VALIDATION_TARGET_1188_QUERY_32: &str = "#/$defs/targets/v1188";
pub(crate) const VALIDATION_TARGET_1189_QUERY_33: &str = "#/$defs/targets/v1189";
pub(crate) const VALIDATION_TARGET_1190_QUERY_34: &str = "#/$defs/targets/v1190";
pub(crate) const VALIDATION_TARGET_1191_QUERY_35: &str = "#/$defs/targets/v1191";
pub(crate) const VALIDATION_TARGET_1192_QUERY_36: &str = "#/$defs/targets/v1192";
pub(crate) const VALIDATION_TARGET_1193_QUERY_37: &str = "#/$defs/targets/v1193";
pub(crate) const VALIDATION_TARGET_1194_QUERY_38: &str = "#/$defs/targets/v1194";
pub(crate) const VALIDATION_TARGET_1195_QUERY_39: &str = "#/$defs/targets/v1195";
pub(crate) const VALIDATION_TARGET_1196_QUERY_40: &str = "#/$defs/targets/v1196";
pub(crate) const VALIDATION_TARGET_1197_QUERY_41: &str = "#/$defs/targets/v1197";
pub(crate) const VALIDATION_TARGET_1198_QUERY_42: &str = "#/$defs/targets/v1198";
pub(crate) const VALIDATION_TARGET_1199_QUERY_43: &str = "#/$defs/targets/v1199";
pub(crate) const VALIDATION_TARGET_1200_QUERY_44: &str = "#/$defs/targets/v1200";
pub(crate) const VALIDATION_TARGET_1201_QUERY_45: &str = "#/$defs/targets/v1201";
pub(crate) const VALIDATION_TARGET_1202_QUERY_46: &str = "#/$defs/targets/v1202";
pub(crate) const VALIDATION_TARGET_1203_QUERY_47: &str = "#/$defs/targets/v1203";
pub(crate) const VALIDATION_TARGET_1204_QUERY_48: &str = "#/$defs/targets/v1204";
pub(crate) const VALIDATION_TARGET_1205_QUERY_49: &str = "#/$defs/targets/v1205";
pub(crate) const VALIDATION_TARGET_1206_QUERY_50: &str = "#/$defs/targets/v1206";
pub(crate) const VALIDATION_TARGET_1207_QUERY_51: &str = "#/$defs/targets/v1207";
pub(crate) const VALIDATION_TARGET_1208_QUERY_52: &str = "#/$defs/targets/v1208";
pub(crate) const VALIDATION_TARGET_1209_QUERY_53: &str = "#/$defs/targets/v1209";
pub(crate) const VALIDATION_TARGET_1210_QUERY_54: &str = "#/$defs/targets/v1210";
pub(crate) const VALIDATION_TARGET_1211_QUERY_55: &str = "#/$defs/targets/v1211";
pub(crate) const VALIDATION_TARGET_1212_QUERY_56: &str = "#/$defs/targets/v1212";
pub(crate) const VALIDATION_TARGET_1213_QUERY_57: &str = "#/$defs/targets/v1213";
pub(crate) const VALIDATION_TARGET_1214_QUERY_58: &str = "#/$defs/targets/v1214";
pub(crate) const VALIDATION_TARGET_1215_QUERY_59: &str = "#/$defs/targets/v1215";
pub(crate) const VALIDATION_TARGET_1216_QUERY_60: &str = "#/$defs/targets/v1216";
pub(crate) const VALIDATION_TARGET_1217_QUERY_61: &str = "#/$defs/targets/v1217";
pub(crate) const VALIDATION_TARGET_1218_QUERY_62: &str = "#/$defs/targets/v1218";
pub(crate) const VALIDATION_TARGET_1219_QUERY_63: &str = "#/$defs/targets/v1219";
pub(crate) const VALIDATION_TARGET_1220_QUERY_64: &str = "#/$defs/targets/v1220";
pub(crate) const VALIDATION_TARGET_1221_QUERY_65: &str = "#/$defs/targets/v1221";
pub(crate) const VALIDATION_TARGET_1222_QUERY_66: &str = "#/$defs/targets/v1222";
pub(crate) const VALIDATION_TARGET_1223_QUERY_67: &str = "#/$defs/targets/v1223";
pub(crate) const VALIDATION_TARGET_1224_QUERY_68: &str = "#/$defs/targets/v1224";
pub(crate) const VALIDATION_TARGET_1225_QUERY_69: &str = "#/$defs/targets/v1225";
pub(crate) const VALIDATION_TARGET_1226_QUERY_70: &str = "#/$defs/targets/v1226";
pub(crate) const VALIDATION_TARGET_1227_QUERY_71: &str = "#/$defs/targets/v1227";
pub(crate) const VALIDATION_TARGET_1228_QUERY_72: &str = "#/$defs/targets/v1228";
pub(crate) const VALIDATION_TARGET_1229_QUERY_73: &str = "#/$defs/targets/v1229";
pub(crate) const VALIDATION_TARGET_1230_QUERY_74: &str = "#/$defs/targets/v1230";
pub(crate) const VALIDATION_TARGET_1231_QUERY_75: &str = "#/$defs/targets/v1231";
pub(crate) const VALIDATION_TARGET_1232_QUERY_76: &str = "#/$defs/targets/v1232";
pub(crate) const VALIDATION_TARGET_1233_QUERY_77: &str = "#/$defs/targets/v1233";
pub(crate) const VALIDATION_TARGET_1234_QUERY_78: &str = "#/$defs/targets/v1234";
pub(crate) const VALIDATION_TARGET_1235_QUERY_79: &str = "#/$defs/targets/v1235";
pub(crate) const VALIDATION_TARGET_1236_QUERY_80: &str = "#/$defs/targets/v1236";
pub(crate) const VALIDATION_TARGET_1237_QUERY_81: &str = "#/$defs/targets/v1237";
pub(crate) const VALIDATION_TARGET_1238_QUERY_82: &str = "#/$defs/targets/v1238";
pub(crate) const VALIDATION_TARGET_1239_QUERY_83: &str = "#/$defs/targets/v1239";
pub(crate) const VALIDATION_TARGET_1240_QUERY_84: &str = "#/$defs/targets/v1240";
pub(crate) const VALIDATION_TARGET_1241_QUERY_85: &str = "#/$defs/targets/v1241";
pub(crate) const VALIDATION_TARGET_1242_QUERY_86: &str = "#/$defs/targets/v1242";
pub(crate) const VALIDATION_TARGET_1243_QUERY_87: &str = "#/$defs/targets/v1243";
pub(crate) const VALIDATION_TARGET_1244_QUERY_88: &str = "#/$defs/targets/v1244";
pub(crate) const VALIDATION_TARGET_1245_QUERY_89: &str = "#/$defs/targets/v1245";
pub(crate) const VALIDATION_TARGET_1246_QUERY_90: &str = "#/$defs/targets/v1246";
pub(crate) const VALIDATION_TARGET_1247_QUERY_91: &str = "#/$defs/targets/v1247";
pub(crate) const VALIDATION_TARGET_1248_QUERY_92: &str = "#/$defs/targets/v1248";
pub(crate) const VALIDATION_TARGET_1249_QUERY_93: &str = "#/$defs/targets/v1249";
pub(crate) const VALIDATION_TARGET_1250_QUERY_94: &str = "#/$defs/targets/v1250";
pub(crate) const VALIDATION_TARGET_1251_QUERY_95: &str = "#/$defs/targets/v1251";
pub(crate) const VALIDATION_TARGET_1252_QUERY_96: &str = "#/$defs/targets/v1252";
pub(crate) const VALIDATION_TARGET_1253_QUERY_97: &str = "#/$defs/targets/v1253";
pub(crate) const VALIDATION_TARGET_1254_QUERY_98: &str = "#/$defs/targets/v1254";
pub(crate) const VALIDATION_TARGET_1255_QUERY_99: &str = "#/$defs/targets/v1255";
pub(crate) const VALIDATION_TARGET_1256_QUERY_100: &str = "#/$defs/targets/v1256";
pub(crate) const VALIDATION_TARGET_1257_QUERY_101: &str = "#/$defs/targets/v1257";
pub(crate) const VALIDATION_TARGET_1258_QUERY_102: &str = "#/$defs/targets/v1258";
pub(crate) const VALIDATION_TARGET_1259_QUERY_103: &str = "#/$defs/targets/v1259";
pub(crate) const VALIDATION_TARGET_1260_QUERY_104: &str = "#/$defs/targets/v1260";
pub(crate) const VALIDATION_TARGET_1261_QUERY_105: &str = "#/$defs/targets/v1261";
pub(crate) const VALIDATION_TARGET_1262_QUERY_106: &str = "#/$defs/targets/v1262";
pub(crate) const VALIDATION_TARGET_1263_QUERY_107: &str = "#/$defs/targets/v1263";
pub(crate) const VALIDATION_TARGET_1264_QUERY_108: &str = "#/$defs/targets/v1264";
pub(crate) const VALIDATION_TARGET_1265_QUERY_109: &str = "#/$defs/targets/v1265";
pub(crate) const VALIDATION_TARGET_1266_QUERY_110: &str = "#/$defs/targets/v1266";
pub(crate) const VALIDATION_TARGET_1267_PATH_0: &str = "#/$defs/targets/v1267";
pub(crate) const VALIDATION_TARGET_1268_QUERY_1: &str = "#/$defs/targets/v1268";
pub(crate) const VALIDATION_TARGET_1269_PATH_0: &str = "#/$defs/targets/v1269";
pub(crate) const VALIDATION_TARGET_1270_QUERY_1: &str = "#/$defs/targets/v1270";
pub(crate) const VALIDATION_TARGET_1271_QUERY_0: &str = "#/$defs/targets/v1271";
pub(crate) const VALIDATION_TARGET_1272_QUERY_1: &str = "#/$defs/targets/v1272";
pub(crate) const VALIDATION_TARGET_1273_QUERY_2: &str = "#/$defs/targets/v1273";
pub(crate) const VALIDATION_TARGET_1274_QUERY_3: &str = "#/$defs/targets/v1274";
pub(crate) const VALIDATION_TARGET_1275_QUERY_4: &str = "#/$defs/targets/v1275";
pub(crate) const VALIDATION_TARGET_1276_QUERY_5: &str = "#/$defs/targets/v1276";
pub(crate) const VALIDATION_TARGET_1277_QUERY_6: &str = "#/$defs/targets/v1277";
pub(crate) const VALIDATION_TARGET_1278_QUERY_7: &str = "#/$defs/targets/v1278";
pub(crate) const VALIDATION_TARGET_1279_QUERY_8: &str = "#/$defs/targets/v1279";
pub(crate) const VALIDATION_TARGET_1280_QUERY_9: &str = "#/$defs/targets/v1280";
pub(crate) const VALIDATION_TARGET_1281_QUERY_10: &str = "#/$defs/targets/v1281";
pub(crate) const VALIDATION_TARGET_1282_QUERY_11: &str = "#/$defs/targets/v1282";
pub(crate) const VALIDATION_TARGET_1283_QUERY_12: &str = "#/$defs/targets/v1283";
pub(crate) const VALIDATION_TARGET_1284_QUERY_13: &str = "#/$defs/targets/v1284";
pub(crate) const VALIDATION_TARGET_1285_QUERY_14: &str = "#/$defs/targets/v1285";
pub(crate) const VALIDATION_TARGET_1286_QUERY_15: &str = "#/$defs/targets/v1286";
pub(crate) const VALIDATION_TARGET_1287_QUERY_16: &str = "#/$defs/targets/v1287";
pub(crate) const VALIDATION_TARGET_1288_QUERY_17: &str = "#/$defs/targets/v1288";
pub(crate) const VALIDATION_TARGET_1289_QUERY_18: &str = "#/$defs/targets/v1289";
pub(crate) const VALIDATION_TARGET_1290_QUERY_19: &str = "#/$defs/targets/v1290";
pub(crate) const VALIDATION_TARGET_1291_QUERY_20: &str = "#/$defs/targets/v1291";
pub(crate) const VALIDATION_TARGET_1292_QUERY_21: &str = "#/$defs/targets/v1292";
pub(crate) const VALIDATION_TARGET_1293_QUERY_22: &str = "#/$defs/targets/v1293";
pub(crate) const VALIDATION_TARGET_1294_QUERY_23: &str = "#/$defs/targets/v1294";
pub(crate) const VALIDATION_TARGET_1295_QUERY_24: &str = "#/$defs/targets/v1295";
pub(crate) const VALIDATION_TARGET_1296_QUERY_25: &str = "#/$defs/targets/v1296";
pub(crate) const VALIDATION_TARGET_1297_QUERY_26: &str = "#/$defs/targets/v1297";
pub(crate) const VALIDATION_TARGET_1298_QUERY_27: &str = "#/$defs/targets/v1298";
pub(crate) const VALIDATION_TARGET_1299_QUERY_28: &str = "#/$defs/targets/v1299";
pub(crate) const VALIDATION_TARGET_1300_QUERY_29: &str = "#/$defs/targets/v1300";
pub(crate) const VALIDATION_TARGET_1301_QUERY_30: &str = "#/$defs/targets/v1301";
pub(crate) const VALIDATION_TARGET_1302_QUERY_31: &str = "#/$defs/targets/v1302";
pub(crate) const VALIDATION_TARGET_1303_QUERY_32: &str = "#/$defs/targets/v1303";
pub(crate) const VALIDATION_TARGET_1304_QUERY_33: &str = "#/$defs/targets/v1304";
pub(crate) const VALIDATION_TARGET_1305_QUERY_34: &str = "#/$defs/targets/v1305";
pub(crate) const VALIDATION_TARGET_1306_QUERY_35: &str = "#/$defs/targets/v1306";
pub(crate) const VALIDATION_TARGET_1307_QUERY_36: &str = "#/$defs/targets/v1307";
pub(crate) const VALIDATION_TARGET_1308_QUERY_37: &str = "#/$defs/targets/v1308";
pub(crate) const VALIDATION_TARGET_1309_QUERY_38: &str = "#/$defs/targets/v1309";
pub(crate) const VALIDATION_TARGET_1310_QUERY_39: &str = "#/$defs/targets/v1310";
pub(crate) const VALIDATION_TARGET_1311_QUERY_40: &str = "#/$defs/targets/v1311";
pub(crate) const VALIDATION_TARGET_1312_QUERY_41: &str = "#/$defs/targets/v1312";
pub(crate) const VALIDATION_TARGET_1313_QUERY_42: &str = "#/$defs/targets/v1313";
pub(crate) const VALIDATION_TARGET_1314_QUERY_43: &str = "#/$defs/targets/v1314";
pub(crate) const VALIDATION_TARGET_1315_QUERY_44: &str = "#/$defs/targets/v1315";
pub(crate) const VALIDATION_TARGET_1316_QUERY_45: &str = "#/$defs/targets/v1316";
pub(crate) const VALIDATION_TARGET_1317_QUERY_46: &str = "#/$defs/targets/v1317";
pub(crate) const VALIDATION_TARGET_1318_QUERY_47: &str = "#/$defs/targets/v1318";
pub(crate) const VALIDATION_TARGET_1319_QUERY_48: &str = "#/$defs/targets/v1319";
pub(crate) const VALIDATION_TARGET_1320_QUERY_49: &str = "#/$defs/targets/v1320";
pub(crate) const VALIDATION_TARGET_1321_QUERY_50: &str = "#/$defs/targets/v1321";
pub(crate) const VALIDATION_TARGET_1322_QUERY_51: &str = "#/$defs/targets/v1322";
pub(crate) const VALIDATION_TARGET_1323_QUERY_52: &str = "#/$defs/targets/v1323";
pub(crate) const VALIDATION_TARGET_1324_QUERY_53: &str = "#/$defs/targets/v1324";
pub(crate) const VALIDATION_TARGET_1325_QUERY_54: &str = "#/$defs/targets/v1325";
pub(crate) const VALIDATION_TARGET_1326_QUERY_55: &str = "#/$defs/targets/v1326";
pub(crate) const VALIDATION_TARGET_1327_QUERY_56: &str = "#/$defs/targets/v1327";
pub(crate) const VALIDATION_TARGET_1328_QUERY_57: &str = "#/$defs/targets/v1328";
pub(crate) const VALIDATION_TARGET_1329_QUERY_58: &str = "#/$defs/targets/v1329";
pub(crate) const VALIDATION_TARGET_1330_QUERY_59: &str = "#/$defs/targets/v1330";
pub(crate) const VALIDATION_TARGET_1331_QUERY_60: &str = "#/$defs/targets/v1331";
pub(crate) const VALIDATION_TARGET_1332_QUERY_61: &str = "#/$defs/targets/v1332";
pub(crate) const VALIDATION_TARGET_1333_QUERY_62: &str = "#/$defs/targets/v1333";
pub(crate) const VALIDATION_TARGET_1334_QUERY_63: &str = "#/$defs/targets/v1334";
pub(crate) const VALIDATION_TARGET_1335_QUERY_64: &str = "#/$defs/targets/v1335";
pub(crate) const VALIDATION_TARGET_1336_QUERY_65: &str = "#/$defs/targets/v1336";
pub(crate) const VALIDATION_TARGET_1337_QUERY_66: &str = "#/$defs/targets/v1337";
pub(crate) const VALIDATION_TARGET_1338_QUERY_67: &str = "#/$defs/targets/v1338";
pub(crate) const VALIDATION_TARGET_1339_QUERY_68: &str = "#/$defs/targets/v1339";
pub(crate) const VALIDATION_TARGET_1340_QUERY_69: &str = "#/$defs/targets/v1340";
pub(crate) const VALIDATION_TARGET_1341_QUERY_70: &str = "#/$defs/targets/v1341";
pub(crate) const VALIDATION_TARGET_1342_QUERY_71: &str = "#/$defs/targets/v1342";
pub(crate) const VALIDATION_TARGET_1343_QUERY_72: &str = "#/$defs/targets/v1343";
pub(crate) const VALIDATION_TARGET_1344_QUERY_73: &str = "#/$defs/targets/v1344";
pub(crate) const VALIDATION_TARGET_1345_QUERY_74: &str = "#/$defs/targets/v1345";
pub(crate) const VALIDATION_TARGET_1346_QUERY_75: &str = "#/$defs/targets/v1346";
pub(crate) const VALIDATION_TARGET_1347_QUERY_76: &str = "#/$defs/targets/v1347";
pub(crate) const VALIDATION_TARGET_1348_QUERY_77: &str = "#/$defs/targets/v1348";
pub(crate) const VALIDATION_TARGET_1349_QUERY_78: &str = "#/$defs/targets/v1349";
pub(crate) const VALIDATION_TARGET_1350_QUERY_79: &str = "#/$defs/targets/v1350";
pub(crate) const VALIDATION_TARGET_1351_QUERY_80: &str = "#/$defs/targets/v1351";
pub(crate) const VALIDATION_TARGET_1352_QUERY_81: &str = "#/$defs/targets/v1352";
pub(crate) const VALIDATION_TARGET_1353_QUERY_82: &str = "#/$defs/targets/v1353";
pub(crate) const VALIDATION_TARGET_1354_QUERY_83: &str = "#/$defs/targets/v1354";
pub(crate) const VALIDATION_TARGET_1355_QUERY_84: &str = "#/$defs/targets/v1355";
pub(crate) const VALIDATION_TARGET_1356_QUERY_85: &str = "#/$defs/targets/v1356";
pub(crate) const VALIDATION_TARGET_1357_QUERY_86: &str = "#/$defs/targets/v1357";
pub(crate) const VALIDATION_TARGET_1358_QUERY_87: &str = "#/$defs/targets/v1358";
pub(crate) const VALIDATION_TARGET_1359_QUERY_88: &str = "#/$defs/targets/v1359";
pub(crate) const VALIDATION_TARGET_1360_QUERY_89: &str = "#/$defs/targets/v1360";
pub(crate) const VALIDATION_TARGET_1361_QUERY_90: &str = "#/$defs/targets/v1361";
pub(crate) const VALIDATION_TARGET_1362_QUERY_91: &str = "#/$defs/targets/v1362";
pub(crate) const VALIDATION_TARGET_1363_QUERY_92: &str = "#/$defs/targets/v1363";
pub(crate) const VALIDATION_TARGET_1364_QUERY_93: &str = "#/$defs/targets/v1364";
pub(crate) const VALIDATION_TARGET_1365_QUERY_94: &str = "#/$defs/targets/v1365";
pub(crate) const VALIDATION_TARGET_1366_QUERY_95: &str = "#/$defs/targets/v1366";
pub(crate) const VALIDATION_TARGET_1367_QUERY_96: &str = "#/$defs/targets/v1367";
pub(crate) const VALIDATION_TARGET_1368_QUERY_97: &str = "#/$defs/targets/v1368";
pub(crate) const VALIDATION_TARGET_1369_QUERY_98: &str = "#/$defs/targets/v1369";
pub(crate) const VALIDATION_TARGET_1370_QUERY_99: &str = "#/$defs/targets/v1370";
pub(crate) const VALIDATION_TARGET_1371_QUERY_100: &str = "#/$defs/targets/v1371";
pub(crate) const VALIDATION_TARGET_1372_QUERY_101: &str = "#/$defs/targets/v1372";
pub(crate) const VALIDATION_TARGET_1373_QUERY_102: &str = "#/$defs/targets/v1373";
pub(crate) const VALIDATION_TARGET_1374_QUERY_103: &str = "#/$defs/targets/v1374";
pub(crate) const VALIDATION_TARGET_1375_QUERY_104: &str = "#/$defs/targets/v1375";
pub(crate) const VALIDATION_TARGET_1376_QUERY_105: &str = "#/$defs/targets/v1376";
pub(crate) const VALIDATION_TARGET_1377_QUERY_106: &str = "#/$defs/targets/v1377";
pub(crate) const VALIDATION_TARGET_1378_QUERY_107: &str = "#/$defs/targets/v1378";
pub(crate) const VALIDATION_TARGET_1379_QUERY_108: &str = "#/$defs/targets/v1379";
pub(crate) const VALIDATION_TARGET_1380_QUERY_109: &str = "#/$defs/targets/v1380";
pub(crate) const VALIDATION_TARGET_1381_QUERY_110: &str = "#/$defs/targets/v1381";
pub(crate) const VALIDATION_TARGET_1382_QUERY_111: &str = "#/$defs/targets/v1382";
pub(crate) const VALIDATION_TARGET_1383_QUERY_112: &str = "#/$defs/targets/v1383";
pub(crate) const VALIDATION_TARGET_1384_QUERY_113: &str = "#/$defs/targets/v1384";
pub(crate) const VALIDATION_TARGET_1385_QUERY_114: &str = "#/$defs/targets/v1385";
pub(crate) const VALIDATION_TARGET_1386_QUERY_115: &str = "#/$defs/targets/v1386";
pub(crate) const VALIDATION_TARGET_1387_QUERY_116: &str = "#/$defs/targets/v1387";
pub(crate) const VALIDATION_TARGET_1388_QUERY_117: &str = "#/$defs/targets/v1388";
pub(crate) const VALIDATION_TARGET_1389_QUERY_118: &str = "#/$defs/targets/v1389";
pub(crate) const VALIDATION_TARGET_1390_QUERY_119: &str = "#/$defs/targets/v1390";
pub(crate) const VALIDATION_TARGET_1391_QUERY_120: &str = "#/$defs/targets/v1391";
pub(crate) const VALIDATION_TARGET_1392_QUERY_121: &str = "#/$defs/targets/v1392";
pub(crate) const VALIDATION_TARGET_1393_QUERY_122: &str = "#/$defs/targets/v1393";
pub(crate) const VALIDATION_TARGET_1394_QUERY_123: &str = "#/$defs/targets/v1394";
pub(crate) const VALIDATION_TARGET_1395_QUERY_124: &str = "#/$defs/targets/v1395";
pub(crate) const VALIDATION_TARGET_1396_QUERY_125: &str = "#/$defs/targets/v1396";
pub(crate) const VALIDATION_TARGET_1397_QUERY_126: &str = "#/$defs/targets/v1397";
pub(crate) const VALIDATION_TARGET_1398_QUERY_127: &str = "#/$defs/targets/v1398";
pub(crate) const VALIDATION_TARGET_1399_QUERY_128: &str = "#/$defs/targets/v1399";
pub(crate) const VALIDATION_TARGET_1400_QUERY_129: &str = "#/$defs/targets/v1400";
pub(crate) const VALIDATION_TARGET_1401_QUERY_130: &str = "#/$defs/targets/v1401";
pub(crate) const VALIDATION_TARGET_1402_QUERY_131: &str = "#/$defs/targets/v1402";
pub(crate) const VALIDATION_TARGET_1403_QUERY_132: &str = "#/$defs/targets/v1403";
pub(crate) const VALIDATION_TARGET_1404_QUERY_133: &str = "#/$defs/targets/v1404";
pub(crate) const VALIDATION_TARGET_1405_QUERY_134: &str = "#/$defs/targets/v1405";
pub(crate) const VALIDATION_TARGET_1406_QUERY_135: &str = "#/$defs/targets/v1406";
pub(crate) const VALIDATION_TARGET_1407_QUERY_136: &str = "#/$defs/targets/v1407";
pub(crate) const VALIDATION_TARGET_1408_QUERY_137: &str = "#/$defs/targets/v1408";
pub(crate) const VALIDATION_TARGET_1409_QUERY_138: &str = "#/$defs/targets/v1409";
pub(crate) const VALIDATION_TARGET_1410_QUERY_139: &str = "#/$defs/targets/v1410";
pub(crate) const VALIDATION_TARGET_1411_QUERY_140: &str = "#/$defs/targets/v1411";
pub(crate) const VALIDATION_TARGET_1412_QUERY_141: &str = "#/$defs/targets/v1412";
pub(crate) const VALIDATION_TARGET_1413_QUERY_142: &str = "#/$defs/targets/v1413";
pub(crate) const VALIDATION_TARGET_1414_QUERY_143: &str = "#/$defs/targets/v1414";
pub(crate) const VALIDATION_TARGET_1415_QUERY_144: &str = "#/$defs/targets/v1415";
pub(crate) const VALIDATION_TARGET_1416_QUERY_145: &str = "#/$defs/targets/v1416";
pub(crate) const VALIDATION_TARGET_1417_QUERY_146: &str = "#/$defs/targets/v1417";
pub(crate) const VALIDATION_TARGET_1418_QUERY_147: &str = "#/$defs/targets/v1418";
pub(crate) const VALIDATION_TARGET_1419_QUERY_148: &str = "#/$defs/targets/v1419";
pub(crate) const VALIDATION_TARGET_1420_QUERY_149: &str = "#/$defs/targets/v1420";
pub(crate) const VALIDATION_TARGET_1421_QUERY_150: &str = "#/$defs/targets/v1421";
pub(crate) const VALIDATION_TARGET_1422_QUERY_151: &str = "#/$defs/targets/v1422";
pub(crate) const VALIDATION_TARGET_1423_QUERY_152: &str = "#/$defs/targets/v1423";
pub(crate) const VALIDATION_TARGET_1424_QUERY_153: &str = "#/$defs/targets/v1424";
pub(crate) const VALIDATION_TARGET_1425_QUERY_154: &str = "#/$defs/targets/v1425";
pub(crate) const VALIDATION_TARGET_1426_QUERY_155: &str = "#/$defs/targets/v1426";
pub(crate) const VALIDATION_TARGET_1427_QUERY_156: &str = "#/$defs/targets/v1427";
pub(crate) const VALIDATION_TARGET_1428_QUERY_157: &str = "#/$defs/targets/v1428";
pub(crate) const VALIDATION_TARGET_1429_BODY: &str = "#/$defs/targets/v1429";
pub(crate) const VALIDATION_TARGET_1430_PATH_0: &str = "#/$defs/targets/v1430";
pub(crate) const VALIDATION_TARGET_1431_QUERY_1: &str = "#/$defs/targets/v1431";
pub(crate) const VALIDATION_TARGET_1432_BODY: &str = "#/$defs/targets/v1432";
pub(crate) const VALIDATION_TARGET_1433_QUERY_0: &str = "#/$defs/targets/v1433";
pub(crate) const VALIDATION_TARGET_1434_PATH_0: &str = "#/$defs/targets/v1434";
pub(crate) const VALIDATION_TARGET_1435_QUERY_1: &str = "#/$defs/targets/v1435";
pub(crate) const VALIDATION_TARGET_1436_PATH_0: &str = "#/$defs/targets/v1436";
pub(crate) const VALIDATION_TARGET_1437_QUERY_1: &str = "#/$defs/targets/v1437";
pub(crate) const VALIDATION_TARGET_1438_QUERY_0: &str = "#/$defs/targets/v1438";
pub(crate) const VALIDATION_TARGET_1439_QUERY_1: &str = "#/$defs/targets/v1439";
pub(crate) const VALIDATION_TARGET_1440_QUERY_2: &str = "#/$defs/targets/v1440";
pub(crate) const VALIDATION_TARGET_1441_QUERY_3: &str = "#/$defs/targets/v1441";
pub(crate) const VALIDATION_TARGET_1442_QUERY_4: &str = "#/$defs/targets/v1442";
pub(crate) const VALIDATION_TARGET_1443_QUERY_5: &str = "#/$defs/targets/v1443";
pub(crate) const VALIDATION_TARGET_1444_QUERY_6: &str = "#/$defs/targets/v1444";
pub(crate) const VALIDATION_TARGET_1445_QUERY_7: &str = "#/$defs/targets/v1445";
pub(crate) const VALIDATION_TARGET_1446_QUERY_8: &str = "#/$defs/targets/v1446";
pub(crate) const VALIDATION_TARGET_1447_QUERY_9: &str = "#/$defs/targets/v1447";
pub(crate) const VALIDATION_TARGET_1448_QUERY_10: &str = "#/$defs/targets/v1448";
pub(crate) const VALIDATION_TARGET_1449_QUERY_11: &str = "#/$defs/targets/v1449";
pub(crate) const VALIDATION_TARGET_1450_QUERY_12: &str = "#/$defs/targets/v1450";
pub(crate) const VALIDATION_TARGET_1451_QUERY_13: &str = "#/$defs/targets/v1451";
pub(crate) const VALIDATION_TARGET_1452_QUERY_14: &str = "#/$defs/targets/v1452";
pub(crate) const VALIDATION_TARGET_1453_QUERY_15: &str = "#/$defs/targets/v1453";
pub(crate) const VALIDATION_TARGET_1454_QUERY_16: &str = "#/$defs/targets/v1454";
pub(crate) const VALIDATION_TARGET_1455_QUERY_17: &str = "#/$defs/targets/v1455";
pub(crate) const VALIDATION_TARGET_1456_QUERY_18: &str = "#/$defs/targets/v1456";
pub(crate) const VALIDATION_TARGET_1457_QUERY_19: &str = "#/$defs/targets/v1457";
pub(crate) const VALIDATION_TARGET_1458_QUERY_20: &str = "#/$defs/targets/v1458";
pub(crate) const VALIDATION_TARGET_1459_QUERY_21: &str = "#/$defs/targets/v1459";
pub(crate) const VALIDATION_TARGET_1460_QUERY_22: &str = "#/$defs/targets/v1460";
pub(crate) const VALIDATION_TARGET_1461_QUERY_23: &str = "#/$defs/targets/v1461";
pub(crate) const VALIDATION_TARGET_1462_QUERY_24: &str = "#/$defs/targets/v1462";
pub(crate) const VALIDATION_TARGET_1463_QUERY_25: &str = "#/$defs/targets/v1463";
pub(crate) const VALIDATION_TARGET_1464_QUERY_26: &str = "#/$defs/targets/v1464";
pub(crate) const VALIDATION_TARGET_1465_QUERY_27: &str = "#/$defs/targets/v1465";
pub(crate) const VALIDATION_TARGET_1466_QUERY_28: &str = "#/$defs/targets/v1466";
pub(crate) const VALIDATION_TARGET_1467_QUERY_29: &str = "#/$defs/targets/v1467";
pub(crate) const VALIDATION_TARGET_1468_QUERY_30: &str = "#/$defs/targets/v1468";
pub(crate) const VALIDATION_TARGET_1469_QUERY_31: &str = "#/$defs/targets/v1469";
pub(crate) const VALIDATION_TARGET_1470_QUERY_32: &str = "#/$defs/targets/v1470";
pub(crate) const VALIDATION_TARGET_1471_QUERY_33: &str = "#/$defs/targets/v1471";
pub(crate) const VALIDATION_TARGET_1472_QUERY_34: &str = "#/$defs/targets/v1472";
pub(crate) const VALIDATION_TARGET_1473_QUERY_35: &str = "#/$defs/targets/v1473";
pub(crate) const VALIDATION_TARGET_1474_QUERY_36: &str = "#/$defs/targets/v1474";
pub(crate) const VALIDATION_TARGET_1475_QUERY_37: &str = "#/$defs/targets/v1475";
pub(crate) const VALIDATION_TARGET_1476_QUERY_38: &str = "#/$defs/targets/v1476";
pub(crate) const VALIDATION_TARGET_1477_QUERY_39: &str = "#/$defs/targets/v1477";
pub(crate) const VALIDATION_TARGET_1478_QUERY_40: &str = "#/$defs/targets/v1478";
pub(crate) const VALIDATION_TARGET_1479_QUERY_41: &str = "#/$defs/targets/v1479";
pub(crate) const VALIDATION_TARGET_1480_QUERY_42: &str = "#/$defs/targets/v1480";
pub(crate) const VALIDATION_TARGET_1481_QUERY_43: &str = "#/$defs/targets/v1481";
pub(crate) const VALIDATION_TARGET_1482_QUERY_44: &str = "#/$defs/targets/v1482";
pub(crate) const VALIDATION_TARGET_1483_QUERY_45: &str = "#/$defs/targets/v1483";
pub(crate) const VALIDATION_TARGET_1484_QUERY_46: &str = "#/$defs/targets/v1484";
pub(crate) const VALIDATION_TARGET_1485_QUERY_47: &str = "#/$defs/targets/v1485";
pub(crate) const VALIDATION_TARGET_1486_QUERY_48: &str = "#/$defs/targets/v1486";
pub(crate) const VALIDATION_TARGET_1487_QUERY_49: &str = "#/$defs/targets/v1487";
pub(crate) const VALIDATION_TARGET_1488_QUERY_50: &str = "#/$defs/targets/v1488";
pub(crate) const VALIDATION_TARGET_1489_QUERY_51: &str = "#/$defs/targets/v1489";
pub(crate) const VALIDATION_TARGET_1490_QUERY_52: &str = "#/$defs/targets/v1490";
pub(crate) const VALIDATION_TARGET_1491_QUERY_53: &str = "#/$defs/targets/v1491";
pub(crate) const VALIDATION_TARGET_1492_QUERY_54: &str = "#/$defs/targets/v1492";
pub(crate) const VALIDATION_TARGET_1493_QUERY_55: &str = "#/$defs/targets/v1493";
pub(crate) const VALIDATION_TARGET_1494_QUERY_56: &str = "#/$defs/targets/v1494";
pub(crate) const VALIDATION_TARGET_1495_QUERY_57: &str = "#/$defs/targets/v1495";
pub(crate) const VALIDATION_TARGET_1496_QUERY_58: &str = "#/$defs/targets/v1496";
pub(crate) const VALIDATION_TARGET_1497_QUERY_59: &str = "#/$defs/targets/v1497";
pub(crate) const VALIDATION_TARGET_1498_QUERY_60: &str = "#/$defs/targets/v1498";
pub(crate) const VALIDATION_TARGET_1499_QUERY_61: &str = "#/$defs/targets/v1499";
pub(crate) const VALIDATION_TARGET_1500_QUERY_62: &str = "#/$defs/targets/v1500";
pub(crate) const VALIDATION_TARGET_1501_QUERY_63: &str = "#/$defs/targets/v1501";
pub(crate) const VALIDATION_TARGET_1502_QUERY_64: &str = "#/$defs/targets/v1502";
pub(crate) const VALIDATION_TARGET_1503_QUERY_65: &str = "#/$defs/targets/v1503";
pub(crate) const VALIDATION_TARGET_1504_QUERY_66: &str = "#/$defs/targets/v1504";
pub(crate) const VALIDATION_TARGET_1505_QUERY_67: &str = "#/$defs/targets/v1505";
pub(crate) const VALIDATION_TARGET_1506_QUERY_68: &str = "#/$defs/targets/v1506";
pub(crate) const VALIDATION_TARGET_1507_QUERY_69: &str = "#/$defs/targets/v1507";
pub(crate) const VALIDATION_TARGET_1508_QUERY_70: &str = "#/$defs/targets/v1508";
pub(crate) const VALIDATION_TARGET_1509_QUERY_71: &str = "#/$defs/targets/v1509";
pub(crate) const VALIDATION_TARGET_1510_QUERY_72: &str = "#/$defs/targets/v1510";
pub(crate) const VALIDATION_TARGET_1511_QUERY_73: &str = "#/$defs/targets/v1511";
pub(crate) const VALIDATION_TARGET_1512_QUERY_74: &str = "#/$defs/targets/v1512";
pub(crate) const VALIDATION_TARGET_1513_QUERY_75: &str = "#/$defs/targets/v1513";
pub(crate) const VALIDATION_TARGET_1514_QUERY_76: &str = "#/$defs/targets/v1514";
pub(crate) const VALIDATION_TARGET_1515_QUERY_77: &str = "#/$defs/targets/v1515";
pub(crate) const VALIDATION_TARGET_1516_QUERY_78: &str = "#/$defs/targets/v1516";
pub(crate) const VALIDATION_TARGET_1517_QUERY_79: &str = "#/$defs/targets/v1517";
pub(crate) const VALIDATION_TARGET_1518_QUERY_80: &str = "#/$defs/targets/v1518";
pub(crate) const VALIDATION_TARGET_1519_QUERY_81: &str = "#/$defs/targets/v1519";
pub(crate) const VALIDATION_TARGET_1520_QUERY_82: &str = "#/$defs/targets/v1520";
pub(crate) const VALIDATION_TARGET_1521_QUERY_83: &str = "#/$defs/targets/v1521";
pub(crate) const VALIDATION_TARGET_1522_QUERY_84: &str = "#/$defs/targets/v1522";
pub(crate) const VALIDATION_TARGET_1523_QUERY_85: &str = "#/$defs/targets/v1523";
pub(crate) const VALIDATION_TARGET_1524_QUERY_86: &str = "#/$defs/targets/v1524";
pub(crate) const VALIDATION_TARGET_1525_QUERY_87: &str = "#/$defs/targets/v1525";
pub(crate) const VALIDATION_TARGET_1526_QUERY_88: &str = "#/$defs/targets/v1526";
pub(crate) const VALIDATION_TARGET_1527_QUERY_89: &str = "#/$defs/targets/v1527";
pub(crate) const VALIDATION_TARGET_1528_QUERY_90: &str = "#/$defs/targets/v1528";
pub(crate) const VALIDATION_TARGET_1529_QUERY_91: &str = "#/$defs/targets/v1529";
pub(crate) const VALIDATION_TARGET_1530_QUERY_92: &str = "#/$defs/targets/v1530";
pub(crate) const VALIDATION_TARGET_1531_QUERY_93: &str = "#/$defs/targets/v1531";
pub(crate) const VALIDATION_TARGET_1532_QUERY_94: &str = "#/$defs/targets/v1532";
pub(crate) const VALIDATION_TARGET_1533_QUERY_95: &str = "#/$defs/targets/v1533";
pub(crate) const VALIDATION_TARGET_1534_QUERY_96: &str = "#/$defs/targets/v1534";
pub(crate) const VALIDATION_TARGET_1535_QUERY_97: &str = "#/$defs/targets/v1535";
pub(crate) const VALIDATION_TARGET_1536_QUERY_98: &str = "#/$defs/targets/v1536";
pub(crate) const VALIDATION_TARGET_1537_QUERY_99: &str = "#/$defs/targets/v1537";
pub(crate) const VALIDATION_TARGET_1538_QUERY_100: &str = "#/$defs/targets/v1538";
pub(crate) const VALIDATION_TARGET_1539_QUERY_101: &str = "#/$defs/targets/v1539";
pub(crate) const VALIDATION_TARGET_1540_QUERY_102: &str = "#/$defs/targets/v1540";
pub(crate) const VALIDATION_TARGET_1541_QUERY_103: &str = "#/$defs/targets/v1541";
pub(crate) const VALIDATION_TARGET_1542_QUERY_104: &str = "#/$defs/targets/v1542";
pub(crate) const VALIDATION_TARGET_1543_QUERY_105: &str = "#/$defs/targets/v1543";
pub(crate) const VALIDATION_TARGET_1544_QUERY_106: &str = "#/$defs/targets/v1544";
pub(crate) const VALIDATION_TARGET_1545_QUERY_107: &str = "#/$defs/targets/v1545";
pub(crate) const VALIDATION_TARGET_1546_QUERY_108: &str = "#/$defs/targets/v1546";
pub(crate) const VALIDATION_TARGET_1547_QUERY_109: &str = "#/$defs/targets/v1547";
pub(crate) const VALIDATION_TARGET_1548_QUERY_110: &str = "#/$defs/targets/v1548";
pub(crate) const VALIDATION_TARGET_1549_QUERY_111: &str = "#/$defs/targets/v1549";
pub(crate) const VALIDATION_TARGET_1550_QUERY_112: &str = "#/$defs/targets/v1550";
pub(crate) const VALIDATION_TARGET_1551_QUERY_113: &str = "#/$defs/targets/v1551";
pub(crate) const VALIDATION_TARGET_1552_QUERY_114: &str = "#/$defs/targets/v1552";
pub(crate) const VALIDATION_TARGET_1553_QUERY_115: &str = "#/$defs/targets/v1553";
pub(crate) const VALIDATION_TARGET_1554_QUERY_116: &str = "#/$defs/targets/v1554";
pub(crate) const VALIDATION_TARGET_1555_QUERY_117: &str = "#/$defs/targets/v1555";
pub(crate) const VALIDATION_TARGET_1556_QUERY_118: &str = "#/$defs/targets/v1556";
pub(crate) const VALIDATION_TARGET_1557_QUERY_119: &str = "#/$defs/targets/v1557";
pub(crate) const VALIDATION_TARGET_1558_QUERY_120: &str = "#/$defs/targets/v1558";
pub(crate) const VALIDATION_TARGET_1559_QUERY_121: &str = "#/$defs/targets/v1559";
pub(crate) const VALIDATION_TARGET_1560_QUERY_122: &str = "#/$defs/targets/v1560";
pub(crate) const VALIDATION_TARGET_1561_QUERY_123: &str = "#/$defs/targets/v1561";
pub(crate) const VALIDATION_TARGET_1562_QUERY_124: &str = "#/$defs/targets/v1562";
pub(crate) const VALIDATION_TARGET_1563_QUERY_125: &str = "#/$defs/targets/v1563";
pub(crate) const VALIDATION_TARGET_1564_QUERY_126: &str = "#/$defs/targets/v1564";
pub(crate) const VALIDATION_TARGET_1565_QUERY_127: &str = "#/$defs/targets/v1565";
pub(crate) const VALIDATION_TARGET_1566_QUERY_128: &str = "#/$defs/targets/v1566";
pub(crate) const VALIDATION_TARGET_1567_QUERY_129: &str = "#/$defs/targets/v1567";
pub(crate) const VALIDATION_TARGET_1568_QUERY_130: &str = "#/$defs/targets/v1568";
pub(crate) const VALIDATION_TARGET_1569_QUERY_131: &str = "#/$defs/targets/v1569";
pub(crate) const VALIDATION_TARGET_1570_QUERY_132: &str = "#/$defs/targets/v1570";
pub(crate) const VALIDATION_TARGET_1571_QUERY_133: &str = "#/$defs/targets/v1571";
pub(crate) const VALIDATION_TARGET_1572_QUERY_134: &str = "#/$defs/targets/v1572";
pub(crate) const VALIDATION_TARGET_1573_QUERY_135: &str = "#/$defs/targets/v1573";
pub(crate) const VALIDATION_TARGET_1574_QUERY_136: &str = "#/$defs/targets/v1574";
pub(crate) const VALIDATION_TARGET_1575_QUERY_137: &str = "#/$defs/targets/v1575";
pub(crate) const VALIDATION_TARGET_1576_QUERY_138: &str = "#/$defs/targets/v1576";
pub(crate) const VALIDATION_TARGET_1577_QUERY_139: &str = "#/$defs/targets/v1577";
pub(crate) const VALIDATION_TARGET_1578_QUERY_140: &str = "#/$defs/targets/v1578";
pub(crate) const VALIDATION_TARGET_1579_QUERY_141: &str = "#/$defs/targets/v1579";
pub(crate) const VALIDATION_TARGET_1580_QUERY_142: &str = "#/$defs/targets/v1580";
pub(crate) const VALIDATION_TARGET_1581_QUERY_143: &str = "#/$defs/targets/v1581";
pub(crate) const VALIDATION_TARGET_1582_QUERY_144: &str = "#/$defs/targets/v1582";
pub(crate) const VALIDATION_TARGET_1583_QUERY_145: &str = "#/$defs/targets/v1583";
pub(crate) const VALIDATION_TARGET_1584_QUERY_146: &str = "#/$defs/targets/v1584";
pub(crate) const VALIDATION_TARGET_1585_QUERY_147: &str = "#/$defs/targets/v1585";
pub(crate) const VALIDATION_TARGET_1586_QUERY_148: &str = "#/$defs/targets/v1586";
pub(crate) const VALIDATION_TARGET_1587_QUERY_149: &str = "#/$defs/targets/v1587";
pub(crate) const VALIDATION_TARGET_1588_QUERY_150: &str = "#/$defs/targets/v1588";
pub(crate) const VALIDATION_TARGET_1589_QUERY_151: &str = "#/$defs/targets/v1589";
pub(crate) const VALIDATION_TARGET_1590_QUERY_152: &str = "#/$defs/targets/v1590";
pub(crate) const VALIDATION_TARGET_1591_QUERY_153: &str = "#/$defs/targets/v1591";
pub(crate) const VALIDATION_TARGET_1592_QUERY_154: &str = "#/$defs/targets/v1592";
pub(crate) const VALIDATION_TARGET_1593_QUERY_155: &str = "#/$defs/targets/v1593";
pub(crate) const VALIDATION_TARGET_1594_QUERY_156: &str = "#/$defs/targets/v1594";
pub(crate) const VALIDATION_TARGET_1595_QUERY_157: &str = "#/$defs/targets/v1595";
pub(crate) const VALIDATION_TARGET_1596_QUERY_158: &str = "#/$defs/targets/v1596";
pub(crate) const VALIDATION_TARGET_1597_QUERY_159: &str = "#/$defs/targets/v1597";
pub(crate) const VALIDATION_TARGET_1598_QUERY_160: &str = "#/$defs/targets/v1598";
pub(crate) const VALIDATION_TARGET_1599_QUERY_161: &str = "#/$defs/targets/v1599";
pub(crate) const VALIDATION_TARGET_1600_QUERY_162: &str = "#/$defs/targets/v1600";
pub(crate) const VALIDATION_TARGET_1601_QUERY_163: &str = "#/$defs/targets/v1601";
pub(crate) const VALIDATION_TARGET_1602_QUERY_164: &str = "#/$defs/targets/v1602";
pub(crate) const VALIDATION_TARGET_1603_QUERY_165: &str = "#/$defs/targets/v1603";
pub(crate) const VALIDATION_TARGET_1604_QUERY_166: &str = "#/$defs/targets/v1604";
pub(crate) const VALIDATION_TARGET_1605_QUERY_167: &str = "#/$defs/targets/v1605";
pub(crate) const VALIDATION_TARGET_1606_QUERY_168: &str = "#/$defs/targets/v1606";
pub(crate) const VALIDATION_TARGET_1607_QUERY_169: &str = "#/$defs/targets/v1607";
pub(crate) const VALIDATION_TARGET_1608_QUERY_170: &str = "#/$defs/targets/v1608";
pub(crate) const VALIDATION_TARGET_1609_QUERY_171: &str = "#/$defs/targets/v1609";
pub(crate) const VALIDATION_TARGET_1610_QUERY_172: &str = "#/$defs/targets/v1610";
pub(crate) const VALIDATION_TARGET_1611_QUERY_173: &str = "#/$defs/targets/v1611";
pub(crate) const VALIDATION_TARGET_1612_QUERY_174: &str = "#/$defs/targets/v1612";
pub(crate) const VALIDATION_TARGET_1613_QUERY_175: &str = "#/$defs/targets/v1613";
pub(crate) const VALIDATION_TARGET_1614_QUERY_176: &str = "#/$defs/targets/v1614";
pub(crate) const VALIDATION_TARGET_1615_QUERY_177: &str = "#/$defs/targets/v1615";
pub(crate) const VALIDATION_TARGET_1616_QUERY_178: &str = "#/$defs/targets/v1616";
pub(crate) const VALIDATION_TARGET_1617_QUERY_179: &str = "#/$defs/targets/v1617";
pub(crate) const VALIDATION_TARGET_1618_QUERY_180: &str = "#/$defs/targets/v1618";
pub(crate) const VALIDATION_TARGET_1619_QUERY_181: &str = "#/$defs/targets/v1619";
pub(crate) const VALIDATION_TARGET_1620_QUERY_182: &str = "#/$defs/targets/v1620";
pub(crate) const VALIDATION_TARGET_1621_QUERY_183: &str = "#/$defs/targets/v1621";
pub(crate) const VALIDATION_TARGET_1622_QUERY_184: &str = "#/$defs/targets/v1622";
pub(crate) const VALIDATION_TARGET_1623_QUERY_185: &str = "#/$defs/targets/v1623";
pub(crate) const VALIDATION_TARGET_1624_QUERY_186: &str = "#/$defs/targets/v1624";
pub(crate) const VALIDATION_TARGET_1625_QUERY_187: &str = "#/$defs/targets/v1625";
pub(crate) const VALIDATION_TARGET_1626_QUERY_188: &str = "#/$defs/targets/v1626";
pub(crate) const VALIDATION_TARGET_1627_QUERY_189: &str = "#/$defs/targets/v1627";
pub(crate) const VALIDATION_TARGET_1628_QUERY_190: &str = "#/$defs/targets/v1628";
pub(crate) const VALIDATION_TARGET_1629_QUERY_191: &str = "#/$defs/targets/v1629";
pub(crate) const VALIDATION_TARGET_1630_QUERY_192: &str = "#/$defs/targets/v1630";
pub(crate) const VALIDATION_TARGET_1631_QUERY_193: &str = "#/$defs/targets/v1631";
pub(crate) const VALIDATION_TARGET_1632_QUERY_194: &str = "#/$defs/targets/v1632";
pub(crate) const VALIDATION_TARGET_1633_QUERY_195: &str = "#/$defs/targets/v1633";
pub(crate) const VALIDATION_TARGET_1634_QUERY_196: &str = "#/$defs/targets/v1634";
pub(crate) const VALIDATION_TARGET_1635_QUERY_197: &str = "#/$defs/targets/v1635";
pub(crate) const VALIDATION_TARGET_1636_QUERY_198: &str = "#/$defs/targets/v1636";
pub(crate) const VALIDATION_TARGET_1637_QUERY_199: &str = "#/$defs/targets/v1637";
pub(crate) const VALIDATION_TARGET_1638_QUERY_200: &str = "#/$defs/targets/v1638";
pub(crate) const VALIDATION_TARGET_1639_QUERY_201: &str = "#/$defs/targets/v1639";
pub(crate) const VALIDATION_TARGET_1640_QUERY_202: &str = "#/$defs/targets/v1640";
pub(crate) const VALIDATION_TARGET_1641_QUERY_203: &str = "#/$defs/targets/v1641";
pub(crate) const VALIDATION_TARGET_1642_QUERY_204: &str = "#/$defs/targets/v1642";
pub(crate) const VALIDATION_TARGET_1643_BODY: &str = "#/$defs/targets/v1643";
pub(crate) const VALIDATION_TARGET_1644_PATH_0: &str = "#/$defs/targets/v1644";
pub(crate) const VALIDATION_TARGET_1645_QUERY_1: &str = "#/$defs/targets/v1645";
pub(crate) const VALIDATION_TARGET_1646_BODY: &str = "#/$defs/targets/v1646";
pub(crate) const VALIDATION_TARGET_1647_QUERY_0: &str = "#/$defs/targets/v1647";
pub(crate) const VALIDATION_TARGET_1648_BODY: &str = "#/$defs/targets/v1648";
pub(crate) const VALIDATION_TARGET_1649_PATH_0: &str = "#/$defs/targets/v1649";
pub(crate) const VALIDATION_TARGET_1650_QUERY_1: &str = "#/$defs/targets/v1650";
pub(crate) const VALIDATION_TARGET_1651_BODY: &str = "#/$defs/targets/v1651";
pub(crate) const VALIDATION_TARGET_1652_PATH_0: &str = "#/$defs/targets/v1652";
pub(crate) const VALIDATION_TARGET_1653_QUERY_1: &str = "#/$defs/targets/v1653";
pub(crate) const VALIDATION_TARGET_1654_BODY: &str = "#/$defs/targets/v1654";
pub(crate) const VALIDATION_TARGET_1655_PATH_0: &str = "#/$defs/targets/v1655";
pub(crate) const VALIDATION_TARGET_1656_QUERY_1: &str = "#/$defs/targets/v1656";
static VALIDATORS: ::std::sync::LazyLock<
    ::std::result::Result<::jsonschema::ValidatorMap, ()>,
> = ::std::sync::LazyLock::new(|| {
    let schema: ::serde_json::Value = ::serde_json::from_str(VALIDATION_SCHEMA)
        .map_err(|_| ())?;
    ::jsonschema::options()
        .with_draft(::jsonschema::Draft::Draft202012)
        .should_validate_formats(true)
        .with_pattern_options(::jsonschema::PatternOptions::regex())
        .build_map(&schema)
        .map_err(|_| ())
});
pub(crate) async fn decode_json_body<T>(
    request: ::axum::extract::Request,
    target: &str,
    expected_media_type: &str,
    required: bool,
    max_body_bytes: usize,
) -> ::std::result::Result<::std::option::Option<T>, RequestValidationRejection>
where
    T: ::serde::de::DeserializeOwned,
{
    let (parts, body) = request.into_parts();
    let content_type = parts
        .headers
        .get(::axum::http::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok());
    let is_json = content_type
        .is_some_and(|value| media_type_is(value, expected_media_type));
    if !is_json {
        if content_type.is_none() && !required {
            let bytes = read_body(body, max_body_bytes).await?;
            if bytes.is_empty() {
                return Ok(None);
            }
        }
        return Err(unsupported_media_type());
    }
    let bytes = read_body(body, max_body_bytes).await?;
    if bytes.is_empty() {
        return if required { Err(malformed_request()) } else { Ok(None) };
    }
    let instance: ::serde_json::Value = ::serde_json::from_slice(&bytes)
        .map_err(|_| malformed_request())?;
    validate(target, "/body", &instance)?;
    let typed = ::serde_json::from_value(instance)
        .map_err(|_| generated_contract_error())?;
    Ok(Some(typed))
}
pub(crate) fn decode_parameter<T>(
    raw: &str,
    target: &str,
    location: &str,
    string_wire: bool,
) -> ::std::result::Result<T, RequestValidationRejection>
where
    T: ::serde::de::DeserializeOwned + ::serde::Serialize,
{
    let typed = if string_wire {
        let instance = ::serde_json::Value::String(raw.to_string());
        validate(target, location, &instance)?;
        ::serde_json::from_value(instance).map_err(|_| generated_contract_error())?
    } else {
        ::serde_json::from_value(::serde_json::Value::String(raw.to_string()))
            .or_else(|_| ::serde_json::from_str(raw))
            .map_err(|_| malformed_parameter(location))?
    };
    validate_parameter(target, location, &typed)?;
    Ok(typed)
}
pub(crate) fn validate_string_parameter(
    target: &str,
    location: &str,
    raw: &str,
) -> ::std::result::Result<(), RequestValidationRejection> {
    validate(target, location, &::serde_json::Value::String(raw.to_string()))
}
pub(crate) fn validate_parameter<T>(
    target: &str,
    location: &str,
    value: &T,
) -> ::std::result::Result<(), RequestValidationRejection>
where
    T: ::serde::Serialize + ?Sized,
{
    let instance = ::serde_json::to_value(value)
        .map_err(|_| generated_contract_error())?;
    validate(target, location, &instance)
}
pub(crate) fn parse_cookies(
    headers: &::axum::http::HeaderMap,
) -> ::std::result::Result<
    ::std::collections::BTreeMap<String, String>,
    RequestValidationRejection,
> {
    let mut cookies = ::std::collections::BTreeMap::new();
    for header in headers.get_all(::axum::http::header::COOKIE).iter() {
        let line = header.to_str().map_err(|_| malformed_parameter("/cookie"))?;
        for field in line.split(';') {
            let field = field.trim();
            if field.is_empty() {
                continue;
            }
            let (name, value) = field
                .split_once('=')
                .ok_or_else(|| malformed_parameter("/cookie"))?;
            let name = name.trim();
            if name.is_empty()
                || cookies.insert(name.to_string(), value.to_string()).is_some()
            {
                return Err(malformed_parameter("/cookie"));
            }
        }
    }
    Ok(cookies)
}
async fn read_body(
    body: ::axum::body::Body,
    max_body_bytes: usize,
) -> ::std::result::Result<::axum::body::Bytes, RequestValidationRejection> {
    ::axum::body::to_bytes(body, max_body_bytes)
        .await
        .map_err(|error| {
            let source = ::std::error::Error::source(&error);
            if source
                .is_some_and(|source| {
                    source.is::<::http_body_util::LengthLimitError>()
                })
            {
                request_body_too_large()
            } else {
                malformed_request()
            }
        })
}
fn media_type_is(content_type: &str, expected: &str) -> bool {
    let Ok(content_type) = content_type.parse::<::mime::Mime>() else {
        return false;
    };
    let Ok(expected) = expected.parse::<::mime::Mime>() else {
        return false;
    };
    content_type.type_() == expected.type_()
        && content_type.subtype() == expected.subtype()
        && content_type.suffix() == expected.suffix()
        && expected
            .params()
            .all(|(name, value)| {
                content_type.get_param(name).is_some_and(|actual| actual == value)
            })
}
pub(crate) fn validate(
    target: &str,
    location: &str,
    instance: &::serde_json::Value,
) -> ::std::result::Result<(), RequestValidationRejection> {
    let validators = VALIDATORS.as_ref().map_err(|_| generated_contract_error())?;
    let Some(validator) = validators.get(target) else {
        return Err(generated_contract_error());
    };
    let mut errors: ::std::vec::Vec<InvalidParameter> = ::std::vec::Vec::new();
    for error in validator.iter_errors(instance) {
        let keyword = error.kind().keyword();
        let (code, message) = public_violation(keyword);
        let mut pointer = format!("{location}{}", error.instance_path());
        if let ::jsonschema::error::ValidationErrorKind::Required { property } = error
            .kind()
        {
            if let Some(property) = property.as_str() {
                pointer.push('/');
                pointer.push_str(&escape_pointer_token(property));
            }
        }
        let violation = InvalidParameter {
            code: code.to_string(),
            location: pointer,
            message: message.to_string(),
        };
        if !errors.contains(&violation) {
            errors.push(violation);
            if errors.len() == MAX_VALIDATION_ERRORS {
                break;
            }
        }
    }
    errors
        .sort_by(|left, right| {
            (&left.location, &left.code, &left.message)
                .cmp(&(&right.location, &right.code, &right.message))
        });
    if errors.is_empty() {
        return Ok(());
    }
    Err(
        RequestValidationRejection(ProblemDetails {
            r#type: "https://openapi-to-rust.dev/problems/validation".to_string(),
            title: "Request validation failed".to_string(),
            status: 422,
            code: "request_validation_failed".to_string(),
            errors,
        }),
    )
}
pub(crate) fn malformed_request() -> RequestValidationRejection {
    public_problem(
        400,
        "https://openapi-to-rust.dev/problems/malformed-request",
        "Malformed request",
        "malformed_request",
    )
}
pub(crate) fn malformed_parameter(location: &str) -> RequestValidationRejection {
    parameter_problem(
        400,
        "https://openapi-to-rust.dev/problems/malformed-parameter",
        "Malformed request parameter",
        "malformed_parameter",
        "malformed",
        location,
        "is malformed",
    )
}
pub(crate) fn missing_parameter(location: &str) -> RequestValidationRejection {
    parameter_problem(
        422,
        "https://openapi-to-rust.dev/problems/validation",
        "Request validation failed",
        "request_validation_failed",
        "required",
        location,
        "is required",
    )
}
fn schema_parameter_problem(
    location: &str,
    code: &str,
    message: &str,
) -> RequestValidationRejection {
    parameter_problem(
        422,
        "https://openapi-to-rust.dev/problems/validation",
        "Request validation failed",
        "request_validation_failed",
        code,
        location,
        message,
    )
}
fn parameter_problem(
    status: u16,
    problem_type: &str,
    title: &str,
    code: &str,
    error_code: &str,
    location: &str,
    message: &str,
) -> RequestValidationRejection {
    RequestValidationRejection(ProblemDetails {
        r#type: problem_type.to_string(),
        title: title.to_string(),
        status,
        code: code.to_string(),
        errors: vec![
            InvalidParameter { code : error_code.to_string(), location : location
            .to_string(), message : message.to_string(), }
        ],
    })
}
pub(crate) fn request_body_too_large() -> RequestValidationRejection {
    public_problem(
        413,
        "https://openapi-to-rust.dev/problems/request-body-too-large",
        "Request body too large",
        "request_body_too_large",
    )
}
pub(crate) fn unsupported_media_type() -> RequestValidationRejection {
    public_problem(
        415,
        "https://openapi-to-rust.dev/problems/unsupported-media-type",
        "Unsupported media type",
        "unsupported_media_type",
    )
}
pub(crate) fn generated_contract_error() -> RequestValidationRejection {
    public_problem(
        500,
        "https://openapi-to-rust.dev/problems/generated-contract-error",
        "Internal server error",
        "generated_contract_error",
    )
}
fn public_problem(
    status: u16,
    problem_type: &str,
    title: &str,
    code: &str,
) -> RequestValidationRejection {
    RequestValidationRejection(ProblemDetails {
        r#type: problem_type.to_string(),
        title: title.to_string(),
        status,
        code: code.to_string(),
        errors: Vec::new(),
    })
}
fn public_violation(keyword: &str) -> (&'static str, &'static str) {
    match keyword {
        "required" => ("required", "is required"),
        "type" => ("type", "has an invalid type"),
        "enum" => ("enum", "has an unsupported value"),
        "const" => ("const", "has an unsupported value"),
        "format" => ("format", "has an invalid format"),
        "pattern" => ("pattern", "does not match the required format"),
        "minLength" => ("min_length", "does not meet the length constraint"),
        "maxLength" => ("max_length", "does not meet the length constraint"),
        "minimum" => ("minimum", "is outside the allowed range"),
        "maximum" => ("maximum", "is outside the allowed range"),
        "exclusiveMinimum" => ("exclusive_minimum", "is outside the allowed range"),
        "exclusiveMaximum" => ("exclusive_maximum", "is outside the allowed range"),
        "multipleOf" => ("multiple_of", "is outside the allowed range"),
        "minItems" => ("min_items", "does not meet the item-count constraint"),
        "maxItems" => ("max_items", "does not meet the item-count constraint"),
        "uniqueItems" => ("unique_items", "contains duplicate items"),
        "minProperties" => {
            ("min_properties", "does not meet the property-count constraint")
        }
        "maxProperties" => {
            ("max_properties", "does not meet the property-count constraint")
        }
        "additionalProperties" => {
            ("additional_properties", "contains unsupported properties")
        }
        "unevaluatedProperties" => {
            ("unevaluated_properties", "contains unsupported properties")
        }
        "anyOf" => ("any_of", "does not match the required shape"),
        "oneOf" => ("one_of", "does not match the required shape"),
        "not" => ("not", "does not match the required shape"),
        _ => ("invalid_value", "is invalid"),
    }
}
fn escape_pointer_token(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}
