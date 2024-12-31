/*
Djangolang

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

API version: 1.0
*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package api_client

import (
	"encoding/json"
	"time"
)

// checks if the Detection type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &Detection{}

// Detection struct for Detection
type Detection struct {
	BoundingBox []ArrayOfVec2Inner `json:"bounding_box,omitempty"`
	CameraId *string `json:"camera_id,omitempty"`
	CameraIdObject *Camera `json:"camera_id_object,omitempty"`
	Centroid *ArrayOfVec2Inner `json:"centroid,omitempty"`
	ClassId *int64 `json:"class_id,omitempty"`
	ClassName *string `json:"class_name,omitempty"`
	CreatedAt *time.Time `json:"created_at,omitempty"`
	DeletedAt *time.Time `json:"deleted_at,omitempty"`
	Id *string `json:"id,omitempty"`
	Score *float64 `json:"score,omitempty"`
	SeenAt *time.Time `json:"seen_at,omitempty"`
	UpdatedAt *time.Time `json:"updated_at,omitempty"`
	VideoId *string `json:"video_id,omitempty"`
	VideoIdObject *Video `json:"video_id_object,omitempty"`
}

// NewDetection instantiates a new Detection object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewDetection() *Detection {
	this := Detection{}
	return &this
}

// NewDetectionWithDefaults instantiates a new Detection object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewDetectionWithDefaults() *Detection {
	this := Detection{}
	return &this
}

// GetBoundingBox returns the BoundingBox field value if set, zero value otherwise.
func (o *Detection) GetBoundingBox() []ArrayOfVec2Inner {
	if o == nil || IsNil(o.BoundingBox) {
		var ret []ArrayOfVec2Inner
		return ret
	}
	return o.BoundingBox
}

// GetBoundingBoxOk returns a tuple with the BoundingBox field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetBoundingBoxOk() ([]ArrayOfVec2Inner, bool) {
	if o == nil || IsNil(o.BoundingBox) {
		return nil, false
	}
	return o.BoundingBox, true
}

// HasBoundingBox returns a boolean if a field has been set.
func (o *Detection) HasBoundingBox() bool {
	if o != nil && !IsNil(o.BoundingBox) {
		return true
	}

	return false
}

// SetBoundingBox gets a reference to the given []ArrayOfVec2Inner and assigns it to the BoundingBox field.
func (o *Detection) SetBoundingBox(v []ArrayOfVec2Inner) {
	o.BoundingBox = v
}

// GetCameraId returns the CameraId field value if set, zero value otherwise.
func (o *Detection) GetCameraId() string {
	if o == nil || IsNil(o.CameraId) {
		var ret string
		return ret
	}
	return *o.CameraId
}

// GetCameraIdOk returns a tuple with the CameraId field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetCameraIdOk() (*string, bool) {
	if o == nil || IsNil(o.CameraId) {
		return nil, false
	}
	return o.CameraId, true
}

// HasCameraId returns a boolean if a field has been set.
func (o *Detection) HasCameraId() bool {
	if o != nil && !IsNil(o.CameraId) {
		return true
	}

	return false
}

// SetCameraId gets a reference to the given string and assigns it to the CameraId field.
func (o *Detection) SetCameraId(v string) {
	o.CameraId = &v
}

// GetCameraIdObject returns the CameraIdObject field value if set, zero value otherwise.
func (o *Detection) GetCameraIdObject() Camera {
	if o == nil || IsNil(o.CameraIdObject) {
		var ret Camera
		return ret
	}
	return *o.CameraIdObject
}

// GetCameraIdObjectOk returns a tuple with the CameraIdObject field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetCameraIdObjectOk() (*Camera, bool) {
	if o == nil || IsNil(o.CameraIdObject) {
		return nil, false
	}
	return o.CameraIdObject, true
}

// HasCameraIdObject returns a boolean if a field has been set.
func (o *Detection) HasCameraIdObject() bool {
	if o != nil && !IsNil(o.CameraIdObject) {
		return true
	}

	return false
}

// SetCameraIdObject gets a reference to the given Camera and assigns it to the CameraIdObject field.
func (o *Detection) SetCameraIdObject(v Camera) {
	o.CameraIdObject = &v
}

// GetCentroid returns the Centroid field value if set, zero value otherwise.
func (o *Detection) GetCentroid() ArrayOfVec2Inner {
	if o == nil || IsNil(o.Centroid) {
		var ret ArrayOfVec2Inner
		return ret
	}
	return *o.Centroid
}

// GetCentroidOk returns a tuple with the Centroid field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetCentroidOk() (*ArrayOfVec2Inner, bool) {
	if o == nil || IsNil(o.Centroid) {
		return nil, false
	}
	return o.Centroid, true
}

// HasCentroid returns a boolean if a field has been set.
func (o *Detection) HasCentroid() bool {
	if o != nil && !IsNil(o.Centroid) {
		return true
	}

	return false
}

// SetCentroid gets a reference to the given ArrayOfVec2Inner and assigns it to the Centroid field.
func (o *Detection) SetCentroid(v ArrayOfVec2Inner) {
	o.Centroid = &v
}

// GetClassId returns the ClassId field value if set, zero value otherwise.
func (o *Detection) GetClassId() int64 {
	if o == nil || IsNil(o.ClassId) {
		var ret int64
		return ret
	}
	return *o.ClassId
}

// GetClassIdOk returns a tuple with the ClassId field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetClassIdOk() (*int64, bool) {
	if o == nil || IsNil(o.ClassId) {
		return nil, false
	}
	return o.ClassId, true
}

// HasClassId returns a boolean if a field has been set.
func (o *Detection) HasClassId() bool {
	if o != nil && !IsNil(o.ClassId) {
		return true
	}

	return false
}

// SetClassId gets a reference to the given int64 and assigns it to the ClassId field.
func (o *Detection) SetClassId(v int64) {
	o.ClassId = &v
}

// GetClassName returns the ClassName field value if set, zero value otherwise.
func (o *Detection) GetClassName() string {
	if o == nil || IsNil(o.ClassName) {
		var ret string
		return ret
	}
	return *o.ClassName
}

// GetClassNameOk returns a tuple with the ClassName field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetClassNameOk() (*string, bool) {
	if o == nil || IsNil(o.ClassName) {
		return nil, false
	}
	return o.ClassName, true
}

// HasClassName returns a boolean if a field has been set.
func (o *Detection) HasClassName() bool {
	if o != nil && !IsNil(o.ClassName) {
		return true
	}

	return false
}

// SetClassName gets a reference to the given string and assigns it to the ClassName field.
func (o *Detection) SetClassName(v string) {
	o.ClassName = &v
}

// GetCreatedAt returns the CreatedAt field value if set, zero value otherwise.
func (o *Detection) GetCreatedAt() time.Time {
	if o == nil || IsNil(o.CreatedAt) {
		var ret time.Time
		return ret
	}
	return *o.CreatedAt
}

// GetCreatedAtOk returns a tuple with the CreatedAt field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetCreatedAtOk() (*time.Time, bool) {
	if o == nil || IsNil(o.CreatedAt) {
		return nil, false
	}
	return o.CreatedAt, true
}

// HasCreatedAt returns a boolean if a field has been set.
func (o *Detection) HasCreatedAt() bool {
	if o != nil && !IsNil(o.CreatedAt) {
		return true
	}

	return false
}

// SetCreatedAt gets a reference to the given time.Time and assigns it to the CreatedAt field.
func (o *Detection) SetCreatedAt(v time.Time) {
	o.CreatedAt = &v
}

// GetDeletedAt returns the DeletedAt field value if set, zero value otherwise.
func (o *Detection) GetDeletedAt() time.Time {
	if o == nil || IsNil(o.DeletedAt) {
		var ret time.Time
		return ret
	}
	return *o.DeletedAt
}

// GetDeletedAtOk returns a tuple with the DeletedAt field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetDeletedAtOk() (*time.Time, bool) {
	if o == nil || IsNil(o.DeletedAt) {
		return nil, false
	}
	return o.DeletedAt, true
}

// HasDeletedAt returns a boolean if a field has been set.
func (o *Detection) HasDeletedAt() bool {
	if o != nil && !IsNil(o.DeletedAt) {
		return true
	}

	return false
}

// SetDeletedAt gets a reference to the given time.Time and assigns it to the DeletedAt field.
func (o *Detection) SetDeletedAt(v time.Time) {
	o.DeletedAt = &v
}

// GetId returns the Id field value if set, zero value otherwise.
func (o *Detection) GetId() string {
	if o == nil || IsNil(o.Id) {
		var ret string
		return ret
	}
	return *o.Id
}

// GetIdOk returns a tuple with the Id field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetIdOk() (*string, bool) {
	if o == nil || IsNil(o.Id) {
		return nil, false
	}
	return o.Id, true
}

// HasId returns a boolean if a field has been set.
func (o *Detection) HasId() bool {
	if o != nil && !IsNil(o.Id) {
		return true
	}

	return false
}

// SetId gets a reference to the given string and assigns it to the Id field.
func (o *Detection) SetId(v string) {
	o.Id = &v
}

// GetScore returns the Score field value if set, zero value otherwise.
func (o *Detection) GetScore() float64 {
	if o == nil || IsNil(o.Score) {
		var ret float64
		return ret
	}
	return *o.Score
}

// GetScoreOk returns a tuple with the Score field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetScoreOk() (*float64, bool) {
	if o == nil || IsNil(o.Score) {
		return nil, false
	}
	return o.Score, true
}

// HasScore returns a boolean if a field has been set.
func (o *Detection) HasScore() bool {
	if o != nil && !IsNil(o.Score) {
		return true
	}

	return false
}

// SetScore gets a reference to the given float64 and assigns it to the Score field.
func (o *Detection) SetScore(v float64) {
	o.Score = &v
}

// GetSeenAt returns the SeenAt field value if set, zero value otherwise.
func (o *Detection) GetSeenAt() time.Time {
	if o == nil || IsNil(o.SeenAt) {
		var ret time.Time
		return ret
	}
	return *o.SeenAt
}

// GetSeenAtOk returns a tuple with the SeenAt field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetSeenAtOk() (*time.Time, bool) {
	if o == nil || IsNil(o.SeenAt) {
		return nil, false
	}
	return o.SeenAt, true
}

// HasSeenAt returns a boolean if a field has been set.
func (o *Detection) HasSeenAt() bool {
	if o != nil && !IsNil(o.SeenAt) {
		return true
	}

	return false
}

// SetSeenAt gets a reference to the given time.Time and assigns it to the SeenAt field.
func (o *Detection) SetSeenAt(v time.Time) {
	o.SeenAt = &v
}

// GetUpdatedAt returns the UpdatedAt field value if set, zero value otherwise.
func (o *Detection) GetUpdatedAt() time.Time {
	if o == nil || IsNil(o.UpdatedAt) {
		var ret time.Time
		return ret
	}
	return *o.UpdatedAt
}

// GetUpdatedAtOk returns a tuple with the UpdatedAt field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetUpdatedAtOk() (*time.Time, bool) {
	if o == nil || IsNil(o.UpdatedAt) {
		return nil, false
	}
	return o.UpdatedAt, true
}

// HasUpdatedAt returns a boolean if a field has been set.
func (o *Detection) HasUpdatedAt() bool {
	if o != nil && !IsNil(o.UpdatedAt) {
		return true
	}

	return false
}

// SetUpdatedAt gets a reference to the given time.Time and assigns it to the UpdatedAt field.
func (o *Detection) SetUpdatedAt(v time.Time) {
	o.UpdatedAt = &v
}

// GetVideoId returns the VideoId field value if set, zero value otherwise.
func (o *Detection) GetVideoId() string {
	if o == nil || IsNil(o.VideoId) {
		var ret string
		return ret
	}
	return *o.VideoId
}

// GetVideoIdOk returns a tuple with the VideoId field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetVideoIdOk() (*string, bool) {
	if o == nil || IsNil(o.VideoId) {
		return nil, false
	}
	return o.VideoId, true
}

// HasVideoId returns a boolean if a field has been set.
func (o *Detection) HasVideoId() bool {
	if o != nil && !IsNil(o.VideoId) {
		return true
	}

	return false
}

// SetVideoId gets a reference to the given string and assigns it to the VideoId field.
func (o *Detection) SetVideoId(v string) {
	o.VideoId = &v
}

// GetVideoIdObject returns the VideoIdObject field value if set, zero value otherwise.
func (o *Detection) GetVideoIdObject() Video {
	if o == nil || IsNil(o.VideoIdObject) {
		var ret Video
		return ret
	}
	return *o.VideoIdObject
}

// GetVideoIdObjectOk returns a tuple with the VideoIdObject field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *Detection) GetVideoIdObjectOk() (*Video, bool) {
	if o == nil || IsNil(o.VideoIdObject) {
		return nil, false
	}
	return o.VideoIdObject, true
}

// HasVideoIdObject returns a boolean if a field has been set.
func (o *Detection) HasVideoIdObject() bool {
	if o != nil && !IsNil(o.VideoIdObject) {
		return true
	}

	return false
}

// SetVideoIdObject gets a reference to the given Video and assigns it to the VideoIdObject field.
func (o *Detection) SetVideoIdObject(v Video) {
	o.VideoIdObject = &v
}

func (o Detection) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o Detection) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	if !IsNil(o.BoundingBox) {
		toSerialize["bounding_box"] = o.BoundingBox
	}
	if !IsNil(o.CameraId) {
		toSerialize["camera_id"] = o.CameraId
	}
	if !IsNil(o.CameraIdObject) {
		toSerialize["camera_id_object"] = o.CameraIdObject
	}
	if !IsNil(o.Centroid) {
		toSerialize["centroid"] = o.Centroid
	}
	if !IsNil(o.ClassId) {
		toSerialize["class_id"] = o.ClassId
	}
	if !IsNil(o.ClassName) {
		toSerialize["class_name"] = o.ClassName
	}
	if !IsNil(o.CreatedAt) {
		toSerialize["created_at"] = o.CreatedAt
	}
	if !IsNil(o.DeletedAt) {
		toSerialize["deleted_at"] = o.DeletedAt
	}
	if !IsNil(o.Id) {
		toSerialize["id"] = o.Id
	}
	if !IsNil(o.Score) {
		toSerialize["score"] = o.Score
	}
	if !IsNil(o.SeenAt) {
		toSerialize["seen_at"] = o.SeenAt
	}
	if !IsNil(o.UpdatedAt) {
		toSerialize["updated_at"] = o.UpdatedAt
	}
	if !IsNil(o.VideoId) {
		toSerialize["video_id"] = o.VideoId
	}
	if !IsNil(o.VideoIdObject) {
		toSerialize["video_id_object"] = o.VideoIdObject
	}
	return toSerialize, nil
}

type NullableDetection struct {
	value *Detection
	isSet bool
}

func (v NullableDetection) Get() *Detection {
	return v.value
}

func (v *NullableDetection) Set(val *Detection) {
	v.value = val
	v.isSet = true
}

func (v NullableDetection) IsSet() bool {
	return v.isSet
}

func (v *NullableDetection) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableDetection(val *Detection) *NullableDetection {
	return &NullableDetection{value: val, isSet: true}
}

func (v NullableDetection) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableDetection) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


