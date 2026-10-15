# Track

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BoundingBox** | Pointer to [**[]ArrayOfVec2Inner**](ArrayOfVec2Inner.md) |  | [optional]
**CameraId** | Pointer to **string** |  | [optional]
**CameraIdObject** | Pointer to [**Camera**](Camera.md) |  | [optional]
**Centroid** | Pointer to [**ArrayOfVec2Inner**](ArrayOfVec2Inner.md) |  | [optional]
**ClassId** | Pointer to **int64** |  | [optional]
**ClassName** | Pointer to **string** |  | [optional]
**CreatedAt** | Pointer to **time.Time** |  | [optional]
**DeletedAt** | Pointer to **time.Time** |  | [optional]
**EndedAt** | Pointer to **time.Time** |  | [optional]
**FrameCount** | Pointer to **int64** |  | [optional]
**Id** | Pointer to **string** |  | [optional]
**IsStatic** | Pointer to **bool** |  | [optional]
**Key** | Pointer to **string** |  | [optional]
**MaxScore** | Pointer to **float64** |  | [optional]
**MeanScore** | Pointer to **float64** |  | [optional]
**ReferencedByDetectionTrackIdObjects** | Pointer to [**[]Detection**](Detection.md) |  | [optional]
**StartedAt** | Pointer to **time.Time** |  | [optional]
**Status** | Pointer to **string** |  | [optional]
**Trajectory** | Pointer to **interface{}** |  | [optional]
**UpdatedAt** | Pointer to **time.Time** |  | [optional]
**VideoId** | Pointer to **string** |  | [optional]
**VideoIdObject** | Pointer to [**Video**](Video.md) |  | [optional]

## Methods

### NewTrack

`func NewTrack() *Track`

NewTrack instantiates a new Track object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTrackWithDefaults

`func NewTrackWithDefaults() *Track`

NewTrackWithDefaults instantiates a new Track object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBoundingBox

`func (o *Track) GetBoundingBox() []ArrayOfVec2Inner`

GetBoundingBox returns the BoundingBox field if non-nil, zero value otherwise.

### GetBoundingBoxOk

`func (o *Track) GetBoundingBoxOk() (*[]ArrayOfVec2Inner, bool)`

GetBoundingBoxOk returns a tuple with the BoundingBox field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBoundingBox

`func (o *Track) SetBoundingBox(v []ArrayOfVec2Inner)`

SetBoundingBox sets BoundingBox field to given value.

### HasBoundingBox

`func (o *Track) HasBoundingBox() bool`

HasBoundingBox returns a boolean if a field has been set.

### GetCameraId

`func (o *Track) GetCameraId() string`

GetCameraId returns the CameraId field if non-nil, zero value otherwise.

### GetCameraIdOk

`func (o *Track) GetCameraIdOk() (*string, bool)`

GetCameraIdOk returns a tuple with the CameraId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCameraId

`func (o *Track) SetCameraId(v string)`

SetCameraId sets CameraId field to given value.

### HasCameraId

`func (o *Track) HasCameraId() bool`

HasCameraId returns a boolean if a field has been set.

### GetCameraIdObject

`func (o *Track) GetCameraIdObject() Camera`

GetCameraIdObject returns the CameraIdObject field if non-nil, zero value otherwise.

### GetCameraIdObjectOk

`func (o *Track) GetCameraIdObjectOk() (*Camera, bool)`

GetCameraIdObjectOk returns a tuple with the CameraIdObject field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCameraIdObject

`func (o *Track) SetCameraIdObject(v Camera)`

SetCameraIdObject sets CameraIdObject field to given value.

### HasCameraIdObject

`func (o *Track) HasCameraIdObject() bool`

HasCameraIdObject returns a boolean if a field has been set.

### GetCentroid

`func (o *Track) GetCentroid() ArrayOfVec2Inner`

GetCentroid returns the Centroid field if non-nil, zero value otherwise.

### GetCentroidOk

`func (o *Track) GetCentroidOk() (*ArrayOfVec2Inner, bool)`

GetCentroidOk returns a tuple with the Centroid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCentroid

`func (o *Track) SetCentroid(v ArrayOfVec2Inner)`

SetCentroid sets Centroid field to given value.

### HasCentroid

`func (o *Track) HasCentroid() bool`

HasCentroid returns a boolean if a field has been set.

### GetClassId

`func (o *Track) GetClassId() int64`

GetClassId returns the ClassId field if non-nil, zero value otherwise.

### GetClassIdOk

`func (o *Track) GetClassIdOk() (*int64, bool)`

GetClassIdOk returns a tuple with the ClassId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetClassId

`func (o *Track) SetClassId(v int64)`

SetClassId sets ClassId field to given value.

### HasClassId

`func (o *Track) HasClassId() bool`

HasClassId returns a boolean if a field has been set.

### GetClassName

`func (o *Track) GetClassName() string`

GetClassName returns the ClassName field if non-nil, zero value otherwise.

### GetClassNameOk

`func (o *Track) GetClassNameOk() (*string, bool)`

GetClassNameOk returns a tuple with the ClassName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetClassName

`func (o *Track) SetClassName(v string)`

SetClassName sets ClassName field to given value.

### HasClassName

`func (o *Track) HasClassName() bool`

HasClassName returns a boolean if a field has been set.

### GetCreatedAt

`func (o *Track) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *Track) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *Track) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *Track) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetDeletedAt

`func (o *Track) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *Track) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *Track) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *Track) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### GetEndedAt

`func (o *Track) GetEndedAt() time.Time`

GetEndedAt returns the EndedAt field if non-nil, zero value otherwise.

### GetEndedAtOk

`func (o *Track) GetEndedAtOk() (*time.Time, bool)`

GetEndedAtOk returns a tuple with the EndedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEndedAt

`func (o *Track) SetEndedAt(v time.Time)`

SetEndedAt sets EndedAt field to given value.

### HasEndedAt

`func (o *Track) HasEndedAt() bool`

HasEndedAt returns a boolean if a field has been set.

### GetFrameCount

`func (o *Track) GetFrameCount() int64`

GetFrameCount returns the FrameCount field if non-nil, zero value otherwise.

### GetFrameCountOk

`func (o *Track) GetFrameCountOk() (*int64, bool)`

GetFrameCountOk returns a tuple with the FrameCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFrameCount

`func (o *Track) SetFrameCount(v int64)`

SetFrameCount sets FrameCount field to given value.

### HasFrameCount

`func (o *Track) HasFrameCount() bool`

HasFrameCount returns a boolean if a field has been set.

### GetId

`func (o *Track) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *Track) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *Track) SetId(v string)`

SetId sets Id field to given value.

### HasId

`func (o *Track) HasId() bool`

HasId returns a boolean if a field has been set.

### GetIsStatic

`func (o *Track) GetIsStatic() bool`

GetIsStatic returns the IsStatic field if non-nil, zero value otherwise.

### GetIsStaticOk

`func (o *Track) GetIsStaticOk() (*bool, bool)`

GetIsStaticOk returns a tuple with the IsStatic field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsStatic

`func (o *Track) SetIsStatic(v bool)`

SetIsStatic sets IsStatic field to given value.

### HasIsStatic

`func (o *Track) HasIsStatic() bool`

HasIsStatic returns a boolean if a field has been set.

### GetKey

`func (o *Track) GetKey() string`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *Track) GetKeyOk() (*string, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *Track) SetKey(v string)`

SetKey sets Key field to given value.

### HasKey

`func (o *Track) HasKey() bool`

HasKey returns a boolean if a field has been set.

### GetMaxScore

`func (o *Track) GetMaxScore() float64`

GetMaxScore returns the MaxScore field if non-nil, zero value otherwise.

### GetMaxScoreOk

`func (o *Track) GetMaxScoreOk() (*float64, bool)`

GetMaxScoreOk returns a tuple with the MaxScore field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxScore

`func (o *Track) SetMaxScore(v float64)`

SetMaxScore sets MaxScore field to given value.

### HasMaxScore

`func (o *Track) HasMaxScore() bool`

HasMaxScore returns a boolean if a field has been set.

### GetMeanScore

`func (o *Track) GetMeanScore() float64`

GetMeanScore returns the MeanScore field if non-nil, zero value otherwise.

### GetMeanScoreOk

`func (o *Track) GetMeanScoreOk() (*float64, bool)`

GetMeanScoreOk returns a tuple with the MeanScore field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMeanScore

`func (o *Track) SetMeanScore(v float64)`

SetMeanScore sets MeanScore field to given value.

### HasMeanScore

`func (o *Track) HasMeanScore() bool`

HasMeanScore returns a boolean if a field has been set.

### GetReferencedByDetectionTrackIdObjects

`func (o *Track) GetReferencedByDetectionTrackIdObjects() []Detection`

GetReferencedByDetectionTrackIdObjects returns the ReferencedByDetectionTrackIdObjects field if non-nil, zero value otherwise.

### GetReferencedByDetectionTrackIdObjectsOk

`func (o *Track) GetReferencedByDetectionTrackIdObjectsOk() (*[]Detection, bool)`

GetReferencedByDetectionTrackIdObjectsOk returns a tuple with the ReferencedByDetectionTrackIdObjects field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReferencedByDetectionTrackIdObjects

`func (o *Track) SetReferencedByDetectionTrackIdObjects(v []Detection)`

SetReferencedByDetectionTrackIdObjects sets ReferencedByDetectionTrackIdObjects field to given value.

### HasReferencedByDetectionTrackIdObjects

`func (o *Track) HasReferencedByDetectionTrackIdObjects() bool`

HasReferencedByDetectionTrackIdObjects returns a boolean if a field has been set.

### GetStartedAt

`func (o *Track) GetStartedAt() time.Time`

GetStartedAt returns the StartedAt field if non-nil, zero value otherwise.

### GetStartedAtOk

`func (o *Track) GetStartedAtOk() (*time.Time, bool)`

GetStartedAtOk returns a tuple with the StartedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStartedAt

`func (o *Track) SetStartedAt(v time.Time)`

SetStartedAt sets StartedAt field to given value.

### HasStartedAt

`func (o *Track) HasStartedAt() bool`

HasStartedAt returns a boolean if a field has been set.

### GetStatus

`func (o *Track) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *Track) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *Track) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *Track) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### GetTrajectory

`func (o *Track) GetTrajectory() interface{}`

GetTrajectory returns the Trajectory field if non-nil, zero value otherwise.

### GetTrajectoryOk

`func (o *Track) GetTrajectoryOk() (*interface{}, bool)`

GetTrajectoryOk returns a tuple with the Trajectory field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTrajectory

`func (o *Track) SetTrajectory(v interface{})`

SetTrajectory sets Trajectory field to given value.

### HasTrajectory

`func (o *Track) HasTrajectory() bool`

HasTrajectory returns a boolean if a field has been set.

### SetTrajectoryNil

`func (o *Track) SetTrajectoryNil(b bool)`

 SetTrajectoryNil sets the value for Trajectory to be an explicit nil

### UnsetTrajectory
`func (o *Track) UnsetTrajectory()`

UnsetTrajectory ensures that no value is present for Trajectory, not even an explicit nil
### GetUpdatedAt

`func (o *Track) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *Track) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *Track) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *Track) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### GetVideoId

`func (o *Track) GetVideoId() string`

GetVideoId returns the VideoId field if non-nil, zero value otherwise.

### GetVideoIdOk

`func (o *Track) GetVideoIdOk() (*string, bool)`

GetVideoIdOk returns a tuple with the VideoId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVideoId

`func (o *Track) SetVideoId(v string)`

SetVideoId sets VideoId field to given value.

### HasVideoId

`func (o *Track) HasVideoId() bool`

HasVideoId returns a boolean if a field has been set.

### GetVideoIdObject

`func (o *Track) GetVideoIdObject() Video`

GetVideoIdObject returns the VideoIdObject field if non-nil, zero value otherwise.

### GetVideoIdObjectOk

`func (o *Track) GetVideoIdObjectOk() (*Video, bool)`

GetVideoIdObjectOk returns a tuple with the VideoIdObject field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVideoIdObject

`func (o *Track) SetVideoIdObject(v Video)`

SetVideoIdObject sets VideoIdObject field to given value.

### HasVideoIdObject

`func (o *Track) HasVideoIdObject() bool`

HasVideoIdObject returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


