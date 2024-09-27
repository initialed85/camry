# Video

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CameraId** | Pointer to **string** |  | [optional] 
**CameraIdObject** | Pointer to [**Camera**](Camera.md) |  | [optional] 
**CreatedAt** | Pointer to **time.Time** |  | [optional] 
**DeletedAt** | Pointer to **time.Time** |  | [optional] 
**DetectionSummary** | Pointer to **interface{}** |  | [optional] 
**Duration** | Pointer to **int64** |  | [optional] 
**EndedAt** | Pointer to **time.Time** |  | [optional] 
**FileName** | Pointer to **string** |  | [optional] 
**FileSize** | Pointer to **float64** |  | [optional] 
**Id** | Pointer to **string** |  | [optional] 
**ObjectDetectorClaimedUntil** | Pointer to **time.Time** |  | [optional] 
**ObjectTrackerClaimedUntil** | Pointer to **time.Time** |  | [optional] 
**ReferencedByDetectionVideoIdObjects** | Pointer to [**[]Detection**](Detection.md) |  | [optional] 
**StartedAt** | Pointer to **time.Time** |  | [optional] 
**Status** | Pointer to **string** |  | [optional] 
**ThumbnailName** | Pointer to **string** |  | [optional] 
**UpdatedAt** | Pointer to **time.Time** |  | [optional] 

## Methods

### NewVideo

`func NewVideo() *Video`

NewVideo instantiates a new Video object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewVideoWithDefaults

`func NewVideoWithDefaults() *Video`

NewVideoWithDefaults instantiates a new Video object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCameraId

`func (o *Video) GetCameraId() string`

GetCameraId returns the CameraId field if non-nil, zero value otherwise.

### GetCameraIdOk

`func (o *Video) GetCameraIdOk() (*string, bool)`

GetCameraIdOk returns a tuple with the CameraId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCameraId

`func (o *Video) SetCameraId(v string)`

SetCameraId sets CameraId field to given value.

### HasCameraId

`func (o *Video) HasCameraId() bool`

HasCameraId returns a boolean if a field has been set.

### GetCameraIdObject

`func (o *Video) GetCameraIdObject() Camera`

GetCameraIdObject returns the CameraIdObject field if non-nil, zero value otherwise.

### GetCameraIdObjectOk

`func (o *Video) GetCameraIdObjectOk() (*Camera, bool)`

GetCameraIdObjectOk returns a tuple with the CameraIdObject field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCameraIdObject

`func (o *Video) SetCameraIdObject(v Camera)`

SetCameraIdObject sets CameraIdObject field to given value.

### HasCameraIdObject

`func (o *Video) HasCameraIdObject() bool`

HasCameraIdObject returns a boolean if a field has been set.

### GetCreatedAt

`func (o *Video) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *Video) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *Video) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *Video) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetDeletedAt

`func (o *Video) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *Video) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *Video) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *Video) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### GetDetectionSummary

`func (o *Video) GetDetectionSummary() interface{}`

GetDetectionSummary returns the DetectionSummary field if non-nil, zero value otherwise.

### GetDetectionSummaryOk

`func (o *Video) GetDetectionSummaryOk() (*interface{}, bool)`

GetDetectionSummaryOk returns a tuple with the DetectionSummary field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDetectionSummary

`func (o *Video) SetDetectionSummary(v interface{})`

SetDetectionSummary sets DetectionSummary field to given value.

### HasDetectionSummary

`func (o *Video) HasDetectionSummary() bool`

HasDetectionSummary returns a boolean if a field has been set.

### SetDetectionSummaryNil

`func (o *Video) SetDetectionSummaryNil(b bool)`

 SetDetectionSummaryNil sets the value for DetectionSummary to be an explicit nil

### UnsetDetectionSummary
`func (o *Video) UnsetDetectionSummary()`

UnsetDetectionSummary ensures that no value is present for DetectionSummary, not even an explicit nil
### GetDuration

`func (o *Video) GetDuration() int64`

GetDuration returns the Duration field if non-nil, zero value otherwise.

### GetDurationOk

`func (o *Video) GetDurationOk() (*int64, bool)`

GetDurationOk returns a tuple with the Duration field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDuration

`func (o *Video) SetDuration(v int64)`

SetDuration sets Duration field to given value.

### HasDuration

`func (o *Video) HasDuration() bool`

HasDuration returns a boolean if a field has been set.

### GetEndedAt

`func (o *Video) GetEndedAt() time.Time`

GetEndedAt returns the EndedAt field if non-nil, zero value otherwise.

### GetEndedAtOk

`func (o *Video) GetEndedAtOk() (*time.Time, bool)`

GetEndedAtOk returns a tuple with the EndedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEndedAt

`func (o *Video) SetEndedAt(v time.Time)`

SetEndedAt sets EndedAt field to given value.

### HasEndedAt

`func (o *Video) HasEndedAt() bool`

HasEndedAt returns a boolean if a field has been set.

### GetFileName

`func (o *Video) GetFileName() string`

GetFileName returns the FileName field if non-nil, zero value otherwise.

### GetFileNameOk

`func (o *Video) GetFileNameOk() (*string, bool)`

GetFileNameOk returns a tuple with the FileName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFileName

`func (o *Video) SetFileName(v string)`

SetFileName sets FileName field to given value.

### HasFileName

`func (o *Video) HasFileName() bool`

HasFileName returns a boolean if a field has been set.

### GetFileSize

`func (o *Video) GetFileSize() float64`

GetFileSize returns the FileSize field if non-nil, zero value otherwise.

### GetFileSizeOk

`func (o *Video) GetFileSizeOk() (*float64, bool)`

GetFileSizeOk returns a tuple with the FileSize field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFileSize

`func (o *Video) SetFileSize(v float64)`

SetFileSize sets FileSize field to given value.

### HasFileSize

`func (o *Video) HasFileSize() bool`

HasFileSize returns a boolean if a field has been set.

### GetId

`func (o *Video) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *Video) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *Video) SetId(v string)`

SetId sets Id field to given value.

### HasId

`func (o *Video) HasId() bool`

HasId returns a boolean if a field has been set.

### GetObjectDetectorClaimedUntil

`func (o *Video) GetObjectDetectorClaimedUntil() time.Time`

GetObjectDetectorClaimedUntil returns the ObjectDetectorClaimedUntil field if non-nil, zero value otherwise.

### GetObjectDetectorClaimedUntilOk

`func (o *Video) GetObjectDetectorClaimedUntilOk() (*time.Time, bool)`

GetObjectDetectorClaimedUntilOk returns a tuple with the ObjectDetectorClaimedUntil field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetObjectDetectorClaimedUntil

`func (o *Video) SetObjectDetectorClaimedUntil(v time.Time)`

SetObjectDetectorClaimedUntil sets ObjectDetectorClaimedUntil field to given value.

### HasObjectDetectorClaimedUntil

`func (o *Video) HasObjectDetectorClaimedUntil() bool`

HasObjectDetectorClaimedUntil returns a boolean if a field has been set.

### GetObjectTrackerClaimedUntil

`func (o *Video) GetObjectTrackerClaimedUntil() time.Time`

GetObjectTrackerClaimedUntil returns the ObjectTrackerClaimedUntil field if non-nil, zero value otherwise.

### GetObjectTrackerClaimedUntilOk

`func (o *Video) GetObjectTrackerClaimedUntilOk() (*time.Time, bool)`

GetObjectTrackerClaimedUntilOk returns a tuple with the ObjectTrackerClaimedUntil field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetObjectTrackerClaimedUntil

`func (o *Video) SetObjectTrackerClaimedUntil(v time.Time)`

SetObjectTrackerClaimedUntil sets ObjectTrackerClaimedUntil field to given value.

### HasObjectTrackerClaimedUntil

`func (o *Video) HasObjectTrackerClaimedUntil() bool`

HasObjectTrackerClaimedUntil returns a boolean if a field has been set.

### GetReferencedByDetectionVideoIdObjects

`func (o *Video) GetReferencedByDetectionVideoIdObjects() []Detection`

GetReferencedByDetectionVideoIdObjects returns the ReferencedByDetectionVideoIdObjects field if non-nil, zero value otherwise.

### GetReferencedByDetectionVideoIdObjectsOk

`func (o *Video) GetReferencedByDetectionVideoIdObjectsOk() (*[]Detection, bool)`

GetReferencedByDetectionVideoIdObjectsOk returns a tuple with the ReferencedByDetectionVideoIdObjects field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReferencedByDetectionVideoIdObjects

`func (o *Video) SetReferencedByDetectionVideoIdObjects(v []Detection)`

SetReferencedByDetectionVideoIdObjects sets ReferencedByDetectionVideoIdObjects field to given value.

### HasReferencedByDetectionVideoIdObjects

`func (o *Video) HasReferencedByDetectionVideoIdObjects() bool`

HasReferencedByDetectionVideoIdObjects returns a boolean if a field has been set.

### SetReferencedByDetectionVideoIdObjectsNil

`func (o *Video) SetReferencedByDetectionVideoIdObjectsNil(b bool)`

 SetReferencedByDetectionVideoIdObjectsNil sets the value for ReferencedByDetectionVideoIdObjects to be an explicit nil

### UnsetReferencedByDetectionVideoIdObjects
`func (o *Video) UnsetReferencedByDetectionVideoIdObjects()`

UnsetReferencedByDetectionVideoIdObjects ensures that no value is present for ReferencedByDetectionVideoIdObjects, not even an explicit nil
### GetStartedAt

`func (o *Video) GetStartedAt() time.Time`

GetStartedAt returns the StartedAt field if non-nil, zero value otherwise.

### GetStartedAtOk

`func (o *Video) GetStartedAtOk() (*time.Time, bool)`

GetStartedAtOk returns a tuple with the StartedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStartedAt

`func (o *Video) SetStartedAt(v time.Time)`

SetStartedAt sets StartedAt field to given value.

### HasStartedAt

`func (o *Video) HasStartedAt() bool`

HasStartedAt returns a boolean if a field has been set.

### GetStatus

`func (o *Video) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *Video) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *Video) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *Video) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### GetThumbnailName

`func (o *Video) GetThumbnailName() string`

GetThumbnailName returns the ThumbnailName field if non-nil, zero value otherwise.

### GetThumbnailNameOk

`func (o *Video) GetThumbnailNameOk() (*string, bool)`

GetThumbnailNameOk returns a tuple with the ThumbnailName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetThumbnailName

`func (o *Video) SetThumbnailName(v string)`

SetThumbnailName sets ThumbnailName field to given value.

### HasThumbnailName

`func (o *Video) HasThumbnailName() bool`

HasThumbnailName returns a boolean if a field has been set.

### GetUpdatedAt

`func (o *Video) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *Video) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *Video) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *Video) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


