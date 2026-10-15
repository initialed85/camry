# \CameraAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DeleteCamera**](CameraAPI.md#DeleteCamera) | **Delete** /api/cameras/{primaryKey} | 
[**GetCamera**](CameraAPI.md#GetCamera) | **Get** /api/cameras/{primaryKey} | 
[**GetCameras**](CameraAPI.md#GetCameras) | **Get** /api/cameras | 
[**PatchCamera**](CameraAPI.md#PatchCamera) | **Patch** /api/cameras/{primaryKey} | 
[**PostCameras**](CameraAPI.md#PostCameras) | **Post** /api/cameras | 
[**PostCamerasSegmentProducerClaim**](CameraAPI.md#PostCamerasSegmentProducerClaim) | **Post** /api/cameras/{primaryKey}/segment-producer-claim | 
[**PostCamerasStreamProducerClaim**](CameraAPI.md#PostCamerasStreamProducerClaim) | **Post** /api/cameras/{primaryKey}/stream-producer-claim | 



## DeleteCamera

> DeleteCamera(ctx, primaryKey).Depth(depth).Execute()



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
	primaryKey := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | Path parameter primaryKey
	depth := int64(789) // int64 | Query parameter depth (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.CameraAPI.DeleteCamera(context.Background(), primaryKey).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `CameraAPI.DeleteCamera``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**primaryKey** | **string** | Path parameter primaryKey | 

### Other Parameters

Other parameters are passed through a pointer to a apiDeleteCameraRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **depth** | **int64** | Query parameter depth | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetCamera

> ResponseWithGenericOfCamera GetCamera(ctx, primaryKey).Depth(depth).Execute()



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
	primaryKey := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | Path parameter primaryKey
	depth := int64(789) // int64 | Query parameter depth (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.CameraAPI.GetCamera(context.Background(), primaryKey).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `CameraAPI.GetCamera``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetCamera`: ResponseWithGenericOfCamera
	fmt.Fprintf(os.Stdout, "Response from `CameraAPI.GetCamera`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**primaryKey** | **string** | Path parameter primaryKey | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetCameraRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **depth** | **int64** | Query parameter depth | 

### Return type

[**ResponseWithGenericOfCamera**](ResponseWithGenericOfCamera.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetCameras

> ResponseWithGenericOfCamera GetCameras(ctx).Limit(limit).Offset(offset).Depth(depth).ReferencedByDetectionCameraIDLoad(referencedByDetectionCameraIDLoad).ReferencedByTrackCameraIDLoad(referencedByTrackCameraIDLoad).ReferencedByVideoCameraIDLoad(referencedByVideoCameraIDLoad).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtAsc(createdAtAsc).CreatedAtDesc(createdAtDesc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtAsc(updatedAtAsc).UpdatedAtDesc(updatedAtDesc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtIsnull(deletedAtIsnull).DeletedAtIsnotnull(deletedAtIsnotnull).DeletedAtAsc(deletedAtAsc).DeletedAtDesc(deletedAtDesc).NameEq(nameEq).NameNe(nameNe).NameIn(nameIn).NameNotin(nameNotin).NameLike(nameLike).NameNotlike(nameNotlike).NameIlike(nameIlike).NameNotilike(nameNotilike).NameAsc(nameAsc).NameDesc(nameDesc).StreamUrlEq(streamUrlEq).StreamUrlNe(streamUrlNe).StreamUrlIn(streamUrlIn).StreamUrlNotin(streamUrlNotin).StreamUrlLike(streamUrlLike).StreamUrlNotlike(streamUrlNotlike).StreamUrlIlike(streamUrlIlike).StreamUrlNotilike(streamUrlNotilike).StreamUrlAsc(streamUrlAsc).StreamUrlDesc(streamUrlDesc).LastSeenEq(lastSeenEq).LastSeenNe(lastSeenNe).LastSeenGt(lastSeenGt).LastSeenGte(lastSeenGte).LastSeenLt(lastSeenLt).LastSeenLte(lastSeenLte).LastSeenIn(lastSeenIn).LastSeenNotin(lastSeenNotin).LastSeenAsc(lastSeenAsc).LastSeenDesc(lastSeenDesc).SegmentProducerClaimedUntilEq(segmentProducerClaimedUntilEq).SegmentProducerClaimedUntilNe(segmentProducerClaimedUntilNe).SegmentProducerClaimedUntilGt(segmentProducerClaimedUntilGt).SegmentProducerClaimedUntilGte(segmentProducerClaimedUntilGte).SegmentProducerClaimedUntilLt(segmentProducerClaimedUntilLt).SegmentProducerClaimedUntilLte(segmentProducerClaimedUntilLte).SegmentProducerClaimedUntilIn(segmentProducerClaimedUntilIn).SegmentProducerClaimedUntilNotin(segmentProducerClaimedUntilNotin).SegmentProducerClaimedUntilAsc(segmentProducerClaimedUntilAsc).SegmentProducerClaimedUntilDesc(segmentProducerClaimedUntilDesc).StreamProducerClaimedUntilEq(streamProducerClaimedUntilEq).StreamProducerClaimedUntilNe(streamProducerClaimedUntilNe).StreamProducerClaimedUntilGt(streamProducerClaimedUntilGt).StreamProducerClaimedUntilGte(streamProducerClaimedUntilGte).StreamProducerClaimedUntilLt(streamProducerClaimedUntilLt).StreamProducerClaimedUntilLte(streamProducerClaimedUntilLte).StreamProducerClaimedUntilIn(streamProducerClaimedUntilIn).StreamProducerClaimedUntilNotin(streamProducerClaimedUntilNotin).StreamProducerClaimedUntilAsc(streamProducerClaimedUntilAsc).StreamProducerClaimedUntilDesc(streamProducerClaimedUntilDesc).LowResStreamUrlEq(lowResStreamUrlEq).LowResStreamUrlNe(lowResStreamUrlNe).LowResStreamUrlIn(lowResStreamUrlIn).LowResStreamUrlNotin(lowResStreamUrlNotin).LowResStreamUrlLike(lowResStreamUrlLike).LowResStreamUrlNotlike(lowResStreamUrlNotlike).LowResStreamUrlIlike(lowResStreamUrlIlike).LowResStreamUrlNotilike(lowResStreamUrlNotilike).LowResStreamUrlIsnull(lowResStreamUrlIsnull).LowResStreamUrlIsnotnull(lowResStreamUrlIsnotnull).LowResStreamUrlAsc(lowResStreamUrlAsc).LowResStreamUrlDesc(lowResStreamUrlDesc).OverlayOffsetMsEq(overlayOffsetMsEq).OverlayOffsetMsNe(overlayOffsetMsNe).OverlayOffsetMsGt(overlayOffsetMsGt).OverlayOffsetMsGte(overlayOffsetMsGte).OverlayOffsetMsLt(overlayOffsetMsLt).OverlayOffsetMsLte(overlayOffsetMsLte).OverlayOffsetMsIn(overlayOffsetMsIn).OverlayOffsetMsNotin(overlayOffsetMsNotin).OverlayOffsetMsAsc(overlayOffsetMsAsc).OverlayOffsetMsDesc(overlayOffsetMsDesc).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
    "time"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	limit := int32(56) // int32 | SQL LIMIT operator (optional)
	offset := int32(56) // int32 | SQL OFFSET operator (optional)
	depth := int32(56) // int32 | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)
	referencedByDetectionCameraIDLoad := "referencedByDetectionCameraIDLoad_example" // string | load the given inbound relationship, value is ignored (presence of key is sufficient) (optional)
	referencedByTrackCameraIDLoad := "referencedByTrackCameraIDLoad_example" // string | load the given inbound relationship, value is ignored (presence of key is sufficient) (optional)
	referencedByVideoCameraIDLoad := "referencedByVideoCameraIDLoad_example" // string | load the given inbound relationship, value is ignored (presence of key is sufficient) (optional)
	createdAtEq := time.Now() // time.Time | SQL = comparison (optional)
	createdAtNe := time.Now() // time.Time | SQL != comparison (optional)
	createdAtGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	createdAtGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	createdAtLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	createdAtLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	createdAtIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	createdAtNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	createdAtAsc := "createdAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	createdAtDesc := "createdAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	updatedAtEq := time.Now() // time.Time | SQL = comparison (optional)
	updatedAtNe := time.Now() // time.Time | SQL != comparison (optional)
	updatedAtGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	updatedAtGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	updatedAtLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	updatedAtLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	updatedAtIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	updatedAtNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	updatedAtAsc := "updatedAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	updatedAtDesc := "updatedAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	deletedAtEq := time.Now() // time.Time | SQL = comparison (optional)
	deletedAtNe := time.Now() // time.Time | SQL != comparison (optional)
	deletedAtGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	deletedAtGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	deletedAtLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	deletedAtLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	deletedAtIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	deletedAtNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	deletedAtIsnull := "deletedAtIsnull_example" // string | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
	deletedAtIsnotnull := "deletedAtIsnotnull_example" // string | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
	deletedAtAsc := "deletedAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	deletedAtDesc := "deletedAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	nameEq := "nameEq_example" // string | SQL = comparison (optional)
	nameNe := "nameNe_example" // string | SQL != comparison (optional)
	nameIn := "nameIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	nameNotin := "nameNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	nameLike := "nameLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	nameNotlike := "nameNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	nameIlike := "nameIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	nameNotilike := "nameNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	nameAsc := "nameAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	nameDesc := "nameDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	streamUrlEq := "streamUrlEq_example" // string | SQL = comparison (optional)
	streamUrlNe := "streamUrlNe_example" // string | SQL != comparison (optional)
	streamUrlIn := "streamUrlIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	streamUrlNotin := "streamUrlNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	streamUrlLike := "streamUrlLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	streamUrlNotlike := "streamUrlNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	streamUrlIlike := "streamUrlIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	streamUrlNotilike := "streamUrlNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	streamUrlAsc := "streamUrlAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	streamUrlDesc := "streamUrlDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	lastSeenEq := time.Now() // time.Time | SQL = comparison (optional)
	lastSeenNe := time.Now() // time.Time | SQL != comparison (optional)
	lastSeenGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	lastSeenGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	lastSeenLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	lastSeenLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	lastSeenIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	lastSeenNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	lastSeenAsc := "lastSeenAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	lastSeenDesc := "lastSeenDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	segmentProducerClaimedUntilEq := time.Now() // time.Time | SQL = comparison (optional)
	segmentProducerClaimedUntilNe := time.Now() // time.Time | SQL != comparison (optional)
	segmentProducerClaimedUntilGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	segmentProducerClaimedUntilGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	segmentProducerClaimedUntilLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	segmentProducerClaimedUntilLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	segmentProducerClaimedUntilIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	segmentProducerClaimedUntilNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	segmentProducerClaimedUntilAsc := "segmentProducerClaimedUntilAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	segmentProducerClaimedUntilDesc := "segmentProducerClaimedUntilDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	streamProducerClaimedUntilEq := time.Now() // time.Time | SQL = comparison (optional)
	streamProducerClaimedUntilNe := time.Now() // time.Time | SQL != comparison (optional)
	streamProducerClaimedUntilGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	streamProducerClaimedUntilGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	streamProducerClaimedUntilLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	streamProducerClaimedUntilLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	streamProducerClaimedUntilIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	streamProducerClaimedUntilNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	streamProducerClaimedUntilAsc := "streamProducerClaimedUntilAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	streamProducerClaimedUntilDesc := "streamProducerClaimedUntilDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	lowResStreamUrlEq := "lowResStreamUrlEq_example" // string | SQL = comparison (optional)
	lowResStreamUrlNe := "lowResStreamUrlNe_example" // string | SQL != comparison (optional)
	lowResStreamUrlIn := "lowResStreamUrlIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	lowResStreamUrlNotin := "lowResStreamUrlNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	lowResStreamUrlLike := "lowResStreamUrlLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	lowResStreamUrlNotlike := "lowResStreamUrlNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	lowResStreamUrlIlike := "lowResStreamUrlIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	lowResStreamUrlNotilike := "lowResStreamUrlNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	lowResStreamUrlIsnull := "lowResStreamUrlIsnull_example" // string | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
	lowResStreamUrlIsnotnull := "lowResStreamUrlIsnotnull_example" // string | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
	lowResStreamUrlAsc := "lowResStreamUrlAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	lowResStreamUrlDesc := "lowResStreamUrlDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	overlayOffsetMsEq := int32(56) // int32 | SQL = comparison (optional)
	overlayOffsetMsNe := int32(56) // int32 | SQL != comparison (optional)
	overlayOffsetMsGt := int32(56) // int32 | SQL > comparison, may not work with all column types (optional)
	overlayOffsetMsGte := int32(56) // int32 | SQL >= comparison, may not work with all column types (optional)
	overlayOffsetMsLt := int32(56) // int32 | SQL < comparison, may not work with all column types (optional)
	overlayOffsetMsLte := int32(56) // int32 | SQL <= comparison, may not work with all column types (optional)
	overlayOffsetMsIn := int32(56) // int32 | SQL IN comparison, permits comma-separated values (optional)
	overlayOffsetMsNotin := int32(56) // int32 | SQL NOT IN comparison, permits comma-separated values (optional)
	overlayOffsetMsAsc := "overlayOffsetMsAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	overlayOffsetMsDesc := "overlayOffsetMsDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.CameraAPI.GetCameras(context.Background()).Limit(limit).Offset(offset).Depth(depth).ReferencedByDetectionCameraIDLoad(referencedByDetectionCameraIDLoad).ReferencedByTrackCameraIDLoad(referencedByTrackCameraIDLoad).ReferencedByVideoCameraIDLoad(referencedByVideoCameraIDLoad).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtAsc(createdAtAsc).CreatedAtDesc(createdAtDesc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtAsc(updatedAtAsc).UpdatedAtDesc(updatedAtDesc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtIsnull(deletedAtIsnull).DeletedAtIsnotnull(deletedAtIsnotnull).DeletedAtAsc(deletedAtAsc).DeletedAtDesc(deletedAtDesc).NameEq(nameEq).NameNe(nameNe).NameIn(nameIn).NameNotin(nameNotin).NameLike(nameLike).NameNotlike(nameNotlike).NameIlike(nameIlike).NameNotilike(nameNotilike).NameAsc(nameAsc).NameDesc(nameDesc).StreamUrlEq(streamUrlEq).StreamUrlNe(streamUrlNe).StreamUrlIn(streamUrlIn).StreamUrlNotin(streamUrlNotin).StreamUrlLike(streamUrlLike).StreamUrlNotlike(streamUrlNotlike).StreamUrlIlike(streamUrlIlike).StreamUrlNotilike(streamUrlNotilike).StreamUrlAsc(streamUrlAsc).StreamUrlDesc(streamUrlDesc).LastSeenEq(lastSeenEq).LastSeenNe(lastSeenNe).LastSeenGt(lastSeenGt).LastSeenGte(lastSeenGte).LastSeenLt(lastSeenLt).LastSeenLte(lastSeenLte).LastSeenIn(lastSeenIn).LastSeenNotin(lastSeenNotin).LastSeenAsc(lastSeenAsc).LastSeenDesc(lastSeenDesc).SegmentProducerClaimedUntilEq(segmentProducerClaimedUntilEq).SegmentProducerClaimedUntilNe(segmentProducerClaimedUntilNe).SegmentProducerClaimedUntilGt(segmentProducerClaimedUntilGt).SegmentProducerClaimedUntilGte(segmentProducerClaimedUntilGte).SegmentProducerClaimedUntilLt(segmentProducerClaimedUntilLt).SegmentProducerClaimedUntilLte(segmentProducerClaimedUntilLte).SegmentProducerClaimedUntilIn(segmentProducerClaimedUntilIn).SegmentProducerClaimedUntilNotin(segmentProducerClaimedUntilNotin).SegmentProducerClaimedUntilAsc(segmentProducerClaimedUntilAsc).SegmentProducerClaimedUntilDesc(segmentProducerClaimedUntilDesc).StreamProducerClaimedUntilEq(streamProducerClaimedUntilEq).StreamProducerClaimedUntilNe(streamProducerClaimedUntilNe).StreamProducerClaimedUntilGt(streamProducerClaimedUntilGt).StreamProducerClaimedUntilGte(streamProducerClaimedUntilGte).StreamProducerClaimedUntilLt(streamProducerClaimedUntilLt).StreamProducerClaimedUntilLte(streamProducerClaimedUntilLte).StreamProducerClaimedUntilIn(streamProducerClaimedUntilIn).StreamProducerClaimedUntilNotin(streamProducerClaimedUntilNotin).StreamProducerClaimedUntilAsc(streamProducerClaimedUntilAsc).StreamProducerClaimedUntilDesc(streamProducerClaimedUntilDesc).LowResStreamUrlEq(lowResStreamUrlEq).LowResStreamUrlNe(lowResStreamUrlNe).LowResStreamUrlIn(lowResStreamUrlIn).LowResStreamUrlNotin(lowResStreamUrlNotin).LowResStreamUrlLike(lowResStreamUrlLike).LowResStreamUrlNotlike(lowResStreamUrlNotlike).LowResStreamUrlIlike(lowResStreamUrlIlike).LowResStreamUrlNotilike(lowResStreamUrlNotilike).LowResStreamUrlIsnull(lowResStreamUrlIsnull).LowResStreamUrlIsnotnull(lowResStreamUrlIsnotnull).LowResStreamUrlAsc(lowResStreamUrlAsc).LowResStreamUrlDesc(lowResStreamUrlDesc).OverlayOffsetMsEq(overlayOffsetMsEq).OverlayOffsetMsNe(overlayOffsetMsNe).OverlayOffsetMsGt(overlayOffsetMsGt).OverlayOffsetMsGte(overlayOffsetMsGte).OverlayOffsetMsLt(overlayOffsetMsLt).OverlayOffsetMsLte(overlayOffsetMsLte).OverlayOffsetMsIn(overlayOffsetMsIn).OverlayOffsetMsNotin(overlayOffsetMsNotin).OverlayOffsetMsAsc(overlayOffsetMsAsc).OverlayOffsetMsDesc(overlayOffsetMsDesc).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `CameraAPI.GetCameras``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetCameras`: ResponseWithGenericOfCamera
	fmt.Fprintf(os.Stdout, "Response from `CameraAPI.GetCameras`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiGetCamerasRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int32** | SQL LIMIT operator | 
 **offset** | **int32** | SQL OFFSET operator | 
 **depth** | **int32** | Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | 
 **referencedByDetectionCameraIDLoad** | **string** | load the given inbound relationship, value is ignored (presence of key is sufficient) | 
 **referencedByTrackCameraIDLoad** | **string** | load the given inbound relationship, value is ignored (presence of key is sufficient) | 
 **referencedByVideoCameraIDLoad** | **string** | load the given inbound relationship, value is ignored (presence of key is sufficient) | 
 **createdAtEq** | **time.Time** | SQL &#x3D; comparison | 
 **createdAtNe** | **time.Time** | SQL !&#x3D; comparison | 
 **createdAtGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **createdAtGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **createdAtLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **createdAtLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **createdAtIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **createdAtNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **createdAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **createdAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **updatedAtEq** | **time.Time** | SQL &#x3D; comparison | 
 **updatedAtNe** | **time.Time** | SQL !&#x3D; comparison | 
 **updatedAtGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **updatedAtGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **updatedAtLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **updatedAtLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **updatedAtIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **updatedAtNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **updatedAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **updatedAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **deletedAtEq** | **time.Time** | SQL &#x3D; comparison | 
 **deletedAtNe** | **time.Time** | SQL !&#x3D; comparison | 
 **deletedAtGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **deletedAtGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **deletedAtLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **deletedAtLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **deletedAtIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **deletedAtNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **deletedAtIsnull** | **string** | SQL IS null comparison, value is ignored (presence of key is sufficient) | 
 **deletedAtIsnotnull** | **string** | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | 
 **deletedAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **deletedAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **nameEq** | **string** | SQL &#x3D; comparison | 
 **nameNe** | **string** | SQL !&#x3D; comparison | 
 **nameIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **nameNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **nameLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **nameNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **nameIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **nameNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **nameAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **nameDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **streamUrlEq** | **string** | SQL &#x3D; comparison | 
 **streamUrlNe** | **string** | SQL !&#x3D; comparison | 
 **streamUrlIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **streamUrlNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **streamUrlLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **streamUrlNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **streamUrlIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **streamUrlNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **streamUrlAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **streamUrlDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **lastSeenEq** | **time.Time** | SQL &#x3D; comparison | 
 **lastSeenNe** | **time.Time** | SQL !&#x3D; comparison | 
 **lastSeenGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **lastSeenGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **lastSeenLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **lastSeenLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **lastSeenIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **lastSeenNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **lastSeenAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **lastSeenDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **segmentProducerClaimedUntilEq** | **time.Time** | SQL &#x3D; comparison | 
 **segmentProducerClaimedUntilNe** | **time.Time** | SQL !&#x3D; comparison | 
 **segmentProducerClaimedUntilGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **segmentProducerClaimedUntilGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **segmentProducerClaimedUntilLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **segmentProducerClaimedUntilLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **segmentProducerClaimedUntilIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **segmentProducerClaimedUntilNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **segmentProducerClaimedUntilAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **segmentProducerClaimedUntilDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **streamProducerClaimedUntilEq** | **time.Time** | SQL &#x3D; comparison | 
 **streamProducerClaimedUntilNe** | **time.Time** | SQL !&#x3D; comparison | 
 **streamProducerClaimedUntilGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **streamProducerClaimedUntilGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **streamProducerClaimedUntilLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **streamProducerClaimedUntilLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **streamProducerClaimedUntilIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **streamProducerClaimedUntilNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **streamProducerClaimedUntilAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **streamProducerClaimedUntilDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **lowResStreamUrlEq** | **string** | SQL &#x3D; comparison | 
 **lowResStreamUrlNe** | **string** | SQL !&#x3D; comparison | 
 **lowResStreamUrlIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **lowResStreamUrlNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **lowResStreamUrlLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **lowResStreamUrlNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **lowResStreamUrlIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **lowResStreamUrlNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **lowResStreamUrlIsnull** | **string** | SQL IS null comparison, value is ignored (presence of key is sufficient) | 
 **lowResStreamUrlIsnotnull** | **string** | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | 
 **lowResStreamUrlAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **lowResStreamUrlDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **overlayOffsetMsEq** | **int32** | SQL &#x3D; comparison | 
 **overlayOffsetMsNe** | **int32** | SQL !&#x3D; comparison | 
 **overlayOffsetMsGt** | **int32** | SQL &gt; comparison, may not work with all column types | 
 **overlayOffsetMsGte** | **int32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **overlayOffsetMsLt** | **int32** | SQL &lt; comparison, may not work with all column types | 
 **overlayOffsetMsLte** | **int32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **overlayOffsetMsIn** | **int32** | SQL IN comparison, permits comma-separated values | 
 **overlayOffsetMsNotin** | **int32** | SQL NOT IN comparison, permits comma-separated values | 
 **overlayOffsetMsAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **overlayOffsetMsDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 

### Return type

[**ResponseWithGenericOfCamera**](ResponseWithGenericOfCamera.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PatchCamera

> ResponseWithGenericOfCamera PatchCamera(ctx, primaryKey).Camera(camera).Depth(depth).Execute()



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
	primaryKey := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | Path parameter primaryKey
	camera := *openapiclient.NewCamera() // Camera | 
	depth := int64(789) // int64 | Query parameter depth (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.CameraAPI.PatchCamera(context.Background(), primaryKey).Camera(camera).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `CameraAPI.PatchCamera``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PatchCamera`: ResponseWithGenericOfCamera
	fmt.Fprintf(os.Stdout, "Response from `CameraAPI.PatchCamera`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**primaryKey** | **string** | Path parameter primaryKey | 

### Other Parameters

Other parameters are passed through a pointer to a apiPatchCameraRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **camera** | [**Camera**](Camera.md) |  | 
 **depth** | **int64** | Query parameter depth | 

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


## PostCameras

> ResponseWithGenericOfCamera PostCameras(ctx).Camera(camera).Depth(depth).Execute()



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
	camera := []openapiclient.Camera{*openapiclient.NewCamera()} // []Camera | 
	depth := int64(789) // int64 | Query parameter depth (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.CameraAPI.PostCameras(context.Background()).Camera(camera).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `CameraAPI.PostCameras``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostCameras`: ResponseWithGenericOfCamera
	fmt.Fprintf(os.Stdout, "Response from `CameraAPI.PostCameras`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostCamerasRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **camera** | [**[]Camera**](Camera.md) |  | 
 **depth** | **int64** | Query parameter depth | 

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


## PostCamerasSegmentProducerClaim

> ResponseWithGenericOfCamera PostCamerasSegmentProducerClaim(ctx, primaryKey).CameraSegmentProducerClaimRequest(cameraSegmentProducerClaimRequest).Depth(depth).Execute()



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
	primaryKey := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | Path parameter primaryKey
	cameraSegmentProducerClaimRequest := *openapiclient.NewCameraSegmentProducerClaimRequest() // CameraSegmentProducerClaimRequest | 
	depth := int64(789) // int64 | Query parameter depth (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.CameraAPI.PostCamerasSegmentProducerClaim(context.Background(), primaryKey).CameraSegmentProducerClaimRequest(cameraSegmentProducerClaimRequest).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `CameraAPI.PostCamerasSegmentProducerClaim``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostCamerasSegmentProducerClaim`: ResponseWithGenericOfCamera
	fmt.Fprintf(os.Stdout, "Response from `CameraAPI.PostCamerasSegmentProducerClaim`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**primaryKey** | **string** | Path parameter primaryKey | 

### Other Parameters

Other parameters are passed through a pointer to a apiPostCamerasSegmentProducerClaimRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **cameraSegmentProducerClaimRequest** | [**CameraSegmentProducerClaimRequest**](CameraSegmentProducerClaimRequest.md) |  | 
 **depth** | **int64** | Query parameter depth | 

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


## PostCamerasStreamProducerClaim

> ResponseWithGenericOfCamera PostCamerasStreamProducerClaim(ctx, primaryKey).CameraStreamProducerClaimRequest(cameraStreamProducerClaimRequest).Depth(depth).Execute()



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
	primaryKey := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | Path parameter primaryKey
	cameraStreamProducerClaimRequest := *openapiclient.NewCameraStreamProducerClaimRequest() // CameraStreamProducerClaimRequest | 
	depth := int64(789) // int64 | Query parameter depth (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.CameraAPI.PostCamerasStreamProducerClaim(context.Background(), primaryKey).CameraStreamProducerClaimRequest(cameraStreamProducerClaimRequest).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `CameraAPI.PostCamerasStreamProducerClaim``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostCamerasStreamProducerClaim`: ResponseWithGenericOfCamera
	fmt.Fprintf(os.Stdout, "Response from `CameraAPI.PostCamerasStreamProducerClaim`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**primaryKey** | **string** | Path parameter primaryKey | 

### Other Parameters

Other parameters are passed through a pointer to a apiPostCamerasStreamProducerClaimRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **cameraStreamProducerClaimRequest** | [**CameraStreamProducerClaimRequest**](CameraStreamProducerClaimRequest.md) |  | 
 **depth** | **int64** | Query parameter depth | 

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

