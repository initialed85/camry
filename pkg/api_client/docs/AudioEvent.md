# AudioEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CameraId** | Pointer to **string** |  | [optional] 
**CameraIdObject** | Pointer to [**Camera**](Camera.md) |  | [optional] 
**CreatedAt** | Pointer to **time.Time** |  | [optional] 
**CrestDb** | Pointer to **float64** |  | [optional] 
**DeletedAt** | Pointer to **time.Time** |  | [optional] 
**DurationMs** | Pointer to **int64** |  | [optional] 
**EndedAt** | Pointer to **time.Time** |  | [optional] 
**FloorDbfs** | Pointer to **float64** |  | [optional] 
**Id** | Pointer to **string** |  | [optional] 
**Kind** | Pointer to **string** |  | [optional] 
**OffsetMs** | Pointer to **int64** |  | [optional] 
**PeakDbfs** | Pointer to **float64** |  | [optional] 
**ProminenceDb** | Pointer to **float64** |  | [optional] 
**RmsDbfs** | Pointer to **float64** |  | [optional] 
**Score** | Pointer to **float64** |  | [optional] 
**SeenAt** | Pointer to **time.Time** |  | [optional] 
**Spectral** | Pointer to **interface{}** |  | [optional] 
**UpdatedAt** | Pointer to **time.Time** |  | [optional] 
**VideoId** | Pointer to **string** |  | [optional] 
**VideoIdObject** | Pointer to [**Video**](Video.md) |  | [optional] 

## Methods

### NewAudioEvent

`func NewAudioEvent() *AudioEvent`

NewAudioEvent instantiates a new AudioEvent object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAudioEventWithDefaults

`func NewAudioEventWithDefaults() *AudioEvent`

NewAudioEventWithDefaults instantiates a new AudioEvent object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCameraId

`func (o *AudioEvent) GetCameraId() string`

GetCameraId returns the CameraId field if non-nil, zero value otherwise.

### GetCameraIdOk

`func (o *AudioEvent) GetCameraIdOk() (*string, bool)`

GetCameraIdOk returns a tuple with the CameraId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCameraId

`func (o *AudioEvent) SetCameraId(v string)`

SetCameraId sets CameraId field to given value.

### HasCameraId

`func (o *AudioEvent) HasCameraId() bool`

HasCameraId returns a boolean if a field has been set.

### GetCameraIdObject

`func (o *AudioEvent) GetCameraIdObject() Camera`

GetCameraIdObject returns the CameraIdObject field if non-nil, zero value otherwise.

### GetCameraIdObjectOk

`func (o *AudioEvent) GetCameraIdObjectOk() (*Camera, bool)`

GetCameraIdObjectOk returns a tuple with the CameraIdObject field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCameraIdObject

`func (o *AudioEvent) SetCameraIdObject(v Camera)`

SetCameraIdObject sets CameraIdObject field to given value.

### HasCameraIdObject

`func (o *AudioEvent) HasCameraIdObject() bool`

HasCameraIdObject returns a boolean if a field has been set.

### GetCreatedAt

`func (o *AudioEvent) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *AudioEvent) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *AudioEvent) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *AudioEvent) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetCrestDb

`func (o *AudioEvent) GetCrestDb() float64`

GetCrestDb returns the CrestDb field if non-nil, zero value otherwise.

### GetCrestDbOk

`func (o *AudioEvent) GetCrestDbOk() (*float64, bool)`

GetCrestDbOk returns a tuple with the CrestDb field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCrestDb

`func (o *AudioEvent) SetCrestDb(v float64)`

SetCrestDb sets CrestDb field to given value.

### HasCrestDb

`func (o *AudioEvent) HasCrestDb() bool`

HasCrestDb returns a boolean if a field has been set.

### GetDeletedAt

`func (o *AudioEvent) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *AudioEvent) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *AudioEvent) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *AudioEvent) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### GetDurationMs

`func (o *AudioEvent) GetDurationMs() int64`

GetDurationMs returns the DurationMs field if non-nil, zero value otherwise.

### GetDurationMsOk

`func (o *AudioEvent) GetDurationMsOk() (*int64, bool)`

GetDurationMsOk returns a tuple with the DurationMs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDurationMs

`func (o *AudioEvent) SetDurationMs(v int64)`

SetDurationMs sets DurationMs field to given value.

### HasDurationMs

`func (o *AudioEvent) HasDurationMs() bool`

HasDurationMs returns a boolean if a field has been set.

### GetEndedAt

`func (o *AudioEvent) GetEndedAt() time.Time`

GetEndedAt returns the EndedAt field if non-nil, zero value otherwise.

### GetEndedAtOk

`func (o *AudioEvent) GetEndedAtOk() (*time.Time, bool)`

GetEndedAtOk returns a tuple with the EndedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEndedAt

`func (o *AudioEvent) SetEndedAt(v time.Time)`

SetEndedAt sets EndedAt field to given value.

### HasEndedAt

`func (o *AudioEvent) HasEndedAt() bool`

HasEndedAt returns a boolean if a field has been set.

### GetFloorDbfs

`func (o *AudioEvent) GetFloorDbfs() float64`

GetFloorDbfs returns the FloorDbfs field if non-nil, zero value otherwise.

### GetFloorDbfsOk

`func (o *AudioEvent) GetFloorDbfsOk() (*float64, bool)`

GetFloorDbfsOk returns a tuple with the FloorDbfs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFloorDbfs

`func (o *AudioEvent) SetFloorDbfs(v float64)`

SetFloorDbfs sets FloorDbfs field to given value.

### HasFloorDbfs

`func (o *AudioEvent) HasFloorDbfs() bool`

HasFloorDbfs returns a boolean if a field has been set.

### GetId

`func (o *AudioEvent) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *AudioEvent) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *AudioEvent) SetId(v string)`

SetId sets Id field to given value.

### HasId

`func (o *AudioEvent) HasId() bool`

HasId returns a boolean if a field has been set.

### GetKind

`func (o *AudioEvent) GetKind() string`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *AudioEvent) GetKindOk() (*string, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *AudioEvent) SetKind(v string)`

SetKind sets Kind field to given value.

### HasKind

`func (o *AudioEvent) HasKind() bool`

HasKind returns a boolean if a field has been set.

### GetOffsetMs

`func (o *AudioEvent) GetOffsetMs() int64`

GetOffsetMs returns the OffsetMs field if non-nil, zero value otherwise.

### GetOffsetMsOk

`func (o *AudioEvent) GetOffsetMsOk() (*int64, bool)`

GetOffsetMsOk returns a tuple with the OffsetMs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOffsetMs

`func (o *AudioEvent) SetOffsetMs(v int64)`

SetOffsetMs sets OffsetMs field to given value.

### HasOffsetMs

`func (o *AudioEvent) HasOffsetMs() bool`

HasOffsetMs returns a boolean if a field has been set.

### GetPeakDbfs

`func (o *AudioEvent) GetPeakDbfs() float64`

GetPeakDbfs returns the PeakDbfs field if non-nil, zero value otherwise.

### GetPeakDbfsOk

`func (o *AudioEvent) GetPeakDbfsOk() (*float64, bool)`

GetPeakDbfsOk returns a tuple with the PeakDbfs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPeakDbfs

`func (o *AudioEvent) SetPeakDbfs(v float64)`

SetPeakDbfs sets PeakDbfs field to given value.

### HasPeakDbfs

`func (o *AudioEvent) HasPeakDbfs() bool`

HasPeakDbfs returns a boolean if a field has been set.

### GetProminenceDb

`func (o *AudioEvent) GetProminenceDb() float64`

GetProminenceDb returns the ProminenceDb field if non-nil, zero value otherwise.

### GetProminenceDbOk

`func (o *AudioEvent) GetProminenceDbOk() (*float64, bool)`

GetProminenceDbOk returns a tuple with the ProminenceDb field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProminenceDb

`func (o *AudioEvent) SetProminenceDb(v float64)`

SetProminenceDb sets ProminenceDb field to given value.

### HasProminenceDb

`func (o *AudioEvent) HasProminenceDb() bool`

HasProminenceDb returns a boolean if a field has been set.

### GetRmsDbfs

`func (o *AudioEvent) GetRmsDbfs() float64`

GetRmsDbfs returns the RmsDbfs field if non-nil, zero value otherwise.

### GetRmsDbfsOk

`func (o *AudioEvent) GetRmsDbfsOk() (*float64, bool)`

GetRmsDbfsOk returns a tuple with the RmsDbfs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRmsDbfs

`func (o *AudioEvent) SetRmsDbfs(v float64)`

SetRmsDbfs sets RmsDbfs field to given value.

### HasRmsDbfs

`func (o *AudioEvent) HasRmsDbfs() bool`

HasRmsDbfs returns a boolean if a field has been set.

### GetScore

`func (o *AudioEvent) GetScore() float64`

GetScore returns the Score field if non-nil, zero value otherwise.

### GetScoreOk

`func (o *AudioEvent) GetScoreOk() (*float64, bool)`

GetScoreOk returns a tuple with the Score field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScore

`func (o *AudioEvent) SetScore(v float64)`

SetScore sets Score field to given value.

### HasScore

`func (o *AudioEvent) HasScore() bool`

HasScore returns a boolean if a field has been set.

### GetSeenAt

`func (o *AudioEvent) GetSeenAt() time.Time`

GetSeenAt returns the SeenAt field if non-nil, zero value otherwise.

### GetSeenAtOk

`func (o *AudioEvent) GetSeenAtOk() (*time.Time, bool)`

GetSeenAtOk returns a tuple with the SeenAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSeenAt

`func (o *AudioEvent) SetSeenAt(v time.Time)`

SetSeenAt sets SeenAt field to given value.

### HasSeenAt

`func (o *AudioEvent) HasSeenAt() bool`

HasSeenAt returns a boolean if a field has been set.

### GetSpectral

`func (o *AudioEvent) GetSpectral() interface{}`

GetSpectral returns the Spectral field if non-nil, zero value otherwise.

### GetSpectralOk

`func (o *AudioEvent) GetSpectralOk() (*interface{}, bool)`

GetSpectralOk returns a tuple with the Spectral field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSpectral

`func (o *AudioEvent) SetSpectral(v interface{})`

SetSpectral sets Spectral field to given value.

### HasSpectral

`func (o *AudioEvent) HasSpectral() bool`

HasSpectral returns a boolean if a field has been set.

### SetSpectralNil

`func (o *AudioEvent) SetSpectralNil(b bool)`

 SetSpectralNil sets the value for Spectral to be an explicit nil

### UnsetSpectral
`func (o *AudioEvent) UnsetSpectral()`

UnsetSpectral ensures that no value is present for Spectral, not even an explicit nil
### GetUpdatedAt

`func (o *AudioEvent) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *AudioEvent) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *AudioEvent) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *AudioEvent) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### GetVideoId

`func (o *AudioEvent) GetVideoId() string`

GetVideoId returns the VideoId field if non-nil, zero value otherwise.

### GetVideoIdOk

`func (o *AudioEvent) GetVideoIdOk() (*string, bool)`

GetVideoIdOk returns a tuple with the VideoId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVideoId

`func (o *AudioEvent) SetVideoId(v string)`

SetVideoId sets VideoId field to given value.

### HasVideoId

`func (o *AudioEvent) HasVideoId() bool`

HasVideoId returns a boolean if a field has been set.

### GetVideoIdObject

`func (o *AudioEvent) GetVideoIdObject() Video`

GetVideoIdObject returns the VideoIdObject field if non-nil, zero value otherwise.

### GetVideoIdObjectOk

`func (o *AudioEvent) GetVideoIdObjectOk() (*Video, bool)`

GetVideoIdObjectOk returns a tuple with the VideoIdObject field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVideoIdObject

`func (o *AudioEvent) SetVideoIdObject(v Video)`

SetVideoIdObject sets VideoIdObject field to given value.

### HasVideoIdObject

`func (o *AudioEvent) HasVideoIdObject() bool`

HasVideoIdObject returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


