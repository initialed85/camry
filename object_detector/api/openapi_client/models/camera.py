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
from pydantic import BaseModel, ConfigDict, StrictStr
from typing import Any, ClassVar, Dict, List, Optional
from typing import Optional, Set
from typing_extensions import Self

class Camera(BaseModel):
    """
    Camera
    """ # noqa: E501
    created_at: Optional[datetime] = None
    deleted_at: Optional[datetime] = None
    id: Optional[StrictStr] = None
    last_seen: Optional[datetime] = None
    name: Optional[StrictStr] = None
    referenced_by_detection_camera_id_objects: Optional[List[Detection]] = None
    referenced_by_video_camera_id_objects: Optional[List[Video]] = None
    segment_producer_claimed_until: Optional[datetime] = None
    stream_producer_claimed_until: Optional[datetime] = None
    stream_url: Optional[StrictStr] = None
    updated_at: Optional[datetime] = None
    __properties: ClassVar[List[str]] = ["created_at", "deleted_at", "id", "last_seen", "name", "referenced_by_detection_camera_id_objects", "referenced_by_video_camera_id_objects", "segment_producer_claimed_until", "stream_producer_claimed_until", "stream_url", "updated_at"]

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
        """Create an instance of Camera from a JSON string"""
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
        # override the default output from pydantic by calling `to_dict()` of each item in referenced_by_detection_camera_id_objects (list)
        _items = []
        if self.referenced_by_detection_camera_id_objects:
            for _item in self.referenced_by_detection_camera_id_objects:
                if _item:
                    _items.append(_item.to_dict())
            _dict['referenced_by_detection_camera_id_objects'] = _items
        # override the default output from pydantic by calling `to_dict()` of each item in referenced_by_video_camera_id_objects (list)
        _items = []
        if self.referenced_by_video_camera_id_objects:
            for _item in self.referenced_by_video_camera_id_objects:
                if _item:
                    _items.append(_item.to_dict())
            _dict['referenced_by_video_camera_id_objects'] = _items
        # set to None if referenced_by_detection_camera_id_objects (nullable) is None
        # and model_fields_set contains the field
        if self.referenced_by_detection_camera_id_objects is None and "referenced_by_detection_camera_id_objects" in self.model_fields_set:
            _dict['referenced_by_detection_camera_id_objects'] = None

        # set to None if referenced_by_video_camera_id_objects (nullable) is None
        # and model_fields_set contains the field
        if self.referenced_by_video_camera_id_objects is None and "referenced_by_video_camera_id_objects" in self.model_fields_set:
            _dict['referenced_by_video_camera_id_objects'] = None

        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of Camera from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "created_at": obj.get("created_at"),
            "deleted_at": obj.get("deleted_at"),
            "id": obj.get("id"),
            "last_seen": obj.get("last_seen"),
            "name": obj.get("name"),
            "referenced_by_detection_camera_id_objects": [Detection.from_dict(_item) for _item in obj["referenced_by_detection_camera_id_objects"]] if obj.get("referenced_by_detection_camera_id_objects") is not None else None,
            "referenced_by_video_camera_id_objects": [Video.from_dict(_item) for _item in obj["referenced_by_video_camera_id_objects"]] if obj.get("referenced_by_video_camera_id_objects") is not None else None,
            "segment_producer_claimed_until": obj.get("segment_producer_claimed_until"),
            "stream_producer_claimed_until": obj.get("stream_producer_claimed_until"),
            "stream_url": obj.get("stream_url"),
            "updated_at": obj.get("updated_at")
        })
        return _obj

from openapi_client.models.detection import Detection
from openapi_client.models.video import Video
# TODO: Rewrite to not use raise_errors
Camera.model_rebuild(raise_errors=False)

