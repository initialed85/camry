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

> ResponseWithGenericOfDetection GetDetections(ctx).Limit(limit).Offset(offset).Depth(depth).VideoLoad(videoLoad).CameraLoad(cameraLoad).IdEq(idEq).IdNe(idNe).IdGt(idGt).IdGte(idGte).IdLt(idLt).IdLte(idLte).IdIn(idIn).IdNotin(idNotin).IdContains(idContains).IdNotcontains(idNotcontains).IdLike(idLike).IdNotlike(idNotlike).IdIlike(idIlike).IdNotilike(idNotilike).IdDesc(idDesc).IdAsc(idAsc).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtContains(createdAtContains).CreatedAtNotcontains(createdAtNotcontains).CreatedAtLike(createdAtLike).CreatedAtNotlike(createdAtNotlike).CreatedAtIlike(createdAtIlike).CreatedAtNotilike(createdAtNotilike).CreatedAtDesc(createdAtDesc).CreatedAtAsc(createdAtAsc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtContains(updatedAtContains).UpdatedAtNotcontains(updatedAtNotcontains).UpdatedAtLike(updatedAtLike).UpdatedAtNotlike(updatedAtNotlike).UpdatedAtIlike(updatedAtIlike).UpdatedAtNotilike(updatedAtNotilike).UpdatedAtDesc(updatedAtDesc).UpdatedAtAsc(updatedAtAsc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtContains(deletedAtContains).DeletedAtNotcontains(deletedAtNotcontains).DeletedAtLike(deletedAtLike).DeletedAtNotlike(deletedAtNotlike).DeletedAtIlike(deletedAtIlike).DeletedAtNotilike(deletedAtNotilike).DeletedAtDesc(deletedAtDesc).DeletedAtAsc(deletedAtAsc).SeenAtEq(seenAtEq).SeenAtNe(seenAtNe).SeenAtGt(seenAtGt).SeenAtGte(seenAtGte).SeenAtLt(seenAtLt).SeenAtLte(seenAtLte).SeenAtIn(seenAtIn).SeenAtNotin(seenAtNotin).SeenAtContains(seenAtContains).SeenAtNotcontains(seenAtNotcontains).SeenAtLike(seenAtLike).SeenAtNotlike(seenAtNotlike).SeenAtIlike(seenAtIlike).SeenAtNotilike(seenAtNotilike).SeenAtDesc(seenAtDesc).SeenAtAsc(seenAtAsc).ClassIdEq(classIdEq).ClassIdNe(classIdNe).ClassIdGt(classIdGt).ClassIdGte(classIdGte).ClassIdLt(classIdLt).ClassIdLte(classIdLte).ClassIdIn(classIdIn).ClassIdNotin(classIdNotin).ClassIdContains(classIdContains).ClassIdNotcontains(classIdNotcontains).ClassIdDesc(classIdDesc).ClassIdAsc(classIdAsc).ClassNameEq(classNameEq).ClassNameNe(classNameNe).ClassNameGt(classNameGt).ClassNameGte(classNameGte).ClassNameLt(classNameLt).ClassNameLte(classNameLte).ClassNameIn(classNameIn).ClassNameNotin(classNameNotin).ClassNameContains(classNameContains).ClassNameNotcontains(classNameNotcontains).ClassNameLike(classNameLike).ClassNameNotlike(classNameNotlike).ClassNameIlike(classNameIlike).ClassNameNotilike(classNameNotilike).ClassNameDesc(classNameDesc).ClassNameAsc(classNameAsc).ScoreEq(scoreEq).ScoreNe(scoreNe).ScoreGt(scoreGt).ScoreGte(scoreGte).ScoreLt(scoreLt).ScoreLte(scoreLte).ScoreIn(scoreIn).ScoreNotin(scoreNotin).ScoreContains(scoreContains).ScoreNotcontains(scoreNotcontains).ScoreDesc(scoreDesc).ScoreAsc(scoreAsc).CentroidContains(centroidContains).CentroidNotcontains(centroidNotcontains).CentroidDesc(centroidDesc).CentroidAsc(centroidAsc).BoundingBoxContains(boundingBoxContains).BoundingBoxNotcontains(boundingBoxNotcontains).BoundingBoxDesc(boundingBoxDesc).BoundingBoxAsc(boundingBoxAsc).VideoIdEq(videoIdEq).VideoIdNe(videoIdNe).VideoIdGt(videoIdGt).VideoIdGte(videoIdGte).VideoIdLt(videoIdLt).VideoIdLte(videoIdLte).VideoIdIn(videoIdIn).VideoIdNotin(videoIdNotin).VideoIdContains(videoIdContains).VideoIdNotcontains(videoIdNotcontains).VideoIdLike(videoIdLike).VideoIdNotlike(videoIdNotlike).VideoIdIlike(videoIdIlike).VideoIdNotilike(videoIdNotilike).VideoIdDesc(videoIdDesc).VideoIdAsc(videoIdAsc).VideoIdObjectContains(videoIdObjectContains).VideoIdObjectNotcontains(videoIdObjectNotcontains).VideoIdObjectDesc(videoIdObjectDesc).VideoIdObjectAsc(videoIdObjectAsc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdContains(cameraIdContains).CameraIdNotcontains(cameraIdNotcontains).CameraIdLike(cameraIdLike).CameraIdNotlike(cameraIdNotlike).CameraIdIlike(cameraIdIlike).CameraIdNotilike(cameraIdNotilike).CameraIdDesc(cameraIdDesc).CameraIdAsc(cameraIdAsc).CameraIdObjectContains(cameraIdObjectContains).CameraIdObjectNotcontains(cameraIdObjectNotcontains).CameraIdObjectDesc(cameraIdObjectDesc).CameraIdObjectAsc(cameraIdObjectAsc).Execute()



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
	videoLoad := "videoLoad_example" // string | load the given directly related object, value is ignored (presence of key is sufficient) (optional)
	cameraLoad := "cameraLoad_example" // string | load the given directly related object, value is ignored (presence of key is sufficient) (optional)
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
	seenAtEq := time.Now() // time.Time | SQL = comparison (optional)
	seenAtNe := time.Now() // time.Time | SQL != comparison (optional)
	seenAtGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	seenAtGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	seenAtLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	seenAtLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	seenAtIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	seenAtNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	seenAtContains := time.Now() // time.Time | SQL @> comparison (optional)
	seenAtNotcontains := time.Now() // time.Time | SQL NOT @> comparison (optional)
	seenAtLike := time.Now() // time.Time | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	seenAtNotlike := time.Now() // time.Time | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	seenAtIlike := time.Now() // time.Time | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	seenAtNotilike := time.Now() // time.Time | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	seenAtDesc := "seenAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	seenAtAsc := "seenAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
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
	scoreEq := float64(1.2) // float64 | SQL = comparison (optional)
	scoreNe := float64(1.2) // float64 | SQL != comparison (optional)
	scoreGt := float64(1.2) // float64 | SQL > comparison, may not work with all column types (optional)
	scoreGte := float64(1.2) // float64 | SQL >= comparison, may not work with all column types (optional)
	scoreLt := float64(1.2) // float64 | SQL < comparison, may not work with all column types (optional)
	scoreLte := float64(1.2) // float64 | SQL <= comparison, may not work with all column types (optional)
	scoreIn := float64(1.2) // float64 | SQL IN comparison, permits comma-separated values (optional)
	scoreNotin := float64(1.2) // float64 | SQL NOT IN comparison, permits comma-separated values (optional)
	scoreContains := float64(1.2) // float64 | SQL @> comparison (optional)
	scoreNotcontains := float64(1.2) // float64 | SQL NOT @> comparison (optional)
	scoreDesc := "scoreDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	scoreAsc := "scoreAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	centroidContains := TODO // interface{} | SQL @> comparison (optional)
	centroidNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	centroidDesc := "centroidDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	centroidAsc := "centroidAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	boundingBoxContains := TODO // interface{} | SQL @> comparison (optional)
	boundingBoxNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	boundingBoxDesc := "boundingBoxDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	boundingBoxAsc := "boundingBoxAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DetectionAPI.GetDetections(context.Background()).Limit(limit).Offset(offset).Depth(depth).VideoLoad(videoLoad).CameraLoad(cameraLoad).IdEq(idEq).IdNe(idNe).IdGt(idGt).IdGte(idGte).IdLt(idLt).IdLte(idLte).IdIn(idIn).IdNotin(idNotin).IdContains(idContains).IdNotcontains(idNotcontains).IdLike(idLike).IdNotlike(idNotlike).IdIlike(idIlike).IdNotilike(idNotilike).IdDesc(idDesc).IdAsc(idAsc).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtContains(createdAtContains).CreatedAtNotcontains(createdAtNotcontains).CreatedAtLike(createdAtLike).CreatedAtNotlike(createdAtNotlike).CreatedAtIlike(createdAtIlike).CreatedAtNotilike(createdAtNotilike).CreatedAtDesc(createdAtDesc).CreatedAtAsc(createdAtAsc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtContains(updatedAtContains).UpdatedAtNotcontains(updatedAtNotcontains).UpdatedAtLike(updatedAtLike).UpdatedAtNotlike(updatedAtNotlike).UpdatedAtIlike(updatedAtIlike).UpdatedAtNotilike(updatedAtNotilike).UpdatedAtDesc(updatedAtDesc).UpdatedAtAsc(updatedAtAsc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtContains(deletedAtContains).DeletedAtNotcontains(deletedAtNotcontains).DeletedAtLike(deletedAtLike).DeletedAtNotlike(deletedAtNotlike).DeletedAtIlike(deletedAtIlike).DeletedAtNotilike(deletedAtNotilike).DeletedAtDesc(deletedAtDesc).DeletedAtAsc(deletedAtAsc).SeenAtEq(seenAtEq).SeenAtNe(seenAtNe).SeenAtGt(seenAtGt).SeenAtGte(seenAtGte).SeenAtLt(seenAtLt).SeenAtLte(seenAtLte).SeenAtIn(seenAtIn).SeenAtNotin(seenAtNotin).SeenAtContains(seenAtContains).SeenAtNotcontains(seenAtNotcontains).SeenAtLike(seenAtLike).SeenAtNotlike(seenAtNotlike).SeenAtIlike(seenAtIlike).SeenAtNotilike(seenAtNotilike).SeenAtDesc(seenAtDesc).SeenAtAsc(seenAtAsc).ClassIdEq(classIdEq).ClassIdNe(classIdNe).ClassIdGt(classIdGt).ClassIdGte(classIdGte).ClassIdLt(classIdLt).ClassIdLte(classIdLte).ClassIdIn(classIdIn).ClassIdNotin(classIdNotin).ClassIdContains(classIdContains).ClassIdNotcontains(classIdNotcontains).ClassIdDesc(classIdDesc).ClassIdAsc(classIdAsc).ClassNameEq(classNameEq).ClassNameNe(classNameNe).ClassNameGt(classNameGt).ClassNameGte(classNameGte).ClassNameLt(classNameLt).ClassNameLte(classNameLte).ClassNameIn(classNameIn).ClassNameNotin(classNameNotin).ClassNameContains(classNameContains).ClassNameNotcontains(classNameNotcontains).ClassNameLike(classNameLike).ClassNameNotlike(classNameNotlike).ClassNameIlike(classNameIlike).ClassNameNotilike(classNameNotilike).ClassNameDesc(classNameDesc).ClassNameAsc(classNameAsc).ScoreEq(scoreEq).ScoreNe(scoreNe).ScoreGt(scoreGt).ScoreGte(scoreGte).ScoreLt(scoreLt).ScoreLte(scoreLte).ScoreIn(scoreIn).ScoreNotin(scoreNotin).ScoreContains(scoreContains).ScoreNotcontains(scoreNotcontains).ScoreDesc(scoreDesc).ScoreAsc(scoreAsc).CentroidContains(centroidContains).CentroidNotcontains(centroidNotcontains).CentroidDesc(centroidDesc).CentroidAsc(centroidAsc).BoundingBoxContains(boundingBoxContains).BoundingBoxNotcontains(boundingBoxNotcontains).BoundingBoxDesc(boundingBoxDesc).BoundingBoxAsc(boundingBoxAsc).VideoIdEq(videoIdEq).VideoIdNe(videoIdNe).VideoIdGt(videoIdGt).VideoIdGte(videoIdGte).VideoIdLt(videoIdLt).VideoIdLte(videoIdLte).VideoIdIn(videoIdIn).VideoIdNotin(videoIdNotin).VideoIdContains(videoIdContains).VideoIdNotcontains(videoIdNotcontains).VideoIdLike(videoIdLike).VideoIdNotlike(videoIdNotlike).VideoIdIlike(videoIdIlike).VideoIdNotilike(videoIdNotilike).VideoIdDesc(videoIdDesc).VideoIdAsc(videoIdAsc).VideoIdObjectContains(videoIdObjectContains).VideoIdObjectNotcontains(videoIdObjectNotcontains).VideoIdObjectDesc(videoIdObjectDesc).VideoIdObjectAsc(videoIdObjectAsc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdContains(cameraIdContains).CameraIdNotcontains(cameraIdNotcontains).CameraIdLike(cameraIdLike).CameraIdNotlike(cameraIdNotlike).CameraIdIlike(cameraIdIlike).CameraIdNotilike(cameraIdNotilike).CameraIdDesc(cameraIdDesc).CameraIdAsc(cameraIdAsc).CameraIdObjectContains(cameraIdObjectContains).CameraIdObjectNotcontains(cameraIdObjectNotcontains).CameraIdObjectDesc(cameraIdObjectDesc).CameraIdObjectAsc(cameraIdObjectAsc).Execute()
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
 **videoLoad** | **string** | load the given directly related object, value is ignored (presence of key is sufficient) | 
 **cameraLoad** | **string** | load the given directly related object, value is ignored (presence of key is sufficient) | 
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
 **seenAtEq** | **time.Time** | SQL &#x3D; comparison | 
 **seenAtNe** | **time.Time** | SQL !&#x3D; comparison | 
 **seenAtGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **seenAtGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **seenAtLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **seenAtLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **seenAtIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **seenAtNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **seenAtContains** | **time.Time** | SQL @&gt; comparison | 
 **seenAtNotcontains** | **time.Time** | SQL NOT @&gt; comparison | 
 **seenAtLike** | **time.Time** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **seenAtNotlike** | **time.Time** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **seenAtIlike** | **time.Time** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **seenAtNotilike** | **time.Time** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **seenAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **seenAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
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
 **scoreEq** | **float64** | SQL &#x3D; comparison | 
 **scoreNe** | **float64** | SQL !&#x3D; comparison | 
 **scoreGt** | **float64** | SQL &gt; comparison, may not work with all column types | 
 **scoreGte** | **float64** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **scoreLt** | **float64** | SQL &lt; comparison, may not work with all column types | 
 **scoreLte** | **float64** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **scoreIn** | **float64** | SQL IN comparison, permits comma-separated values | 
 **scoreNotin** | **float64** | SQL NOT IN comparison, permits comma-separated values | 
 **scoreContains** | **float64** | SQL @&gt; comparison | 
 **scoreNotcontains** | **float64** | SQL NOT @&gt; comparison | 
 **scoreDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **scoreAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **centroidContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison | 
 **centroidNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison | 
 **centroidDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **centroidAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **boundingBoxContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison | 
 **boundingBoxNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison | 
 **boundingBoxDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **boundingBoxAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
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

