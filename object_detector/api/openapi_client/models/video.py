# coding: utf-8

"""
    Djangolang

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

    The version of the OpenAPI document: 1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from datetime import datetime
from pydantic import BaseModel, ConfigDict, StrictFloat, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List, Optional, Union
from typing import Optional, Set
from typing_extensions import Self

class Video(BaseModel):
    """
    Video
    """ # noqa: E501
    camera_id: Optional[StrictStr] = None
    camera_id_object: Optional[Camera] = None
    created_at: Optional[datetime] = None
    deleted_at: Optional[datetime] = None
    detection_summary: Optional[Any] = None
    duration: Optional[StrictInt] = None
    ended_at: Optional[datetime] = None
    file_name: Optional[StrictStr] = None
    file_size: Optional[Union[StrictFloat, StrictInt]] = None
    id: Optional[StrictStr] = None
    object_detector_claimed_until: Optional[datetime] = None
    object_tracker_claimed_until: Optional[datetime] = None
    referenced_by_detection_video_id_objects: Optional[List[Detection]] = None
    started_at: Optional[datetime] = None
    status: Optional[StrictStr] = None
    thumbnail_name: Optional[StrictStr] = None
    updated_at: Optional[datetime] = None
    __properties: ClassVar[List[str]] = ["camera_id", "camera_id_object", "created_at", "deleted_at", "detection_summary", "duration", "ended_at", "file_name", "file_size", "id", "object_detector_claimed_until", "object_tracker_claimed_until", "referenced_by_detection_video_id_objects", "started_at", "status", "thumbnail_name", "updated_at"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of Video from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        # override the default output from pydantic by calling `to_dict()` of camera_id_object
        if self.camera_id_object:
            _dict['camera_id_object'] = self.camera_id_object.to_dict()
        # override the default output from pydantic by calling `to_dict()` of each item in referenced_by_detection_video_id_objects (list)
        _items = []
        if self.referenced_by_detection_video_id_objects:
            for _item in self.referenced_by_detection_video_id_objects:
                if _item:
                    _items.append(_item.to_dict())
            _dict['referenced_by_detection_video_id_objects'] = _items
        # set to None if deleted_at (nullable) is None
        # and model_fields_set contains the field
        if self.deleted_at is None and "deleted_at" in self.model_fields_set:
            _dict['deleted_at'] = None

        # set to None if detection_summary (nullable) is None
        # and model_fields_set contains the field
        if self.detection_summary is None and "detection_summary" in self.model_fields_set:
            _dict['detection_summary'] = None

        # set to None if duration (nullable) is None
        # and model_fields_set contains the field
        if self.duration is None and "duration" in self.model_fields_set:
            _dict['duration'] = None

        # set to None if ended_at (nullable) is None
        # and model_fields_set contains the field
        if self.ended_at is None and "ended_at" in self.model_fields_set:
            _dict['ended_at'] = None

        # set to None if file_size (nullable) is None
        # and model_fields_set contains the field
        if self.file_size is None and "file_size" in self.model_fields_set:
            _dict['file_size'] = None

        # set to None if referenced_by_detection_video_id_objects (nullable) is None
        # and model_fields_set contains the field
        if self.referenced_by_detection_video_id_objects is None and "referenced_by_detection_video_id_objects" in self.model_fields_set:
            _dict['referenced_by_detection_video_id_objects'] = None

        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of Video from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "camera_id": obj.get("camera_id"),
            "camera_id_object": Camera.from_dict(obj["camera_id_object"]) if obj.get("camera_id_object") is not None else None,
            "created_at": obj.get("created_at"),
            "deleted_at": obj.get("deleted_at"),
            "detection_summary": obj.get("detection_summary"),
            "duration": obj.get("duration"),
            "ended_at": obj.get("ended_at"),
            "file_name": obj.get("file_name"),
            "file_size": obj.get("file_size"),
            "id": obj.get("id"),
            "object_detector_claimed_until": obj.get("object_detector_claimed_until"),
            "object_tracker_claimed_until": obj.get("object_tracker_claimed_until"),
            "referenced_by_detection_video_id_objects": [Detection.from_dict(_item) for _item in obj["referenced_by_detection_video_id_objects"]] if obj.get("referenced_by_detection_video_id_objects") is not None else None,
            "started_at": obj.get("started_at"),
            "status": obj.get("status"),
            "thumbnail_name": obj.get("thumbnail_name"),
            "updated_at": obj.get("updated_at")
        })
        return _obj

from openapi_client.models.camera import Camera
from openapi_client.models.detection import Detection
# TODO: Rewrite to not use raise_errors
Video.model_rebuild(raise_errors=False)

