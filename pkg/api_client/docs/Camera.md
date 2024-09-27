# Camera

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **time.Time** |  | [optional] 
**DeletedAt** | Pointer to **time.Time** |  | [optional] 
**Id** | Pointer to **string** |  | [optional] 
**LastSeen** | Pointer to **time.Time** |  | [optional] 
**Name** | Pointer to **string** |  | [optional] 
**ReferencedByDetectionCameraIdObjects** | Pointer to [**[]Detection**](Detection.md) |  | [optional] 
**ReferencedByVideoCameraIdObjects** | Pointer to [**[]Video**](Video.md) |  | [optional] 
**SegmentProducerClaimedUntil** | Pointer to **time.Time** |  | [optional] 
**StreamProducerClaimedUntil** | Pointer to **time.Time** |  | [optional] 
**StreamUrl** | Pointer to **string** |  | [optional] 
**UpdatedAt** | Pointer to **time.Time** |  | [optional] 

## Methods

### NewCamera

`func NewCamera() *Camera`

NewCamera instantiates a new Camera object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCameraWithDefaults

`func NewCameraWithDefaults() *Camera`

NewCameraWithDefaults instantiates a new Camera object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *Camera) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *Camera) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *Camera) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *Camera) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetDeletedAt

`func (o *Camera) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *Camera) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *Camera) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *Camera) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### GetId

`func (o *Camera) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *Camera) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *Camera) SetId(v string)`

SetId sets Id field to given value.

### HasId

`func (o *Camera) HasId() bool`

HasId returns a boolean if a field has been set.

### GetLastSeen

`func (o *Camera) GetLastSeen() time.Time`

GetLastSeen returns the LastSeen field if non-nil, zero value otherwise.

### GetLastSeenOk

`func (o *Camera) GetLastSeenOk() (*time.Time, bool)`

GetLastSeenOk returns a tuple with the LastSeen field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastSeen

`func (o *Camera) SetLastSeen(v time.Time)`

SetLastSeen sets LastSeen field to given value.

### HasLastSeen

`func (o *Camera) HasLastSeen() bool`

HasLastSeen returns a boolean if a field has been set.

### GetName

`func (o *Camera) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *Camera) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *Camera) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *Camera) HasName() bool`

HasName returns a boolean if a field has been set.

### GetReferencedByDetectionCameraIdObjects

`func (o *Camera) GetReferencedByDetectionCameraIdObjects() []Detection`

GetReferencedByDetectionCameraIdObjects returns the ReferencedByDetectionCameraIdObjects field if non-nil, zero value otherwise.

### GetReferencedByDetectionCameraIdObjectsOk

`func (o *Camera) GetReferencedByDetectionCameraIdObjectsOk() (*[]Detection, bool)`

GetReferencedByDetectionCameraIdObjectsOk returns a tuple with the ReferencedByDetectionCameraIdObjects field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReferencedByDetectionCameraIdObjects

`func (o *Camera) SetReferencedByDetectionCameraIdObjects(v []Detection)`

SetReferencedByDetectionCameraIdObjects sets ReferencedByDetectionCameraIdObjects field to given value.

### HasReferencedByDetectionCameraIdObjects

`func (o *Camera) HasReferencedByDetectionCameraIdObjects() bool`

HasReferencedByDetectionCameraIdObjects returns a boolean if a field has been set.

### SetReferencedByDetectionCameraIdObjectsNil

`func (o *Camera) SetReferencedByDetectionCameraIdObjectsNil(b bool)`

 SetReferencedByDetectionCameraIdObjectsNil sets the value for ReferencedByDetectionCameraIdObjects to be an explicit nil

### UnsetReferencedByDetectionCameraIdObjects
`func (o *Camera) UnsetReferencedByDetectionCameraIdObjects()`

UnsetReferencedByDetectionCameraIdObjects ensures that no value is present for ReferencedByDetectionCameraIdObjects, not even an explicit nil
### GetReferencedByVideoCameraIdObjects

`func (o *Camera) GetReferencedByVideoCameraIdObjects() []Video`

GetReferencedByVideoCameraIdObjects returns the ReferencedByVideoCameraIdObjects field if non-nil, zero value otherwise.

### GetReferencedByVideoCameraIdObjectsOk

`func (o *Camera) GetReferencedByVideoCameraIdObjectsOk() (*[]Video, bool)`

GetReferencedByVideoCameraIdObjectsOk returns a tuple with the ReferencedByVideoCameraIdObjects field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReferencedByVideoCameraIdObjects

`func (o *Camera) SetReferencedByVideoCameraIdObjects(v []Video)`

SetReferencedByVideoCameraIdObjects sets ReferencedByVideoCameraIdObjects field to given value.

### HasReferencedByVideoCameraIdObjects

`func (o *Camera) HasReferencedByVideoCameraIdObjects() bool`

HasReferencedByVideoCameraIdObjects returns a boolean if a field has been set.

### SetReferencedByVideoCameraIdObjectsNil

`func (o *Camera) SetReferencedByVideoCameraIdObjectsNil(b bool)`

 SetReferencedByVideoCameraIdObjectsNil sets the value for ReferencedByVideoCameraIdObjects to be an explicit nil

### UnsetReferencedByVideoCameraIdObjects
`func (o *Camera) UnsetReferencedByVideoCameraIdObjects()`

UnsetReferencedByVideoCameraIdObjects ensures that no value is present for ReferencedByVideoCameraIdObjects, not even an explicit nil
### GetSegmentProducerClaimedUntil

`func (o *Camera) GetSegmentProducerClaimedUntil() time.Time`

GetSegmentProducerClaimedUntil returns the SegmentProducerClaimedUntil field if non-nil, zero value otherwise.

### GetSegmentProducerClaimedUntilOk

`func (o *Camera) GetSegmentProducerClaimedUntilOk() (*time.Time, bool)`

GetSegmentProducerClaimedUntilOk returns a tuple with the SegmentProducerClaimedUntil field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSegmentProducerClaimedUntil

`func (o *Camera) SetSegmentProducerClaimedUntil(v time.Time)`

SetSegmentProducerClaimedUntil sets SegmentProducerClaimedUntil field to given value.

### HasSegmentProducerClaimedUntil

`func (o *Camera) HasSegmentProducerClaimedUntil() bool`

HasSegmentProducerClaimedUntil returns a boolean if a field has been set.

### GetStreamProducerClaimedUntil

`func (o *Camera) GetStreamProducerClaimedUntil() time.Time`

GetStreamProducerClaimedUntil returns the StreamProducerClaimedUntil field if non-nil, zero value otherwise.

### GetStreamProducerClaimedUntilOk

`func (o *Camera) GetStreamProducerClaimedUntilOk() (*time.Time, bool)`

GetStreamProducerClaimedUntilOk returns a tuple with the StreamProducerClaimedUntil field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStreamProducerClaimedUntil

`func (o *Camera) SetStreamProducerClaimedUntil(v time.Time)`

SetStreamProducerClaimedUntil sets StreamProducerClaimedUntil field to given value.

### HasStreamProducerClaimedUntil

`func (o *Camera) HasStreamProducerClaimedUntil() bool`

HasStreamProducerClaimedUntil returns a boolean if a field has been set.

### GetStreamUrl

`func (o *Camera) GetStreamUrl() string`

GetStreamUrl returns the StreamUrl field if non-nil, zero value otherwise.

### GetStreamUrlOk

`func (o *Camera) GetStreamUrlOk() (*string, bool)`

GetStreamUrlOk returns a tuple with the StreamUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStreamUrl

`func (o *Camera) SetStreamUrl(v string)`

SetStreamUrl sets StreamUrl field to given value.

### HasStreamUrl

`func (o *Camera) HasStreamUrl() bool`

HasStreamUrl returns a boolean if a field has been set.

### GetUpdatedAt

`func (o *Camera) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *Camera) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *Camera) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *Camera) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


