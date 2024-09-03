# GetCamerasDefaultResponse


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | **List[str]** |  | [optional] 
**status** | **int** |  | 
**success** | **bool** |  | 

## Example

```python
from openapi_client.models.get_cameras_default_response import GetCamerasDefaultResponse

# TODO update the JSON string below
json = "{}"
# create an instance of GetCamerasDefaultResponse from a JSON string
get_cameras_default_response_instance = GetCamerasDefaultResponse.from_json(json)
# print the JSON string representation of the object
print(GetCamerasDefaultResponse.to_json())

# convert the object into a dict
get_cameras_default_response_dict = get_cameras_default_response_instance.to_dict()
# create an instance of GetCamerasDefaultResponse from a dict
get_cameras_default_response_from_dict = GetCamerasDefaultResponse.from_dict(get_cameras_default_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


