# CameraSegmentProducerClaimRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**TimeoutSeconds** | Pointer to **float64** |  | [optional] 
**Until** | Pointer to **time.Time** |  | [optional] 

## Methods

### NewCameraSegmentProducerClaimRequest

`func NewCameraSegmentProducerClaimRequest() *CameraSegmentProducerClaimRequest`

NewCameraSegmentProducerClaimRequest instantiates a new CameraSegmentProducerClaimRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCameraSegmentProducerClaimRequestWithDefaults

`func NewCameraSegmentProducerClaimRequestWithDefaults() *CameraSegmentProducerClaimRequest`

NewCameraSegmentProducerClaimRequestWithDefaults instantiates a new CameraSegmentProducerClaimRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetTimeoutSeconds

`func (o *CameraSegmentProducerClaimRequest) GetTimeoutSeconds() float64`

GetTimeoutSeconds returns the TimeoutSeconds field if non-nil, zero value otherwise.

### GetTimeoutSecondsOk

`func (o *CameraSegmentProducerClaimRequest) GetTimeoutSecondsOk() (*float64, bool)`

GetTimeoutSecondsOk returns a tuple with the TimeoutSeconds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimeoutSeconds

`func (o *CameraSegmentProducerClaimRequest) SetTimeoutSeconds(v float64)`

SetTimeoutSeconds sets TimeoutSeconds field to given value.

### HasTimeoutSeconds

`func (o *CameraSegmentProducerClaimRequest) HasTimeoutSeconds() bool`

HasTimeoutSeconds returns a boolean if a field has been set.

### GetUntil

`func (o *CameraSegmentProducerClaimRequest) GetUntil() time.Time`

GetUntil returns the Until field if non-nil, zero value otherwise.

### GetUntilOk

`func (o *CameraSegmentProducerClaimRequest) GetUntilOk() (*time.Time, bool)`

GetUntilOk returns a tuple with the Until field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUntil

`func (o *CameraSegmentProducerClaimRequest) SetUntil(v time.Time)`

SetUntil sets Until field to given value.

### HasUntil

`func (o *CameraSegmentProducerClaimRequest) HasUntil() bool`

HasUntil returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


