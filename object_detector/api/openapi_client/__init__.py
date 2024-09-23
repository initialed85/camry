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
from openapi_client.api.custom_api import CustomApi
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
from openapi_client.models.array_of_vec2_inner import ArrayOfVec2Inner
from openapi_client.models.camera import Camera
from openapi_client.models.claim_request import ClaimRequest
from openapi_client.models.detection import Detection
from openapi_client.models.get_cameras_default_response import GetCamerasDefaultResponse
from openapi_client.models.response_with_generic_of_camera import ResponseWithGenericOfCamera
from openapi_client.models.response_with_generic_of_detection import ResponseWithGenericOfDetection
from openapi_client.models.response_with_generic_of_video import ResponseWithGenericOfVideo
from openapi_client.models.vec2 import Vec2
from openapi_client.models.video import Video
