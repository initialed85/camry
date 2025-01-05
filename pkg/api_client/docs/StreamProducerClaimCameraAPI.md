# \StreamProducerClaimCameraAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**PostStreamProducerClaimCameras**](StreamProducerClaimCameraAPI.md#PostStreamProducerClaimCameras) | **Post** /api/stream-producer-claim-camera | 



## PostStreamProducerClaimCameras

> ResponseWithGenericOfCamera PostStreamProducerClaimCameras(ctx).CameraStreamProducerClaimRequest(cameraStreamProducerClaimRequest).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	cameraStreamProducerClaimRequest := *openapiclient.NewCameraStreamProducerClaimRequest() // CameraStreamProducerClaimRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.StreamProducerClaimCameraAPI.PostStreamProducerClaimCameras(context.Background()).CameraStreamProducerClaimRequest(cameraStreamProducerClaimRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `StreamProducerClaimCameraAPI.PostStreamProducerClaimCameras``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostStreamProducerClaimCameras`: ResponseWithGenericOfCamera
	fmt.Fprintf(os.Stdout, "Response from `StreamProducerClaimCameraAPI.PostStreamProducerClaimCameras`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostStreamProducerClaimCamerasRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cameraStreamProducerClaimRequest** | [**CameraStreamProducerClaimRequest**](CameraStreamProducerClaimRequest.md) |  | 

### Return type

[**ResponseWithGenericOfCamera**](ResponseWithGenericOfCamera.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

