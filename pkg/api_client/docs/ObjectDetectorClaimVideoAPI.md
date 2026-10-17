# \ObjectDetectorClaimVideoAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**PostObjectDetectorClaimVideos**](ObjectDetectorClaimVideoAPI.md#PostObjectDetectorClaimVideos) | **Post** /api/object-detector-claim-video | 



## PostObjectDetectorClaimVideos

> ResponseWithGenericOfVideo PostObjectDetectorClaimVideos(ctx).VideoObjectDetectorClaimRequest(videoObjectDetectorClaimRequest).Limit(limit).Offset(offset).Depth(depth).CameraLoad(cameraLoad).ReferencedByAudioEventVideoIDLoad(referencedByAudioEventVideoIDLoad).ReferencedByDetectionVideoIDLoad(referencedByDetectionVideoIDLoad).ReferencedByTrackVideoIDLoad(referencedByTrackVideoIDLoad).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtAsc(createdAtAsc).CreatedAtDesc(createdAtDesc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtAsc(updatedAtAsc).UpdatedAtDesc(updatedAtDesc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtIsnull(deletedAtIsnull).DeletedAtIsnotnull(deletedAtIsnotnull).DeletedAtAsc(deletedAtAsc).DeletedAtDesc(deletedAtDesc).FileNameEq(fileNameEq).FileNameNe(fileNameNe).FileNameIn(fileNameIn).FileNameNotin(fileNameNotin).FileNameLike(fileNameLike).FileNameNotlike(fileNameNotlike).FileNameIlike(fileNameIlike).FileNameNotilike(fileNameNotilike).FileNameAsc(fileNameAsc).FileNameDesc(fileNameDesc).StartedAtEq(startedAtEq).StartedAtNe(startedAtNe).StartedAtGt(startedAtGt).StartedAtGte(startedAtGte).StartedAtLt(startedAtLt).StartedAtLte(startedAtLte).StartedAtIn(startedAtIn).StartedAtNotin(startedAtNotin).StartedAtAsc(startedAtAsc).StartedAtDesc(startedAtDesc).EndedAtEq(endedAtEq).EndedAtNe(endedAtNe).EndedAtGt(endedAtGt).EndedAtGte(endedAtGte).EndedAtLt(endedAtLt).EndedAtLte(endedAtLte).EndedAtIn(endedAtIn).EndedAtNotin(endedAtNotin).EndedAtIsnull(endedAtIsnull).EndedAtIsnotnull(endedAtIsnotnull).EndedAtAsc(endedAtAsc).EndedAtDesc(endedAtDesc).DurationEq(durationEq).DurationNe(durationNe).DurationGt(durationGt).DurationGte(durationGte).DurationLt(durationLt).DurationLte(durationLte).DurationIn(durationIn).DurationNotin(durationNotin).DurationIsnull(durationIsnull).DurationIsnotnull(durationIsnotnull).DurationAsc(durationAsc).DurationDesc(durationDesc).FileSizeEq(fileSizeEq).FileSizeNe(fileSizeNe).FileSizeGt(fileSizeGt).FileSizeGte(fileSizeGte).FileSizeLt(fileSizeLt).FileSizeLte(fileSizeLte).FileSizeIn(fileSizeIn).FileSizeNotin(fileSizeNotin).FileSizeIsnull(fileSizeIsnull).FileSizeIsnotnull(fileSizeIsnotnull).FileSizeAsc(fileSizeAsc).FileSizeDesc(fileSizeDesc).ThumbnailNameEq(thumbnailNameEq).ThumbnailNameNe(thumbnailNameNe).ThumbnailNameIn(thumbnailNameIn).ThumbnailNameNotin(thumbnailNameNotin).ThumbnailNameLike(thumbnailNameLike).ThumbnailNameNotlike(thumbnailNameNotlike).ThumbnailNameIlike(thumbnailNameIlike).ThumbnailNameNotilike(thumbnailNameNotilike).ThumbnailNameIsnull(thumbnailNameIsnull).ThumbnailNameIsnotnull(thumbnailNameIsnotnull).ThumbnailNameAsc(thumbnailNameAsc).ThumbnailNameDesc(thumbnailNameDesc).StatusEq(statusEq).StatusNe(statusNe).StatusIn(statusIn).StatusNotin(statusNotin).StatusLike(statusLike).StatusNotlike(statusNotlike).StatusIlike(statusIlike).StatusNotilike(statusNotilike).StatusIsnull(statusIsnull).StatusIsnotnull(statusIsnotnull).StatusAsc(statusAsc).StatusDesc(statusDesc).ObjectDetectorClaimedUntilEq(objectDetectorClaimedUntilEq).ObjectDetectorClaimedUntilNe(objectDetectorClaimedUntilNe).ObjectDetectorClaimedUntilGt(objectDetectorClaimedUntilGt).ObjectDetectorClaimedUntilGte(objectDetectorClaimedUntilGte).ObjectDetectorClaimedUntilLt(objectDetectorClaimedUntilLt).ObjectDetectorClaimedUntilLte(objectDetectorClaimedUntilLte).ObjectDetectorClaimedUntilIn(objectDetectorClaimedUntilIn).ObjectDetectorClaimedUntilNotin(objectDetectorClaimedUntilNotin).ObjectDetectorClaimedUntilAsc(objectDetectorClaimedUntilAsc).ObjectDetectorClaimedUntilDesc(objectDetectorClaimedUntilDesc).ObjectTrackerClaimedUntilEq(objectTrackerClaimedUntilEq).ObjectTrackerClaimedUntilNe(objectTrackerClaimedUntilNe).ObjectTrackerClaimedUntilGt(objectTrackerClaimedUntilGt).ObjectTrackerClaimedUntilGte(objectTrackerClaimedUntilGte).ObjectTrackerClaimedUntilLt(objectTrackerClaimedUntilLt).ObjectTrackerClaimedUntilLte(objectTrackerClaimedUntilLte).ObjectTrackerClaimedUntilIn(objectTrackerClaimedUntilIn).ObjectTrackerClaimedUntilNotin(objectTrackerClaimedUntilNotin).ObjectTrackerClaimedUntilAsc(objectTrackerClaimedUntilAsc).ObjectTrackerClaimedUntilDesc(objectTrackerClaimedUntilDesc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdAsc(cameraIdAsc).CameraIdDesc(cameraIdDesc).DetectionSummaryContains(detectionSummaryContains).DetectionSummaryNotcontains(detectionSummaryNotcontains).DetectionSummaryAsc(detectionSummaryAsc).DetectionSummaryDesc(detectionSummaryDesc).IsLowResEq(isLowResEq).IsLowResNe(isLowResNe).IsLowResIsfalse(isLowResIsfalse).IsLowResIstrue(isLowResIstrue).IsLowResAsc(isLowResAsc).IsLowResDesc(isLowResDesc).TrackSummaryContains(trackSummaryContains).TrackSummaryNotcontains(trackSummaryNotcontains).TrackSummaryAsc(trackSummaryAsc).TrackSummaryDesc(trackSummaryDesc).OverlayOffsetMsEq(overlayOffsetMsEq).OverlayOffsetMsNe(overlayOffsetMsNe).OverlayOffsetMsGt(overlayOffsetMsGt).OverlayOffsetMsGte(overlayOffsetMsGte).OverlayOffsetMsLt(overlayOffsetMsLt).OverlayOffsetMsLte(overlayOffsetMsLte).OverlayOffsetMsIn(overlayOffsetMsIn).OverlayOffsetMsNotin(overlayOffsetMsNotin).OverlayOffsetMsIsnull(overlayOffsetMsIsnull).OverlayOffsetMsIsnotnull(overlayOffsetMsIsnotnull).OverlayOffsetMsAsc(overlayOffsetMsAsc).OverlayOffsetMsDesc(overlayOffsetMsDesc).AudioDetectorClaimedUntilEq(audioDetectorClaimedUntilEq).AudioDetectorClaimedUntilNe(audioDetectorClaimedUntilNe).AudioDetectorClaimedUntilGt(audioDetectorClaimedUntilGt).AudioDetectorClaimedUntilGte(audioDetectorClaimedUntilGte).AudioDetectorClaimedUntilLt(audioDetectorClaimedUntilLt).AudioDetectorClaimedUntilLte(audioDetectorClaimedUntilLte).AudioDetectorClaimedUntilIn(audioDetectorClaimedUntilIn).AudioDetectorClaimedUntilNotin(audioDetectorClaimedUntilNotin).AudioDetectorClaimedUntilAsc(audioDetectorClaimedUntilAsc).AudioDetectorClaimedUntilDesc(audioDetectorClaimedUntilDesc).AudioDetectedAtEq(audioDetectedAtEq).AudioDetectedAtNe(audioDetectedAtNe).AudioDetectedAtGt(audioDetectedAtGt).AudioDetectedAtGte(audioDetectedAtGte).AudioDetectedAtLt(audioDetectedAtLt).AudioDetectedAtLte(audioDetectedAtLte).AudioDetectedAtIn(audioDetectedAtIn).AudioDetectedAtNotin(audioDetectedAtNotin).AudioDetectedAtIsnull(audioDetectedAtIsnull).AudioDetectedAtIsnotnull(audioDetectedAtIsnotnull).AudioDetectedAtAsc(audioDetectedAtAsc).AudioDetectedAtDesc(audioDetectedAtDesc).AudioProfileContains(audioProfileContains).AudioProfileNotcontains(audioProfileNotcontains).AudioProfileAsc(audioProfileAsc).AudioProfileDesc(audioProfileDesc).AudioSummaryContains(audioSummaryContains).AudioSummaryNotcontains(audioSummaryNotcontains).AudioSummaryAsc(audioSummaryAsc).AudioSummaryDesc(audioSummaryDesc).Execute()



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
	videoObjectDetectorClaimRequest := *openapiclient.NewVideoObjectDetectorClaimRequest() // VideoObjectDetectorClaimRequest | 
	limit := int32(56) // int32 | SQL LIMIT operator (optional)
	offset := int32(56) // int32 | SQL OFFSET operator (optional)
	depth := int32(56) // int32 | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)
	cameraLoad := "cameraLoad_example" // string | load the given outbound relationship, value is ignored (presence of key is sufficient) (optional)
	referencedByAudioEventVideoIDLoad := "referencedByAudioEventVideoIDLoad_example" // string | load the given inbound relationship, value is ignored (presence of key is sufficient) (optional)
	referencedByDetectionVideoIDLoad := "referencedByDetectionVideoIDLoad_example" // string | load the given inbound relationship, value is ignored (presence of key is sufficient) (optional)
	referencedByTrackVideoIDLoad := "referencedByTrackVideoIDLoad_example" // string | load the given inbound relationship, value is ignored (presence of key is sufficient) (optional)
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
	fileNameEq := "fileNameEq_example" // string | SQL = comparison (optional)
	fileNameNe := "fileNameNe_example" // string | SQL != comparison (optional)
	fileNameIn := "fileNameIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	fileNameNotin := "fileNameNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	fileNameLike := "fileNameLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	fileNameNotlike := "fileNameNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	fileNameIlike := "fileNameIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	fileNameNotilike := "fileNameNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	fileNameAsc := "fileNameAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	fileNameDesc := "fileNameDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
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
	endedAtIsnull := "endedAtIsnull_example" // string | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
	endedAtIsnotnull := "endedAtIsnotnull_example" // string | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
	endedAtAsc := "endedAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	endedAtDesc := "endedAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	durationEq := "durationEq_example" // string | SQL = comparison (optional)
	durationNe := "durationNe_example" // string | SQL != comparison (optional)
	durationGt := "durationGt_example" // string | SQL > comparison, may not work with all column types (optional)
	durationGte := "durationGte_example" // string | SQL >= comparison, may not work with all column types (optional)
	durationLt := "durationLt_example" // string | SQL < comparison, may not work with all column types (optional)
	durationLte := "durationLte_example" // string | SQL <= comparison, may not work with all column types (optional)
	durationIn := "durationIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	durationNotin := "durationNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	durationIsnull := "durationIsnull_example" // string | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
	durationIsnotnull := "durationIsnotnull_example" // string | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
	durationAsc := "durationAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	durationDesc := "durationDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	fileSizeEq := float32(8.14) // float32 | SQL = comparison (optional)
	fileSizeNe := float32(8.14) // float32 | SQL != comparison (optional)
	fileSizeGt := float32(8.14) // float32 | SQL > comparison, may not work with all column types (optional)
	fileSizeGte := float32(8.14) // float32 | SQL >= comparison, may not work with all column types (optional)
	fileSizeLt := float32(8.14) // float32 | SQL < comparison, may not work with all column types (optional)
	fileSizeLte := float32(8.14) // float32 | SQL <= comparison, may not work with all column types (optional)
	fileSizeIn := float32(8.14) // float32 | SQL IN comparison, permits comma-separated values (optional)
	fileSizeNotin := float32(8.14) // float32 | SQL NOT IN comparison, permits comma-separated values (optional)
	fileSizeIsnull := "fileSizeIsnull_example" // string | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
	fileSizeIsnotnull := "fileSizeIsnotnull_example" // string | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
	fileSizeAsc := "fileSizeAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	fileSizeDesc := "fileSizeDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	thumbnailNameEq := "thumbnailNameEq_example" // string | SQL = comparison (optional)
	thumbnailNameNe := "thumbnailNameNe_example" // string | SQL != comparison (optional)
	thumbnailNameIn := "thumbnailNameIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	thumbnailNameNotin := "thumbnailNameNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	thumbnailNameLike := "thumbnailNameLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	thumbnailNameNotlike := "thumbnailNameNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	thumbnailNameIlike := "thumbnailNameIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	thumbnailNameNotilike := "thumbnailNameNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	thumbnailNameIsnull := "thumbnailNameIsnull_example" // string | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
	thumbnailNameIsnotnull := "thumbnailNameIsnotnull_example" // string | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
	thumbnailNameAsc := "thumbnailNameAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	thumbnailNameDesc := "thumbnailNameDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	statusEq := "statusEq_example" // string | SQL = comparison (optional)
	statusNe := "statusNe_example" // string | SQL != comparison (optional)
	statusIn := "statusIn_example" // string | SQL IN comparison, permits comma-separated values (optional)
	statusNotin := "statusNotin_example" // string | SQL NOT IN comparison, permits comma-separated values (optional)
	statusLike := "statusLike_example" // string | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	statusNotlike := "statusNotlike_example" // string | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	statusIlike := "statusIlike_example" // string | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	statusNotilike := "statusNotilike_example" // string | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
	statusIsnull := "statusIsnull_example" // string | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
	statusIsnotnull := "statusIsnotnull_example" // string | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
	statusAsc := "statusAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	statusDesc := "statusDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	objectDetectorClaimedUntilEq := time.Now() // time.Time | SQL = comparison (optional)
	objectDetectorClaimedUntilNe := time.Now() // time.Time | SQL != comparison (optional)
	objectDetectorClaimedUntilGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	objectDetectorClaimedUntilGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	objectDetectorClaimedUntilLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	objectDetectorClaimedUntilLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	objectDetectorClaimedUntilIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	objectDetectorClaimedUntilNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	objectDetectorClaimedUntilAsc := "objectDetectorClaimedUntilAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	objectDetectorClaimedUntilDesc := "objectDetectorClaimedUntilDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	objectTrackerClaimedUntilEq := time.Now() // time.Time | SQL = comparison (optional)
	objectTrackerClaimedUntilNe := time.Now() // time.Time | SQL != comparison (optional)
	objectTrackerClaimedUntilGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	objectTrackerClaimedUntilGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	objectTrackerClaimedUntilLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	objectTrackerClaimedUntilLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	objectTrackerClaimedUntilIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	objectTrackerClaimedUntilNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	objectTrackerClaimedUntilAsc := "objectTrackerClaimedUntilAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	objectTrackerClaimedUntilDesc := "objectTrackerClaimedUntilDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
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
	detectionSummaryContains := TODO // interface{} | SQL @> comparison (optional)
	detectionSummaryNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	detectionSummaryAsc := "detectionSummaryAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	detectionSummaryDesc := "detectionSummaryDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	isLowResEq := true // bool | SQL = comparison (optional)
	isLowResNe := true // bool | SQL != comparison (optional)
	isLowResIsfalse := "isLowResIsfalse_example" // string | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
	isLowResIstrue := "isLowResIstrue_example" // string | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
	isLowResAsc := "isLowResAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	isLowResDesc := "isLowResDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	trackSummaryContains := TODO // interface{} | SQL @> comparison (optional)
	trackSummaryNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	trackSummaryAsc := "trackSummaryAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	trackSummaryDesc := "trackSummaryDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	overlayOffsetMsEq := int32(56) // int32 | SQL = comparison (optional)
	overlayOffsetMsNe := int32(56) // int32 | SQL != comparison (optional)
	overlayOffsetMsGt := int32(56) // int32 | SQL > comparison, may not work with all column types (optional)
	overlayOffsetMsGte := int32(56) // int32 | SQL >= comparison, may not work with all column types (optional)
	overlayOffsetMsLt := int32(56) // int32 | SQL < comparison, may not work with all column types (optional)
	overlayOffsetMsLte := int32(56) // int32 | SQL <= comparison, may not work with all column types (optional)
	overlayOffsetMsIn := int32(56) // int32 | SQL IN comparison, permits comma-separated values (optional)
	overlayOffsetMsNotin := int32(56) // int32 | SQL NOT IN comparison, permits comma-separated values (optional)
	overlayOffsetMsIsnull := "overlayOffsetMsIsnull_example" // string | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
	overlayOffsetMsIsnotnull := "overlayOffsetMsIsnotnull_example" // string | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
	overlayOffsetMsAsc := "overlayOffsetMsAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	overlayOffsetMsDesc := "overlayOffsetMsDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	audioDetectorClaimedUntilEq := time.Now() // time.Time | SQL = comparison (optional)
	audioDetectorClaimedUntilNe := time.Now() // time.Time | SQL != comparison (optional)
	audioDetectorClaimedUntilGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	audioDetectorClaimedUntilGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	audioDetectorClaimedUntilLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	audioDetectorClaimedUntilLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	audioDetectorClaimedUntilIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	audioDetectorClaimedUntilNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	audioDetectorClaimedUntilAsc := "audioDetectorClaimedUntilAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	audioDetectorClaimedUntilDesc := "audioDetectorClaimedUntilDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	audioDetectedAtEq := time.Now() // time.Time | SQL = comparison (optional)
	audioDetectedAtNe := time.Now() // time.Time | SQL != comparison (optional)
	audioDetectedAtGt := time.Now() // time.Time | SQL > comparison, may not work with all column types (optional)
	audioDetectedAtGte := time.Now() // time.Time | SQL >= comparison, may not work with all column types (optional)
	audioDetectedAtLt := time.Now() // time.Time | SQL < comparison, may not work with all column types (optional)
	audioDetectedAtLte := time.Now() // time.Time | SQL <= comparison, may not work with all column types (optional)
	audioDetectedAtIn := time.Now() // time.Time | SQL IN comparison, permits comma-separated values (optional)
	audioDetectedAtNotin := time.Now() // time.Time | SQL NOT IN comparison, permits comma-separated values (optional)
	audioDetectedAtIsnull := "audioDetectedAtIsnull_example" // string | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
	audioDetectedAtIsnotnull := "audioDetectedAtIsnotnull_example" // string | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
	audioDetectedAtAsc := "audioDetectedAtAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	audioDetectedAtDesc := "audioDetectedAtDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	audioProfileContains := TODO // interface{} | SQL @> comparison (optional)
	audioProfileNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	audioProfileAsc := "audioProfileAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	audioProfileDesc := "audioProfileDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
	audioSummaryContains := TODO // interface{} | SQL @> comparison (optional)
	audioSummaryNotcontains := TODO // interface{} | SQL NOT @> comparison (optional)
	audioSummaryAsc := "audioSummaryAsc_example" // string | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
	audioSummaryDesc := "audioSummaryDesc_example" // string | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ObjectDetectorClaimVideoAPI.PostObjectDetectorClaimVideos(context.Background()).VideoObjectDetectorClaimRequest(videoObjectDetectorClaimRequest).Limit(limit).Offset(offset).Depth(depth).CameraLoad(cameraLoad).ReferencedByAudioEventVideoIDLoad(referencedByAudioEventVideoIDLoad).ReferencedByDetectionVideoIDLoad(referencedByDetectionVideoIDLoad).ReferencedByTrackVideoIDLoad(referencedByTrackVideoIDLoad).CreatedAtEq(createdAtEq).CreatedAtNe(createdAtNe).CreatedAtGt(createdAtGt).CreatedAtGte(createdAtGte).CreatedAtLt(createdAtLt).CreatedAtLte(createdAtLte).CreatedAtIn(createdAtIn).CreatedAtNotin(createdAtNotin).CreatedAtAsc(createdAtAsc).CreatedAtDesc(createdAtDesc).UpdatedAtEq(updatedAtEq).UpdatedAtNe(updatedAtNe).UpdatedAtGt(updatedAtGt).UpdatedAtGte(updatedAtGte).UpdatedAtLt(updatedAtLt).UpdatedAtLte(updatedAtLte).UpdatedAtIn(updatedAtIn).UpdatedAtNotin(updatedAtNotin).UpdatedAtAsc(updatedAtAsc).UpdatedAtDesc(updatedAtDesc).DeletedAtEq(deletedAtEq).DeletedAtNe(deletedAtNe).DeletedAtGt(deletedAtGt).DeletedAtGte(deletedAtGte).DeletedAtLt(deletedAtLt).DeletedAtLte(deletedAtLte).DeletedAtIn(deletedAtIn).DeletedAtNotin(deletedAtNotin).DeletedAtIsnull(deletedAtIsnull).DeletedAtIsnotnull(deletedAtIsnotnull).DeletedAtAsc(deletedAtAsc).DeletedAtDesc(deletedAtDesc).FileNameEq(fileNameEq).FileNameNe(fileNameNe).FileNameIn(fileNameIn).FileNameNotin(fileNameNotin).FileNameLike(fileNameLike).FileNameNotlike(fileNameNotlike).FileNameIlike(fileNameIlike).FileNameNotilike(fileNameNotilike).FileNameAsc(fileNameAsc).FileNameDesc(fileNameDesc).StartedAtEq(startedAtEq).StartedAtNe(startedAtNe).StartedAtGt(startedAtGt).StartedAtGte(startedAtGte).StartedAtLt(startedAtLt).StartedAtLte(startedAtLte).StartedAtIn(startedAtIn).StartedAtNotin(startedAtNotin).StartedAtAsc(startedAtAsc).StartedAtDesc(startedAtDesc).EndedAtEq(endedAtEq).EndedAtNe(endedAtNe).EndedAtGt(endedAtGt).EndedAtGte(endedAtGte).EndedAtLt(endedAtLt).EndedAtLte(endedAtLte).EndedAtIn(endedAtIn).EndedAtNotin(endedAtNotin).EndedAtIsnull(endedAtIsnull).EndedAtIsnotnull(endedAtIsnotnull).EndedAtAsc(endedAtAsc).EndedAtDesc(endedAtDesc).DurationEq(durationEq).DurationNe(durationNe).DurationGt(durationGt).DurationGte(durationGte).DurationLt(durationLt).DurationLte(durationLte).DurationIn(durationIn).DurationNotin(durationNotin).DurationIsnull(durationIsnull).DurationIsnotnull(durationIsnotnull).DurationAsc(durationAsc).DurationDesc(durationDesc).FileSizeEq(fileSizeEq).FileSizeNe(fileSizeNe).FileSizeGt(fileSizeGt).FileSizeGte(fileSizeGte).FileSizeLt(fileSizeLt).FileSizeLte(fileSizeLte).FileSizeIn(fileSizeIn).FileSizeNotin(fileSizeNotin).FileSizeIsnull(fileSizeIsnull).FileSizeIsnotnull(fileSizeIsnotnull).FileSizeAsc(fileSizeAsc).FileSizeDesc(fileSizeDesc).ThumbnailNameEq(thumbnailNameEq).ThumbnailNameNe(thumbnailNameNe).ThumbnailNameIn(thumbnailNameIn).ThumbnailNameNotin(thumbnailNameNotin).ThumbnailNameLike(thumbnailNameLike).ThumbnailNameNotlike(thumbnailNameNotlike).ThumbnailNameIlike(thumbnailNameIlike).ThumbnailNameNotilike(thumbnailNameNotilike).ThumbnailNameIsnull(thumbnailNameIsnull).ThumbnailNameIsnotnull(thumbnailNameIsnotnull).ThumbnailNameAsc(thumbnailNameAsc).ThumbnailNameDesc(thumbnailNameDesc).StatusEq(statusEq).StatusNe(statusNe).StatusIn(statusIn).StatusNotin(statusNotin).StatusLike(statusLike).StatusNotlike(statusNotlike).StatusIlike(statusIlike).StatusNotilike(statusNotilike).StatusIsnull(statusIsnull).StatusIsnotnull(statusIsnotnull).StatusAsc(statusAsc).StatusDesc(statusDesc).ObjectDetectorClaimedUntilEq(objectDetectorClaimedUntilEq).ObjectDetectorClaimedUntilNe(objectDetectorClaimedUntilNe).ObjectDetectorClaimedUntilGt(objectDetectorClaimedUntilGt).ObjectDetectorClaimedUntilGte(objectDetectorClaimedUntilGte).ObjectDetectorClaimedUntilLt(objectDetectorClaimedUntilLt).ObjectDetectorClaimedUntilLte(objectDetectorClaimedUntilLte).ObjectDetectorClaimedUntilIn(objectDetectorClaimedUntilIn).ObjectDetectorClaimedUntilNotin(objectDetectorClaimedUntilNotin).ObjectDetectorClaimedUntilAsc(objectDetectorClaimedUntilAsc).ObjectDetectorClaimedUntilDesc(objectDetectorClaimedUntilDesc).ObjectTrackerClaimedUntilEq(objectTrackerClaimedUntilEq).ObjectTrackerClaimedUntilNe(objectTrackerClaimedUntilNe).ObjectTrackerClaimedUntilGt(objectTrackerClaimedUntilGt).ObjectTrackerClaimedUntilGte(objectTrackerClaimedUntilGte).ObjectTrackerClaimedUntilLt(objectTrackerClaimedUntilLt).ObjectTrackerClaimedUntilLte(objectTrackerClaimedUntilLte).ObjectTrackerClaimedUntilIn(objectTrackerClaimedUntilIn).ObjectTrackerClaimedUntilNotin(objectTrackerClaimedUntilNotin).ObjectTrackerClaimedUntilAsc(objectTrackerClaimedUntilAsc).ObjectTrackerClaimedUntilDesc(objectTrackerClaimedUntilDesc).CameraIdEq(cameraIdEq).CameraIdNe(cameraIdNe).CameraIdGt(cameraIdGt).CameraIdGte(cameraIdGte).CameraIdLt(cameraIdLt).CameraIdLte(cameraIdLte).CameraIdIn(cameraIdIn).CameraIdNotin(cameraIdNotin).CameraIdAsc(cameraIdAsc).CameraIdDesc(cameraIdDesc).DetectionSummaryContains(detectionSummaryContains).DetectionSummaryNotcontains(detectionSummaryNotcontains).DetectionSummaryAsc(detectionSummaryAsc).DetectionSummaryDesc(detectionSummaryDesc).IsLowResEq(isLowResEq).IsLowResNe(isLowResNe).IsLowResIsfalse(isLowResIsfalse).IsLowResIstrue(isLowResIstrue).IsLowResAsc(isLowResAsc).IsLowResDesc(isLowResDesc).TrackSummaryContains(trackSummaryContains).TrackSummaryNotcontains(trackSummaryNotcontains).TrackSummaryAsc(trackSummaryAsc).TrackSummaryDesc(trackSummaryDesc).OverlayOffsetMsEq(overlayOffsetMsEq).OverlayOffsetMsNe(overlayOffsetMsNe).OverlayOffsetMsGt(overlayOffsetMsGt).OverlayOffsetMsGte(overlayOffsetMsGte).OverlayOffsetMsLt(overlayOffsetMsLt).OverlayOffsetMsLte(overlayOffsetMsLte).OverlayOffsetMsIn(overlayOffsetMsIn).OverlayOffsetMsNotin(overlayOffsetMsNotin).OverlayOffsetMsIsnull(overlayOffsetMsIsnull).OverlayOffsetMsIsnotnull(overlayOffsetMsIsnotnull).OverlayOffsetMsAsc(overlayOffsetMsAsc).OverlayOffsetMsDesc(overlayOffsetMsDesc).AudioDetectorClaimedUntilEq(audioDetectorClaimedUntilEq).AudioDetectorClaimedUntilNe(audioDetectorClaimedUntilNe).AudioDetectorClaimedUntilGt(audioDetectorClaimedUntilGt).AudioDetectorClaimedUntilGte(audioDetectorClaimedUntilGte).AudioDetectorClaimedUntilLt(audioDetectorClaimedUntilLt).AudioDetectorClaimedUntilLte(audioDetectorClaimedUntilLte).AudioDetectorClaimedUntilIn(audioDetectorClaimedUntilIn).AudioDetectorClaimedUntilNotin(audioDetectorClaimedUntilNotin).AudioDetectorClaimedUntilAsc(audioDetectorClaimedUntilAsc).AudioDetectorClaimedUntilDesc(audioDetectorClaimedUntilDesc).AudioDetectedAtEq(audioDetectedAtEq).AudioDetectedAtNe(audioDetectedAtNe).AudioDetectedAtGt(audioDetectedAtGt).AudioDetectedAtGte(audioDetectedAtGte).AudioDetectedAtLt(audioDetectedAtLt).AudioDetectedAtLte(audioDetectedAtLte).AudioDetectedAtIn(audioDetectedAtIn).AudioDetectedAtNotin(audioDetectedAtNotin).AudioDetectedAtIsnull(audioDetectedAtIsnull).AudioDetectedAtIsnotnull(audioDetectedAtIsnotnull).AudioDetectedAtAsc(audioDetectedAtAsc).AudioDetectedAtDesc(audioDetectedAtDesc).AudioProfileContains(audioProfileContains).AudioProfileNotcontains(audioProfileNotcontains).AudioProfileAsc(audioProfileAsc).AudioProfileDesc(audioProfileDesc).AudioSummaryContains(audioSummaryContains).AudioSummaryNotcontains(audioSummaryNotcontains).AudioSummaryAsc(audioSummaryAsc).AudioSummaryDesc(audioSummaryDesc).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ObjectDetectorClaimVideoAPI.PostObjectDetectorClaimVideos``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostObjectDetectorClaimVideos`: ResponseWithGenericOfVideo
	fmt.Fprintf(os.Stdout, "Response from `ObjectDetectorClaimVideoAPI.PostObjectDetectorClaimVideos`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostObjectDetectorClaimVideosRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **videoObjectDetectorClaimRequest** | [**VideoObjectDetectorClaimRequest**](VideoObjectDetectorClaimRequest.md) |  | 
 **limit** | **int32** | SQL LIMIT operator | 
 **offset** | **int32** | SQL OFFSET operator | 
 **depth** | **int32** | Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | 
 **cameraLoad** | **string** | load the given outbound relationship, value is ignored (presence of key is sufficient) | 
 **referencedByAudioEventVideoIDLoad** | **string** | load the given inbound relationship, value is ignored (presence of key is sufficient) | 
 **referencedByDetectionVideoIDLoad** | **string** | load the given inbound relationship, value is ignored (presence of key is sufficient) | 
 **referencedByTrackVideoIDLoad** | **string** | load the given inbound relationship, value is ignored (presence of key is sufficient) | 
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
 **fileNameEq** | **string** | SQL &#x3D; comparison | 
 **fileNameNe** | **string** | SQL !&#x3D; comparison | 
 **fileNameIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **fileNameNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **fileNameLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **fileNameNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **fileNameIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **fileNameNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **fileNameAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **fileNameDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
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
 **endedAtIsnull** | **string** | SQL IS null comparison, value is ignored (presence of key is sufficient) | 
 **endedAtIsnotnull** | **string** | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | 
 **endedAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **endedAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **durationEq** | **string** | SQL &#x3D; comparison | 
 **durationNe** | **string** | SQL !&#x3D; comparison | 
 **durationGt** | **string** | SQL &gt; comparison, may not work with all column types | 
 **durationGte** | **string** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **durationLt** | **string** | SQL &lt; comparison, may not work with all column types | 
 **durationLte** | **string** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **durationIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **durationNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **durationIsnull** | **string** | SQL IS null comparison, value is ignored (presence of key is sufficient) | 
 **durationIsnotnull** | **string** | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | 
 **durationAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **durationDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **fileSizeEq** | **float32** | SQL &#x3D; comparison | 
 **fileSizeNe** | **float32** | SQL !&#x3D; comparison | 
 **fileSizeGt** | **float32** | SQL &gt; comparison, may not work with all column types | 
 **fileSizeGte** | **float32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **fileSizeLt** | **float32** | SQL &lt; comparison, may not work with all column types | 
 **fileSizeLte** | **float32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **fileSizeIn** | **float32** | SQL IN comparison, permits comma-separated values | 
 **fileSizeNotin** | **float32** | SQL NOT IN comparison, permits comma-separated values | 
 **fileSizeIsnull** | **string** | SQL IS null comparison, value is ignored (presence of key is sufficient) | 
 **fileSizeIsnotnull** | **string** | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | 
 **fileSizeAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **fileSizeDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **thumbnailNameEq** | **string** | SQL &#x3D; comparison | 
 **thumbnailNameNe** | **string** | SQL !&#x3D; comparison | 
 **thumbnailNameIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **thumbnailNameNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **thumbnailNameLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **thumbnailNameNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **thumbnailNameIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **thumbnailNameNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **thumbnailNameIsnull** | **string** | SQL IS null comparison, value is ignored (presence of key is sufficient) | 
 **thumbnailNameIsnotnull** | **string** | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | 
 **thumbnailNameAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **thumbnailNameDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **statusEq** | **string** | SQL &#x3D; comparison | 
 **statusNe** | **string** | SQL !&#x3D; comparison | 
 **statusIn** | **string** | SQL IN comparison, permits comma-separated values | 
 **statusNotin** | **string** | SQL NOT IN comparison, permits comma-separated values | 
 **statusLike** | **string** | SQL LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **statusNotlike** | **string** | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | 
 **statusIlike** | **string** | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **statusNotilike** | **string** | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | 
 **statusIsnull** | **string** | SQL IS null comparison, value is ignored (presence of key is sufficient) | 
 **statusIsnotnull** | **string** | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | 
 **statusAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **statusDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **objectDetectorClaimedUntilEq** | **time.Time** | SQL &#x3D; comparison | 
 **objectDetectorClaimedUntilNe** | **time.Time** | SQL !&#x3D; comparison | 
 **objectDetectorClaimedUntilGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **objectDetectorClaimedUntilGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **objectDetectorClaimedUntilLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **objectDetectorClaimedUntilLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **objectDetectorClaimedUntilIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **objectDetectorClaimedUntilNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **objectDetectorClaimedUntilAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **objectDetectorClaimedUntilDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **objectTrackerClaimedUntilEq** | **time.Time** | SQL &#x3D; comparison | 
 **objectTrackerClaimedUntilNe** | **time.Time** | SQL !&#x3D; comparison | 
 **objectTrackerClaimedUntilGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **objectTrackerClaimedUntilGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **objectTrackerClaimedUntilLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **objectTrackerClaimedUntilLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **objectTrackerClaimedUntilIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **objectTrackerClaimedUntilNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **objectTrackerClaimedUntilAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **objectTrackerClaimedUntilDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
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
 **detectionSummaryContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison | 
 **detectionSummaryNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison | 
 **detectionSummaryAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **detectionSummaryDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **isLowResEq** | **bool** | SQL &#x3D; comparison | 
 **isLowResNe** | **bool** | SQL !&#x3D; comparison | 
 **isLowResIsfalse** | **string** | SQL IS false comparison, value is ignored (presence of key is sufficient) | 
 **isLowResIstrue** | **string** | SQL IS true comparison, value is ignored (presence of key is sufficient) | 
 **isLowResAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **isLowResDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **trackSummaryContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison | 
 **trackSummaryNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison | 
 **trackSummaryAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **trackSummaryDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **overlayOffsetMsEq** | **int32** | SQL &#x3D; comparison | 
 **overlayOffsetMsNe** | **int32** | SQL !&#x3D; comparison | 
 **overlayOffsetMsGt** | **int32** | SQL &gt; comparison, may not work with all column types | 
 **overlayOffsetMsGte** | **int32** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **overlayOffsetMsLt** | **int32** | SQL &lt; comparison, may not work with all column types | 
 **overlayOffsetMsLte** | **int32** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **overlayOffsetMsIn** | **int32** | SQL IN comparison, permits comma-separated values | 
 **overlayOffsetMsNotin** | **int32** | SQL NOT IN comparison, permits comma-separated values | 
 **overlayOffsetMsIsnull** | **string** | SQL IS null comparison, value is ignored (presence of key is sufficient) | 
 **overlayOffsetMsIsnotnull** | **string** | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | 
 **overlayOffsetMsAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **overlayOffsetMsDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **audioDetectorClaimedUntilEq** | **time.Time** | SQL &#x3D; comparison | 
 **audioDetectorClaimedUntilNe** | **time.Time** | SQL !&#x3D; comparison | 
 **audioDetectorClaimedUntilGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **audioDetectorClaimedUntilGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **audioDetectorClaimedUntilLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **audioDetectorClaimedUntilLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **audioDetectorClaimedUntilIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **audioDetectorClaimedUntilNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **audioDetectorClaimedUntilAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **audioDetectorClaimedUntilDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **audioDetectedAtEq** | **time.Time** | SQL &#x3D; comparison | 
 **audioDetectedAtNe** | **time.Time** | SQL !&#x3D; comparison | 
 **audioDetectedAtGt** | **time.Time** | SQL &gt; comparison, may not work with all column types | 
 **audioDetectedAtGte** | **time.Time** | SQL &gt;&#x3D; comparison, may not work with all column types | 
 **audioDetectedAtLt** | **time.Time** | SQL &lt; comparison, may not work with all column types | 
 **audioDetectedAtLte** | **time.Time** | SQL &lt;&#x3D; comparison, may not work with all column types | 
 **audioDetectedAtIn** | **time.Time** | SQL IN comparison, permits comma-separated values | 
 **audioDetectedAtNotin** | **time.Time** | SQL NOT IN comparison, permits comma-separated values | 
 **audioDetectedAtIsnull** | **string** | SQL IS null comparison, value is ignored (presence of key is sufficient) | 
 **audioDetectedAtIsnotnull** | **string** | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | 
 **audioDetectedAtAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **audioDetectedAtDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **audioProfileContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison | 
 **audioProfileNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison | 
 **audioProfileAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **audioProfileDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 
 **audioSummaryContains** | [**interface{}**](interface{}.md) | SQL @&gt; comparison | 
 **audioSummaryNotcontains** | [**interface{}**](interface{}.md) | SQL NOT @&gt; comparison | 
 **audioSummaryAsc** | **string** | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | 
 **audioSummaryDesc** | **string** | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | 

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

