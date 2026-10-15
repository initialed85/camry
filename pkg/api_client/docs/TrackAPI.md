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

> ResponseWithGenericOfTrack GetTracks(ctx).Limit(limit).Offset(offset).Depth(depth).CameraLoad(cameraLoad).VideoLoad(videoLoad).ReferencedByDetectionLoad(referencedByDetectionLoad).IdEq(idEq).IdNe(idNe).IdGt(idGt).IdGte(idGte).IdLt(idLt).IdLte(idLte).IdIn(idIn).IdNotin(idNotin).IdContains(idContains).IdNotcontains(idNotcontains).IdLike(idLike).IdNotlike(idNotlike).IdIlike(idIlike).IdNotilike(idNotilike).IdDesc(idDesc).IdAsc(idAsc).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtContains(createdAtContains).CreatedAtNotcontains(createdAtNotcontains).CreatedAtLike(createdAtLike).CreatedAtNotlike(createdAtNotlike).CreatedAtIlike(createdAtIlike).CreatedAtNotilike(createdAtNotilike).CreatedAtDesc(createdAtDesc).CreatedAtAsc(createdAtAsc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtContains(updatedAtContains).UpdatedAtNotcontains(updatedAtNotcontains).UpdatedAtLike(updatedAtLike).UpdatedAtNotlike(updatedAtNotlike).UpdatedAtIlike(updatedAtIlike).UpdatedAtNotilike(updatedAtNotilike).UpdatedAtDesc(updatedAtDesc).UpdatedAtAsc(updatedAtAsc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtContains(deletedAtContains).DeletedAtNotcontains(deletedAtNotcontains).DeletedAtLike(deletedAtLike).DeletedAtNotlike(deletedAtNotlike).DeletedAtIlike(deletedAtIlike).DeletedAtNotilike(deletedAtNotilike).DeletedAtDesc(deletedAtDesc).DeletedAtAsc(deletedAtAsc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdContains(cameraIdContains).CameraIdNotcontains(cameraIdNotcontains).CameraIdLike(cameraIdLike).CameraIdNotlike(cameraIdNotlike).CameraIdIlike(cameraIdIlike).CameraIdNotilike(cameraIdNotilike).CameraIdDesc(cameraIdDesc).CameraIdAsc(cameraIdAsc).CameraIdObjectContains(cameraIdObjectContains).CameraIdObjectNotcontains(cameraIdObjectNotcontains).CameraIdObjectDesc(cameraIdObjectDesc).CameraIdObjectAsc(cameraIdObjectAsc).VideoIdEq(videoIdEq).VideoIdNe(videoIdNe).VideoIdGt(videoIdGt).VideoIdGte(videoIdGte).VideoIdLt(videoIdLt).VideoIdLte(videoIdLte).VideoIdIn(videoIdIn).VideoIdNotin(videoIdNotin).VideoIdContains(videoIdContains).VideoIdNotcontains(videoIdNotcontains).VideoIdLike(videoIdLike).VideoIdNotlike(videoIdNotlike).VideoIdIlike(videoIdIlike).VideoIdNotilike(videoIdNotilike).VideoIdDesc(videoIdDesc).VideoIdAsc(videoIdAsc).VideoIdObjectContains(videoIdObjectContains).VideoIdObjectNotcontains(videoIdObjectNotcontains).VideoIdObjectDesc(videoIdObjectDesc).VideoIdObjectAsc(videoIdObjectAsc).ClassIdEq(classIdEq).ClassIdNe(classIdNe).ClassIdGt(classIdGt).ClassIdGte(classIdGte).ClassIdLt(classIdLt).ClassIdLte(classIdLte).ClassIdIn(classIdIn).ClassIdNotin(classIdNotin).ClassIdContains(classIdContains).ClassIdNotcontains(classIdNotcontains).ClassIdDesc(classIdDesc).ClassIdAsc(classIdAsc).ClassNameEq(classNameEq).ClassNameNe(classNameNe).ClassNameGt(classNameGt).ClassNameGte(classNameGte).ClassNameLt(classNameLt).ClassNameLte(classNameLte).ClassNameIn(classNameIn).ClassNameNotin(classNameNotin).ClassNameContains(classNameContains).ClassNameNotcontains(classNameNotcontains).ClassNameLike(classNameLike).ClassNameNotlike(classNameNotlike).ClassNameIlike(classNameIlike).ClassNameNotilike(classNameNotilike).ClassNameDesc(classNameDesc).ClassNameAsc(classNameAsc).StartedAtEq(startedAtEq).StartedAtNe(startedAtNe).StartedAtGt(startedAtGt).StartedAtGte(startedAtGte).StartedAtLt(startedAtLt).StartedAtLte(startedAtLte).StartedAtIn(startedAtIn).StartedAtNotin(startedAtNotin).StartedAtContains(startedAtContains).StartedAtNotcontains(startedAtNotcontains).StartedAtLike(startedAtLike).StartedAtNotlike(startedAtNotlike).StartedAtIlike(startedAtIlike).StartedAtNotilike(startedAtNotilike).StartedAtDesc(startedAtDesc).StartedAtAsc(startedAtAsc).EndedAtEq(endedAtEq).EndedAtNe(endedAtNe).EndedAtGt(endedAtGt).EndedAtGte(endedAtGte).EndedAtLt(endedAtLt).EndedAtLte(endedAtLte).EndedAtIn(endedAtIn).EndedAtNotin(endedAtNotin).EndedAtContains(endedAtContains).EndedAtNotcontains(endedAtNotcontains).EndedAtLike(endedAtLike).EndedAtNotlike(endedAtNotlike).EndedAtIlike(endedAtIlike).EndedAtNotilike(endedAtNotilike).EndedAtDesc(endedAtDesc).EndedAtAsc(endedAtAsc).FrameCountEq(frameCountEq).FrameCountNe(frameCountNe).FrameCountGt(frameCountGt).FrameCountGte(frameCountGte).FrameCountLt(frameCountLt).FrameCountLte(frameCountLte).FrameCountIn(frameCountIn).FrameCountNotin(frameCountNotin).FrameCountContains(frameCountContains).FrameCountNotcontains(frameCountNotcontains).FrameCountDesc(frameCountDesc).FrameCountAsc(frameCountAsc).MeanScoreEq(meanScoreEq).MeanScoreNe(meanScoreNe).MeanScoreGt(meanScoreGt).MeanScoreGte(meanScoreGte).MeanScoreLt(meanScoreLt).MeanScoreLte(meanScoreLte).MeanScoreIn(meanScoreIn).MeanScoreNotin(meanScoreNotin).MeanScoreContains(meanScoreContains).MeanScoreNotcontains(meanScoreNotcontains).MeanScoreDesc(meanScoreDesc).MeanScoreAsc(meanScoreAsc).MaxScoreEq(maxScoreEq).MaxScoreNe(maxScoreNe).MaxScoreGt(maxScoreGt).MaxScoreGte(maxScoreGte).MaxScoreLt(maxScoreLt).MaxScoreLte(maxScoreLte).MaxScoreIn(maxScoreIn).MaxScoreNotin(maxScoreNotin).MaxScoreContains(maxScoreContains).MaxScoreNotcontains(maxScoreNotcontains).MaxScoreDesc(maxScoreDesc).MaxScoreAsc(maxScoreAsc).CentroidContains(centroidContains).CentroidNotcontains(centroidNotcontains).CentroidDesc(centroidDesc).CentroidAsc(centroidAsc).BoundingBoxContains(boundingBoxContains).BoundingBoxNotcontains(boundingBoxNotcontains).BoundingBoxDesc(boundingBoxDesc).BoundingBoxAsc(boundingBoxAsc).TrajectoryContains(trajectoryContains).TrajectoryNotcontains(trajectoryNotcontains).TrajectoryDesc(trajectoryDesc).TrajectoryAsc(trajectoryAsc).IsStaticEq(isStaticEq).IsStaticNe(isStaticNe).IsStaticGt(isStaticGt).IsStaticGte(isStaticGte).IsStaticLt(isStaticLt).IsStaticLte(isStaticLte).IsStaticIn(isStaticIn).IsStaticNotin(isStaticNotin).IsStaticContains(isStaticContains).IsStaticNotcontains(isStaticNotcontains).IsStaticDesc(isStaticDesc).IsStaticAsc(isStaticAsc).StatusEq(statusEq).StatusNe(statusNe).StatusGt(statusGt).StatusGte(statusGte).StatusLt(statusLt).StatusLte(statusLte).StatusIn(statusIn).StatusNotin(statusNotin).StatusContains(statusContains).StatusNotcontains(statusNotcontains).StatusLike(statusLike).StatusNotlike(statusNotlike).StatusIlike(statusIlike).StatusNotilike(statusNotilike).StatusDesc(statusDesc).StatusAsc(statusAsc).KeyEq(keyEq).KeyNe(keyNe).KeyGt(keyGt).KeyGte(keyGte).KeyLt(keyLt).KeyLte(keyLte).KeyIn(keyIn).KeyNotin(keyNotin).KeyContains(keyContains).KeyNotcontains(keyNotcontains).KeyLike(keyLike).KeyNotlike(keyNotlike).KeyIlike(keyIlike).KeyNotilike(keyNotilike).KeyDesc(keyDesc).KeyAsc(keyAsc).ReferencedByDetectionTrackIdObjectsContains(referencedByDetectionTrackIdObjectsContains).ReferencedByDetectionTrackIdObjectsNotcontains(referencedByDetectionTrackIdObjectsNotcontains).ReferencedByDetectionTrackIdObjectsDesc(referencedByDetectionTrackIdObjectsDesc).ReferencedByDetectionTrackIdObjectsAsc(referencedByDetectionTrackIdObjectsAsc).Execute()



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
	cameraLoad := "cameraLoad_example" // string | load the given directly related object, value is ignored (presence of key is sufficient) (optional)
	videoLoad := "videoLoad_example" // string | load the given directly related object, value is ignored (presence of key is sufficient) (optional)
	referencedByDetectionLoad := "referencedByDetectionLoad_example" // string | load the given indirectly related objects, value is ignored (presence of key is sufficient) (optional)
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
	cameraIdEq := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL = comparison (optional)
	cameraIdNe := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL != comparison (optional)
	cameraIdGt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL > comparison, may not work with all column types (optional)
	cameraIdGte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL >= comparison, may not work with all column types (optional)
	cameraIdLt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL < comparison, may not work with all column types (optional)
	cameraIdLte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL <= comparison, may not work with all column types (optional)
	cameraIdIn := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL IN comparison, permits comma-separated values (optional)
	cameraIdNotin := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	cameraIdContains := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL @> comparison (optional)
	cameraIdNotcontains := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT @> comparison (optional)
	cameraIdLike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	cameraIdNotlike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	cameraIdIlike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	cameraIdNotilike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	cameraIdDesc := "cameraIdDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	cameraIdAsc := "cameraIdAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	cameraIdObjectContains := TODO // interface{} | SQL @> comparison (optional)
	cameraIdObjectNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	cameraIdObjectDesc := "cameraIdObjectDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	cameraIdObjectAsc := "cameraIdObjectAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	videoIdEq := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL = comparison (optional)
	videoIdNe := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL != comparison (optional)
	videoIdGt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL > comparison, may not work with all column types (optional)
	videoIdGte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL >= comparison, may not work with all column types (optional)
	videoIdLt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL < comparison, may not work with all column types (optional)
	videoIdLte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL <= comparison, may not work with all column types (optional)
	videoIdIn := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL IN comparison, permits comma-separated values (optional)
	videoIdNotin := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	videoIdContains := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL @> comparison (optional)
	videoIdNotcontains := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT @> comparison (optional)
	videoIdLike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	videoIdNotlike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	videoIdIlike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	videoIdNotilike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	videoIdDesc := "videoIdDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	videoIdAsc := "videoIdAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	videoIdObjectContains := TODO // interface{} | SQL @> comparison (optional)
	videoIdObjectNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	videoIdObjectDesc := "videoIdObjectDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	videoIdObjectAsc := "videoIdObjectAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	classIdEq := int64(789) // int64 | SQL = comparison (optional)
	classIdNe := int64(789) // int64 | SQL != comparison (optional)
	classIdGt := int64(789) // int64 | SQL > comparison, may not work with all column types (optional)
	classIdGte := int64(789) // int64 | SQL >= comparison, may not work with all column types (optional)
	classIdLt := int64(789) // int64 | SQL < comparison, may not work with all column types (optional)
	classIdLte := int64(789) // int64 | SQL <= comparison, may not work with all column types (optional)
	classIdIn := int64(789) // int64 | SQL IN comparison, permits comma-separated values (optional)
	classIdNotin := int64(789) // int64 | SQL NOT IN comparison, permits comma-separated values (optional)
	classIdContains := int64(789) // int64 | SQL @> comparison (optional)
	classIdNotcontains := int64(789) // int64 | SQL NOT @> comparison (optional)
	classIdDesc := "classIdDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	classIdAsc := "classIdAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	classNameEq := "classNameEq_example" // string | SQL = comparison (optional)
	classNameNe := "classNameNe_example" // string | SQL != comparison (optional)
	classNameGt := "classNameGt_example" // string | SQL > comparison, may not work with all column types (optional)
	classNameGte := "classNameGte_example" // string | SQL >= comparison, may not work with all column types (optional)
	classNameLt := "classNameLt_example" // string | SQL < comparison, may not work with all column types (optional)
	classNameLte := "classNameLte_example" // string | SQL <= comparison, may not work with all column types (optional)
	classNameIn := "classNameIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	classNameNotin := "classNameNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	classNameContains := "classNameContains_example" // string | SQL @> comparison (optional)
	classNameNotcontains := "classNameNotcontains_example" // string | SQL NOT @> comparison (optional)
	classNameLike := "classNameLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	classNameNotlike := "classNameNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	classNameIlike := "classNameIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	classNameNotilike := "classNameNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	classNameDesc := "classNameDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	classNameAsc := "classNameAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	startedAtEq := time.Now() // time.Time | SQL = comparison (optional)
	startedAtNe := time.Now() // time.Time | SQL != comparison (optional)
	startedAtGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	startedAtGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	startedAtLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	startedAtLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	startedAtIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	startedAtNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	startedAtContains := time.Now() // time.Time | SQL @> comparison (optional)
	startedAtNotcontains := time.Now() // time.Time | SQL NOT @> comparison (optional)
	startedAtLike := time.Now() // time.Time | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	startedAtNotlike := time.Now() // time.Time | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	startedAtIlike := time.Now() // time.Time | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	startedAtNotilike := time.Now() // time.Time | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	startedAtDesc := "startedAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	startedAtAsc := "startedAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	endedAtEq := time.Now() // time.Time | SQL = comparison (optional)
	endedAtNe := time.Now() // time.Time | SQL != comparison (optional)
	endedAtGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	endedAtGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	endedAtLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	endedAtLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	endedAtIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	endedAtNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	endedAtContains := time.Now() // time.Time | SQL @> comparison (optional)
	endedAtNotcontains := time.Now() // time.Time | SQL NOT @> comparison (optional)
	endedAtLike := time.Now() // time.Time | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	endedAtNotlike := time.Now() // time.Time | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	endedAtIlike := time.Now() // time.Time | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	endedAtNotilike := time.Now() // time.Time | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	endedAtDesc := "endedAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	endedAtAsc := "endedAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	frameCountEq := int64(789) // int64 | SQL = comparison (optional)
	frameCountNe := int64(789) // int64 | SQL != comparison (optional)
	frameCountGt := int64(789) // int64 | SQL > comparison, may not work with all column types (optional)
	frameCountGte := int64(789) // int64 | SQL >= comparison, may not work with all column types (optional)
	frameCountLt := int64(789) // int64 | SQL < comparison, may not work with all column types (optional)
	frameCountLte := int64(789) // int64 | SQL <= comparison, may not work with all column types (optional)
	frameCountIn := int64(789) // int64 | SQL IN comparison, permits comma-separated values (optional)
	frameCountNotin := int64(789) // int64 | SQL NOT IN comparison, permits comma-separated values (optional)
	frameCountContains := int64(789) // int64 | SQL @> comparison (optional)
	frameCountNotcontains := int64(789) // int64 | SQL NOT @> comparison (optional)
	frameCountDesc := "frameCountDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	frameCountAsc := "frameCountAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	meanScoreEq := float64(1.2) // float64 | SQL = comparison (optional)
	meanScoreNe := float64(1.2) // float64 | SQL != comparison (optional)
	meanScoreGt := float64(1.2) // float64 | SQL > comparison, may not work with all column types (optional)
	meanScoreGte := float64(1.2) // float64 | SQL >= comparison, may not work with all column types (optional)
	meanScoreLt := float64(1.2) // float64 | SQL < comparison, may not work with all column types (optional)
	meanScoreLte := float64(1.2) // float64 | SQL <= comparison, may not work with all column types (optional)
	meanScoreIn := float64(1.2) // float64 | SQL IN comparison, permits comma-separated values (optional)
	meanScoreNotin := float64(1.2) // float64 | SQL NOT IN comparison, permits comma-separated values (optional)
	meanScoreContains := float64(1.2) // float64 | SQL @> comparison (optional)
	meanScoreNotcontains := float64(1.2) // float64 | SQL NOT @> comparison (optional)
	meanScoreDesc := "meanScoreDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	meanScoreAsc := "meanScoreAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	maxScoreEq := float64(1.2) // float64 | SQL = comparison (optional)
	maxScoreNe := float64(1.2) // float64 | SQL != comparison (optional)
	maxScoreGt := float64(1.2) // float64 | SQL > comparison, may not work with all column types (optional)
	maxScoreGte := float64(1.2) // float64 | SQL >= comparison, may not work with all column types (optional)
	maxScoreLt := float64(1.2) // float64 | SQL < comparison, may not work with all column types (optional)
	maxScoreLte := float64(1.2) // float64 | SQL <= comparison, may not work with all column types (optional)
	maxScoreIn := float64(1.2) // float64 | SQL IN comparison, permits comma-separated values (optional)
	maxScoreNotin := float64(1.2) // float64 | SQL NOT IN comparison, permits comma-separated values (optional)
	maxScoreContains := float64(1.2) // float64 | SQL @> comparison (optional)
	maxScoreNotcontains := float64(1.2) // float64 | SQL NOT @> comparison (optional)
	maxScoreDesc := "maxScoreDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	maxScoreAsc := "maxScoreAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	centroidContains := TODO // interface{} | SQL @> comparison (optional)
	centroidNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	centroidDesc := "centroidDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	centroidAsc := "centroidAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	boundingBoxContains := TODO // interface{} | SQL @> comparison (optional)
	boundingBoxNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	boundingBoxDesc := "boundingBoxDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	boundingBoxAsc := "boundingBoxAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	trajectoryContains := TODO // interface{} | SQL @> comparison (optional)
	trajectoryNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	trajectoryDesc := "trajectoryDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	trajectoryAsc := "trajectoryAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	isStaticEq := true // bool | SQL = comparison (optional)
	isStaticNe := true // bool | SQL != comparison (optional)
	isStaticGt := true // bool | SQL > comparison, may not work with all column types (optional)
	isStaticGte := true // bool | SQL >= comparison, may not work with all column types (optional)
	isStaticLt := true // bool | SQL < comparison, may not work with all column types (optional)
	isStaticLte := true // bool | SQL <= comparison, may not work with all column types (optional)
	isStaticIn := true // bool | SQL IN comparison, permits comma-separated values (optional)
	isStaticNotin := true // bool | SQL NOT IN comparison, permits comma-separated values (optional)
	isStaticContains := true // bool | SQL @> comparison (optional)
	isStaticNotcontains := true // bool | SQL NOT @> comparison (optional)
	isStaticDesc := "isStaticDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	isStaticAsc := "isStaticAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	statusEq := "statusEq_example" // string | SQL = comparison (optional)
	statusNe := "statusNe_example" // string | SQL != comparison (optional)
	statusGt := "statusGt_example" // string | SQL > comparison, may not work with all column types (optional)
	statusGte := "statusGte_example" // string | SQL >= comparison, may not work with all column types (optional)
	statusLt := "statusLt_example" // string | SQL < comparison, may not work with all column types (optional)
	statusLte := "statusLte_example" // string | SQL <= comparison, may not work with all column types (optional)
	statusIn := "statusIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	statusNotin := "statusNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	statusContains := "statusContains_example" // string | SQL @> comparison (optional)
	statusNotcontains := "statusNotcontains_example" // string | SQL NOT @> comparison (optional)
	statusLike := "statusLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	statusNotlike := "statusNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	statusIlike := "statusIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	statusNotilike := "statusNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	statusDesc := "statusDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	statusAsc := "statusAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	keyEq := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL = comparison (optional)
	keyNe := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL != comparison (optional)
	keyGt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL > comparison, may not work with all column types (optional)
	keyGte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL >= comparison, may not work with all column types (optional)
	keyLt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL < comparison, may not work with all column types (optional)
	keyLte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL <= comparison, may not work with all column types (optional)
	keyIn := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL IN comparison, permits comma-separated values (optional)
	keyNotin := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	keyContains := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL @> comparison (optional)
	keyNotcontains := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT @> comparison (optional)
	keyLike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	keyNotlike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	keyIlike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	keyNotilike := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	keyDesc := "keyDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	keyAsc := "keyAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	referencedByDetectionTrackIdObjectsContains := TODO // interface{} | SQL @> comparison (optional)
	referencedByDetectionTrackIdObjectsNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	referencedByDetectionTrackIdObjectsDesc := "referencedByDetectionTrackIdObjectsDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	referencedByDetectionTrackIdObjectsAsc := "referencedByDetectionTrackIdObjectsAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.TrackAPI.GetTracks(context.Background()).Limit(limit).Offset(offset).Depth(depth).CameraLoad(cameraLoad).VideoLoad(videoLoad).ReferencedByDetectionLoad(referencedByDetectionLoad).IdEq(idEq).IdNe(idNe).IdGt(idGt).IdGte(idGte).IdLt(idLt).IdLte(idLte).IdIn(idIn).IdNotin(idNotin).IdContains(idContains).IdNotcontains(idNotcontains).IdLike(idLike).IdNotlike(idNotlike).IdIlike(idIlike).IdNotilike(idNotilike).IdDesc(idDesc).IdAsc(idAsc).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtContains(createdAtContains).CreatedAtNotcontains(createdAtNotcontains).CreatedAtLike(createdAtLike).CreatedAtNotlike(createdAtNotlike).CreatedAtIlike(createdAtIlike).CreatedAtNotilike(createdAtNotilike).CreatedAtDesc(createdAtDesc).CreatedAtAsc(createdAtAsc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtContains(updatedAtContains).UpdatedAtNotcontains(updatedAtNotcontains).UpdatedAtLike(updatedAtLike).UpdatedAtNotlike(updatedAtNotlike).UpdatedAtIlike(updatedAtIlike).UpdatedAtNotilike(updatedAtNotilike).UpdatedAtDesc(updatedAtDesc).UpdatedAtAsc(updatedAtAsc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtContains(deletedAtContains).DeletedAtNotcontains(deletedAtNotcontains).DeletedAtLike(deletedAtLike).DeletedAtNotlike(deletedAtNotlike).DeletedAtIlike(deletedAtIlike).DeletedAtNotilike(deletedAtNotilike).DeletedAtDesc(deletedAtDesc).DeletedAtAsc(deletedAtAsc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdContains(cameraIdContains).CameraIdNotcontains(cameraIdNotcontains).CameraIdLike(cameraIdLike).CameraIdNotlike(cameraIdNotlike).CameraIdIlike(cameraIdIlike).CameraIdNotilike(cameraIdNotilike).CameraIdDesc(cameraIdDesc).CameraIdAsc(cameraIdAsc).CameraIdObjectContains(cameraIdObjectContains).CameraIdObjectNotcontains(cameraIdObjectNotcontains).CameraIdObjectDesc(cameraIdObjectDesc).CameraIdObjectAsc(cameraIdObjectAsc).VideoIdEq(videoIdEq).VideoIdNe(videoIdNe).VideoIdGt(videoIdGt).VideoIdGte(videoIdGte).VideoIdLt(videoIdLt).VideoIdLte(videoIdLte).VideoIdIn(videoIdIn).VideoIdNotin(videoIdNotin).VideoIdContains(videoIdContains).VideoIdNotcontains(videoIdNotcontains).VideoIdLike(videoIdLike).VideoIdNotlike(videoIdNotlike).VideoIdIlike(videoIdIlike).VideoIdNotilike(videoIdNotilike).VideoIdDesc(videoIdDesc).VideoIdAsc(videoIdAsc).VideoIdObjectContains(videoIdObjectContains).VideoIdObjectNotcontains(videoIdObjectNotcontains).VideoIdObjectDesc(videoIdObjectDesc).VideoIdObjectAsc(videoIdObjectAsc).ClassIdEq(classIdEq).ClassIdNe(classIdNe).ClassIdGt(classIdGt).ClassIdGte(classIdGte).ClassIdLt(classIdLt).ClassIdLte(classIdLte).ClassIdIn(classIdIn).ClassIdNotin(classIdNotin).ClassIdContains(classIdContains).ClassIdNotcontains(classIdNotcontains).ClassIdDesc(classIdDesc).ClassIdAsc(classIdAsc).ClassNameEq(classNameEq).ClassNameNe(classNameNe).ClassNameGt(classNameGt).ClassNameGte(classNameGte).ClassNameLt(classNameLt).ClassNameLte(classNameLte).ClassNameIn(classNameIn).ClassNameNotin(classNameNotin).ClassNameContains(classNameContains).ClassNameNotcontains(classNameNotcontains).ClassNameLike(classNameLike).ClassNameNotlike(classNameNotlike).ClassNameIlike(classNameIlike).ClassNameNotilike(classNameNotilike).ClassNameDesc(classNameDesc).ClassNameAsc(classNameAsc).StartedAtEq(startedAtEq).StartedAtNe(startedAtNe).StartedAtGt(startedAtGt).StartedAtGte(startedAtGte).StartedAtLt(startedAtLt).StartedAtLte(startedAtLte).StartedAtIn(startedAtIn).StartedAtNotin(startedAtNotin).StartedAtContains(startedAtContains).StartedAtNotcontains(startedAtNotcontains).StartedAtLike(startedAtLike).StartedAtNotlike(startedAtNotlike).StartedAtIlike(startedAtIlike).StartedAtNotilike(startedAtNotilike).StartedAtDesc(startedAtDesc).StartedAtAsc(startedAtAsc).EndedAtEq(endedAtEq).EndedAtNe(endedAtNe).EndedAtGt(endedAtGt).EndedAtGte(endedAtGte).EndedAtLt(endedAtLt).EndedAtLte(endedAtLte).EndedAtIn(endedAtIn).EndedAtNotin(endedAtNotin).EndedAtContains(endedAtContains).EndedAtNotcontains(endedAtNotcontains).EndedAtLike(endedAtLike).EndedAtNotlike(endedAtNotlike).EndedAtIlike(endedAtIlike).EndedAtNotilike(endedAtNotilike).EndedAtDesc(endedAtDesc).EndedAtAsc(endedAtAsc).FrameCountEq(frameCountEq).FrameCountNe(frameCountNe).FrameCountGt(frameCountGt).FrameCountGte(frameCountGte).FrameCountLt(frameCountLt).FrameCountLte(frameCountLte).FrameCountIn(frameCountIn).FrameCountNotin(frameCountNotin).FrameCountContains(frameCountContains).FrameCountNotcontains(frameCountNotcontains).FrameCountDesc(frameCountDesc).FrameCountAsc(frameCountAsc).MeanScoreEq(meanScoreEq).MeanScoreNe(meanScoreNe).MeanScoreGt(meanScoreGt).MeanScoreGte(meanScoreGte).MeanScoreLt(meanScoreLt).MeanScoreLte(meanScoreLte).MeanScoreIn(meanScoreIn).MeanScoreNotin(meanScoreNotin).MeanScoreContains(meanScoreContains).MeanScoreNotcontains(meanScoreNotcontains).MeanScoreDesc(meanScoreDesc).MeanScoreAsc(meanScoreAsc).MaxScoreEq(maxScoreEq).MaxScoreNe(maxScoreNe).MaxScoreGt(maxScoreGt).MaxScoreGte(maxScoreGte).MaxScoreLt(maxScoreLt).MaxScoreLte(maxScoreLte).MaxScoreIn(maxScoreIn).MaxScoreNotin(maxScoreNotin).MaxScoreContains(maxScoreContains).MaxScoreNotcontains(maxScoreNotcontains).MaxScoreDesc(maxScoreDesc).MaxScoreAsc(maxScoreAsc).CentroidContains(centroidContains).CentroidNotcontains(centroidNotcontains).CentroidDesc(centroidDesc).CentroidAsc(centroidAsc).BoundingBoxContains(boundingBoxContains).BoundingBoxNotcontains(boundingBoxNotcontains).BoundingBoxDesc(boundingBoxDesc).BoundingBoxAsc(boundingBoxAsc).TrajectoryContains(trajectoryContains).TrajectoryNotcontains(trajectoryNotcontains).TrajectoryDesc(trajectoryDesc).TrajectoryAsc(trajectoryAsc).IsStaticEq(isStaticEq).IsStaticNe(isStaticNe).IsStaticGt(isStaticGt).IsStaticGte(isStaticGte).IsStaticLt(isStaticLt).IsStaticLte(isStaticLte).IsStaticIn(isStaticIn).IsStaticNotin(isStaticNotin).IsStaticContains(isStaticContains).IsStaticNotcontains(isStaticNotcontains).IsStaticDesc(isStaticDesc).IsStaticAsc(isStaticAsc).StatusEq(statusEq).StatusNe(statusNe).StatusGt(statusGt).StatusGte(statusGte).StatusLt(statusLt).StatusLte(statusLte).StatusIn(statusIn).StatusNotin(statusNotin).StatusContains(statusContains).StatusNotcontains(statusNotcontains).StatusLike(statusLike).StatusNotlike(statusNotlike).StatusIlike(statusIlike).StatusNotilike(statusNotilike).StatusDesc(statusDesc).StatusAsc(statusAsc).KeyEq(keyEq).KeyNe(keyNe).KeyGt(keyGt).KeyGte(keyGte).KeyLt(keyLt).KeyLte(keyLte).KeyIn(keyIn).KeyNotin(keyNotin).KeyContains(keyContains).KeyNotcontains(keyNotcontains).KeyLike(keyLike).KeyNotlike(keyNotlike).KeyIlike(keyIlike).KeyNotilike(keyNotilike).KeyDesc(keyDesc).KeyAsc(keyAsc).ReferencedByDetectionTrackIdObjectsContains(referencedByDetectionTrackIdObjectsContains).ReferencedByDetectionTrackIdObjectsNotcontains(referencedByDetectionTrackIdObjectsNotcontains).ReferencedByDetectionTrackIdObjectsDesc(referencedByDetectionTrackIdObjectsDesc).ReferencedByDetectionTrackIdObjectsAsc(referencedByDetectionTrackIdObjectsAsc).Execute()
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
 **cameraLoad** | **string** | load the given directly related object, value is ignored (presence of key is sufficient) |
 **videoLoad** | **string** | load the given directly related object, value is ignored (presence of key is sufficient) |
 **referencedByDetectionLoad** | **string** | load the given indirectly related objects, value is ignored (presence of key is sufficient) |
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
 **cameraIdEq** | **string** | SQL &#x3D; comparison |
 **cameraIdNe** | **string** | SQL !&#x3D; comparison |
 **cameraIdGt** | **string** | SQL &gt; comparison, may not work with all column types |
 **cameraIdGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types |
 **cameraIdLt** | **string** | SQL &lt; comparison, may not work with all column types |
 **cameraIdLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types |
 **cameraIdIn** | **string** | SQL IN comparison, permits comma-separated values |
 **cameraIdNotin** | **string** | SQL NOT IN comparison, permits comma-separated values |
 **cameraIdContains** | **string** | SQL @&gt; comparison |
 **cameraIdNotcontains** | **string** | SQL NOT @&gt; comparison |
 **cameraIdLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % |
 **cameraIdNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % |
 **cameraIdIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **cameraIdNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **cameraIdDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **cameraIdAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **cameraIdObjectContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison |
 **cameraIdObjectNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison |
 **cameraIdObjectDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **cameraIdObjectAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **videoIdEq** | **string** | SQL &#x3D; comparison |
 **videoIdNe** | **string** | SQL !&#x3D; comparison |
 **videoIdGt** | **string** | SQL &gt; comparison, may not work with all column types |
 **videoIdGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types |
 **videoIdLt** | **string** | SQL &lt; comparison, may not work with all column types |
 **videoIdLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types |
 **videoIdIn** | **string** | SQL IN comparison, permits comma-separated values |
 **videoIdNotin** | **string** | SQL NOT IN comparison, permits comma-separated values |
 **videoIdContains** | **string** | SQL @&gt; comparison |
 **videoIdNotcontains** | **string** | SQL NOT @&gt; comparison |
 **videoIdLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % |
 **videoIdNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % |
 **videoIdIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **videoIdNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **videoIdDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **videoIdAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **videoIdObjectContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison |
 **videoIdObjectNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison |
 **videoIdObjectDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **videoIdObjectAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **classIdEq** | **int64** | SQL &#x3D; comparison |
 **classIdNe** | **int64** | SQL !&#x3D; comparison |
 **classIdGt** | **int64** | SQL &gt; comparison, may not work with all column types |
 **classIdGte** | **int64** | SQL &gt;&#x3D; comparison, may not work with all column types |
 **classIdLt** | **int64** | SQL &lt; comparison, may not work with all column types |
 **classIdLte** | **int64** | SQL &lt;&#x3D; comparison, may not work with all column types |
 **classIdIn** | **int64** | SQL IN comparison, permits comma-separated values |
 **classIdNotin** | **int64** | SQL NOT IN comparison, permits comma-separated values |
 **classIdContains** | **int64** | SQL @&gt; comparison |
 **classIdNotcontains** | **int64** | SQL NOT @&gt; comparison |
 **classIdDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **classIdAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **classNameEq** | **string** | SQL &#x3D; comparison |
 **classNameNe** | **string** | SQL !&#x3D; comparison |
 **classNameGt** | **string** | SQL &gt; comparison, may not work with all column types |
 **classNameGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types |
 **classNameLt** | **string** | SQL &lt; comparison, may not work with all column types |
 **classNameLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types |
 **classNameIn** | **string** | SQL IN comparison, permits comma-separated values |
 **classNameNotin** | **string** | SQL NOT IN comparison, permits comma-separated values |
 **classNameContains** | **string** | SQL @&gt; comparison |
 **classNameNotcontains** | **string** | SQL NOT @&gt; comparison |
 **classNameLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % |
 **classNameNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % |
 **classNameIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **classNameNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **classNameDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **classNameAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **startedAtEq** | **time.Time** | SQL &#x3D; comparison |
 **startedAtNe** | **time.Time** | SQL !&#x3D; comparison |
 **startedAtGt** | **time.Time** | SQL &gt; comparison, may not work with all column types |
 **startedAtGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types |
 **startedAtLt** | **time.Time** | SQL &lt; comparison, may not work with all column types |
 **startedAtLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types |
 **startedAtIn** | **time.Time** | SQL IN comparison, permits comma-separated values |
 **startedAtNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values |
 **startedAtContains** | **time.Time** | SQL @&gt; comparison |
 **startedAtNotcontains** | **time.Time** | SQL NOT @&gt; comparison |
 **startedAtLike** | **time.Time** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % |
 **startedAtNotlike** | **time.Time** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % |
 **startedAtIlike** | **time.Time** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **startedAtNotilike** | **time.Time** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **startedAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **startedAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **endedAtEq** | **time.Time** | SQL &#x3D; comparison |
 **endedAtNe** | **time.Time** | SQL !&#x3D; comparison |
 **endedAtGt** | **time.Time** | SQL &gt; comparison, may not work with all column types |
 **endedAtGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types |
 **endedAtLt** | **time.Time** | SQL &lt; comparison, may not work with all column types |
 **endedAtLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types |
 **endedAtIn** | **time.Time** | SQL IN comparison, permits comma-separated values |
 **endedAtNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values |
 **endedAtContains** | **time.Time** | SQL @&gt; comparison |
 **endedAtNotcontains** | **time.Time** | SQL NOT @&gt; comparison |
 **endedAtLike** | **time.Time** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % |
 **endedAtNotlike** | **time.Time** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % |
 **endedAtIlike** | **time.Time** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **endedAtNotilike** | **time.Time** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **endedAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **endedAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **frameCountEq** | **int64** | SQL &#x3D; comparison |
 **frameCountNe** | **int64** | SQL !&#x3D; comparison |
 **frameCountGt** | **int64** | SQL &gt; comparison, may not work with all column types |
 **frameCountGte** | **int64** | SQL &gt;&#x3D; comparison, may not work with all column types |
 **frameCountLt** | **int64** | SQL &lt; comparison, may not work with all column types |
 **frameCountLte** | **int64** | SQL &lt;&#x3D; comparison, may not work with all column types |
 **frameCountIn** | **int64** | SQL IN comparison, permits comma-separated values |
 **frameCountNotin** | **int64** | SQL NOT IN comparison, permits comma-separated values |
 **frameCountContains** | **int64** | SQL @&gt; comparison |
 **frameCountNotcontains** | **int64** | SQL NOT @&gt; comparison |
 **frameCountDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **frameCountAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **meanScoreEq** | **float64** | SQL &#x3D; comparison |
 **meanScoreNe** | **float64** | SQL !&#x3D; comparison |
 **meanScoreGt** | **float64** | SQL &gt; comparison, may not work with all column types |
 **meanScoreGte** | **float64** | SQL &gt;&#x3D; comparison, may not work with all column types |
 **meanScoreLt** | **float64** | SQL &lt; comparison, may not work with all column types |
 **meanScoreLte** | **float64** | SQL &lt;&#x3D; comparison, may not work with all column types |
 **meanScoreIn** | **float64** | SQL IN comparison, permits comma-separated values |
 **meanScoreNotin** | **float64** | SQL NOT IN comparison, permits comma-separated values |
 **meanScoreContains** | **float64** | SQL @&gt; comparison |
 **meanScoreNotcontains** | **float64** | SQL NOT @&gt; comparison |
 **meanScoreDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **meanScoreAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **maxScoreEq** | **float64** | SQL &#x3D; comparison |
 **maxScoreNe** | **float64** | SQL !&#x3D; comparison |
 **maxScoreGt** | **float64** | SQL &gt; comparison, may not work with all column types |
 **maxScoreGte** | **float64** | SQL &gt;&#x3D; comparison, may not work with all column types |
 **maxScoreLt** | **float64** | SQL &lt; comparison, may not work with all column types |
 **maxScoreLte** | **float64** | SQL &lt;&#x3D; comparison, may not work with all column types |
 **maxScoreIn** | **float64** | SQL IN comparison, permits comma-separated values |
 **maxScoreNotin** | **float64** | SQL NOT IN comparison, permits comma-separated values |
 **maxScoreContains** | **float64** | SQL @&gt; comparison |
 **maxScoreNotcontains** | **float64** | SQL NOT @&gt; comparison |
 **maxScoreDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **maxScoreAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **centroidContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison |
 **centroidNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison |
 **centroidDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **centroidAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **boundingBoxContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison |
 **boundingBoxNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison |
 **boundingBoxDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **boundingBoxAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **trajectoryContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison |
 **trajectoryNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison |
 **trajectoryDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **trajectoryAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **isStaticEq** | **bool** | SQL &#x3D; comparison |
 **isStaticNe** | **bool** | SQL !&#x3D; comparison |
 **isStaticGt** | **bool** | SQL &gt; comparison, may not work with all column types |
 **isStaticGte** | **bool** | SQL &gt;&#x3D; comparison, may not work with all column types |
 **isStaticLt** | **bool** | SQL &lt; comparison, may not work with all column types |
 **isStaticLte** | **bool** | SQL &lt;&#x3D; comparison, may not work with all column types |
 **isStaticIn** | **bool** | SQL IN comparison, permits comma-separated values |
 **isStaticNotin** | **bool** | SQL NOT IN comparison, permits comma-separated values |
 **isStaticContains** | **bool** | SQL @&gt; comparison |
 **isStaticNotcontains** | **bool** | SQL NOT @&gt; comparison |
 **isStaticDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **isStaticAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **statusEq** | **string** | SQL &#x3D; comparison |
 **statusNe** | **string** | SQL !&#x3D; comparison |
 **statusGt** | **string** | SQL &gt; comparison, may not work with all column types |
 **statusGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types |
 **statusLt** | **string** | SQL &lt; comparison, may not work with all column types |
 **statusLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types |
 **statusIn** | **string** | SQL IN comparison, permits comma-separated values |
 **statusNotin** | **string** | SQL NOT IN comparison, permits comma-separated values |
 **statusContains** | **string** | SQL @&gt; comparison |
 **statusNotcontains** | **string** | SQL NOT @&gt; comparison |
 **statusLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % |
 **statusNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % |
 **statusIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **statusNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **statusDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **statusAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **keyEq** | **string** | SQL &#x3D; comparison |
 **keyNe** | **string** | SQL !&#x3D; comparison |
 **keyGt** | **string** | SQL &gt; comparison, may not work with all column types |
 **keyGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types |
 **keyLt** | **string** | SQL &lt; comparison, may not work with all column types |
 **keyLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types |
 **keyIn** | **string** | SQL IN comparison, permits comma-separated values |
 **keyNotin** | **string** | SQL NOT IN comparison, permits comma-separated values |
 **keyContains** | **string** | SQL @&gt; comparison |
 **keyNotcontains** | **string** | SQL NOT @&gt; comparison |
 **keyLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % |
 **keyNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % |
 **keyIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **keyNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % |
 **keyDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **keyAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |
 **referencedByDetectionTrackIdObjectsContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison |
 **referencedByDetectionTrackIdObjectsNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison |
 **referencedByDetectionTrackIdObjectsDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) |
 **referencedByDetectionTrackIdObjectsAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) |

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

