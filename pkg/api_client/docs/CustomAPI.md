# \CustomAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**PatchCustomClaimVideoForObjectDetector**](CustomAPI.md#PatchCustomClaimVideoForObjectDetector) | **Patch** /api/custom/claim-video-for-object-detector | 



## PatchCustomClaimVideoForObjectDetector

> Video PatchCustomClaimVideoForObjectDetector(ctx).ClaimRequest(claimRequest).Execute()



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
	claimRequest := *openapiclient.NewClaimRequest() // ClaimRequest | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.CustomAPI.PatchCustomClaimVideoForObjectDetector(context.Background()).ClaimRequest(claimRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `CustomAPI.PatchCustomClaimVideoForObjectDetector``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PatchCustomClaimVideoForObjectDetector`: Video
	fmt.Fprintf(os.Stdout, "Response from `CustomAPI.PatchCustomClaimVideoForObjectDetector`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPatchCustomClaimVideoForObjectDetectorRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **claimRequest** | [**ClaimRequest**](ClaimRequest.md) |  | 

### Return type

[**Video**](Video.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

