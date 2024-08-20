# coding: utf-8

# flake8: noqa

"""
    Djangolang

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


__version__ = "1.0.0"

# import apis into sdk package
from openapi_client.api.camera_api import CameraApi
from openapi_client.api.detection_api import DetectionApi
from openapi_client.api.video_api import VideoApi

# import ApiClient
from openapi_client.api_response import ApiResponse
from openapi_client.api_client import ApiClient
from openapi_client.configuration import Configuration
from openapi_client.exceptions import OpenApiException
from openapi_client.exceptions import ApiTypeError
from openapi_client.exceptions import ApiValueError
from openapi_client.exceptions import ApiKeyError
from openapi_client.exceptions import ApiAttributeError
from openapi_client.exceptions import ApiException

# import models into sdk package
from openapi_client.models.camera import Camera
from openapi_client.models.detection import Detection
from openapi_client.models.detection_bounding_box_inner import DetectionBoundingBoxInner
from openapi_client.models.get_cameras200_response import GetCameras200Response
from openapi_client.models.get_cameras_default_response import GetCamerasDefaultResponse
from openapi_client.models.get_detections200_response import GetDetections200Response
from openapi_client.models.get_videos200_response import GetVideos200Response
from openapi_client.models.vec2 import Vec2
from openapi_client.models.video import Video