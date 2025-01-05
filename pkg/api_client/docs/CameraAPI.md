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

> ResponseWithGenericOfCamera GetCameras(ctx).Limit(limit).Offset(offset).Depth(depth).ReferencedByDetectionLoad(referencedByDetectionLoad).ReferencedByVideoLoad(referencedByVideoLoad).IdEq(idEq).IdNe(idNe).IdGt(idGt).IdGte(idGte).IdLt(idLt).IdLte(idLte).IdIn(idIn).IdNotin(idNotin).IdContains(idContains).IdNotcontains(idNotcontains).IdLike(idLike).IdNotlike(idNotlike).IdIlike(idIlike).IdNotilike(idNotilike).IdDesc(idDesc).IdAsc(idAsc).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtContains(createdAtContains).CreatedAtNotcontains(createdAtNotcontains).CreatedAtLike(createdAtLike).CreatedAtNotlike(createdAtNotlike).CreatedAtIlike(createdAtIlike).CreatedAtNotilike(createdAtNotilike).CreatedAtDesc(createdAtDesc).CreatedAtAsc(createdAtAsc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtContains(updatedAtContains).UpdatedAtNotcontains(updatedAtNotcontains).UpdatedAtLike(updatedAtLike).UpdatedAtNotlike(updatedAtNotlike).UpdatedAtIlike(updatedAtIlike).UpdatedAtNotilike(updatedAtNotilike).UpdatedAtDesc(updatedAtDesc).UpdatedAtAsc(updatedAtAsc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtContains(deletedAtContains).DeletedAtNotcontains(deletedAtNotcontains).DeletedAtLike(deletedAtLike).DeletedAtNotlike(deletedAtNotlike).DeletedAtIlike(deletedAtIlike).DeletedAtNotilike(deletedAtNotilike).DeletedAtDesc(deletedAtDesc).DeletedAtAsc(deletedAtAsc).NameEq(nameEq).NameNe(nameNe).NameGt(nameGt).NameGte(nameGte).NameLt(nameLt).NameLte(nameLte).NameIn(nameIn).NameNotin(nameNotin).NameContains(nameContains).NameNotcontains(nameNotcontains).NameLike(nameLike).NameNotlike(nameNotlike).NameIlike(nameIlike).NameNotilike(nameNotilike).NameDesc(nameDesc).NameAsc(nameAsc).StreamUrlEq(streamUrlEq).StreamUrlNe(streamUrlNe).StreamUrlGt(streamUrlGt).StreamUrlGte(streamUrlGte).StreamUrlLt(streamUrlLt).StreamUrlLte(streamUrlLte).StreamUrlIn(streamUrlIn).StreamUrlNotin(streamUrlNotin).StreamUrlContains(streamUrlContains).StreamUrlNotcontains(streamUrlNotcontains).StreamUrlLike(streamUrlLike).StreamUrlNotlike(streamUrlNotlike).StreamUrlIlike(streamUrlIlike).StreamUrlNotilike(streamUrlNotilike).StreamUrlDesc(streamUrlDesc).StreamUrlAsc(streamUrlAsc).LastSeenEq(lastSeenEq).LastSeenNe(lastSeenNe).LastSeenGt(lastSeenGt).LastSeenGte(lastSeenGte).LastSeenLt(lastSeenLt).LastSeenLte(lastSeenLte).LastSeenIn(lastSeenIn).LastSeenNotin(lastSeenNotin).LastSeenContains(lastSeenContains).LastSeenNotcontains(lastSeenNotcontains).LastSeenLike(lastSeenLike).LastSeenNotlike(lastSeenNotlike).LastSeenIlike(lastSeenIlike).LastSeenNotilike(lastSeenNotilike).LastSeenDesc(lastSeenDesc).LastSeenAsc(lastSeenAsc).SegmentProducerClaimedUntilEq(segmentProducerClaimedUntilEq).SegmentProducerClaimedUntilNe(segmentProducerClaimedUntilNe).SegmentProducerClaimedUntilGt(segmentProducerClaimedUntilGt).SegmentProducerClaimedUntilGte(segmentProducerClaimedUntilGte).SegmentProducerClaimedUntilLt(segmentProducerClaimedUntilLt).SegmentProducerClaimedUntilLte(segmentProducerClaimedUntilLte).SegmentProducerClaimedUntilIn(segmentProducerClaimedUntilIn).SegmentProducerClaimedUntilNotin(segmentProducerClaimedUntilNotin).SegmentProducerClaimedUntilContains(segmentProducerClaimedUntilContains).SegmentProducerClaimedUntilNotcontains(segmentProducerClaimedUntilNotcontains).SegmentProducerClaimedUntilLike(segmentProducerClaimedUntilLike).SegmentProducerClaimedUntilNotlike(segmentProducerClaimedUntilNotlike).SegmentProducerClaimedUntilIlike(segmentProducerClaimedUntilIlike).SegmentProducerClaimedUntilNotilike(segmentProducerClaimedUntilNotilike).SegmentProducerClaimedUntilDesc(segmentProducerClaimedUntilDesc).SegmentProducerClaimedUntilAsc(segmentProducerClaimedUntilAsc).StreamProducerClaimedUntilEq(streamProducerClaimedUntilEq).StreamProducerClaimedUntilNe(streamProducerClaimedUntilNe).StreamProducerClaimedUntilGt(streamProducerClaimedUntilGt).StreamProducerClaimedUntilGte(streamProducerClaimedUntilGte).StreamProducerClaimedUntilLt(streamProducerClaimedUntilLt).StreamProducerClaimedUntilLte(streamProducerClaimedUntilLte).StreamProducerClaimedUntilIn(streamProducerClaimedUntilIn).StreamProducerClaimedUntilNotin(streamProducerClaimedUntilNotin).StreamProducerClaimedUntilContains(streamProducerClaimedUntilContains).StreamProducerClaimedUntilNotcontains(streamProducerClaimedUntilNotcontains).StreamProducerClaimedUntilLike(streamProducerClaimedUntilLike).StreamProducerClaimedUntilNotlike(streamProducerClaimedUntilNotlike).StreamProducerClaimedUntilIlike(streamProducerClaimedUntilIlike).StreamProducerClaimedUntilNotilike(streamProducerClaimedUntilNotilike).StreamProducerClaimedUntilDesc(streamProducerClaimedUntilDesc).StreamProducerClaimedUntilAsc(streamProducerClaimedUntilAsc).ReferencedByDetectionCameraIdObjectsContains(referencedByDetectionCameraIdObjectsContains).ReferencedByDetectionCameraIdObjectsNotcontains(referencedByDetectionCameraIdObjectsNotcontains).ReferencedByDetectionCameraIdObjectsDesc(referencedByDetectionCameraIdObjectsDesc).ReferencedByDetectionCameraIdObjectsAsc(referencedByDetectionCameraIdObjectsAsc).ReferencedByVideoCameraIdObjectsContains(referencedByVideoCameraIdObjectsContains).ReferencedByVideoCameraIdObjectsNotcontains(referencedByVideoCameraIdObjectsNotcontains).ReferencedByVideoCameraIdObjectsDesc(referencedByVideoCameraIdObjectsDesc).ReferencedByVideoCameraIdObjectsAsc(referencedByVideoCameraIdObjectsAsc).Execute()



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
	referencedByDetectionLoad := "referencedByDetectionLoad_example" // string | load the given indirectly related objects, value is ignored (presence of key is sufficient) (optional)
	referencedByVideoLoad := "referencedByVideoLoad_example" // string | load the given indirectly related objects, value is ignored (presence of key is sufficient) (optional)
	idEq := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL = comparison (optional)
	idNe := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL != comparison (optional)
	idGt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL > comparison, may not work with all column types (optional)
	idGte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL >= comparison, may not work with all column types (optional)
	idLt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL < comparison, may not work with all column types (optional)
	idLte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL <= comparison, may not work with all column types (optional)
	idIn := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL IN comparison, permits comma-separated values (optional)
	idNotin := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	idContains := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL @> comparison (optional)
	idNotcontains := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT @> comparison (optional)
	idLike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	idNotlike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	idIlike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	idNotilike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	idDesc := "idDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	idAsc := "idAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	createdAtEq := time.Now() // time.Time | SQL = comparison (optional)
	createdAtNe := time.Now() // time.Time | SQL != comparison (optional)
	createdAtGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	createdAtGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	createdAtLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	createdAtLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	createdAtIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	createdAtNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	createdAtContains := time.Now() // time.Time | SQL @> comparison (optional)
	createdAtNotcontains := time.Now() // time.Time | SQL NOT @> comparison (optional)
	createdAtLike := time.Now() // time.Time | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	createdAtNotlike := time.Now() // time.Time | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	createdAtIlike := time.Now() // time.Time | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	createdAtNotilike := time.Now() // time.Time | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	createdAtDesc := "createdAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	createdAtAsc := "createdAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	updatedAtEq := time.Now() // time.Time | SQL = comparison (optional)
	updatedAtNe := time.Now() // time.Time | SQL != comparison (optional)
	updatedAtGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	updatedAtGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	updatedAtLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	updatedAtLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	updatedAtIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	updatedAtNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	updatedAtContains := time.Now() // time.Time | SQL @> comparison (optional)
	updatedAtNotcontains := time.Now() // time.Time | SQL NOT @> comparison (optional)
	updatedAtLike := time.Now() // time.Time | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	updatedAtNotlike := time.Now() // time.Time | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	updatedAtIlike := time.Now() // time.Time | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	updatedAtNotilike := time.Now() // time.Time | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	updatedAtDesc := "updatedAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	updatedAtAsc := "updatedAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	deletedAtEq := time.Now() // time.Time | SQL = comparison (optional)
	deletedAtNe := time.Now() // time.Time | SQL != comparison (optional)
	deletedAtGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	deletedAtGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	deletedAtLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	deletedAtLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	deletedAtIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	deletedAtNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	deletedAtContains := time.Now() // time.Time | SQL @> comparison (optional)
	deletedAtNotcontains := time.Now() // time.Time | SQL NOT @> comparison (optional)
	deletedAtLike := time.Now() // time.Time | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	deletedAtNotlike := time.Now() // time.Time | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	deletedAtIlike := time.Now() // time.Time | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	deletedAtNotilike := time.Now() // time.Time | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	deletedAtDesc := "deletedAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	deletedAtAsc := "deletedAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	nameEq := "nameEq_example" // string | SQL = comparison (optional)
	nameNe := "nameNe_example" // string | SQL != comparison (optional)
	nameGt := "nameGt_example" // string | SQL > comparison, may not work with all column types (optional)
	nameGte := "nameGte_example" // string | SQL >= comparison, may not work with all column types (optional)
	nameLt := "nameLt_example" // string | SQL < comparison, may not work with all column types (optional)
	nameLte := "nameLte_example" // string | SQL <= comparison, may not work with all column types (optional)
	nameIn := "nameIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	nameNotin := "nameNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	nameContains := "nameContains_example" // string | SQL @> comparison (optional)
	nameNotcontains := "nameNotcontains_example" // string | SQL NOT @> comparison (optional)
	nameLike := "nameLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	nameNotlike := "nameNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	nameIlike := "nameIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	nameNotilike := "nameNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	nameDesc := "nameDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	nameAsc := "nameAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	streamUrlEq := "streamUrlEq_example" // string | SQL = comparison (optional)
	streamUrlNe := "streamUrlNe_example" // string | SQL != comparison (optional)
	streamUrlGt := "streamUrlGt_example" // string | SQL > comparison, may not work with all column types (optional)
	streamUrlGte := "streamUrlGte_example" // string | SQL >= comparison, may not work with all column types (optional)
	streamUrlLt := "streamUrlLt_example" // string | SQL < comparison, may not work with all column types (optional)
	streamUrlLte := "streamUrlLte_example" // string | SQL <= comparison, may not work with all column types (optional)
	streamUrlIn := "streamUrlIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	streamUrlNotin := "streamUrlNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	streamUrlContains := "streamUrlContains_example" // string | SQL @> comparison (optional)
	streamUrlNotcontains := "streamUrlNotcontains_example" // string | SQL NOT @> comparison (optional)
	streamUrlLike := "streamUrlLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	streamUrlNotlike := "streamUrlNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	streamUrlIlike := "streamUrlIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	streamUrlNotilike := "streamUrlNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	streamUrlDesc := "streamUrlDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	streamUrlAsc := "streamUrlAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	lastSeenEq := time.Now() // time.Time | SQL = comparison (optional)
	lastSeenNe := time.Now() // time.Time | SQL != comparison (optional)
	lastSeenGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	lastSeenGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	lastSeenLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	lastSeenLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	lastSeenIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	lastSeenNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	lastSeenContains := time.Now() // time.Time | SQL @> comparison (optional)
	lastSeenNotcontains := time.Now() // time.Time | SQL NOT @> comparison (optional)
	lastSeenLike := time.Now() // time.Time | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	lastSeenNotlike := time.Now() // time.Time | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	lastSeenIlike := time.Now() // time.Time | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	lastSeenNotilike := time.Now() // time.Time | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	lastSeenDesc := "lastSeenDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	lastSeenAsc := "lastSeenAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	segmentProducerClaimedUntilEq := time.Now() // time.Time | SQL = comparison (optional)
	segmentProducerClaimedUntilNe := time.Now() // time.Time | SQL != comparison (optional)
	segmentProducerClaimedUntilGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	segmentProducerClaimedUntilGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	segmentProducerClaimedUntilLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	segmentProducerClaimedUntilLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	segmentProducerClaimedUntilIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	segmentProducerClaimedUntilNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	segmentProducerClaimedUntilContains := time.Now() // time.Time | SQL @> comparison (optional)
	segmentProducerClaimedUntilNotcontains := time.Now() // time.Time | SQL NOT @> comparison (optional)
	segmentProducerClaimedUntilLike := time.Now() // time.Time | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	segmentProducerClaimedUntilNotlike := time.Now() // time.Time | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	segmentProducerClaimedUntilIlike := time.Now() // time.Time | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	segmentProducerClaimedUntilNotilike := time.Now() // time.Time | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	segmentProducerClaimedUntilDesc := "segmentProducerClaimedUntilDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	segmentProducerClaimedUntilAsc := "segmentProducerClaimedUntilAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	streamProducerClaimedUntilEq := time.Now() // time.Time | SQL = comparison (optional)
	streamProducerClaimedUntilNe := time.Now() // time.Time | SQL != comparison (optional)
	streamProducerClaimedUntilGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	streamProducerClaimedUntilGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	streamProducerClaimedUntilLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	streamProducerClaimedUntilLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	streamProducerClaimedUntilIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	streamProducerClaimedUntilNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	streamProducerClaimedUntilContains := time.Now() // time.Time | SQL @> comparison (optional)
	streamProducerClaimedUntilNotcontains := time.Now() // time.Time | SQL NOT @> comparison (optional)
	streamProducerClaimedUntilLike := time.Now() // time.Time | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	streamProducerClaimedUntilNotlike := time.Now() // time.Time | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	streamProducerClaimedUntilIlike := time.Now() // time.Time | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	streamProducerClaimedUntilNotilike := time.Now() // time.Time | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	streamProducerClaimedUntilDesc := "streamProducerClaimedUntilDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	streamProducerClaimedUntilAsc := "streamProducerClaimedUntilAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	referencedByDetectionCameraIdObjectsContains := TODO // interface{} | SQL @> comparison (optional)
	referencedByDetectionCameraIdObjectsNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	referencedByDetectionCameraIdObjectsDesc := "referencedByDetectionCameraIdObjectsDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	referencedByDetectionCameraIdObjectsAsc := "referencedByDetectionCameraIdObjectsAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	referencedByVideoCameraIdObjectsContains := TODO // interface{} | SQL @> comparison (optional)
	referencedByVideoCameraIdObjectsNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	referencedByVideoCameraIdObjectsDesc := "referencedByVideoCameraIdObjectsDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	referencedByVideoCameraIdObjectsAsc := "referencedByVideoCameraIdObjectsAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.CameraAPI.GetCameras(context.Background()).Limit(limit).Offset(offset).Depth(depth).ReferencedByDetectionLoad(referencedByDetectionLoad).ReferencedByVideoLoad(referencedByVideoLoad).IdEq(idEq).IdNe(idNe).IdGt(idGt).IdGte(idGte).IdLt(idLt).IdLte(idLte).IdIn(idIn).IdNotin(idNotin).IdContains(idContains).IdNotcontains(idNotcontains).IdLike(idLike).IdNotlike(idNotlike).IdIlike(idIlike).IdNotilike(idNotilike).IdDesc(idDesc).IdAsc(idAsc).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtContains(createdAtContains).CreatedAtNotcontains(createdAtNotcontains).CreatedAtLike(createdAtLike).CreatedAtNotlike(createdAtNotlike).CreatedAtIlike(createdAtIlike).CreatedAtNotilike(createdAtNotilike).CreatedAtDesc(createdAtDesc).CreatedAtAsc(createdAtAsc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtContains(updatedAtContains).UpdatedAtNotcontains(updatedAtNotcontains).UpdatedAtLike(updatedAtLike).UpdatedAtNotlike(updatedAtNotlike).UpdatedAtIlike(updatedAtIlike).UpdatedAtNotilike(updatedAtNotilike).UpdatedAtDesc(updatedAtDesc).UpdatedAtAsc(updatedAtAsc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtContains(deletedAtContains).DeletedAtNotcontains(deletedAtNotcontains).DeletedAtLike(deletedAtLike).DeletedAtNotlike(deletedAtNotlike).DeletedAtIlike(deletedAtIlike).DeletedAtNotilike(deletedAtNotilike).DeletedAtDesc(deletedAtDesc).DeletedAtAsc(deletedAtAsc).NameEq(nameEq).NameNe(nameNe).NameGt(nameGt).NameGte(nameGte).NameLt(nameLt).NameLte(nameLte).NameIn(nameIn).NameNotin(nameNotin).NameContains(nameContains).NameNotcontains(nameNotcontains).NameLike(nameLike).NameNotlike(nameNotlike).NameIlike(nameIlike).NameNotilike(nameNotilike).NameDesc(nameDesc).NameAsc(nameAsc).StreamUrlEq(streamUrlEq).StreamUrlNe(streamUrlNe).StreamUrlGt(streamUrlGt).StreamUrlGte(streamUrlGte).StreamUrlLt(streamUrlLt).StreamUrlLte(streamUrlLte).StreamUrlIn(streamUrlIn).StreamUrlNotin(streamUrlNotin).StreamUrlContains(streamUrlContains).StreamUrlNotcontains(streamUrlNotcontains).StreamUrlLike(streamUrlLike).StreamUrlNotlike(streamUrlNotlike).StreamUrlIlike(streamUrlIlike).StreamUrlNotilike(streamUrlNotilike).StreamUrlDesc(streamUrlDesc).StreamUrlAsc(streamUrlAsc).LastSeenEq(lastSeenEq).LastSeenNe(lastSeenNe).LastSeenGt(lastSeenGt).LastSeenGte(lastSeenGte).LastSeenLt(lastSeenLt).LastSeenLte(lastSeenLte).LastSeenIn(lastSeenIn).LastSeenNotin(lastSeenNotin).LastSeenContains(lastSeenContains).LastSeenNotcontains(lastSeenNotcontains).LastSeenLike(lastSeenLike).LastSeenNotlike(lastSeenNotlike).LastSeenIlike(lastSeenIlike).LastSeenNotilike(lastSeenNotilike).LastSeenDesc(lastSeenDesc).LastSeenAsc(lastSeenAsc).SegmentProducerClaimedUntilEq(segmentProducerClaimedUntilEq).SegmentProducerClaimedUntilNe(segmentProducerClaimedUntilNe).SegmentProducerClaimedUntilGt(segmentProducerClaimedUntilGt).SegmentProducerClaimedUntilGte(segmentProducerClaimedUntilGte).SegmentProducerClaimedUntilLt(segmentProducerClaimedUntilLt).SegmentProducerClaimedUntilLte(segmentProducerClaimedUntilLte).SegmentProducerClaimedUntilIn(segmentProducerClaimedUntilIn).SegmentProducerClaimedUntilNotin(segmentProducerClaimedUntilNotin).SegmentProducerClaimedUntilContains(segmentProducerClaimedUntilContains).SegmentProducerClaimedUntilNotcontains(segmentProducerClaimedUntilNotcontains).SegmentProducerClaimedUntilLike(segmentProducerClaimedUntilLike).SegmentProducerClaimedUntilNotlike(segmentProducerClaimedUntilNotlike).SegmentProducerClaimedUntilIlike(segmentProducerClaimedUntilIlike).SegmentProducerClaimedUntilNotilike(segmentProducerClaimedUntilNotilike).SegmentProducerClaimedUntilDesc(segmentProducerClaimedUntilDesc).SegmentProducerClaimedUntilAsc(segmentProducerClaimedUntilAsc).StreamProducerClaimedUntilEq(streamProducerClaimedUntilEq).StreamProducerClaimedUntilNe(streamProducerClaimedUntilNe).StreamProducerClaimedUntilGt(streamProducerClaimedUntilGt).StreamProducerClaimedUntilGte(streamProducerClaimedUntilGte).StreamProducerClaimedUntilLt(streamProducerClaimedUntilLt).StreamProducerClaimedUntilLte(streamProducerClaimedUntilLte).StreamProducerClaimedUntilIn(streamProducerClaimedUntilIn).StreamProducerClaimedUntilNotin(streamProducerClaimedUntilNotin).StreamProducerClaimedUntilContains(streamProducerClaimedUntilContains).StreamProducerClaimedUntilNotcontains(streamProducerClaimedUntilNotcontains).StreamProducerClaimedUntilLike(streamProducerClaimedUntilLike).StreamProducerClaimedUntilNotlike(streamProducerClaimedUntilNotlike).StreamProducerClaimedUntilIlike(streamProducerClaimedUntilIlike).StreamProducerClaimedUntilNotilike(streamProducerClaimedUntilNotilike).StreamProducerClaimedUntilDesc(streamProducerClaimedUntilDesc).StreamProducerClaimedUntilAsc(streamProducerClaimedUntilAsc).ReferencedByDetectionCameraIdObjectsContains(referencedByDetectionCameraIdObjectsContains).ReferencedByDetectionCameraIdObjectsNotcontains(referencedByDetectionCameraIdObjectsNotcontains).ReferencedByDetectionCameraIdObjectsDesc(referencedByDetectionCameraIdObjectsDesc).ReferencedByDetectionCameraIdObjectsAsc(referencedByDetectionCameraIdObjectsAsc).ReferencedByVideoCameraIdObjectsContains(referencedByVideoCameraIdObjectsContains).ReferencedByVideoCameraIdObjectsNotcontains(referencedByVideoCameraIdObjectsNotcontains).ReferencedByVideoCameraIdObjectsDesc(referencedByVideoCameraIdObjectsDesc).ReferencedByVideoCameraIdObjectsAsc(referencedByVideoCameraIdObjectsAsc).Execute()
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
 **referencedByDetectionLoad** | **string** | load the given indirectly related objects, value is ignored (presence of key is sufficient) | 
 **referencedByVideoLoad** | **string** | load the given indirectly related objects, value is ignored (presence of key is sufficient) | 
 **idEq** | **string** | SQL &#x3D; comparison | 
 **idNe** | **string** | SQL !&#x3D; comparison | 
 **idGt** | **string** | SQL &gt; comparison, may not work with all column types | 
 **idGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **idLt** | **string** | SQL &lt; comparison, may not work with all column types | 
 **idLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **idIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **idNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **idContains** | **string** | SQL @&gt; comparison | 
 **idNotcontains** | **string** | SQL NOT @&gt; comparison | 
 **idLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **idNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **idIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **idNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **idDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **idAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **createdAtEq** | **time.Time** | SQL &#x3D; comparison | 
 **createdAtNe** | **time.Time** | SQL !&#x3D; comparison | 
 **createdAtGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **createdAtGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **createdAtLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **createdAtLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **createdAtIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **createdAtNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **createdAtContains** | **time.Time** | SQL @&gt; comparison | 
 **createdAtNotcontains** | **time.Time** | SQL NOT @&gt; comparison | 
 **createdAtLike** | **time.Time** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **createdAtNotlike** | **time.Time** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **createdAtIlike** | **time.Time** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **createdAtNotilike** | **time.Time** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **createdAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **createdAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **updatedAtEq** | **time.Time** | SQL &#x3D; comparison | 
 **updatedAtNe** | **time.Time** | SQL !&#x3D; comparison | 
 **updatedAtGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **updatedAtGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **updatedAtLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **updatedAtLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **updatedAtIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **updatedAtNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **updatedAtContains** | **time.Time** | SQL @&gt; comparison | 
 **updatedAtNotcontains** | **time.Time** | SQL NOT @&gt; comparison | 
 **updatedAtLike** | **time.Time** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **updatedAtNotlike** | **time.Time** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **updatedAtIlike** | **time.Time** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **updatedAtNotilike** | **time.Time** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **updatedAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **updatedAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **deletedAtEq** | **time.Time** | SQL &#x3D; comparison | 
 **deletedAtNe** | **time.Time** | SQL !&#x3D; comparison | 
 **deletedAtGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **deletedAtGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **deletedAtLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **deletedAtLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **deletedAtIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **deletedAtNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **deletedAtContains** | **time.Time** | SQL @&gt; comparison | 
 **deletedAtNotcontains** | **time.Time** | SQL NOT @&gt; comparison | 
 **deletedAtLike** | **time.Time** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **deletedAtNotlike** | **time.Time** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **deletedAtIlike** | **time.Time** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **deletedAtNotilike** | **time.Time** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **deletedAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **deletedAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **nameEq** | **string** | SQL &#x3D; comparison | 
 **nameNe** | **string** | SQL !&#x3D; comparison | 
 **nameGt** | **string** | SQL &gt; comparison, may not work with all column types | 
 **nameGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **nameLt** | **string** | SQL &lt; comparison, may not work with all column types | 
 **nameLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **nameIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **nameNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **nameContains** | **string** | SQL @&gt; comparison | 
 **nameNotcontains** | **string** | SQL NOT @&gt; comparison | 
 **nameLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **nameNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **nameIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **nameNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **nameDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **nameAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **streamUrlEq** | **string** | SQL &#x3D; comparison | 
 **streamUrlNe** | **string** | SQL !&#x3D; comparison | 
 **streamUrlGt** | **string** | SQL &gt; comparison, may not work with all column types | 
 **streamUrlGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **streamUrlLt** | **string** | SQL &lt; comparison, may not work with all column types | 
 **streamUrlLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **streamUrlIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **streamUrlNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **streamUrlContains** | **string** | SQL @&gt; comparison | 
 **streamUrlNotcontains** | **string** | SQL NOT @&gt; comparison | 
 **streamUrlLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **streamUrlNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **streamUrlIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **streamUrlNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **streamUrlDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **streamUrlAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **lastSeenEq** | **time.Time** | SQL &#x3D; comparison | 
 **lastSeenNe** | **time.Time** | SQL !&#x3D; comparison | 
 **lastSeenGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **lastSeenGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **lastSeenLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **lastSeenLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **lastSeenIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **lastSeenNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **lastSeenContains** | **time.Time** | SQL @&gt; comparison | 
 **lastSeenNotcontains** | **time.Time** | SQL NOT @&gt; comparison | 
 **lastSeenLike** | **time.Time** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **lastSeenNotlike** | **time.Time** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **lastSeenIlike** | **time.Time** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **lastSeenNotilike** | **time.Time** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **lastSeenDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **lastSeenAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **segmentProducerClaimedUntilEq** | **time.Time** | SQL &#x3D; comparison | 
 **segmentProducerClaimedUntilNe** | **time.Time** | SQL !&#x3D; comparison | 
 **segmentProducerClaimedUntilGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **segmentProducerClaimedUntilGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **segmentProducerClaimedUntilLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **segmentProducerClaimedUntilLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **segmentProducerClaimedUntilIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **segmentProducerClaimedUntilNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **segmentProducerClaimedUntilContains** | **time.Time** | SQL @&gt; comparison | 
 **segmentProducerClaimedUntilNotcontains** | **time.Time** | SQL NOT @&gt; comparison | 
 **segmentProducerClaimedUntilLike** | **time.Time** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **segmentProducerClaimedUntilNotlike** | **time.Time** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **segmentProducerClaimedUntilIlike** | **time.Time** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **segmentProducerClaimedUntilNotilike** | **time.Time** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **segmentProducerClaimedUntilDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **segmentProducerClaimedUntilAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **streamProducerClaimedUntilEq** | **time.Time** | SQL &#x3D; comparison | 
 **streamProducerClaimedUntilNe** | **time.Time** | SQL !&#x3D; comparison | 
 **streamProducerClaimedUntilGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **streamProducerClaimedUntilGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **streamProducerClaimedUntilLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **streamProducerClaimedUntilLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **streamProducerClaimedUntilIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **streamProducerClaimedUntilNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **streamProducerClaimedUntilContains** | **time.Time** | SQL @&gt; comparison | 
 **streamProducerClaimedUntilNotcontains** | **time.Time** | SQL NOT @&gt; comparison | 
 **streamProducerClaimedUntilLike** | **time.Time** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **streamProducerClaimedUntilNotlike** | **time.Time** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **streamProducerClaimedUntilIlike** | **time.Time** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **streamProducerClaimedUntilNotilike** | **time.Time** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **streamProducerClaimedUntilDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **streamProducerClaimedUntilAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **referencedByDetectionCameraIdObjectsContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison | 
 **referencedByDetectionCameraIdObjectsNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison | 
 **referencedByDetectionCameraIdObjectsDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **referencedByDetectionCameraIdObjectsAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **referencedByVideoCameraIdObjectsContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison | 
 **referencedByVideoCameraIdObjectsNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison | 
 **referencedByVideoCameraIdObjectsDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **referencedByVideoCameraIdObjectsAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 

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

