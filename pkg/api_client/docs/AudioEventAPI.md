# \AudioEventAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DeleteAudioEvent**](AudioEventAPI.md#DeleteAudioEvent) | **Delete** /api/audio-events/{primaryKey} | 
[**GetAudioEvent**](AudioEventAPI.md#GetAudioEvent) | **Get** /api/audio-events/{primaryKey} | 
[**GetAudioEvents**](AudioEventAPI.md#GetAudioEvents) | **Get** /api/audio-events | 
[**PatchAudioEvent**](AudioEventAPI.md#PatchAudioEvent) | **Patch** /api/audio-events/{primaryKey} | 
[**PostAudioEvents**](AudioEventAPI.md#PostAudioEvents) | **Post** /api/audio-events | 



## DeleteAudioEvent

> DeleteAudioEvent(ctx, primaryKey).Depth(depth).Execute()



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
	r, err := apiClient.AudioEventAPI.DeleteAudioEvent(context.Background(), primaryKey).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AudioEventAPI.DeleteAudioEvent``: %v\n", err)
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

Other parameters are passed through a pointer to a apiDeleteAudioEventRequest struct via the builder pattern


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


## GetAudioEvent

> ResponseWithGenericOfAudioEvent GetAudioEvent(ctx, primaryKey).Depth(depth).Execute()



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
	resp, r, err := apiClient.AudioEventAPI.GetAudioEvent(context.Background(), primaryKey).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AudioEventAPI.GetAudioEvent``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAudioEvent`: ResponseWithGenericOfAudioEvent
	fmt.Fprintf(os.Stdout, "Response from `AudioEventAPI.GetAudioEvent`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**primaryKey** | **string** | Path parameter primaryKey | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetAudioEventRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **depth** | **int64** | Query parameter depth | 

### Return type

[**ResponseWithGenericOfAudioEvent**](ResponseWithGenericOfAudioEvent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetAudioEvents

> ResponseWithGenericOfAudioEvent GetAudioEvents(ctx).Limit(limit).Offset(offset).Depth(depth).CameraLoad(cameraLoad).VideoLoad(videoLoad).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtAsc(createdAtAsc).CreatedAtDesc(createdAtDesc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtAsc(updatedAtAsc).UpdatedAtDesc(updatedAtDesc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtIsnull(deletedAtIsnull).DeletedAtIsnotnull(deletedAtIsnotnull).DeletedAtAsc(deletedAtAsc).DeletedAtDesc(deletedAtDesc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdAsc(cameraIdAsc).CameraIdDesc(cameraIdDesc).VideoIdEq(videoIdEq).VideoIdNe(videoIdNe).VideoIdGt(videoIdGt).VideoIdGte(videoIdGte).VideoIdLt(videoIdLt).VideoIdLte(videoIdLte).VideoIdIn(videoIdIn).VideoIdNotin(videoIdNotin).VideoIdAsc(videoIdAsc).VideoIdDesc(videoIdDesc).SeenAtEq(seenAtEq).SeenAtNe(seenAtNe).SeenAtGt(seenAtGt).SeenAtGte(seenAtGte).SeenAtLt(seenAtLt).SeenAtLte(seenAtLte).SeenAtIn(seenAtIn).SeenAtNotin(seenAtNotin).SeenAtAsc(seenAtAsc).SeenAtDesc(seenAtDesc).OffsetMsEq(offsetMsEq).OffsetMsNe(offsetMsNe).OffsetMsGt(offsetMsGt).OffsetMsGte(offsetMsGte).OffsetMsLt(offsetMsLt).OffsetMsLte(offsetMsLte).OffsetMsIn(offsetMsIn).OffsetMsNotin(offsetMsNotin).OffsetMsAsc(offsetMsAsc).OffsetMsDesc(offsetMsDesc).EndedAtEq(endedAtEq).EndedAtNe(endedAtNe).EndedAtGt(endedAtGt).EndedAtGte(endedAtGte).EndedAtLt(endedAtLt).EndedAtLte(endedAtLte).EndedAtIn(endedAtIn).EndedAtNotin(endedAtNotin).EndedAtAsc(endedAtAsc).EndedAtDesc(endedAtDesc).DurationMsEq(durationMsEq).DurationMsNe(durationMsNe).DurationMsGt(durationMsGt).DurationMsGte(durationMsGte).DurationMsLt(durationMsLt).DurationMsLte(durationMsLte).DurationMsIn(durationMsIn).DurationMsNotin(durationMsNotin).DurationMsAsc(durationMsAsc).DurationMsDesc(durationMsDesc).KindEq(kindEq).KindNe(kindNe).KindIn(kindIn).KindNotin(kindNotin).KindLike(kindLike).KindNotlike(kindNotlike).KindIlike(kindIlike).KindNotilike(kindNotilike).KindAsc(kindAsc).KindDesc(kindDesc).RmsDbfsEq(rmsDbfsEq).RmsDbfsNe(rmsDbfsNe).RmsDbfsGt(rmsDbfsGt).RmsDbfsGte(rmsDbfsGte).RmsDbfsLt(rmsDbfsLt).RmsDbfsLte(rmsDbfsLte).RmsDbfsIn(rmsDbfsIn).RmsDbfsNotin(rmsDbfsNotin).RmsDbfsAsc(rmsDbfsAsc).RmsDbfsDesc(rmsDbfsDesc).PeakDbfsEq(peakDbfsEq).PeakDbfsNe(peakDbfsNe).PeakDbfsGt(peakDbfsGt).PeakDbfsGte(peakDbfsGte).PeakDbfsLt(peakDbfsLt).PeakDbfsLte(peakDbfsLte).PeakDbfsIn(peakDbfsIn).PeakDbfsNotin(peakDbfsNotin).PeakDbfsAsc(peakDbfsAsc).PeakDbfsDesc(peakDbfsDesc).FloorDbfsEq(floorDbfsEq).FloorDbfsNe(floorDbfsNe).FloorDbfsGt(floorDbfsGt).FloorDbfsGte(floorDbfsGte).FloorDbfsLt(floorDbfsLt).FloorDbfsLte(floorDbfsLte).FloorDbfsIn(floorDbfsIn).FloorDbfsNotin(floorDbfsNotin).FloorDbfsAsc(floorDbfsAsc).FloorDbfsDesc(floorDbfsDesc).ProminenceDbEq(prominenceDbEq).ProminenceDbNe(prominenceDbNe).ProminenceDbGt(prominenceDbGt).ProminenceDbGte(prominenceDbGte).ProminenceDbLt(prominenceDbLt).ProminenceDbLte(prominenceDbLte).ProminenceDbIn(prominenceDbIn).ProminenceDbNotin(prominenceDbNotin).ProminenceDbAsc(prominenceDbAsc).ProminenceDbDesc(prominenceDbDesc).CrestDbEq(crestDbEq).CrestDbNe(crestDbNe).CrestDbGt(crestDbGt).CrestDbGte(crestDbGte).CrestDbLt(crestDbLt).CrestDbLte(crestDbLte).CrestDbIn(crestDbIn).CrestDbNotin(crestDbNotin).CrestDbAsc(crestDbAsc).CrestDbDesc(crestDbDesc).ScoreEq(scoreEq).ScoreNe(scoreNe).ScoreGt(scoreGt).ScoreGte(scoreGte).ScoreLt(scoreLt).ScoreLte(scoreLte).ScoreIn(scoreIn).ScoreNotin(scoreNotin).ScoreAsc(scoreAsc).ScoreDesc(scoreDesc).SpectralContains(spectralContains).SpectralNotcontains(spectralNotcontains).SpectralAsc(spectralAsc).SpectralDesc(spectralDesc).Execute()



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
	offsetMsEq := int32(56) // int32 | SQL = comparison (optional)
	offsetMsNe := int32(56) // int32 | SQL != comparison (optional)
	offsetMsGt := int32(56) // int32 | SQL > comparison, may not work with all column types (optional)
	offsetMsGte := int32(56) // int32 | SQL >= comparison, may not work with all column types (optional)
	offsetMsLt := int32(56) // int32 | SQL < comparison, may not work with all column types (optional)
	offsetMsLte := int32(56) // int32 | SQL <= comparison, may not work with all column types (optional)
	offsetMsIn := int32(56) // int32 | SQL IN comparison, permits comma-separated values (optional)
	offsetMsNotin := int32(56) // int32 | SQL NOT IN comparison, permits comma-separated values (optional)
	offsetMsAsc := "offsetMsAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	offsetMsDesc := "offsetMsDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
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
	durationMsEq := int32(56) // int32 | SQL = comparison (optional)
	durationMsNe := int32(56) // int32 | SQL != comparison (optional)
	durationMsGt := int32(56) // int32 | SQL > comparison, may not work with all column types (optional)
	durationMsGte := int32(56) // int32 | SQL >= comparison, may not work with all column types (optional)
	durationMsLt := int32(56) // int32 | SQL < comparison, may not work with all column types (optional)
	durationMsLte := int32(56) // int32 | SQL <= comparison, may not work with all column types (optional)
	durationMsIn := int32(56) // int32 | SQL IN comparison, permits comma-separated values (optional)
	durationMsNotin := int32(56) // int32 | SQL NOT IN comparison, permits comma-separated values (optional)
	durationMsAsc := "durationMsAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	durationMsDesc := "durationMsDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	kindEq := "kindEq_example" // string | SQL = comparison (optional)
	kindNe := "kindNe_example" // string | SQL != comparison (optional)
	kindIn := "kindIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	kindNotin := "kindNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	kindLike := "kindLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	kindNotlike := "kindNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	kindIlike := "kindIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	kindNotilike := "kindNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	kindAsc := "kindAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	kindDesc := "kindDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	rmsDbfsEq := float32(8.14) // float32 | SQL = comparison (optional)
	rmsDbfsNe := float32(8.14) // float32 | SQL != comparison (optional)
	rmsDbfsGt := float32(8.14) // float32 | SQL > comparison, may not work with all column types (optional)
	rmsDbfsGte := float32(8.14) // float32 | SQL >= comparison, may not work with all column types (optional)
	rmsDbfsLt := float32(8.14) // float32 | SQL < comparison, may not work with all column types (optional)
	rmsDbfsLte := float32(8.14) // float32 | SQL <= comparison, may not work with all column types (optional)
	rmsDbfsIn := float32(8.14) // float32 | SQL IN comparison, permits comma-separated values (optional)
	rmsDbfsNotin := float32(8.14) // float32 | SQL NOT IN comparison, permits comma-separated values (optional)
	rmsDbfsAsc := "rmsDbfsAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	rmsDbfsDesc := "rmsDbfsDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	peakDbfsEq := float32(8.14) // float32 | SQL = comparison (optional)
	peakDbfsNe := float32(8.14) // float32 | SQL != comparison (optional)
	peakDbfsGt := float32(8.14) // float32 | SQL > comparison, may not work with all column types (optional)
	peakDbfsGte := float32(8.14) // float32 | SQL >= comparison, may not work with all column types (optional)
	peakDbfsLt := float32(8.14) // float32 | SQL < comparison, may not work with all column types (optional)
	peakDbfsLte := float32(8.14) // float32 | SQL <= comparison, may not work with all column types (optional)
	peakDbfsIn := float32(8.14) // float32 | SQL IN comparison, permits comma-separated values (optional)
	peakDbfsNotin := float32(8.14) // float32 | SQL NOT IN comparison, permits comma-separated values (optional)
	peakDbfsAsc := "peakDbfsAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	peakDbfsDesc := "peakDbfsDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	floorDbfsEq := float32(8.14) // float32 | SQL = comparison (optional)
	floorDbfsNe := float32(8.14) // float32 | SQL != comparison (optional)
	floorDbfsGt := float32(8.14) // float32 | SQL > comparison, may not work with all column types (optional)
	floorDbfsGte := float32(8.14) // float32 | SQL >= comparison, may not work with all column types (optional)
	floorDbfsLt := float32(8.14) // float32 | SQL < comparison, may not work with all column types (optional)
	floorDbfsLte := float32(8.14) // float32 | SQL <= comparison, may not work with all column types (optional)
	floorDbfsIn := float32(8.14) // float32 | SQL IN comparison, permits comma-separated values (optional)
	floorDbfsNotin := float32(8.14) // float32 | SQL NOT IN comparison, permits comma-separated values (optional)
	floorDbfsAsc := "floorDbfsAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	floorDbfsDesc := "floorDbfsDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	prominenceDbEq := float32(8.14) // float32 | SQL = comparison (optional)
	prominenceDbNe := float32(8.14) // float32 | SQL != comparison (optional)
	prominenceDbGt := float32(8.14) // float32 | SQL > comparison, may not work with all column types (optional)
	prominenceDbGte := float32(8.14) // float32 | SQL >= comparison, may not work with all column types (optional)
	prominenceDbLt := float32(8.14) // float32 | SQL < comparison, may not work with all column types (optional)
	prominenceDbLte := float32(8.14) // float32 | SQL <= comparison, may not work with all column types (optional)
	prominenceDbIn := float32(8.14) // float32 | SQL IN comparison, permits comma-separated values (optional)
	prominenceDbNotin := float32(8.14) // float32 | SQL NOT IN comparison, permits comma-separated values (optional)
	prominenceDbAsc := "prominenceDbAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	prominenceDbDesc := "prominenceDbDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	crestDbEq := float32(8.14) // float32 | SQL = comparison (optional)
	crestDbNe := float32(8.14) // float32 | SQL != comparison (optional)
	crestDbGt := float32(8.14) // float32 | SQL > comparison, may not work with all column types (optional)
	crestDbGte := float32(8.14) // float32 | SQL >= comparison, may not work with all column types (optional)
	crestDbLt := float32(8.14) // float32 | SQL < comparison, may not work with all column types (optional)
	crestDbLte := float32(8.14) // float32 | SQL <= comparison, may not work with all column types (optional)
	crestDbIn := float32(8.14) // float32 | SQL IN comparison, permits comma-separated values (optional)
	crestDbNotin := float32(8.14) // float32 | SQL NOT IN comparison, permits comma-separated values (optional)
	crestDbAsc := "crestDbAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	crestDbDesc := "crestDbDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
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
	spectralContains := TODO // interface{} | SQL @> comparison (optional)
	spectralNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	spectralAsc := "spectralAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	spectralDesc := "spectralDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.AudioEventAPI.GetAudioEvents(context.Background()).Limit(limit).Offset(offset).Depth(depth).CameraLoad(cameraLoad).VideoLoad(videoLoad).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtAsc(createdAtAsc).CreatedAtDesc(createdAtDesc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtAsc(updatedAtAsc).UpdatedAtDesc(updatedAtDesc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtIsnull(deletedAtIsnull).DeletedAtIsnotnull(deletedAtIsnotnull).DeletedAtAsc(deletedAtAsc).DeletedAtDesc(deletedAtDesc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdAsc(cameraIdAsc).CameraIdDesc(cameraIdDesc).VideoIdEq(videoIdEq).VideoIdNe(videoIdNe).VideoIdGt(videoIdGt).VideoIdGte(videoIdGte).VideoIdLt(videoIdLt).VideoIdLte(videoIdLte).VideoIdIn(videoIdIn).VideoIdNotin(videoIdNotin).VideoIdAsc(videoIdAsc).VideoIdDesc(videoIdDesc).SeenAtEq(seenAtEq).SeenAtNe(seenAtNe).SeenAtGt(seenAtGt).SeenAtGte(seenAtGte).SeenAtLt(seenAtLt).SeenAtLte(seenAtLte).SeenAtIn(seenAtIn).SeenAtNotin(seenAtNotin).SeenAtAsc(seenAtAsc).SeenAtDesc(seenAtDesc).OffsetMsEq(offsetMsEq).OffsetMsNe(offsetMsNe).OffsetMsGt(offsetMsGt).OffsetMsGte(offsetMsGte).OffsetMsLt(offsetMsLt).OffsetMsLte(offsetMsLte).OffsetMsIn(offsetMsIn).OffsetMsNotin(offsetMsNotin).OffsetMsAsc(offsetMsAsc).OffsetMsDesc(offsetMsDesc).EndedAtEq(endedAtEq).EndedAtNe(endedAtNe).EndedAtGt(endedAtGt).EndedAtGte(endedAtGte).EndedAtLt(endedAtLt).EndedAtLte(endedAtLte).EndedAtIn(endedAtIn).EndedAtNotin(endedAtNotin).EndedAtAsc(endedAtAsc).EndedAtDesc(endedAtDesc).DurationMsEq(durationMsEq).DurationMsNe(durationMsNe).DurationMsGt(durationMsGt).DurationMsGte(durationMsGte).DurationMsLt(durationMsLt).DurationMsLte(durationMsLte).DurationMsIn(durationMsIn).DurationMsNotin(durationMsNotin).DurationMsAsc(durationMsAsc).DurationMsDesc(durationMsDesc).KindEq(kindEq).KindNe(kindNe).KindIn(kindIn).KindNotin(kindNotin).KindLike(kindLike).KindNotlike(kindNotlike).KindIlike(kindIlike).KindNotilike(kindNotilike).KindAsc(kindAsc).KindDesc(kindDesc).RmsDbfsEq(rmsDbfsEq).RmsDbfsNe(rmsDbfsNe).RmsDbfsGt(rmsDbfsGt).RmsDbfsGte(rmsDbfsGte).RmsDbfsLt(rmsDbfsLt).RmsDbfsLte(rmsDbfsLte).RmsDbfsIn(rmsDbfsIn).RmsDbfsNotin(rmsDbfsNotin).RmsDbfsAsc(rmsDbfsAsc).RmsDbfsDesc(rmsDbfsDesc).PeakDbfsEq(peakDbfsEq).PeakDbfsNe(peakDbfsNe).PeakDbfsGt(peakDbfsGt).PeakDbfsGte(peakDbfsGte).PeakDbfsLt(peakDbfsLt).PeakDbfsLte(peakDbfsLte).PeakDbfsIn(peakDbfsIn).PeakDbfsNotin(peakDbfsNotin).PeakDbfsAsc(peakDbfsAsc).PeakDbfsDesc(peakDbfsDesc).FloorDbfsEq(floorDbfsEq).FloorDbfsNe(floorDbfsNe).FloorDbfsGt(floorDbfsGt).FloorDbfsGte(floorDbfsGte).FloorDbfsLt(floorDbfsLt).FloorDbfsLte(floorDbfsLte).FloorDbfsIn(floorDbfsIn).FloorDbfsNotin(floorDbfsNotin).FloorDbfsAsc(floorDbfsAsc).FloorDbfsDesc(floorDbfsDesc).ProminenceDbEq(prominenceDbEq).ProminenceDbNe(prominenceDbNe).ProminenceDbGt(prominenceDbGt).ProminenceDbGte(prominenceDbGte).ProminenceDbLt(prominenceDbLt).ProminenceDbLte(prominenceDbLte).ProminenceDbIn(prominenceDbIn).ProminenceDbNotin(prominenceDbNotin).ProminenceDbAsc(prominenceDbAsc).ProminenceDbDesc(prominenceDbDesc).CrestDbEq(crestDbEq).CrestDbNe(crestDbNe).CrestDbGt(crestDbGt).CrestDbGte(crestDbGte).CrestDbLt(crestDbLt).CrestDbLte(crestDbLte).CrestDbIn(crestDbIn).CrestDbNotin(crestDbNotin).CrestDbAsc(crestDbAsc).CrestDbDesc(crestDbDesc).ScoreEq(scoreEq).ScoreNe(scoreNe).ScoreGt(scoreGt).ScoreGte(scoreGte).ScoreLt(scoreLt).ScoreLte(scoreLte).ScoreIn(scoreIn).ScoreNotin(scoreNotin).ScoreAsc(scoreAsc).ScoreDesc(scoreDesc).SpectralContains(spectralContains).SpectralNotcontains(spectralNotcontains).SpectralAsc(spectralAsc).SpectralDesc(spectralDesc).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AudioEventAPI.GetAudioEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetAudioEvents`: ResponseWithGenericOfAudioEvent
	fmt.Fprintf(os.Stdout, "Response from `AudioEventAPI.GetAudioEvents`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiGetAudioEventsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int32** | SQL LIMIT operator | 
 **offset** | **int32** | SQL OFFSET operator | 
 **depth** | **int32** | Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | 
 **cameraLoad** | **string** | load the given outbound relationship, value is ignored (presence of key is sufficient) | 
 **videoLoad** | **string** | load the given outbound relationship, value is ignored (presence of key is sufficient) | 
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
 **offsetMsEq** | **int32** | SQL &#x3D; comparison | 
 **offsetMsNe** | **int32** | SQL !&#x3D; comparison | 
 **offsetMsGt** | **int32** | SQL &gt; comparison, may not work with all column types | 
 **offsetMsGte** | **int32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **offsetMsLt** | **int32** | SQL &lt; comparison, may not work with all column types | 
 **offsetMsLte** | **int32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **offsetMsIn** | **int32** | SQL IN comparison, permits comma-separated values | 
 **offsetMsNotin** | **int32** | SQL NOT IN comparison, permits comma-separated values | 
 **offsetMsAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **offsetMsDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
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
 **durationMsEq** | **int32** | SQL &#x3D; comparison | 
 **durationMsNe** | **int32** | SQL !&#x3D; comparison | 
 **durationMsGt** | **int32** | SQL &gt; comparison, may not work with all column types | 
 **durationMsGte** | **int32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **durationMsLt** | **int32** | SQL &lt; comparison, may not work with all column types | 
 **durationMsLte** | **int32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **durationMsIn** | **int32** | SQL IN comparison, permits comma-separated values | 
 **durationMsNotin** | **int32** | SQL NOT IN comparison, permits comma-separated values | 
 **durationMsAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **durationMsDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **kindEq** | **string** | SQL &#x3D; comparison | 
 **kindNe** | **string** | SQL !&#x3D; comparison | 
 **kindIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **kindNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **kindLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **kindNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **kindIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **kindNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **kindAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **kindDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **rmsDbfsEq** | **float32** | SQL &#x3D; comparison | 
 **rmsDbfsNe** | **float32** | SQL !&#x3D; comparison | 
 **rmsDbfsGt** | **float32** | SQL &gt; comparison, may not work with all column types | 
 **rmsDbfsGte** | **float32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **rmsDbfsLt** | **float32** | SQL &lt; comparison, may not work with all column types | 
 **rmsDbfsLte** | **float32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **rmsDbfsIn** | **float32** | SQL IN comparison, permits comma-separated values | 
 **rmsDbfsNotin** | **float32** | SQL NOT IN comparison, permits comma-separated values | 
 **rmsDbfsAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **rmsDbfsDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **peakDbfsEq** | **float32** | SQL &#x3D; comparison | 
 **peakDbfsNe** | **float32** | SQL !&#x3D; comparison | 
 **peakDbfsGt** | **float32** | SQL &gt; comparison, may not work with all column types | 
 **peakDbfsGte** | **float32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **peakDbfsLt** | **float32** | SQL &lt; comparison, may not work with all column types | 
 **peakDbfsLte** | **float32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **peakDbfsIn** | **float32** | SQL IN comparison, permits comma-separated values | 
 **peakDbfsNotin** | **float32** | SQL NOT IN comparison, permits comma-separated values | 
 **peakDbfsAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **peakDbfsDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **floorDbfsEq** | **float32** | SQL &#x3D; comparison | 
 **floorDbfsNe** | **float32** | SQL !&#x3D; comparison | 
 **floorDbfsGt** | **float32** | SQL &gt; comparison, may not work with all column types | 
 **floorDbfsGte** | **float32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **floorDbfsLt** | **float32** | SQL &lt; comparison, may not work with all column types | 
 **floorDbfsLte** | **float32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **floorDbfsIn** | **float32** | SQL IN comparison, permits comma-separated values | 
 **floorDbfsNotin** | **float32** | SQL NOT IN comparison, permits comma-separated values | 
 **floorDbfsAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **floorDbfsDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **prominenceDbEq** | **float32** | SQL &#x3D; comparison | 
 **prominenceDbNe** | **float32** | SQL !&#x3D; comparison | 
 **prominenceDbGt** | **float32** | SQL &gt; comparison, may not work with all column types | 
 **prominenceDbGte** | **float32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **prominenceDbLt** | **float32** | SQL &lt; comparison, may not work with all column types | 
 **prominenceDbLte** | **float32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **prominenceDbIn** | **float32** | SQL IN comparison, permits comma-separated values | 
 **prominenceDbNotin** | **float32** | SQL NOT IN comparison, permits comma-separated values | 
 **prominenceDbAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **prominenceDbDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **crestDbEq** | **float32** | SQL &#x3D; comparison | 
 **crestDbNe** | **float32** | SQL !&#x3D; comparison | 
 **crestDbGt** | **float32** | SQL &gt; comparison, may not work with all column types | 
 **crestDbGte** | **float32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **crestDbLt** | **float32** | SQL &lt; comparison, may not work with all column types | 
 **crestDbLte** | **float32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **crestDbIn** | **float32** | SQL IN comparison, permits comma-separated values | 
 **crestDbNotin** | **float32** | SQL NOT IN comparison, permits comma-separated values | 
 **crestDbAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **crestDbDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
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
 **spectralContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison | 
 **spectralNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison | 
 **spectralAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **spectralDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 

### Return type

[**ResponseWithGenericOfAudioEvent**](ResponseWithGenericOfAudioEvent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PatchAudioEvent

> ResponseWithGenericOfAudioEvent PatchAudioEvent(ctx, primaryKey).AudioEvent(audioEvent).Depth(depth).Execute()



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
	audioEvent := *openapiclient.NewAudioEvent() // AudioEvent | 
	depth := int64(789) // int64 | Query parameter depth (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.AudioEventAPI.PatchAudioEvent(context.Background(), primaryKey).AudioEvent(audioEvent).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AudioEventAPI.PatchAudioEvent``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PatchAudioEvent`: ResponseWithGenericOfAudioEvent
	fmt.Fprintf(os.Stdout, "Response from `AudioEventAPI.PatchAudioEvent`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**primaryKey** | **string** | Path parameter primaryKey | 

### Other Parameters

Other parameters are passed through a pointer to a apiPatchAudioEventRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **audioEvent** | [**AudioEvent**](AudioEvent.md) |  | 
 **depth** | **int64** | Query parameter depth | 

### Return type

[**ResponseWithGenericOfAudioEvent**](ResponseWithGenericOfAudioEvent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostAudioEvents

> ResponseWithGenericOfAudioEvent PostAudioEvents(ctx).AudioEvent(audioEvent).Depth(depth).Execute()



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
	audioEvent := []openapiclient.AudioEvent{*openapiclient.NewAudioEvent()} // []AudioEvent | 
	depth := int64(789) // int64 | Query parameter depth (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.AudioEventAPI.PostAudioEvents(context.Background()).AudioEvent(audioEvent).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AudioEventAPI.PostAudioEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostAudioEvents`: ResponseWithGenericOfAudioEvent
	fmt.Fprintf(os.Stdout, "Response from `AudioEventAPI.PostAudioEvents`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostAudioEventsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **audioEvent** | [**[]AudioEvent**](AudioEvent.md) |  | 
 **depth** | **int64** | Query parameter depth | 

### Return type

[**ResponseWithGenericOfAudioEvent**](ResponseWithGenericOfAudioEvent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

