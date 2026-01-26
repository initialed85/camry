# \ObjectTrackerClaimVideoAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**PostObjectTrackerClaimVideos**](ObjectTrackerClaimVideoAPI.md#PostObjectTrackerClaimVideos) | **Post** /api/object-tracker-claim-video | 



## PostObjectTrackerClaimVideos

> ResponseWithGenericOfVideo PostObjectTrackerClaimVideos(ctx).VideoObjectTrackerClaimRequest(videoObjectTrackerClaimRequest).Limit(limit).Offset(offset).Depth(depth).CameraLoad(cameraLoad).ReferencedByDetectionLoad(referencedByDetectionLoad).IdEq(idEq).IdNe(idNe).IdGt(idGt).IdGte(idGte).IdLt(idLt).IdLte(idLte).IdIn(idIn).IdNotin(idNotin).IdContains(idContains).IdNotcontains(idNotcontains).IdLike(idLike).IdNotlike(idNotlike).IdIlike(idIlike).IdNotilike(idNotilike).IdDesc(idDesc).IdAsc(idAsc).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtContains(createdAtContains).CreatedAtNotcontains(createdAtNotcontains).CreatedAtLike(createdAtLike).CreatedAtNotlike(createdAtNotlike).CreatedAtIlike(createdAtIlike).CreatedAtNotilike(createdAtNotilike).CreatedAtDesc(createdAtDesc).CreatedAtAsc(createdAtAsc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtContains(updatedAtContains).UpdatedAtNotcontains(updatedAtNotcontains).UpdatedAtLike(updatedAtLike).UpdatedAtNotlike(updatedAtNotlike).UpdatedAtIlike(updatedAtIlike).UpdatedAtNotilike(updatedAtNotilike).UpdatedAtDesc(updatedAtDesc).UpdatedAtAsc(updatedAtAsc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtContains(deletedAtContains).DeletedAtNotcontains(deletedAtNotcontains).DeletedAtLike(deletedAtLike).DeletedAtNotlike(deletedAtNotlike).DeletedAtIlike(deletedAtIlike).DeletedAtNotilike(deletedAtNotilike).DeletedAtDesc(deletedAtDesc).DeletedAtAsc(deletedAtAsc).FileNameEq(fileNameEq).FileNameNe(fileNameNe).FileNameGt(fileNameGt).FileNameGte(fileNameGte).FileNameLt(fileNameLt).FileNameLte(fileNameLte).FileNameIn(fileNameIn).FileNameNotin(fileNameNotin).FileNameContains(fileNameContains).FileNameNotcontains(fileNameNotcontains).FileNameLike(fileNameLike).FileNameNotlike(fileNameNotlike).FileNameIlike(fileNameIlike).FileNameNotilike(fileNameNotilike).FileNameDesc(fileNameDesc).FileNameAsc(fileNameAsc).StartedAtEq(startedAtEq).StartedAtNe(startedAtNe).StartedAtGt(startedAtGt).StartedAtGte(startedAtGte).StartedAtLt(startedAtLt).StartedAtLte(startedAtLte).StartedAtIn(startedAtIn).StartedAtNotin(startedAtNotin).StartedAtContains(startedAtContains).StartedAtNotcontains(startedAtNotcontains).StartedAtLike(startedAtLike).StartedAtNotlike(startedAtNotlike).StartedAtIlike(startedAtIlike).StartedAtNotilike(startedAtNotilike).StartedAtDesc(startedAtDesc).StartedAtAsc(startedAtAsc).EndedAtEq(endedAtEq).EndedAtNe(endedAtNe).EndedAtGt(endedAtGt).EndedAtGte(endedAtGte).EndedAtLt(endedAtLt).EndedAtLte(endedAtLte).EndedAtIn(endedAtIn).EndedAtNotin(endedAtNotin).EndedAtContains(endedAtContains).EndedAtNotcontains(endedAtNotcontains).EndedAtLike(endedAtLike).EndedAtNotlike(endedAtNotlike).EndedAtIlike(endedAtIlike).EndedAtNotilike(endedAtNotilike).EndedAtDesc(endedAtDesc).EndedAtAsc(endedAtAsc).DurationEq(durationEq).DurationNe(durationNe).DurationGt(durationGt).DurationGte(durationGte).DurationLt(durationLt).DurationLte(durationLte).DurationIn(durationIn).DurationNotin(durationNotin).DurationContains(durationContains).DurationNotcontains(durationNotcontains).DurationDesc(durationDesc).DurationAsc(durationAsc).FileSizeEq(fileSizeEq).FileSizeNe(fileSizeNe).FileSizeGt(fileSizeGt).FileSizeGte(fileSizeGte).FileSizeLt(fileSizeLt).FileSizeLte(fileSizeLte).FileSizeIn(fileSizeIn).FileSizeNotin(fileSizeNotin).FileSizeContains(fileSizeContains).FileSizeNotcontains(fileSizeNotcontains).FileSizeDesc(fileSizeDesc).FileSizeAsc(fileSizeAsc).ThumbnailNameEq(thumbnailNameEq).ThumbnailNameNe(thumbnailNameNe).ThumbnailNameGt(thumbnailNameGt).ThumbnailNameGte(thumbnailNameGte).ThumbnailNameLt(thumbnailNameLt).ThumbnailNameLte(thumbnailNameLte).ThumbnailNameIn(thumbnailNameIn).ThumbnailNameNotin(thumbnailNameNotin).ThumbnailNameContains(thumbnailNameContains).ThumbnailNameNotcontains(thumbnailNameNotcontains).ThumbnailNameLike(thumbnailNameLike).ThumbnailNameNotlike(thumbnailNameNotlike).ThumbnailNameIlike(thumbnailNameIlike).ThumbnailNameNotilike(thumbnailNameNotilike).ThumbnailNameDesc(thumbnailNameDesc).ThumbnailNameAsc(thumbnailNameAsc).StatusEq(statusEq).StatusNe(statusNe).StatusGt(statusGt).StatusGte(statusGte).StatusLt(statusLt).StatusLte(statusLte).StatusIn(statusIn).StatusNotin(statusNotin).StatusContains(statusContains).StatusNotcontains(statusNotcontains).StatusLike(statusLike).StatusNotlike(statusNotlike).StatusIlike(statusIlike).StatusNotilike(statusNotilike).StatusDesc(statusDesc).StatusAsc(statusAsc).ObjectDetectorClaimedUntilEq(objectDetectorClaimedUntilEq).ObjectDetectorClaimedUntilNe(objectDetectorClaimedUntilNe).ObjectDetectorClaimedUntilGt(objectDetectorClaimedUntilGt).ObjectDetectorClaimedUntilGte(objectDetectorClaimedUntilGte).ObjectDetectorClaimedUntilLt(objectDetectorClaimedUntilLt).ObjectDetectorClaimedUntilLte(objectDetectorClaimedUntilLte).ObjectDetectorClaimedUntilIn(objectDetectorClaimedUntilIn).ObjectDetectorClaimedUntilNotin(objectDetectorClaimedUntilNotin).ObjectDetectorClaimedUntilContains(objectDetectorClaimedUntilContains).ObjectDetectorClaimedUntilNotcontains(objectDetectorClaimedUntilNotcontains).ObjectDetectorClaimedUntilLike(objectDetectorClaimedUntilLike).ObjectDetectorClaimedUntilNotlike(objectDetectorClaimedUntilNotlike).ObjectDetectorClaimedUntilIlike(objectDetectorClaimedUntilIlike).ObjectDetectorClaimedUntilNotilike(objectDetectorClaimedUntilNotilike).ObjectDetectorClaimedUntilDesc(objectDetectorClaimedUntilDesc).ObjectDetectorClaimedUntilAsc(objectDetectorClaimedUntilAsc).ObjectTrackerClaimedUntilEq(objectTrackerClaimedUntilEq).ObjectTrackerClaimedUntilNe(objectTrackerClaimedUntilNe).ObjectTrackerClaimedUntilGt(objectTrackerClaimedUntilGt).ObjectTrackerClaimedUntilGte(objectTrackerClaimedUntilGte).ObjectTrackerClaimedUntilLt(objectTrackerClaimedUntilLt).ObjectTrackerClaimedUntilLte(objectTrackerClaimedUntilLte).ObjectTrackerClaimedUntilIn(objectTrackerClaimedUntilIn).ObjectTrackerClaimedUntilNotin(objectTrackerClaimedUntilNotin).ObjectTrackerClaimedUntilContains(objectTrackerClaimedUntilContains).ObjectTrackerClaimedUntilNotcontains(objectTrackerClaimedUntilNotcontains).ObjectTrackerClaimedUntilLike(objectTrackerClaimedUntilLike).ObjectTrackerClaimedUntilNotlike(objectTrackerClaimedUntilNotlike).ObjectTrackerClaimedUntilIlike(objectTrackerClaimedUntilIlike).ObjectTrackerClaimedUntilNotilike(objectTrackerClaimedUntilNotilike).ObjectTrackerClaimedUntilDesc(objectTrackerClaimedUntilDesc).ObjectTrackerClaimedUntilAsc(objectTrackerClaimedUntilAsc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdContains(cameraIdContains).CameraIdNotcontains(cameraIdNotcontains).CameraIdLike(cameraIdLike).CameraIdNotlike(cameraIdNotlike).CameraIdIlike(cameraIdIlike).CameraIdNotilike(cameraIdNotilike).CameraIdDesc(cameraIdDesc).CameraIdAsc(cameraIdAsc).CameraIdObjectContains(cameraIdObjectContains).CameraIdObjectNotcontains(cameraIdObjectNotcontains).CameraIdObjectDesc(cameraIdObjectDesc).CameraIdObjectAsc(cameraIdObjectAsc).DetectionSummaryContains(detectionSummaryContains).DetectionSummaryNotcontains(detectionSummaryNotcontains).DetectionSummaryDesc(detectionSummaryDesc).DetectionSummaryAsc(detectionSummaryAsc).ReferencedByDetectionVideoIdObjectsContains(referencedByDetectionVideoIdObjectsContains).ReferencedByDetectionVideoIdObjectsNotcontains(referencedByDetectionVideoIdObjectsNotcontains).ReferencedByDetectionVideoIdObjectsDesc(referencedByDetectionVideoIdObjectsDesc).ReferencedByDetectionVideoIdObjectsAsc(referencedByDetectionVideoIdObjectsAsc).Execute()



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
	videoObjectTrackerClaimRequest := *openapiclient.NewVideoObjectTrackerClaimRequest() // VideoObjectTrackerClaimRequest | 
	limit := int32(56) // int32 | SQL LIMIT operator (optional)
	offset := int32(56) // int32 | SQL OFFSET operator (optional)
	depth := int32(56) // int32 | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)
	cameraLoad := "cameraLoad_example" // string | load the given directly related object, value is ignored (presence of key is sufficient) (optional)
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
	fileNameEq := "fileNameEq_example" // string | SQL = comparison (optional)
	fileNameNe := "fileNameNe_example" // string | SQL != comparison (optional)
	fileNameGt := "fileNameGt_example" // string | SQL > comparison, may not work with all column types (optional)
	fileNameGte := "fileNameGte_example" // string | SQL >= comparison, may not work with all column types (optional)
	fileNameLt := "fileNameLt_example" // string | SQL < comparison, may not work with all column types (optional)
	fileNameLte := "fileNameLte_example" // string | SQL <= comparison, may not work with all column types (optional)
	fileNameIn := "fileNameIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	fileNameNotin := "fileNameNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	fileNameContains := "fileNameContains_example" // string | SQL @> comparison (optional)
	fileNameNotcontains := "fileNameNotcontains_example" // string | SQL NOT @> comparison (optional)
	fileNameLike := "fileNameLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	fileNameNotlike := "fileNameNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	fileNameIlike := "fileNameIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	fileNameNotilike := "fileNameNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	fileNameDesc := "fileNameDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	fileNameAsc := "fileNameAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
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
	durationEq := int64(789) // int64 | SQL = comparison (optional)
	durationNe := int64(789) // int64 | SQL != comparison (optional)
	durationGt := int64(789) // int64 | SQL > comparison, may not work with all column types (optional)
	durationGte := int64(789) // int64 | SQL >= comparison, may not work with all column types (optional)
	durationLt := int64(789) // int64 | SQL < comparison, may not work with all column types (optional)
	durationLte := int64(789) // int64 | SQL <= comparison, may not work with all column types (optional)
	durationIn := int64(789) // int64 | SQL IN comparison, permits comma-separated values (optional)
	durationNotin := int64(789) // int64 | SQL NOT IN comparison, permits comma-separated values (optional)
	durationContains := int64(789) // int64 | SQL @> comparison (optional)
	durationNotcontains := int64(789) // int64 | SQL NOT @> comparison (optional)
	durationDesc := "durationDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	durationAsc := "durationAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	fileSizeEq := float64(1.2) // float64 | SQL = comparison (optional)
	fileSizeNe := float64(1.2) // float64 | SQL != comparison (optional)
	fileSizeGt := float64(1.2) // float64 | SQL > comparison, may not work with all column types (optional)
	fileSizeGte := float64(1.2) // float64 | SQL >= comparison, may not work with all column types (optional)
	fileSizeLt := float64(1.2) // float64 | SQL < comparison, may not work with all column types (optional)
	fileSizeLte := float64(1.2) // float64 | SQL <= comparison, may not work with all column types (optional)
	fileSizeIn := float64(1.2) // float64 | SQL IN comparison, permits comma-separated values (optional)
	fileSizeNotin := float64(1.2) // float64 | SQL NOT IN comparison, permits comma-separated values (optional)
	fileSizeContains := float64(1.2) // float64 | SQL @> comparison (optional)
	fileSizeNotcontains := float64(1.2) // float64 | SQL NOT @> comparison (optional)
	fileSizeDesc := "fileSizeDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	fileSizeAsc := "fileSizeAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	thumbnailNameEq := "thumbnailNameEq_example" // string | SQL = comparison (optional)
	thumbnailNameNe := "thumbnailNameNe_example" // string | SQL != comparison (optional)
	thumbnailNameGt := "thumbnailNameGt_example" // string | SQL > comparison, may not work with all column types (optional)
	thumbnailNameGte := "thumbnailNameGte_example" // string | SQL >= comparison, may not work with all column types (optional)
	thumbnailNameLt := "thumbnailNameLt_example" // string | SQL < comparison, may not work with all column types (optional)
	thumbnailNameLte := "thumbnailNameLte_example" // string | SQL <= comparison, may not work with all column types (optional)
	thumbnailNameIn := "thumbnailNameIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	thumbnailNameNotin := "thumbnailNameNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	thumbnailNameContains := "thumbnailNameContains_example" // string | SQL @> comparison (optional)
	thumbnailNameNotcontains := "thumbnailNameNotcontains_example" // string | SQL NOT @> comparison (optional)
	thumbnailNameLike := "thumbnailNameLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	thumbnailNameNotlike := "thumbnailNameNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	thumbnailNameIlike := "thumbnailNameIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	thumbnailNameNotilike := "thumbnailNameNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	thumbnailNameDesc := "thumbnailNameDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	thumbnailNameAsc := "thumbnailNameAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
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
	objectDetectorClaimedUntilEq := time.Now() // time.Time | SQL = comparison (optional)
	objectDetectorClaimedUntilNe := time.Now() // time.Time | SQL != comparison (optional)
	objectDetectorClaimedUntilGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	objectDetectorClaimedUntilGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	objectDetectorClaimedUntilLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	objectDetectorClaimedUntilLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	objectDetectorClaimedUntilIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	objectDetectorClaimedUntilNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	objectDetectorClaimedUntilContains := time.Now() // time.Time | SQL @> comparison (optional)
	objectDetectorClaimedUntilNotcontains := time.Now() // time.Time | SQL NOT @> comparison (optional)
	objectDetectorClaimedUntilLike := time.Now() // time.Time | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	objectDetectorClaimedUntilNotlike := time.Now() // time.Time | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	objectDetectorClaimedUntilIlike := time.Now() // time.Time | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	objectDetectorClaimedUntilNotilike := time.Now() // time.Time | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	objectDetectorClaimedUntilDesc := "objectDetectorClaimedUntilDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	objectDetectorClaimedUntilAsc := "objectDetectorClaimedUntilAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	objectTrackerClaimedUntilEq := time.Now() // time.Time | SQL = comparison (optional)
	objectTrackerClaimedUntilNe := time.Now() // time.Time | SQL != comparison (optional)
	objectTrackerClaimedUntilGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	objectTrackerClaimedUntilGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	objectTrackerClaimedUntilLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	objectTrackerClaimedUntilLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	objectTrackerClaimedUntilIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	objectTrackerClaimedUntilNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	objectTrackerClaimedUntilContains := time.Now() // time.Time | SQL @> comparison (optional)
	objectTrackerClaimedUntilNotcontains := time.Now() // time.Time | SQL NOT @> comparison (optional)
	objectTrackerClaimedUntilLike := time.Now() // time.Time | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	objectTrackerClaimedUntilNotlike := time.Now() // time.Time | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	objectTrackerClaimedUntilIlike := time.Now() // time.Time | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	objectTrackerClaimedUntilNotilike := time.Now() // time.Time | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	objectTrackerClaimedUntilDesc := "objectTrackerClaimedUntilDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	objectTrackerClaimedUntilAsc := "objectTrackerClaimedUntilAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
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
	detectionSummaryContains := TODO // interface{} | SQL @> comparison (optional)
	detectionSummaryNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	detectionSummaryDesc := "detectionSummaryDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	detectionSummaryAsc := "detectionSummaryAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	referencedByDetectionVideoIdObjectsContains := TODO // interface{} | SQL @> comparison (optional)
	referencedByDetectionVideoIdObjectsNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	referencedByDetectionVideoIdObjectsDesc := "referencedByDetectionVideoIdObjectsDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	referencedByDetectionVideoIdObjectsAsc := "referencedByDetectionVideoIdObjectsAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ObjectTrackerClaimVideoAPI.PostObjectTrackerClaimVideos(context.Background()).VideoObjectTrackerClaimRequest(videoObjectTrackerClaimRequest).Limit(limit).Offset(offset).Depth(depth).CameraLoad(cameraLoad).ReferencedByDetectionLoad(referencedByDetectionLoad).IdEq(idEq).IdNe(idNe).IdGt(idGt).IdGte(idGte).IdLt(idLt).IdLte(idLte).IdIn(idIn).IdNotin(idNotin).IdContains(idContains).IdNotcontains(idNotcontains).IdLike(idLike).IdNotlike(idNotlike).IdIlike(idIlike).IdNotilike(idNotilike).IdDesc(idDesc).IdAsc(idAsc).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtContains(createdAtContains).CreatedAtNotcontains(createdAtNotcontains).CreatedAtLike(createdAtLike).CreatedAtNotlike(createdAtNotlike).CreatedAtIlike(createdAtIlike).CreatedAtNotilike(createdAtNotilike).CreatedAtDesc(createdAtDesc).CreatedAtAsc(createdAtAsc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtContains(updatedAtContains).UpdatedAtNotcontains(updatedAtNotcontains).UpdatedAtLike(updatedAtLike).UpdatedAtNotlike(updatedAtNotlike).UpdatedAtIlike(updatedAtIlike).UpdatedAtNotilike(updatedAtNotilike).UpdatedAtDesc(updatedAtDesc).UpdatedAtAsc(updatedAtAsc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtContains(deletedAtContains).DeletedAtNotcontains(deletedAtNotcontains).DeletedAtLike(deletedAtLike).DeletedAtNotlike(deletedAtNotlike).DeletedAtIlike(deletedAtIlike).DeletedAtNotilike(deletedAtNotilike).DeletedAtDesc(deletedAtDesc).DeletedAtAsc(deletedAtAsc).FileNameEq(fileNameEq).FileNameNe(fileNameNe).FileNameGt(fileNameGt).FileNameGte(fileNameGte).FileNameLt(fileNameLt).FileNameLte(fileNameLte).FileNameIn(fileNameIn).FileNameNotin(fileNameNotin).FileNameContains(fileNameContains).FileNameNotcontains(fileNameNotcontains).FileNameLike(fileNameLike).FileNameNotlike(fileNameNotlike).FileNameIlike(fileNameIlike).FileNameNotilike(fileNameNotilike).FileNameDesc(fileNameDesc).FileNameAsc(fileNameAsc).StartedAtEq(startedAtEq).StartedAtNe(startedAtNe).StartedAtGt(startedAtGt).StartedAtGte(startedAtGte).StartedAtLt(startedAtLt).StartedAtLte(startedAtLte).StartedAtIn(startedAtIn).StartedAtNotin(startedAtNotin).StartedAtContains(startedAtContains).StartedAtNotcontains(startedAtNotcontains).StartedAtLike(startedAtLike).StartedAtNotlike(startedAtNotlike).StartedAtIlike(startedAtIlike).StartedAtNotilike(startedAtNotilike).StartedAtDesc(startedAtDesc).StartedAtAsc(startedAtAsc).EndedAtEq(endedAtEq).EndedAtNe(endedAtNe).EndedAtGt(endedAtGt).EndedAtGte(endedAtGte).EndedAtLt(endedAtLt).EndedAtLte(endedAtLte).EndedAtIn(endedAtIn).EndedAtNotin(endedAtNotin).EndedAtContains(endedAtContains).EndedAtNotcontains(endedAtNotcontains).EndedAtLike(endedAtLike).EndedAtNotlike(endedAtNotlike).EndedAtIlike(endedAtIlike).EndedAtNotilike(endedAtNotilike).EndedAtDesc(endedAtDesc).EndedAtAsc(endedAtAsc).DurationEq(durationEq).DurationNe(durationNe).DurationGt(durationGt).DurationGte(durationGte).DurationLt(durationLt).DurationLte(durationLte).DurationIn(durationIn).DurationNotin(durationNotin).DurationContains(durationContains).DurationNotcontains(durationNotcontains).DurationDesc(durationDesc).DurationAsc(durationAsc).FileSizeEq(fileSizeEq).FileSizeNe(fileSizeNe).FileSizeGt(fileSizeGt).FileSizeGte(fileSizeGte).FileSizeLt(fileSizeLt).FileSizeLte(fileSizeLte).FileSizeIn(fileSizeIn).FileSizeNotin(fileSizeNotin).FileSizeContains(fileSizeContains).FileSizeNotcontains(fileSizeNotcontains).FileSizeDesc(fileSizeDesc).FileSizeAsc(fileSizeAsc).ThumbnailNameEq(thumbnailNameEq).ThumbnailNameNe(thumbnailNameNe).ThumbnailNameGt(thumbnailNameGt).ThumbnailNameGte(thumbnailNameGte).ThumbnailNameLt(thumbnailNameLt).ThumbnailNameLte(thumbnailNameLte).ThumbnailNameIn(thumbnailNameIn).ThumbnailNameNotin(thumbnailNameNotin).ThumbnailNameContains(thumbnailNameContains).ThumbnailNameNotcontains(thumbnailNameNotcontains).ThumbnailNameLike(thumbnailNameLike).ThumbnailNameNotlike(thumbnailNameNotlike).ThumbnailNameIlike(thumbnailNameIlike).ThumbnailNameNotilike(thumbnailNameNotilike).ThumbnailNameDesc(thumbnailNameDesc).ThumbnailNameAsc(thumbnailNameAsc).StatusEq(statusEq).StatusNe(statusNe).StatusGt(statusGt).StatusGte(statusGte).StatusLt(statusLt).StatusLte(statusLte).StatusIn(statusIn).StatusNotin(statusNotin).StatusContains(statusContains).StatusNotcontains(statusNotcontains).StatusLike(statusLike).StatusNotlike(statusNotlike).StatusIlike(statusIlike).StatusNotilike(statusNotilike).StatusDesc(statusDesc).StatusAsc(statusAsc).ObjectDetectorClaimedUntilEq(objectDetectorClaimedUntilEq).ObjectDetectorClaimedUntilNe(objectDetectorClaimedUntilNe).ObjectDetectorClaimedUntilGt(objectDetectorClaimedUntilGt).ObjectDetectorClaimedUntilGte(objectDetectorClaimedUntilGte).ObjectDetectorClaimedUntilLt(objectDetectorClaimedUntilLt).ObjectDetectorClaimedUntilLte(objectDetectorClaimedUntilLte).ObjectDetectorClaimedUntilIn(objectDetectorClaimedUntilIn).ObjectDetectorClaimedUntilNotin(objectDetectorClaimedUntilNotin).ObjectDetectorClaimedUntilContains(objectDetectorClaimedUntilContains).ObjectDetectorClaimedUntilNotcontains(objectDetectorClaimedUntilNotcontains).ObjectDetectorClaimedUntilLike(objectDetectorClaimedUntilLike).ObjectDetectorClaimedUntilNotlike(objectDetectorClaimedUntilNotlike).ObjectDetectorClaimedUntilIlike(objectDetectorClaimedUntilIlike).ObjectDetectorClaimedUntilNotilike(objectDetectorClaimedUntilNotilike).ObjectDetectorClaimedUntilDesc(objectDetectorClaimedUntilDesc).ObjectDetectorClaimedUntilAsc(objectDetectorClaimedUntilAsc).ObjectTrackerClaimedUntilEq(objectTrackerClaimedUntilEq).ObjectTrackerClaimedUntilNe(objectTrackerClaimedUntilNe).ObjectTrackerClaimedUntilGt(objectTrackerClaimedUntilGt).ObjectTrackerClaimedUntilGte(objectTrackerClaimedUntilGte).ObjectTrackerClaimedUntilLt(objectTrackerClaimedUntilLt).ObjectTrackerClaimedUntilLte(objectTrackerClaimedUntilLte).ObjectTrackerClaimedUntilIn(objectTrackerClaimedUntilIn).ObjectTrackerClaimedUntilNotin(objectTrackerClaimedUntilNotin).ObjectTrackerClaimedUntilContains(objectTrackerClaimedUntilContains).ObjectTrackerClaimedUntilNotcontains(objectTrackerClaimedUntilNotcontains).ObjectTrackerClaimedUntilLike(objectTrackerClaimedUntilLike).ObjectTrackerClaimedUntilNotlike(objectTrackerClaimedUntilNotlike).ObjectTrackerClaimedUntilIlike(objectTrackerClaimedUntilIlike).ObjectTrackerClaimedUntilNotilike(objectTrackerClaimedUntilNotilike).ObjectTrackerClaimedUntilDesc(objectTrackerClaimedUntilDesc).ObjectTrackerClaimedUntilAsc(objectTrackerClaimedUntilAsc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdContains(cameraIdContains).CameraIdNotcontains(cameraIdNotcontains).CameraIdLike(cameraIdLike).CameraIdNotlike(cameraIdNotlike).CameraIdIlike(cameraIdIlike).CameraIdNotilike(cameraIdNotilike).CameraIdDesc(cameraIdDesc).CameraIdAsc(cameraIdAsc).CameraIdObjectContains(cameraIdObjectContains).CameraIdObjectNotcontains(cameraIdObjectNotcontains).CameraIdObjectDesc(cameraIdObjectDesc).CameraIdObjectAsc(cameraIdObjectAsc).DetectionSummaryContains(detectionSummaryContains).DetectionSummaryNotcontains(detectionSummaryNotcontains).DetectionSummaryDesc(detectionSummaryDesc).DetectionSummaryAsc(detectionSummaryAsc).ReferencedByDetectionVideoIdObjectsContains(referencedByDetectionVideoIdObjectsContains).ReferencedByDetectionVideoIdObjectsNotcontains(referencedByDetectionVideoIdObjectsNotcontains).ReferencedByDetectionVideoIdObjectsDesc(referencedByDetectionVideoIdObjectsDesc).ReferencedByDetectionVideoIdObjectsAsc(referencedByDetectionVideoIdObjectsAsc).Execute()
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
 **limit** | **int32** | SQL LIMIT operator | 
 **offset** | **int32** | SQL OFFSET operator | 
 **depth** | **int32** | Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | 
 **cameraLoad** | **string** | load the given directly related object, value is ignored (presence of key is sufficient) | 
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
 **fileNameEq** | **string** | SQL &#x3D; comparison | 
 **fileNameNe** | **string** | SQL !&#x3D; comparison | 
 **fileNameGt** | **string** | SQL &gt; comparison, may not work with all column types | 
 **fileNameGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **fileNameLt** | **string** | SQL &lt; comparison, may not work with all column types | 
 **fileNameLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **fileNameIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **fileNameNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **fileNameContains** | **string** | SQL @&gt; comparison | 
 **fileNameNotcontains** | **string** | SQL NOT @&gt; comparison | 
 **fileNameLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **fileNameNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **fileNameIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **fileNameNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **fileNameDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **fileNameAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
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
 **durationEq** | **int64** | SQL &#x3D; comparison | 
 **durationNe** | **int64** | SQL !&#x3D; comparison | 
 **durationGt** | **int64** | SQL &gt; comparison, may not work with all column types | 
 **durationGte** | **int64** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **durationLt** | **int64** | SQL &lt; comparison, may not work with all column types | 
 **durationLte** | **int64** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **durationIn** | **int64** | SQL IN comparison, permits comma-separated values | 
 **durationNotin** | **int64** | SQL NOT IN comparison, permits comma-separated values | 
 **durationContains** | **int64** | SQL @&gt; comparison | 
 **durationNotcontains** | **int64** | SQL NOT @&gt; comparison | 
 **durationDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **durationAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **fileSizeEq** | **float64** | SQL &#x3D; comparison | 
 **fileSizeNe** | **float64** | SQL !&#x3D; comparison | 
 **fileSizeGt** | **float64** | SQL &gt; comparison, may not work with all column types | 
 **fileSizeGte** | **float64** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **fileSizeLt** | **float64** | SQL &lt; comparison, may not work with all column types | 
 **fileSizeLte** | **float64** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **fileSizeIn** | **float64** | SQL IN comparison, permits comma-separated values | 
 **fileSizeNotin** | **float64** | SQL NOT IN comparison, permits comma-separated values | 
 **fileSizeContains** | **float64** | SQL @&gt; comparison | 
 **fileSizeNotcontains** | **float64** | SQL NOT @&gt; comparison | 
 **fileSizeDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **fileSizeAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **thumbnailNameEq** | **string** | SQL &#x3D; comparison | 
 **thumbnailNameNe** | **string** | SQL !&#x3D; comparison | 
 **thumbnailNameGt** | **string** | SQL &gt; comparison, may not work with all column types | 
 **thumbnailNameGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **thumbnailNameLt** | **string** | SQL &lt; comparison, may not work with all column types | 
 **thumbnailNameLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **thumbnailNameIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **thumbnailNameNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **thumbnailNameContains** | **string** | SQL @&gt; comparison | 
 **thumbnailNameNotcontains** | **string** | SQL NOT @&gt; comparison | 
 **thumbnailNameLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **thumbnailNameNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **thumbnailNameIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **thumbnailNameNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **thumbnailNameDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **thumbnailNameAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
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
 **objectDetectorClaimedUntilEq** | **time.Time** | SQL &#x3D; comparison | 
 **objectDetectorClaimedUntilNe** | **time.Time** | SQL !&#x3D; comparison | 
 **objectDetectorClaimedUntilGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **objectDetectorClaimedUntilGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **objectDetectorClaimedUntilLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **objectDetectorClaimedUntilLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **objectDetectorClaimedUntilIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **objectDetectorClaimedUntilNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **objectDetectorClaimedUntilContains** | **time.Time** | SQL @&gt; comparison | 
 **objectDetectorClaimedUntilNotcontains** | **time.Time** | SQL NOT @&gt; comparison | 
 **objectDetectorClaimedUntilLike** | **time.Time** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **objectDetectorClaimedUntilNotlike** | **time.Time** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **objectDetectorClaimedUntilIlike** | **time.Time** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **objectDetectorClaimedUntilNotilike** | **time.Time** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **objectDetectorClaimedUntilDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **objectDetectorClaimedUntilAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **objectTrackerClaimedUntilEq** | **time.Time** | SQL &#x3D; comparison | 
 **objectTrackerClaimedUntilNe** | **time.Time** | SQL !&#x3D; comparison | 
 **objectTrackerClaimedUntilGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **objectTrackerClaimedUntilGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **objectTrackerClaimedUntilLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **objectTrackerClaimedUntilLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **objectTrackerClaimedUntilIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **objectTrackerClaimedUntilNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **objectTrackerClaimedUntilContains** | **time.Time** | SQL @&gt; comparison | 
 **objectTrackerClaimedUntilNotcontains** | **time.Time** | SQL NOT @&gt; comparison | 
 **objectTrackerClaimedUntilLike** | **time.Time** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **objectTrackerClaimedUntilNotlike** | **time.Time** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **objectTrackerClaimedUntilIlike** | **time.Time** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **objectTrackerClaimedUntilNotilike** | **time.Time** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **objectTrackerClaimedUntilDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **objectTrackerClaimedUntilAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
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
 **detectionSummaryContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison | 
 **detectionSummaryNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison | 
 **detectionSummaryDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **detectionSummaryAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **referencedByDetectionVideoIdObjectsContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison | 
 **referencedByDetectionVideoIdObjectsNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison | 
 **referencedByDetectionVideoIdObjectsDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **referencedByDetectionVideoIdObjectsAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 

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

