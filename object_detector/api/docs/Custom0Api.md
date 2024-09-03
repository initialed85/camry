# openapi_client.Custom0Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**patch_custom0**](Custom0Api.md#patch_custom0) | **PATCH** /api/custom/claim-video-for-object-detector | 


# **patch_custom0**
> PatchCustom0200Response patch_custom0(patch_custom0_request)



### Example


```python
import openapi_client
from openapi_client.models.patch_custom0200_response import PatchCustom0200Response
from openapi_client.models.patch_custom0_request import PatchCustom0Request
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
    api_instance = openapi_client.Custom0Api(api_client)
    patch_custom0_request = openapi_client.PatchCustom0Request() # PatchCustom0Request | 

    try:
        api_response = api_instance.patch_custom0(patch_custom0_request)
        print("The response of Custom0Api->patch_custom0:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling Custom0Api->patch_custom0: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **patch_custom0_request** | [**PatchCustom0Request**](PatchCustom0Request.md)|  | 

### Return type

[**PatchCustom0200Response**](PatchCustom0200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Custom0 success |  -  |
**0** | Custom0 failure |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

