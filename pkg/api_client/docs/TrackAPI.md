# \TrackAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DeleteTrack**](TrackAPI.md#DeleteTrack) | **Delete** /api/tracks/{primaryKey} | 
[**GetTrack**](TrackAPI.md#GetTrack) | **Get** /api/tracks/{primaryKey} | 
[**GetTracks**](TrackAPI.md#GetTracks) | **Get** /api/tracks | 
[**PatchTrack**](TrackAPI.md#PatchTrack) | **Patch** /api/tracks/{primaryKey} | 
[**PostTracks**](TrackAPI.md#PostTracks) | **Post** /api/tracks | 



## DeleteTrack

> DeleteTrack(ctx, primaryKey).Depth(depth).Execute()



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
	r, err := apiClient.TrackAPI.DeleteTrack(context.Background(), primaryKey).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `TrackAPI.DeleteTrack``: %v\n", err)
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

Other parameters are passed through a pointer to a apiDeleteTrackRequest struct via the builder pattern


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


## GetTrack

> ResponseWithGenericOfTrack GetTrack(ctx, primaryKey).Depth(depth).Execute()



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
	resp, r, err := apiClient.TrackAPI.GetTrack(context.Background(), primaryKey).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `TrackAPI.GetTrack``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetTrack`: ResponseWithGenericOfTrack
	fmt.Fprintf(os.Stdout, "Response from `TrackAPI.GetTrack`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**primaryKey** | **string** | Path parameter primaryKey | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetTrackRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **depth** | **int64** | Query parameter depth | 

### Return type

[**ResponseWithGenericOfTrack**](ResponseWithGenericOfTrack.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetTracks

> ResponseWithGenericOfTrack GetTracks(ctx).Limit(limit).Offset(offset).Depth(depth).CameraLoad(cameraLoad).VideoLoad(videoLoad).ReferencedByDetectionTrackIDLoad(referencedByDetectionTrackIDLoad).KeyEq(keyEq).KeyNe(keyNe).KeyGt(keyGt).KeyGte(keyGte).KeyLt(keyLt).KeyLte(keyLte).KeyIn(keyIn).KeyNotin(keyNotin).KeyAsc(keyAsc).KeyDesc(keyDesc).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtAsc(createdAtAsc).CreatedAtDesc(createdAtDesc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtAsc(updatedAtAsc).UpdatedAtDesc(updatedAtDesc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtIsnull(deletedAtIsnull).DeletedAtIsnotnull(deletedAtIsnotnull).DeletedAtAsc(deletedAtAsc).DeletedAtDesc(deletedAtDesc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdAsc(cameraIdAsc).CameraIdDesc(cameraIdDesc).VideoIdEq(videoIdEq).VideoIdNe(videoIdNe).VideoIdGt(videoIdGt).VideoIdGte(videoIdGte).VideoIdLt(videoIdLt).VideoIdLte(videoIdLte).VideoIdIn(videoIdIn).VideoIdNotin(videoIdNotin).VideoIdAsc(videoIdAsc).VideoIdDesc(videoIdDesc).ClassIdEq(classIdEq).ClassIdNe(classIdNe).ClassIdGt(classIdGt).ClassIdGte(classIdGte).ClassIdLt(classIdLt).ClassIdLte(classIdLte).ClassIdIn(classIdIn).ClassIdNotin(classIdNotin).ClassIdAsc(classIdAsc).ClassIdDesc(classIdDesc).ClassNameEq(classNameEq).ClassNameNe(classNameNe).ClassNameIn(classNameIn).ClassNameNotin(classNameNotin).ClassNameLike(classNameLike).ClassNameNotlike(classNameNotlike).ClassNameIlike(classNameIlike).ClassNameNotilike(classNameNotilike).ClassNameAsc(classNameAsc).ClassNameDesc(classNameDesc).StartedAtEq(startedAtEq).StartedAtNe(startedAtNe).StartedAtGt(startedAtGt).StartedAtGte(startedAtGte).StartedAtLt(startedAtLt).StartedAtLte(startedAtLte).StartedAtIn(startedAtIn).StartedAtNotin(startedAtNotin).StartedAtAsc(startedAtAsc).StartedAtDesc(startedAtDesc).EndedAtEq(endedAtEq).EndedAtNe(endedAtNe).EndedAtGt(endedAtGt).EndedAtGte(endedAtGte).EndedAtLt(endedAtLt).EndedAtLte(endedAtLte).EndedAtIn(endedAtIn).EndedAtNotin(endedAtNotin).EndedAtAsc(endedAtAsc).EndedAtDesc(endedAtDesc).FrameCountEq(frameCountEq).FrameCountNe(frameCountNe).FrameCountGt(frameCountGt).FrameCountGte(frameCountGte).FrameCountLt(frameCountLt).FrameCountLte(frameCountLte).FrameCountIn(frameCountIn).FrameCountNotin(frameCountNotin).FrameCountAsc(frameCountAsc).FrameCountDesc(frameCountDesc).MeanScoreEq(meanScoreEq).MeanScoreNe(meanScoreNe).MeanScoreGt(meanScoreGt).MeanScoreGte(meanScoreGte).MeanScoreLt(meanScoreLt).MeanScoreLte(meanScoreLte).MeanScoreIn(meanScoreIn).MeanScoreNotin(meanScoreNotin).MeanScoreAsc(meanScoreAsc).MeanScoreDesc(meanScoreDesc).MaxScoreEq(maxScoreEq).MaxScoreNe(maxScoreNe).MaxScoreGt(maxScoreGt).MaxScoreGte(maxScoreGte).MaxScoreLt(maxScoreLt).MaxScoreLte(maxScoreLte).MaxScoreIn(maxScoreIn).MaxScoreNotin(maxScoreNotin).MaxScoreAsc(maxScoreAsc).MaxScoreDesc(maxScoreDesc).TrajectoryContains(trajectoryContains).TrajectoryNotcontains(trajectoryNotcontains).TrajectoryAsc(trajectoryAsc).TrajectoryDesc(trajectoryDesc).IsStaticEq(isStaticEq).IsStaticNe(isStaticNe).IsStaticIsfalse(isStaticIsfalse).IsStaticIstrue(isStaticIstrue).IsStaticAsc(isStaticAsc).IsStaticDesc(isStaticDesc).StatusEq(statusEq).StatusNe(statusNe).StatusIn(statusIn).StatusNotin(statusNotin).StatusLike(statusLike).StatusNotlike(statusNotlike).StatusIlike(statusIlike).StatusNotilike(statusNotilike).StatusAsc(statusAsc).StatusDesc(statusDesc).Execute()



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
	cameraLoad := "cameraLoad_example" // string | load the given outbound relationship, value is ignored (presence of key is sufficient) (optional)
	videoLoad := "videoLoad_example" // string | load the given outbound relationship, value is ignored (presence of key is sufficient) (optional)
	referencedByDetectionTrackIDLoad := "referencedByDetectionTrackIDLoad_example" // string | load the given inbound relationship, value is ignored (presence of key is sufficient) (optional)
	keyEq := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL = comparison (optional)
	keyNe := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL != comparison (optional)
	keyGt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL > comparison, may not work with all column types (optional)
	keyGte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL >= comparison, may not work with all column types (optional)
	keyLt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL < comparison, may not work with all column types (optional)
	keyLte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL <= comparison, may not work with all column types (optional)
	keyIn := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL IN comparison, permits comma-separated values (optional)
	keyNotin := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	keyAsc := "keyAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	keyDesc := "keyDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
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
	cameraIdEq := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL = comparison (optional)
	cameraIdNe := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL != comparison (optional)
	cameraIdGt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL > comparison, may not work with all column types (optional)
	cameraIdGte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL >= comparison, may not work with all column types (optional)
	cameraIdLt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL < comparison, may not work with all column types (optional)
	cameraIdLte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL <= comparison, may not work with all column types (optional)
	cameraIdIn := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL IN comparison, permits comma-separated values (optional)
	cameraIdNotin := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	cameraIdAsc := "cameraIdAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	cameraIdDesc := "cameraIdDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	videoIdEq := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL = comparison (optional)
	videoIdNe := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL != comparison (optional)
	videoIdGt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL > comparison, may not work with all column types (optional)
	videoIdGte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL >= comparison, may not work with all column types (optional)
	videoIdLt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL < comparison, may not work with all column types (optional)
	videoIdLte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL <= comparison, may not work with all column types (optional)
	videoIdIn := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL IN comparison, permits comma-separated values (optional)
	videoIdNotin := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	videoIdAsc := "videoIdAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	videoIdDesc := "videoIdDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	classIdEq := int32(56) // int32 | SQL = comparison (optional)
	classIdNe := int32(56) // int32 | SQL != comparison (optional)
	classIdGt := int32(56) // int32 | SQL > comparison, may not work with all column types (optional)
	classIdGte := int32(56) // int32 | SQL >= comparison, may not work with all column types (optional)
	classIdLt := int32(56) // int32 | SQL < comparison, may not work with all column types (optional)
	classIdLte := int32(56) // int32 | SQL <= comparison, may not work with all column types (optional)
	classIdIn := int32(56) // int32 | SQL IN comparison, permits comma-separated values (optional)
	classIdNotin := int32(56) // int32 | SQL NOT IN comparison, permits comma-separated values (optional)
	classIdAsc := "classIdAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	classIdDesc := "classIdDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	classNameEq := "classNameEq_example" // string | SQL = comparison (optional)
	classNameNe := "classNameNe_example" // string | SQL != comparison (optional)
	classNameIn := "classNameIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	classNameNotin := "classNameNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	classNameLike := "classNameLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	classNameNotlike := "classNameNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	classNameIlike := "classNameIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	classNameNotilike := "classNameNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	classNameAsc := "classNameAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	classNameDesc := "classNameDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	startedAtEq := time.Now() // time.Time | SQL = comparison (optional)
	startedAtNe := time.Now() // time.Time | SQL != comparison (optional)
	startedAtGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	startedAtGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	startedAtLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	startedAtLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	startedAtIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	startedAtNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	startedAtAsc := "startedAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	startedAtDesc := "startedAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	endedAtEq := time.Now() // time.Time | SQL = comparison (optional)
	endedAtNe := time.Now() // time.Time | SQL != comparison (optional)
	endedAtGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	endedAtGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	endedAtLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	endedAtLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	endedAtIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	endedAtNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	endedAtAsc := "endedAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	endedAtDesc := "endedAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	frameCountEq := int32(56) // int32 | SQL = comparison (optional)
	frameCountNe := int32(56) // int32 | SQL != comparison (optional)
	frameCountGt := int32(56) // int32 | SQL > comparison, may not work with all column types (optional)
	frameCountGte := int32(56) // int32 | SQL >= comparison, may not work with all column types (optional)
	frameCountLt := int32(56) // int32 | SQL < comparison, may not work with all column types (optional)
	frameCountLte := int32(56) // int32 | SQL <= comparison, may not work with all column types (optional)
	frameCountIn := int32(56) // int32 | SQL IN comparison, permits comma-separated values (optional)
	frameCountNotin := int32(56) // int32 | SQL NOT IN comparison, permits comma-separated values (optional)
	frameCountAsc := "frameCountAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	frameCountDesc := "frameCountDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	meanScoreEq := float32(8.14) // float32 | SQL = comparison (optional)
	meanScoreNe := float32(8.14) // float32 | SQL != comparison (optional)
	meanScoreGt := float32(8.14) // float32 | SQL > comparison, may not work with all column types (optional)
	meanScoreGte := float32(8.14) // float32 | SQL >= comparison, may not work with all column types (optional)
	meanScoreLt := float32(8.14) // float32 | SQL < comparison, may not work with all column types (optional)
	meanScoreLte := float32(8.14) // float32 | SQL <= comparison, may not work with all column types (optional)
	meanScoreIn := float32(8.14) // float32 | SQL IN comparison, permits comma-separated values (optional)
	meanScoreNotin := float32(8.14) // float32 | SQL NOT IN comparison, permits comma-separated values (optional)
	meanScoreAsc := "meanScoreAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	meanScoreDesc := "meanScoreDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	maxScoreEq := float32(8.14) // float32 | SQL = comparison (optional)
	maxScoreNe := float32(8.14) // float32 | SQL != comparison (optional)
	maxScoreGt := float32(8.14) // float32 | SQL > comparison, may not work with all column types (optional)
	maxScoreGte := float32(8.14) // float32 | SQL >= comparison, may not work with all column types (optional)
	maxScoreLt := float32(8.14) // float32 | SQL < comparison, may not work with all column types (optional)
	maxScoreLte := float32(8.14) // float32 | SQL <= comparison, may not work with all column types (optional)
	maxScoreIn := float32(8.14) // float32 | SQL IN comparison, permits comma-separated values (optional)
	maxScoreNotin := float32(8.14) // float32 | SQL NOT IN comparison, permits comma-separated values (optional)
	maxScoreAsc := "maxScoreAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	maxScoreDesc := "maxScoreDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	trajectoryContains := TODO // interface{} | SQL @> comparison (optional)
	trajectoryNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	trajectoryAsc := "trajectoryAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	trajectoryDesc := "trajectoryDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	isStaticEq := true // bool | SQL = comparison (optional)
	isStaticNe := true // bool | SQL != comparison (optional)
	isStaticIsfalse := "isStaticIsfalse_example" // string | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
	isStaticIstrue := "isStaticIstrue_example" // string | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
	isStaticAsc := "isStaticAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	isStaticDesc := "isStaticDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	statusEq := "statusEq_example" // string | SQL = comparison (optional)
	statusNe := "statusNe_example" // string | SQL != comparison (optional)
	statusIn := "statusIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	statusNotin := "statusNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	statusLike := "statusLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	statusNotlike := "statusNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	statusIlike := "statusIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	statusNotilike := "statusNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	statusAsc := "statusAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	statusDesc := "statusDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.TrackAPI.GetTracks(context.Background()).Limit(limit).Offset(offset).Depth(depth).CameraLoad(cameraLoad).VideoLoad(videoLoad).ReferencedByDetectionTrackIDLoad(referencedByDetectionTrackIDLoad).KeyEq(keyEq).KeyNe(keyNe).KeyGt(keyGt).KeyGte(keyGte).KeyLt(keyLt).KeyLte(keyLte).KeyIn(keyIn).KeyNotin(keyNotin).KeyAsc(keyAsc).KeyDesc(keyDesc).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtAsc(createdAtAsc).CreatedAtDesc(createdAtDesc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtAsc(updatedAtAsc).UpdatedAtDesc(updatedAtDesc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtIsnull(deletedAtIsnull).DeletedAtIsnotnull(deletedAtIsnotnull).DeletedAtAsc(deletedAtAsc).DeletedAtDesc(deletedAtDesc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdAsc(cameraIdAsc).CameraIdDesc(cameraIdDesc).VideoIdEq(videoIdEq).VideoIdNe(videoIdNe).VideoIdGt(videoIdGt).VideoIdGte(videoIdGte).VideoIdLt(videoIdLt).VideoIdLte(videoIdLte).VideoIdIn(videoIdIn).VideoIdNotin(videoIdNotin).VideoIdAsc(videoIdAsc).VideoIdDesc(videoIdDesc).ClassIdEq(classIdEq).ClassIdNe(classIdNe).ClassIdGt(classIdGt).ClassIdGte(classIdGte).ClassIdLt(classIdLt).ClassIdLte(classIdLte).ClassIdIn(classIdIn).ClassIdNotin(classIdNotin).ClassIdAsc(classIdAsc).ClassIdDesc(classIdDesc).ClassNameEq(classNameEq).ClassNameNe(classNameNe).ClassNameIn(classNameIn).ClassNameNotin(classNameNotin).ClassNameLike(classNameLike).ClassNameNotlike(classNameNotlike).ClassNameIlike(classNameIlike).ClassNameNotilike(classNameNotilike).ClassNameAsc(classNameAsc).ClassNameDesc(classNameDesc).StartedAtEq(startedAtEq).StartedAtNe(startedAtNe).StartedAtGt(startedAtGt).StartedAtGte(startedAtGte).StartedAtLt(startedAtLt).StartedAtLte(startedAtLte).StartedAtIn(startedAtIn).StartedAtNotin(startedAtNotin).StartedAtAsc(startedAtAsc).StartedAtDesc(startedAtDesc).EndedAtEq(endedAtEq).EndedAtNe(endedAtNe).EndedAtGt(endedAtGt).EndedAtGte(endedAtGte).EndedAtLt(endedAtLt).EndedAtLte(endedAtLte).EndedAtIn(endedAtIn).EndedAtNotin(endedAtNotin).EndedAtAsc(endedAtAsc).EndedAtDesc(endedAtDesc).FrameCountEq(frameCountEq).FrameCountNe(frameCountNe).FrameCountGt(frameCountGt).FrameCountGte(frameCountGte).FrameCountLt(frameCountLt).FrameCountLte(frameCountLte).FrameCountIn(frameCountIn).FrameCountNotin(frameCountNotin).FrameCountAsc(frameCountAsc).FrameCountDesc(frameCountDesc).MeanScoreEq(meanScoreEq).MeanScoreNe(meanScoreNe).MeanScoreGt(meanScoreGt).MeanScoreGte(meanScoreGte).MeanScoreLt(meanScoreLt).MeanScoreLte(meanScoreLte).MeanScoreIn(meanScoreIn).MeanScoreNotin(meanScoreNotin).MeanScoreAsc(meanScoreAsc).MeanScoreDesc(meanScoreDesc).MaxScoreEq(maxScoreEq).MaxScoreNe(maxScoreNe).MaxScoreGt(maxScoreGt).MaxScoreGte(maxScoreGte).MaxScoreLt(maxScoreLt).MaxScoreLte(maxScoreLte).MaxScoreIn(maxScoreIn).MaxScoreNotin(maxScoreNotin).MaxScoreAsc(maxScoreAsc).MaxScoreDesc(maxScoreDesc).TrajectoryContains(trajectoryContains).TrajectoryNotcontains(trajectoryNotcontains).TrajectoryAsc(trajectoryAsc).TrajectoryDesc(trajectoryDesc).IsStaticEq(isStaticEq).IsStaticNe(isStaticNe).IsStaticIsfalse(isStaticIsfalse).IsStaticIstrue(isStaticIstrue).IsStaticAsc(isStaticAsc).IsStaticDesc(isStaticDesc).StatusEq(statusEq).StatusNe(statusNe).StatusIn(statusIn).StatusNotin(statusNotin).StatusLike(statusLike).StatusNotlike(statusNotlike).StatusIlike(statusIlike).StatusNotilike(statusNotilike).StatusAsc(statusAsc).StatusDesc(statusDesc).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `TrackAPI.GetTracks``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetTracks`: ResponseWithGenericOfTrack
	fmt.Fprintf(os.Stdout, "Response from `TrackAPI.GetTracks`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiGetTracksRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int32** | SQL LIMIT operator | 
 **offset** | **int32** | SQL OFFSET operator | 
 **depth** | **int32** | Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | 
 **cameraLoad** | **string** | load the given outbound relationship, value is ignored (presence of key is sufficient) | 
 **videoLoad** | **string** | load the given outbound relationship, value is ignored (presence of key is sufficient) | 
 **referencedByDetectionTrackIDLoad** | **string** | load the given inbound relationship, value is ignored (presence of key is sufficient) | 
 **keyEq** | **string** | SQL &#x3D; comparison | 
 **keyNe** | **string** | SQL !&#x3D; comparison | 
 **keyGt** | **string** | SQL &gt; comparison, may not work with all column types | 
 **keyGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **keyLt** | **string** | SQL &lt; comparison, may not work with all column types | 
 **keyLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **keyIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **keyNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **keyAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **keyDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
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
 **cameraIdEq** | **string** | SQL &#x3D; comparison | 
 **cameraIdNe** | **string** | SQL !&#x3D; comparison | 
 **cameraIdGt** | **string** | SQL &gt; comparison, may not work with all column types | 
 **cameraIdGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **cameraIdLt** | **string** | SQL &lt; comparison, may not work with all column types | 
 **cameraIdLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **cameraIdIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **cameraIdNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **cameraIdAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **cameraIdDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **videoIdEq** | **string** | SQL &#x3D; comparison | 
 **videoIdNe** | **string** | SQL !&#x3D; comparison | 
 **videoIdGt** | **string** | SQL &gt; comparison, may not work with all column types | 
 **videoIdGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **videoIdLt** | **string** | SQL &lt; comparison, may not work with all column types | 
 **videoIdLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **videoIdIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **videoIdNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **videoIdAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **videoIdDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **classIdEq** | **int32** | SQL &#x3D; comparison | 
 **classIdNe** | **int32** | SQL !&#x3D; comparison | 
 **classIdGt** | **int32** | SQL &gt; comparison, may not work with all column types | 
 **classIdGte** | **int32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **classIdLt** | **int32** | SQL &lt; comparison, may not work with all column types | 
 **classIdLte** | **int32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **classIdIn** | **int32** | SQL IN comparison, permits comma-separated values | 
 **classIdNotin** | **int32** | SQL NOT IN comparison, permits comma-separated values | 
 **classIdAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **classIdDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **classNameEq** | **string** | SQL &#x3D; comparison | 
 **classNameNe** | **string** | SQL !&#x3D; comparison | 
 **classNameIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **classNameNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **classNameLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **classNameNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **classNameIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **classNameNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **classNameAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **classNameDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **startedAtEq** | **time.Time** | SQL &#x3D; comparison | 
 **startedAtNe** | **time.Time** | SQL !&#x3D; comparison | 
 **startedAtGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **startedAtGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **startedAtLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **startedAtLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **startedAtIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **startedAtNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **startedAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **startedAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **endedAtEq** | **time.Time** | SQL &#x3D; comparison | 
 **endedAtNe** | **time.Time** | SQL !&#x3D; comparison | 
 **endedAtGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **endedAtGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **endedAtLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **endedAtLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **endedAtIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **endedAtNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **endedAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **endedAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **frameCountEq** | **int32** | SQL &#x3D; comparison | 
 **frameCountNe** | **int32** | SQL !&#x3D; comparison | 
 **frameCountGt** | **int32** | SQL &gt; comparison, may not work with all column types | 
 **frameCountGte** | **int32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **frameCountLt** | **int32** | SQL &lt; comparison, may not work with all column types | 
 **frameCountLte** | **int32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **frameCountIn** | **int32** | SQL IN comparison, permits comma-separated values | 
 **frameCountNotin** | **int32** | SQL NOT IN comparison, permits comma-separated values | 
 **frameCountAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **frameCountDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **meanScoreEq** | **float32** | SQL &#x3D; comparison | 
 **meanScoreNe** | **float32** | SQL !&#x3D; comparison | 
 **meanScoreGt** | **float32** | SQL &gt; comparison, may not work with all column types | 
 **meanScoreGte** | **float32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **meanScoreLt** | **float32** | SQL &lt; comparison, may not work with all column types | 
 **meanScoreLte** | **float32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **meanScoreIn** | **float32** | SQL IN comparison, permits comma-separated values | 
 **meanScoreNotin** | **float32** | SQL NOT IN comparison, permits comma-separated values | 
 **meanScoreAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **meanScoreDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **maxScoreEq** | **float32** | SQL &#x3D; comparison | 
 **maxScoreNe** | **float32** | SQL !&#x3D; comparison | 
 **maxScoreGt** | **float32** | SQL &gt; comparison, may not work with all column types | 
 **maxScoreGte** | **float32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **maxScoreLt** | **float32** | SQL &lt; comparison, may not work with all column types | 
 **maxScoreLte** | **float32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **maxScoreIn** | **float32** | SQL IN comparison, permits comma-separated values | 
 **maxScoreNotin** | **float32** | SQL NOT IN comparison, permits comma-separated values | 
 **maxScoreAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **maxScoreDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **trajectoryContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison | 
 **trajectoryNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison | 
 **trajectoryAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **trajectoryDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **isStaticEq** | **bool** | SQL &#x3D; comparison | 
 **isStaticNe** | **bool** | SQL !&#x3D; comparison | 
 **isStaticIsfalse** | **string** | SQL IS false comparison, value is ignored (presence of key is sufficient) | 
 **isStaticIstrue** | **string** | SQL IS true comparison, value is ignored (presence of key is sufficient) | 
 **isStaticAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **isStaticDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **statusEq** | **string** | SQL &#x3D; comparison | 
 **statusNe** | **string** | SQL !&#x3D; comparison | 
 **statusIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **statusNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **statusLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **statusNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **statusIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **statusNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **statusAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **statusDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 

### Return type

[**ResponseWithGenericOfTrack**](ResponseWithGenericOfTrack.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PatchTrack

> ResponseWithGenericOfTrack PatchTrack(ctx, primaryKey).Track(track).Depth(depth).Execute()



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
	track := *openapiclient.NewTrack() // Track | 
	depth := int64(789) // int64 | Query parameter depth (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.TrackAPI.PatchTrack(context.Background(), primaryKey).Track(track).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `TrackAPI.PatchTrack``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PatchTrack`: ResponseWithGenericOfTrack
	fmt.Fprintf(os.Stdout, "Response from `TrackAPI.PatchTrack`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**primaryKey** | **string** | Path parameter primaryKey | 

### Other Parameters

Other parameters are passed through a pointer to a apiPatchTrackRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **track** | [**Track**](Track.md) |  | 
 **depth** | **int64** | Query parameter depth | 

### Return type

[**ResponseWithGenericOfTrack**](ResponseWithGenericOfTrack.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostTracks

> ResponseWithGenericOfTrack PostTracks(ctx).Track(track).Depth(depth).Execute()



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
	track := []openapiclient.Track{*openapiclient.NewTrack()} // []Track | 
	depth := int64(789) // int64 | Query parameter depth (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.TrackAPI.PostTracks(context.Background()).Track(track).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `TrackAPI.PostTracks``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostTracks`: ResponseWithGenericOfTrack
	fmt.Fprintf(os.Stdout, "Response from `TrackAPI.PostTracks`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostTracksRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **track** | [**[]Track**](Track.md) |  | 
 **depth** | **int64** | Query parameter depth | 

### Return type

[**ResponseWithGenericOfTrack**](ResponseWithGenericOfTrack.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

