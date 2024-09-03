# PatchCustom0200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**camera_id** | **str** |  | [optional] 
**camera_id_object** | [**Camera**](Camera.md) |  | [optional] 
**created_at** | **datetime** |  | [optional] 
**deleted_at** | **datetime** |  | [optional] 
**detection_summary** | **object** |  | [optional] 
**duration** | **int** |  | [optional] 
**ended_at** | **datetime** |  | [optional] 
**file_name** | **str** |  | [optional] 
**file_size** | **float** |  | [optional] 
**id** | **str** |  | [optional] 
**object_detector_claimed_until** | **datetime** |  | [optional] 
**object_tracker_claimed_until** | **datetime** |  | [optional] 
**referenced_by_detection_video_id_objects** | [**List[Detection]**](Detection.md) |  | [optional] 
**started_at** | **datetime** |  | [optional] 
**status** | **str** |  | [optional] 
**thumbnail_name** | **str** |  | [optional] 
**updated_at** | **datetime** |  | [optional] 

## Example

```python
from openapi_client.models.patch_custom0200_response import PatchCustom0200Response

# TODO update the JSON string below
json = "{}"
# create an instance of PatchCustom0200Response from a JSON string
patch_custom0200_response_instance = PatchCustom0200Response.from_json(json)
# print the JSON string representation of the object
print(PatchCustom0200Response.to_json())

# convert the object into a dict
patch_custom0200_response_dict = patch_custom0200_response_instance.to_dict()
# create an instance of PatchCustom0200Response from a dict
patch_custom0200_response_from_dict = PatchCustom0200Response.from_dict(patch_custom0200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


