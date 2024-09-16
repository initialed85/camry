# openapi_client.CustomApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**patch_claim_video_for_object_detector**](CustomApi.md#patch_claim_video_for_object_detector) | **PATCH** /api/custom/claim-video-for-object-detector | 


# **patch_claim_video_for_object_detector**
> Video patch_claim_video_for_object_detector(claim_request)



### Example


```python
import openapi_client
from openapi_client.models.claim_request import ClaimRequest
from openapi_client.models.video import Video
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.CustomApi(api_client)
    claim_request = openapi_client.ClaimRequest() # ClaimRequest | 

    try:
        api_response = api_instance.patch_claim_video_for_object_detector(claim_request)
        print("The response of CustomApi->patch_claim_video_for_object_detector:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling CustomApi->patch_claim_video_for_object_detector: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **claim_request** | [**ClaimRequest**](ClaimRequest.md)|  | 

### Return type

[**Video**](Video.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | PatchClaimVideoForObjectDetector success |  -  |
**0** | PatchClaimVideoForObjectDetector failure |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

