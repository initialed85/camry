/*
Djangolang

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

API version: 1.0
*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package api_client

import (
	"encoding/json"
)

// checks if the ResponseWithGenericOfDetection type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &ResponseWithGenericOfDetection{}

// ResponseWithGenericOfDetection struct for ResponseWithGenericOfDetection
type ResponseWithGenericOfDetection struct {
	Count *int64 `json:"count,omitempty"`
	Error []string `json:"error,omitempty"`
	Limit *int64 `json:"limit,omitempty"`
	Objects []Detection `json:"objects,omitempty"`
	Offset *int64 `json:"offset,omitempty"`
	Status *int64 `json:"status,omitempty"`
	Success *bool `json:"success,omitempty"`
	TotalCount *int64 `json:"total_count,omitempty"`
}

// NewResponseWithGenericOfDetection instantiates a new ResponseWithGenericOfDetection object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewResponseWithGenericOfDetection() *ResponseWithGenericOfDetection {
	this := ResponseWithGenericOfDetection{}
	return &this
}

// NewResponseWithGenericOfDetectionWithDefaults instantiates a new ResponseWithGenericOfDetection object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewResponseWithGenericOfDetectionWithDefaults() *ResponseWithGenericOfDetection {
	this := ResponseWithGenericOfDetection{}
	return &this
}

// GetCount returns the Count field value if set, zero value otherwise.
func (o *ResponseWithGenericOfDetection) GetCount() int64 {
	if o == nil || IsNil(o.Count) {
		var ret int64
		return ret
	}
	return *o.Count
}

// GetCountOk returns a tuple with the Count field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ResponseWithGenericOfDetection) GetCountOk() (*int64, bool) {
	if o == nil || IsNil(o.Count) {
		return nil, false
	}
	return o.Count, true
}

// HasCount returns a boolean if a field has been set.
func (o *ResponseWithGenericOfDetection) HasCount() bool {
	if o != nil && !IsNil(o.Count) {
		return true
	}

	return false
}

// SetCount gets a reference to the given int64 and assigns it to the Count field.
func (o *ResponseWithGenericOfDetection) SetCount(v int64) {
	o.Count = &v
}

// GetError returns the Error field value if set, zero value otherwise.
func (o *ResponseWithGenericOfDetection) GetError() []string {
	if o == nil || IsNil(o.Error) {
		var ret []string
		return ret
	}
	return o.Error
}

// GetErrorOk returns a tuple with the Error field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ResponseWithGenericOfDetection) GetErrorOk() ([]string, bool) {
	if o == nil || IsNil(o.Error) {
		return nil, false
	}
	return o.Error, true
}

// HasError returns a boolean if a field has been set.
func (o *ResponseWithGenericOfDetection) HasError() bool {
	if o != nil && !IsNil(o.Error) {
		return true
	}

	return false
}

// SetError gets a reference to the given []string and assigns it to the Error field.
func (o *ResponseWithGenericOfDetection) SetError(v []string) {
	o.Error = v
}

// GetLimit returns the Limit field value if set, zero value otherwise.
func (o *ResponseWithGenericOfDetection) GetLimit() int64 {
	if o == nil || IsNil(o.Limit) {
		var ret int64
		return ret
	}
	return *o.Limit
}

// GetLimitOk returns a tuple with the Limit field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ResponseWithGenericOfDetection) GetLimitOk() (*int64, bool) {
	if o == nil || IsNil(o.Limit) {
		return nil, false
	}
	return o.Limit, true
}

// HasLimit returns a boolean if a field has been set.
func (o *ResponseWithGenericOfDetection) HasLimit() bool {
	if o != nil && !IsNil(o.Limit) {
		return true
	}

	return false
}

// SetLimit gets a reference to the given int64 and assigns it to the Limit field.
func (o *ResponseWithGenericOfDetection) SetLimit(v int64) {
	o.Limit = &v
}

// GetObjects returns the Objects field value if set, zero value otherwise.
func (o *ResponseWithGenericOfDetection) GetObjects() []Detection {
	if o == nil || IsNil(o.Objects) {
		var ret []Detection
		return ret
	}
	return o.Objects
}

// GetObjectsOk returns a tuple with the Objects field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ResponseWithGenericOfDetection) GetObjectsOk() ([]Detection, bool) {
	if o == nil || IsNil(o.Objects) {
		return nil, false
	}
	return o.Objects, true
}

// HasObjects returns a boolean if a field has been set.
func (o *ResponseWithGenericOfDetection) HasObjects() bool {
	if o != nil && !IsNil(o.Objects) {
		return true
	}

	return false
}

// SetObjects gets a reference to the given []Detection and assigns it to the Objects field.
func (o *ResponseWithGenericOfDetection) SetObjects(v []Detection) {
	o.Objects = v
}

// GetOffset returns the Offset field value if set, zero value otherwise.
func (o *ResponseWithGenericOfDetection) GetOffset() int64 {
	if o == nil || IsNil(o.Offset) {
		var ret int64
		return ret
	}
	return *o.Offset
}

// GetOffsetOk returns a tuple with the Offset field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ResponseWithGenericOfDetection) GetOffsetOk() (*int64, bool) {
	if o == nil || IsNil(o.Offset) {
		return nil, false
	}
	return o.Offset, true
}

// HasOffset returns a boolean if a field has been set.
func (o *ResponseWithGenericOfDetection) HasOffset() bool {
	if o != nil && !IsNil(o.Offset) {
		return true
	}

	return false
}

// SetOffset gets a reference to the given int64 and assigns it to the Offset field.
func (o *ResponseWithGenericOfDetection) SetOffset(v int64) {
	o.Offset = &v
}

// GetStatus returns the Status field value if set, zero value otherwise.
func (o *ResponseWithGenericOfDetection) GetStatus() int64 {
	if o == nil || IsNil(o.Status) {
		var ret int64
		return ret
	}
	return *o.Status
}

// GetStatusOk returns a tuple with the Status field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ResponseWithGenericOfDetection) GetStatusOk() (*int64, bool) {
	if o == nil || IsNil(o.Status) {
		return nil, false
	}
	return o.Status, true
}

// HasStatus returns a boolean if a field has been set.
func (o *ResponseWithGenericOfDetection) HasStatus() bool {
	if o != nil && !IsNil(o.Status) {
		return true
	}

	return false
}

// SetStatus gets a reference to the given int64 and assigns it to the Status field.
func (o *ResponseWithGenericOfDetection) SetStatus(v int64) {
	o.Status = &v
}

// GetSuccess returns the Success field value if set, zero value otherwise.
func (o *ResponseWithGenericOfDetection) GetSuccess() bool {
	if o == nil || IsNil(o.Success) {
		var ret bool
		return ret
	}
	return *o.Success
}

// GetSuccessOk returns a tuple with the Success field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ResponseWithGenericOfDetection) GetSuccessOk() (*bool, bool) {
	if o == nil || IsNil(o.Success) {
		return nil, false
	}
	return o.Success, true
}

// HasSuccess returns a boolean if a field has been set.
func (o *ResponseWithGenericOfDetection) HasSuccess() bool {
	if o != nil && !IsNil(o.Success) {
		return true
	}

	return false
}

// SetSuccess gets a reference to the given bool and assigns it to the Success field.
func (o *ResponseWithGenericOfDetection) SetSuccess(v bool) {
	o.Success = &v
}

// GetTotalCount returns the TotalCount field value if set, zero value otherwise.
func (o *ResponseWithGenericOfDetection) GetTotalCount() int64 {
	if o == nil || IsNil(o.TotalCount) {
		var ret int64
		return ret
	}
	return *o.TotalCount
}

// GetTotalCountOk returns a tuple with the TotalCount field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ResponseWithGenericOfDetection) GetTotalCountOk() (*int64, bool) {
	if o == nil || IsNil(o.TotalCount) {
		return nil, false
	}
	return o.TotalCount, true
}

// HasTotalCount returns a boolean if a field has been set.
func (o *ResponseWithGenericOfDetection) HasTotalCount() bool {
	if o != nil && !IsNil(o.TotalCount) {
		return true
	}

	return false
}

// SetTotalCount gets a reference to the given int64 and assigns it to the TotalCount field.
func (o *ResponseWithGenericOfDetection) SetTotalCount(v int64) {
	o.TotalCount = &v
}

func (o ResponseWithGenericOfDetection) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o ResponseWithGenericOfDetection) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	if !IsNil(o.Count) {
		toSerialize["count"] = o.Count
	}
	if !IsNil(o.Error) {
		toSerialize["error"] = o.Error
	}
	if !IsNil(o.Limit) {
		toSerialize["limit"] = o.Limit
	}
	if !IsNil(o.Objects) {
		toSerialize["objects"] = o.Objects
	}
	if !IsNil(o.Offset) {
		toSerialize["offset"] = o.Offset
	}
	if !IsNil(o.Status) {
		toSerialize["status"] = o.Status
	}
	if !IsNil(o.Success) {
		toSerialize["success"] = o.Success
	}
	if !IsNil(o.TotalCount) {
		toSerialize["total_count"] = o.TotalCount
	}
	return toSerialize, nil
}

type NullableResponseWithGenericOfDetection struct {
	value *ResponseWithGenericOfDetection
	isSet bool
}

func (v NullableResponseWithGenericOfDetection) Get() *ResponseWithGenericOfDetection {
	return v.value
}

func (v *NullableResponseWithGenericOfDetection) Set(val *ResponseWithGenericOfDetection) {
	v.value = val
	v.isSet = true
}

func (v NullableResponseWithGenericOfDetection) IsSet() bool {
	return v.isSet
}

func (v *NullableResponseWithGenericOfDetection) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableResponseWithGenericOfDetection(val *ResponseWithGenericOfDetection) *NullableResponseWithGenericOfDetection {
	return &NullableResponseWithGenericOfDetection{value: val, isSet: true}
}

func (v NullableResponseWithGenericOfDetection) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableResponseWithGenericOfDetection) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


