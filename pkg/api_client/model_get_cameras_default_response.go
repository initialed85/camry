/*
Djangolang

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

API version: 1.0
*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package api_client

import (
	"encoding/json"
	"bytes"
	"fmt"
)

// checks if the GetCamerasDefaultResponse type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &GetCamerasDefaultResponse{}

// GetCamerasDefaultResponse struct for GetCamerasDefaultResponse
type GetCamerasDefaultResponse struct {
	Error []string `json:"error"`
	Status int32 `json:"status"`
	Success bool `json:"success"`
}

type _GetCamerasDefaultResponse GetCamerasDefaultResponse

// NewGetCamerasDefaultResponse instantiates a new GetCamerasDefaultResponse object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewGetCamerasDefaultResponse(error_ []string, status int32, success bool) *GetCamerasDefaultResponse {
	this := GetCamerasDefaultResponse{}
	this.Error = error_
	this.Status = status
	this.Success = success
	return &this
}

// NewGetCamerasDefaultResponseWithDefaults instantiates a new GetCamerasDefaultResponse object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewGetCamerasDefaultResponseWithDefaults() *GetCamerasDefaultResponse {
	this := GetCamerasDefaultResponse{}
	return &this
}

// GetError returns the Error field value
func (o *GetCamerasDefaultResponse) GetError() []string {
	if o == nil {
		var ret []string
		return ret
	}

	return o.Error
}

// GetErrorOk returns a tuple with the Error field value
// and a boolean to check if the value has been set.
func (o *GetCamerasDefaultResponse) GetErrorOk() ([]string, bool) {
	if o == nil {
		return nil, false
	}
	return o.Error, true
}

// SetError sets field value
func (o *GetCamerasDefaultResponse) SetError(v []string) {
	o.Error = v
}

// GetStatus returns the Status field value
func (o *GetCamerasDefaultResponse) GetStatus() int32 {
	if o == nil {
		var ret int32
		return ret
	}

	return o.Status
}

// GetStatusOk returns a tuple with the Status field value
// and a boolean to check if the value has been set.
func (o *GetCamerasDefaultResponse) GetStatusOk() (*int32, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Status, true
}

// SetStatus sets field value
func (o *GetCamerasDefaultResponse) SetStatus(v int32) {
	o.Status = v
}

// GetSuccess returns the Success field value
func (o *GetCamerasDefaultResponse) GetSuccess() bool {
	if o == nil {
		var ret bool
		return ret
	}

	return o.Success
}

// GetSuccessOk returns a tuple with the Success field value
// and a boolean to check if the value has been set.
func (o *GetCamerasDefaultResponse) GetSuccessOk() (*bool, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Success, true
}

// SetSuccess sets field value
func (o *GetCamerasDefaultResponse) SetSuccess(v bool) {
	o.Success = v
}

func (o GetCamerasDefaultResponse) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o GetCamerasDefaultResponse) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	toSerialize["error"] = o.Error
	toSerialize["status"] = o.Status
	toSerialize["success"] = o.Success
	return toSerialize, nil
}

func (o *GetCamerasDefaultResponse) UnmarshalJSON(data []byte) (err error) {
	// This validates that all required properties are included in the JSON object
	// by unmarshalling the object into a generic map with string keys and checking
	// that every required field exists as a key in the generic map.
	requiredProperties := []string{
		"error",
		"status",
		"success",
	}

	allProperties := make(map[string]interface{})

	err = json.Unmarshal(data, &allProperties)

	if err != nil {
		return err;
	}

	for _, requiredProperty := range(requiredProperties) {
		if _, exists := allProperties[requiredProperty]; !exists {
			return fmt.Errorf("no value given for required property %v", requiredProperty)
		}
	}

	varGetCamerasDefaultResponse := _GetCamerasDefaultResponse{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varGetCamerasDefaultResponse)

	if err != nil {
		return err
	}

	*o = GetCamerasDefaultResponse(varGetCamerasDefaultResponse)

	return err
}

type NullableGetCamerasDefaultResponse struct {
	value *GetCamerasDefaultResponse
	isSet bool
}

func (v NullableGetCamerasDefaultResponse) Get() *GetCamerasDefaultResponse {
	return v.value
}

func (v *NullableGetCamerasDefaultResponse) Set(val *GetCamerasDefaultResponse) {
	v.value = val
	v.isSet = true
}

func (v NullableGetCamerasDefaultResponse) IsSet() bool {
	return v.isSet
}

func (v *NullableGetCamerasDefaultResponse) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableGetCamerasDefaultResponse(val *GetCamerasDefaultResponse) *NullableGetCamerasDefaultResponse {
	return &NullableGetCamerasDefaultResponse{value: val, isSet: true}
}

func (v NullableGetCamerasDefaultResponse) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableGetCamerasDefaultResponse) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


