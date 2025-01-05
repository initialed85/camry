# \SegmentProducerClaimCameraAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**PostSegmentProducerClaimCameras**](SegmentProducerClaimCameraAPI.md#PostSegmentProducerClaimCameras) | **Post** /api/segment-producer-claim-camera | 



## PostSegmentProducerClaimCameras

> ResponseWithGenericOfCamera PostSegmentProducerClaimCameras(ctx).CameraSegmentProducerClaimRequest(cameraSegmentProducerClaimRequest).Execute()



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
	cameraSegmentProducerClaimRequest := *openapiclient.NewCameraSegmentProducerClaimRequest() // CameraSegmentProducerClaimRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.SegmentProducerClaimCameraAPI.PostSegmentProducerClaimCameras(context.Background()).CameraSegmentProducerClaimRequest(cameraSegmentProducerClaimRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `SegmentProducerClaimCameraAPI.PostSegmentProducerClaimCameras``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostSegmentProducerClaimCameras`: ResponseWithGenericOfCamera
	fmt.Fprintf(os.Stdout, "Response from `SegmentProducerClaimCameraAPI.PostSegmentProducerClaimCameras`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostSegmentProducerClaimCamerasRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cameraSegmentProducerClaimRequest** | [**CameraSegmentProducerClaimRequest**](CameraSegmentProducerClaimRequest.md) |  | 

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

