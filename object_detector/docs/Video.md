# Video


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**camera_id** | **str** |  | [optional] 
**camera_id_object** | [**Camera**](Camera.md) |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**duration** | **int** |  | [optional] 
**ended_at** | **datetime** |  | [optional] 
**file_name** | **str** |  | [optional] 
**file_size** | **float** |  | [optional] 
**id** | **str** |  | [optional] 
**referenced_by_detection_video_id_objects** | [**List[Detection]**](Detection.md) |  | [optional] 
**started_at** | **datetime** |  | [optional] 
**status** | **str** |  | [optional] 
**thumbnail_name** | **str** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from openapi_client.models.video import Video

# TODO update the JSON string below
json = "{}"
# create an instance of Video from a JSON string
video_instance = Video.from_json(json)
# print the JSON string representation of the object
print(Video.to_json())

# convert the object into a dict
video_dict = video_instance.to_dict()
# create an instance of Video from a dict
video_from_dict = Video.from_dict(video_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


