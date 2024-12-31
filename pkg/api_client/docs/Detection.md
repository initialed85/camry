# Detection

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
**Id** | Pointer to **string** |  | [optional] 
**Score** | Pointer to **float64** |  | [optional] 
**SeenAt** | Pointer to **time.Time** |  | [optional] 
**UpdatedAt** | Pointer to **time.Time** |  | [optional] 
**VideoId** | Pointer to **string** |  | [optional] 
**VideoIdObject** | Pointer to [**Video**](Video.md) |  | [optional] 

## Methods

### NewDetection

`func NewDetection() *Detection`

NewDetection instantiates a new Detection object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDetectionWithDefaults

`func NewDetectionWithDefaults() *Detection`

NewDetectionWithDefaults instantiates a new Detection object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBoundingBox

`func (o *Detection) GetBoundingBox() []ArrayOfVec2Inner`

GetBoundingBox returns the BoundingBox field if non-nil, zero value otherwise.

### GetBoundingBoxOk

`func (o *Detection) GetBoundingBoxOk() (*[]ArrayOfVec2Inner, bool)`

GetBoundingBoxOk returns a tuple with the BoundingBox field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBoundingBox

`func (o *Detection) SetBoundingBox(v []ArrayOfVec2Inner)`

SetBoundingBox sets BoundingBox field to given value.

### HasBoundingBox

`func (o *Detection) HasBoundingBox() bool`

HasBoundingBox returns a boolean if a field has been set.

### GetCameraId

`func (o *Detection) GetCameraId() string`

GetCameraId returns the CameraId field if non-nil, zero value otherwise.

### GetCameraIdOk

`func (o *Detection) GetCameraIdOk() (*string, bool)`

GetCameraIdOk returns a tuple with the CameraId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCameraId

`func (o *Detection) SetCameraId(v string)`

SetCameraId sets CameraId field to given value.

### HasCameraId

`func (o *Detection) HasCameraId() bool`

HasCameraId returns a boolean if a field has been set.

### GetCameraIdObject

`func (o *Detection) GetCameraIdObject() Camera`

GetCameraIdObject returns the CameraIdObject field if non-nil, zero value otherwise.

### GetCameraIdObjectOk

`func (o *Detection) GetCameraIdObjectOk() (*Camera, bool)`

GetCameraIdObjectOk returns a tuple with the CameraIdObject field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCameraIdObject

`func (o *Detection) SetCameraIdObject(v Camera)`

SetCameraIdObject sets CameraIdObject field to given value.

### HasCameraIdObject

`func (o *Detection) HasCameraIdObject() bool`

HasCameraIdObject returns a boolean if a field has been set.

### GetCentroid

`func (o *Detection) GetCentroid() ArrayOfVec2Inner`

GetCentroid returns the Centroid field if non-nil, zero value otherwise.

### GetCentroidOk

`func (o *Detection) GetCentroidOk() (*ArrayOfVec2Inner, bool)`

GetCentroidOk returns a tuple with the Centroid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCentroid

`func (o *Detection) SetCentroid(v ArrayOfVec2Inner)`

SetCentroid sets Centroid field to given value.

### HasCentroid

`func (o *Detection) HasCentroid() bool`

HasCentroid returns a boolean if a field has been set.

### GetClassId

`func (o *Detection) GetClassId() int64`

GetClassId returns the ClassId field if non-nil, zero value otherwise.

### GetClassIdOk

`func (o *Detection) GetClassIdOk() (*int64, bool)`

GetClassIdOk returns a tuple with the ClassId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetClassId

`func (o *Detection) SetClassId(v int64)`

SetClassId sets ClassId field to given value.

### HasClassId

`func (o *Detection) HasClassId() bool`

HasClassId returns a boolean if a field has been set.

### GetClassName

`func (o *Detection) GetClassName() string`

GetClassName returns the ClassName field if non-nil, zero value otherwise.

### GetClassNameOk

`func (o *Detection) GetClassNameOk() (*string, bool)`

GetClassNameOk returns a tuple with the ClassName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetClassName

`func (o *Detection) SetClassName(v string)`

SetClassName sets ClassName field to given value.

### HasClassName

`func (o *Detection) HasClassName() bool`

HasClassName returns a boolean if a field has been set.

### GetCreatedAt

`func (o *Detection) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *Detection) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *Detection) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *Detection) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetDeletedAt

`func (o *Detection) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *Detection) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *Detection) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *Detection) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### GetId

`func (o *Detection) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *Detection) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *Detection) SetId(v string)`

SetId sets Id field to given value.

### HasId

`func (o *Detection) HasId() bool`

HasId returns a boolean if a field has been set.

### GetScore

`func (o *Detection) GetScore() float64`

GetScore returns the Score field if non-nil, zero value otherwise.

### GetScoreOk

`func (o *Detection) GetScoreOk() (*float64, bool)`

GetScoreOk returns a tuple with the Score field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScore

`func (o *Detection) SetScore(v float64)`

SetScore sets Score field to given value.

### HasScore

`func (o *Detection) HasScore() bool`

HasScore returns a boolean if a field has been set.

### GetSeenAt

`func (o *Detection) GetSeenAt() time.Time`

GetSeenAt returns the SeenAt field if non-nil, zero value otherwise.

### GetSeenAtOk

`func (o *Detection) GetSeenAtOk() (*time.Time, bool)`

GetSeenAtOk returns a tuple with the SeenAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSeenAt

`func (o *Detection) SetSeenAt(v time.Time)`

SetSeenAt sets SeenAt field to given value.

### HasSeenAt

`func (o *Detection) HasSeenAt() bool`

HasSeenAt returns a boolean if a field has been set.

### GetUpdatedAt

`func (o *Detection) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *Detection) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *Detection) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *Detection) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### GetVideoId

`func (o *Detection) GetVideoId() string`

GetVideoId returns the VideoId field if non-nil, zero value otherwise.

### GetVideoIdOk

`func (o *Detection) GetVideoIdOk() (*string, bool)`

GetVideoIdOk returns a tuple with the VideoId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVideoId

`func (o *Detection) SetVideoId(v string)`

SetVideoId sets VideoId field to given value.

### HasVideoId

`func (o *Detection) HasVideoId() bool`

HasVideoId returns a boolean if a field has been set.

### GetVideoIdObject

`func (o *Detection) GetVideoIdObject() Video`

GetVideoIdObject returns the VideoIdObject field if non-nil, zero value otherwise.

### GetVideoIdObjectOk

`func (o *Detection) GetVideoIdObjectOk() (*Video, bool)`

GetVideoIdObjectOk returns a tuple with the VideoIdObject field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVideoIdObject

`func (o *Detection) SetVideoIdObject(v Video)`

SetVideoIdObject sets VideoIdObject field to given value.

### HasVideoIdObject

`func (o *Detection) HasVideoIdObject() bool`

HasVideoIdObject returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


