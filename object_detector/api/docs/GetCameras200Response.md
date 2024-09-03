# GetCameras200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | **int** |  | [optional] 
**error** | **List[str]** |  | [optional] 
**limit** | **int** |  | [optional] 
**objects** | [**List[Camera]**](Camera.md) |  | [optional] 
**offset** | **int** |  | [optional] 
**status** | **int** |  | 
**success** | **bool** |  | 
**total_count** | **int** |  | [optional] 

## Example

```python
from openapi_client.models.get_cameras200_response import GetCameras200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetCameras200Response from a JSON string
get_cameras200_response_instance = GetCameras200Response.from_json(json)
# print the JSON string representation of the object
print(GetCameras200Response.to_json())

# convert the object into a dict
get_cameras200_response_dict = get_cameras200_response_instance.to_dict()
# create an instance of GetCameras200Response from a dict
get_cameras200_response_from_dict = GetCameras200Response.from_dict(get_cameras200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


