# \DetectionAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DeleteDetection**](DetectionAPI.md#DeleteDetection) | **Delete** /api/detections/{primaryKey} | 
[**GetDetection**](DetectionAPI.md#GetDetection) | **Get** /api/detections/{primaryKey} | 
[**GetDetections**](DetectionAPI.md#GetDetections) | **Get** /api/detections | 
[**PatchDetection**](DetectionAPI.md#PatchDetection) | **Patch** /api/detections/{primaryKey} | 
[**PostDetections**](DetectionAPI.md#PostDetections) | **Post** /api/detections | 



## DeleteDetection

> DeleteDetection(ctx, primaryKey).Depth(depth).Execute()



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
	r, err := apiClient.DetectionAPI.DeleteDetection(context.Background(), primaryKey).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DetectionAPI.DeleteDetection``: %v\n", err)
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

Other parameters are passed through a pointer to a apiDeleteDetectionRequest struct via the builder pattern


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


## GetDetection

> ResponseWithGenericOfDetection GetDetection(ctx, primaryKey).Depth(depth).Execute()



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
	resp, r, err := apiClient.DetectionAPI.GetDetection(context.Background(), primaryKey).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DetectionAPI.GetDetection``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetDetection`: ResponseWithGenericOfDetection
	fmt.Fprintf(os.Stdout, "Response from `DetectionAPI.GetDetection`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**primaryKey** | **string** | Path parameter primaryKey | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetDetectionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **depth** | **int64** | Query parameter depth | 

### Return type

[**ResponseWithGenericOfDetection**](ResponseWithGenericOfDetection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetDetections

> ResponseWithGenericOfDetection GetDetections(ctx).Limit(limit).Offset(offset).Depth(depth).VideoLoad(videoLoad).CameraLoad(cameraLoad).TrackLoad(trackLoad).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtAsc(createdAtAsc).CreatedAtDesc(createdAtDesc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtAsc(updatedAtAsc).UpdatedAtDesc(updatedAtDesc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtIsnull(deletedAtIsnull).DeletedAtIsnotnull(deletedAtIsnotnull).DeletedAtAsc(deletedAtAsc).DeletedAtDesc(deletedAtDesc).SeenAtEq(seenAtEq).SeenAtNe(seenAtNe).SeenAtGt(seenAtGt).SeenAtGte(seenAtGte).SeenAtLt(seenAtLt).SeenAtLte(seenAtLte).SeenAtIn(seenAtIn).SeenAtNotin(seenAtNotin).SeenAtAsc(seenAtAsc).SeenAtDesc(seenAtDesc).ClassIdEq(classIdEq).ClassIdNe(classIdNe).ClassIdGt(classIdGt).ClassIdGte(classIdGte).ClassIdLt(classIdLt).ClassIdLte(classIdLte).ClassIdIn(classIdIn).ClassIdNotin(classIdNotin).ClassIdAsc(classIdAsc).ClassIdDesc(classIdDesc).ClassNameEq(classNameEq).ClassNameNe(classNameNe).ClassNameIn(classNameIn).ClassNameNotin(classNameNotin).ClassNameLike(classNameLike).ClassNameNotlike(classNameNotlike).ClassNameIlike(classNameIlike).ClassNameNotilike(classNameNotilike).ClassNameAsc(classNameAsc).ClassNameDesc(classNameDesc).ScoreEq(scoreEq).ScoreNe(scoreNe).ScoreGt(scoreGt).ScoreGte(scoreGte).ScoreLt(scoreLt).ScoreLte(scoreLte).ScoreIn(scoreIn).ScoreNotin(scoreNotin).ScoreAsc(scoreAsc).ScoreDesc(scoreDesc).VideoIdEq(videoIdEq).VideoIdNe(videoIdNe).VideoIdGt(videoIdGt).VideoIdGte(videoIdGte).VideoIdLt(videoIdLt).VideoIdLte(videoIdLte).VideoIdIn(videoIdIn).VideoIdNotin(videoIdNotin).VideoIdAsc(videoIdAsc).VideoIdDesc(videoIdDesc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdAsc(cameraIdAsc).CameraIdDesc(cameraIdDesc).TrackIdEq(trackIdEq).TrackIdNe(trackIdNe).TrackIdGt(trackIdGt).TrackIdGte(trackIdGte).TrackIdLt(trackIdLt).TrackIdLte(trackIdLte).TrackIdIn(trackIdIn).TrackIdNotin(trackIdNotin).TrackIdIsnull(trackIdIsnull).TrackIdIsnotnull(trackIdIsnotnull).TrackIdAsc(trackIdAsc).TrackIdDesc(trackIdDesc).Execute()



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
	videoLoad := "videoLoad_example" // string | load the given outbound relationship, value is ignored (presence of key is sufficient) (optional)
	cameraLoad := "cameraLoad_example" // string | load the given outbound relationship, value is ignored (presence of key is sufficient) (optional)
	trackLoad := "trackLoad_example" // string | load the given outbound relationship, value is ignored (presence of key is sufficient) (optional)
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
	seenAtEq := time.Now() // time.Time | SQL = comparison (optional)
	seenAtNe := time.Now() // time.Time | SQL != comparison (optional)
	seenAtGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	seenAtGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	seenAtLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	seenAtLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	seenAtIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	seenAtNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	seenAtAsc := "seenAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	seenAtDesc := "seenAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
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
	scoreEq := float32(8.14) // float32 | SQL = comparison (optional)
	scoreNe := float32(8.14) // float32 | SQL != comparison (optional)
	scoreGt := float32(8.14) // float32 | SQL > comparison, may not work with all column types (optional)
	scoreGte := float32(8.14) // float32 | SQL >= comparison, may not work with all column types (optional)
	scoreLt := float32(8.14) // float32 | SQL < comparison, may not work with all column types (optional)
	scoreLte := float32(8.14) // float32 | SQL <= comparison, may not work with all column types (optional)
	scoreIn := float32(8.14) // float32 | SQL IN comparison, permits comma-separated values (optional)
	scoreNotin := float32(8.14) // float32 | SQL NOT IN comparison, permits comma-separated values (optional)
	scoreAsc := "scoreAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	scoreDesc := "scoreDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
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
	trackIdEq := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL = comparison (optional)
	trackIdNe := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL != comparison (optional)
	trackIdGt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL > comparison, may not work with all column types (optional)
	trackIdGte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL >= comparison, may not work with all column types (optional)
	trackIdLt := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL < comparison, may not work with all column types (optional)
	trackIdLte := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL <= comparison, may not work with all column types (optional)
	trackIdIn := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL IN comparison, permits comma-separated values (optional)
	trackIdNotin := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	trackIdIsnull := "trackIdIsnull_example" // string | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
	trackIdIsnotnull := "trackIdIsnotnull_example" // string | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
	trackIdAsc := "trackIdAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	trackIdDesc := "trackIdDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DetectionAPI.GetDetections(context.Background()).Limit(limit).Offset(offset).Depth(depth).VideoLoad(videoLoad).CameraLoad(cameraLoad).TrackLoad(trackLoad).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtAsc(createdAtAsc).CreatedAtDesc(createdAtDesc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtAsc(updatedAtAsc).UpdatedAtDesc(updatedAtDesc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtIsnull(deletedAtIsnull).DeletedAtIsnotnull(deletedAtIsnotnull).DeletedAtAsc(deletedAtAsc).DeletedAtDesc(deletedAtDesc).SeenAtEq(seenAtEq).SeenAtNe(seenAtNe).SeenAtGt(seenAtGt).SeenAtGte(seenAtGte).SeenAtLt(seenAtLt).SeenAtLte(seenAtLte).SeenAtIn(seenAtIn).SeenAtNotin(seenAtNotin).SeenAtAsc(seenAtAsc).SeenAtDesc(seenAtDesc).ClassIdEq(classIdEq).ClassIdNe(classIdNe).ClassIdGt(classIdGt).ClassIdGte(classIdGte).ClassIdLt(classIdLt).ClassIdLte(classIdLte).ClassIdIn(classIdIn).ClassIdNotin(classIdNotin).ClassIdAsc(classIdAsc).ClassIdDesc(classIdDesc).ClassNameEq(classNameEq).ClassNameNe(classNameNe).ClassNameIn(classNameIn).ClassNameNotin(classNameNotin).ClassNameLike(classNameLike).ClassNameNotlike(classNameNotlike).ClassNameIlike(classNameIlike).ClassNameNotilike(classNameNotilike).ClassNameAsc(classNameAsc).ClassNameDesc(classNameDesc).ScoreEq(scoreEq).ScoreNe(scoreNe).ScoreGt(scoreGt).ScoreGte(scoreGte).ScoreLt(scoreLt).ScoreLte(scoreLte).ScoreIn(scoreIn).ScoreNotin(scoreNotin).ScoreAsc(scoreAsc).ScoreDesc(scoreDesc).VideoIdEq(videoIdEq).VideoIdNe(videoIdNe).VideoIdGt(videoIdGt).VideoIdGte(videoIdGte).VideoIdLt(videoIdLt).VideoIdLte(videoIdLte).VideoIdIn(videoIdIn).VideoIdNotin(videoIdNotin).VideoIdAsc(videoIdAsc).VideoIdDesc(videoIdDesc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdAsc(cameraIdAsc).CameraIdDesc(cameraIdDesc).TrackIdEq(trackIdEq).TrackIdNe(trackIdNe).TrackIdGt(trackIdGt).TrackIdGte(trackIdGte).TrackIdLt(trackIdLt).TrackIdLte(trackIdLte).TrackIdIn(trackIdIn).TrackIdNotin(trackIdNotin).TrackIdIsnull(trackIdIsnull).TrackIdIsnotnull(trackIdIsnotnull).TrackIdAsc(trackIdAsc).TrackIdDesc(trackIdDesc).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DetectionAPI.GetDetections``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetDetections`: ResponseWithGenericOfDetection
	fmt.Fprintf(os.Stdout, "Response from `DetectionAPI.GetDetections`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiGetDetectionsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int32** | SQL LIMIT operator | 
 **offset** | **int32** | SQL OFFSET operator | 
 **depth** | **int32** | Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | 
 **videoLoad** | **string** | load the given outbound relationship, value is ignored (presence of key is sufficient) | 
 **cameraLoad** | **string** | load the given outbound relationship, value is ignored (presence of key is sufficient) | 
 **trackLoad** | **string** | load the given outbound relationship, value is ignored (presence of key is sufficient) | 
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
 **seenAtEq** | **time.Time** | SQL &#x3D; comparison | 
 **seenAtNe** | **time.Time** | SQL !&#x3D; comparison | 
 **seenAtGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **seenAtGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **seenAtLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **seenAtLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **seenAtIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **seenAtNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **seenAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **seenAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
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
 **scoreEq** | **float32** | SQL &#x3D; comparison | 
 **scoreNe** | **float32** | SQL !&#x3D; comparison | 
 **scoreGt** | **float32** | SQL &gt; comparison, may not work with all column types | 
 **scoreGte** | **float32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **scoreLt** | **float32** | SQL &lt; comparison, may not work with all column types | 
 **scoreLte** | **float32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **scoreIn** | **float32** | SQL IN comparison, permits comma-separated values | 
 **scoreNotin** | **float32** | SQL NOT IN comparison, permits comma-separated values | 
 **scoreAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **scoreDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
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
 **trackIdEq** | **string** | SQL &#x3D; comparison | 
 **trackIdNe** | **string** | SQL !&#x3D; comparison | 
 **trackIdGt** | **string** | SQL &gt; comparison, may not work with all column types | 
 **trackIdGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **trackIdLt** | **string** | SQL &lt; comparison, may not work with all column types | 
 **trackIdLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **trackIdIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **trackIdNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **trackIdIsnull** | **string** | SQL IS null comparison, value is ignored (presence of key is sufficient) | 
 **trackIdIsnotnull** | **string** | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | 
 **trackIdAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **trackIdDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 

### Return type

[**ResponseWithGenericOfDetection**](ResponseWithGenericOfDetection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PatchDetection

> ResponseWithGenericOfDetection PatchDetection(ctx, primaryKey).Detection(detection).Depth(depth).Execute()



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
	detection := *openapiclient.NewDetection() // Detection | 
	depth := int64(789) // int64 | Query parameter depth (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DetectionAPI.PatchDetection(context.Background(), primaryKey).Detection(detection).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DetectionAPI.PatchDetection``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PatchDetection`: ResponseWithGenericOfDetection
	fmt.Fprintf(os.Stdout, "Response from `DetectionAPI.PatchDetection`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**primaryKey** | **string** | Path parameter primaryKey | 

### Other Parameters

Other parameters are passed through a pointer to a apiPatchDetectionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **detection** | [**Detection**](Detection.md) |  | 
 **depth** | **int64** | Query parameter depth | 

### Return type

[**ResponseWithGenericOfDetection**](ResponseWithGenericOfDetection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostDetections

> ResponseWithGenericOfDetection PostDetections(ctx).Detection(detection).Depth(depth).Execute()



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
	detection := []openapiclient.Detection{*openapiclient.NewDetection()} // []Detection | 
	depth := int64(789) // int64 | Query parameter depth (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DetectionAPI.PostDetections(context.Background()).Detection(detection).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DetectionAPI.PostDetections``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostDetections`: ResponseWithGenericOfDetection
	fmt.Fprintf(os.Stdout, "Response from `DetectionAPI.PostDetections`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostDetectionsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **detection** | [**[]Detection**](Detection.md) |  | 
 **depth** | **int64** | Query parameter depth | 

### Return type

[**ResponseWithGenericOfDetection**](ResponseWithGenericOfDetection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

