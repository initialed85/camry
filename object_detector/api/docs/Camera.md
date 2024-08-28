# Camera


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**id** | **str** |  | [optional] 
**last_seen** | **datetime** |  | [optional] 
**name** | **str** |  | [optional] 
**referenced_by_detection_camera_id_objects** | [**List[Detection]**](Detection.md) |  | [optional] 
**referenced_by_video_camera_id_objects** | [**List[Video]**](Video.md) |  | [optional] 
**segment_producer_claimed_until** | **datetime** |  | [optional] 
**stream_producer_claimed_until** | **datetime** |  | [optional] 
**stream_url** | **str** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from openapi_client.models.camera import Camera

# TODO update the JSON string below
json = "{}"
# create an instance of Camera from a JSON string
camera_instance = Camera.from_json(json)
# print the JSON string representation of the object
print(Camera.to_json())

# convert the object into a dict
camera_dict = camera_instance.to_dict()
# create an instance of Camera from a dict
camera_from_dict = Camera.from_dict(camera_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


