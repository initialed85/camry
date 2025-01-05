# \ObjectTrackerClaimVideoAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**PostObjectTrackerClaimVideos**](ObjectTrackerClaimVideoAPI.md#PostObjectTrackerClaimVideos) | **Post** /api/object-tracker-claim-video | 



## PostObjectTrackerClaimVideos

> ResponseWithGenericOfVideo PostObjectTrackerClaimVideos(ctx).VideoObjectTrackerClaimRequest(videoObjectTrackerClaimRequest).Execute()



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
	videoObjectTrackerClaimRequest := *openapiclient.NewVideoObjectTrackerClaimRequest() // VideoObjectTrackerClaimRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ObjectTrackerClaimVideoAPI.PostObjectTrackerClaimVideos(context.Background()).VideoObjectTrackerClaimRequest(videoObjectTrackerClaimRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ObjectTrackerClaimVideoAPI.PostObjectTrackerClaimVideos``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostObjectTrackerClaimVideos`: ResponseWithGenericOfVideo
	fmt.Fprintf(os.Stdout, "Response from `ObjectTrackerClaimVideoAPI.PostObjectTrackerClaimVideos`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostObjectTrackerClaimVideosRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **videoObjectTrackerClaimRequest** | [**VideoObjectTrackerClaimRequest**](VideoObjectTrackerClaimRequest.md) |  | 

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

