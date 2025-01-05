# \ObjectDetectorClaimVideoAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**PostObjectDetectorClaimVideos**](ObjectDetectorClaimVideoAPI.md#PostObjectDetectorClaimVideos) | **Post** /api/object-detector-claim-video | 



## PostObjectDetectorClaimVideos

> ResponseWithGenericOfVideo PostObjectDetectorClaimVideos(ctx).VideoObjectDetectorClaimRequest(videoObjectDetectorClaimRequest).Execute()



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
	videoObjectDetectorClaimRequest := *openapiclient.NewVideoObjectDetectorClaimRequest() // VideoObjectDetectorClaimRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ObjectDetectorClaimVideoAPI.PostObjectDetectorClaimVideos(context.Background()).VideoObjectDetectorClaimRequest(videoObjectDetectorClaimRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ObjectDetectorClaimVideoAPI.PostObjectDetectorClaimVideos``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostObjectDetectorClaimVideos`: ResponseWithGenericOfVideo
	fmt.Fprintf(os.Stdout, "Response from `ObjectDetectorClaimVideoAPI.PostObjectDetectorClaimVideos`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostObjectDetectorClaimVideosRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **videoObjectDetectorClaimRequest** | [**VideoObjectDetectorClaimRequest**](VideoObjectDetectorClaimRequest.md) |  | 

### Return type

[**ResponseWithGenericOfVideo**](ResponseWithGenericOfVideo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

