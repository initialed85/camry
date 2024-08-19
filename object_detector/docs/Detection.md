# Detection


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bounding_box** | [**List[DetectionBoundingBoxInner]**](DetectionBoundingBoxInner.md) |  | [optional] 
**camera_id** | **str** |  | [optional] 
**camera_id_object** | [**Camera**](Camera.md) |  | [optional] 
**centroid** | [**DetectionBoundingBoxInner**](DetectionBoundingBoxInner.md) |  | [optional] 
**class_id** | **int** |  | [optional] 
**class_name** | **str** |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**id** | **str** |  | [optional] 
**score** | **float** |  | [optional] 
**seen_at** | **datetime** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 
**video_id** | **str** |  | [optional] 
**video_id_object** | [**Video**](Video.md) |  | [optional] 

## Example

```python
from openapi_client.models.detection import Detection

# TODO update the JSON string below
json = "{}"
# create an instance of Detection from a JSON string
detection_instance = Detection.from_json(json)
# print the JSON string representation of the object
print(Detection.to_json())

# convert the object into a dict
detection_dict = detection_instance.to_dict()
# create an instance of Detection from a dict
detection_from_dict = Detection.from_dict(detection_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


