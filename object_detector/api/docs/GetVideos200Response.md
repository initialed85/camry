# GetVideos200Response


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | **str** |  | [optional] 
**objects** | [**List[Video]**](Video.md) |  | [optional] 
**status** | **int** |  | 
**success** | **bool** |  | 

## Example

```python
from openapi_client.models.get_videos200_response import GetVideos200Response

# TODO update the JSON string below
json = "{}"
# create an instance of GetVideos200Response from a JSON string
get_videos200_response_instance = GetVideos200Response.from_json(json)
# print the JSON string representation of the object
print(GetVideos200Response.to_json())

# convert the object into a dict
get_videos200_response_dict = get_videos200_response_instance.to_dict()
# create an instance of GetVideos200Response from a dict
get_videos200_response_from_dict = GetVideos200Response.from_dict(get_videos200_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


