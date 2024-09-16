# ClaimRequest


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**claim_duration_seconds** | **float** |  | 

## Example

```python
from openapi_client.models.claim_request import ClaimRequest

# TODO update the JSON string below
json = "{}"
# create an instance of ClaimRequest from a JSON string
claim_request_instance = ClaimRequest.from_json(json)
# print the JSON string representation of the object
print(ClaimRequest.to_json())

# convert the object into a dict
claim_request_dict = claim_request_instance.to_dict()
# create an instance of ClaimRequest from a dict
claim_request_from_dict = ClaimRequest.from_dict(claim_request_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


