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

// checks if the ClaimRequest type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &ClaimRequest{}

// ClaimRequest struct for ClaimRequest
type ClaimRequest struct {
	ClaimDurationSeconds *float64 `json:"claim_duration_seconds,omitempty"`
}

// NewClaimRequest instantiates a new ClaimRequest object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewClaimRequest() *ClaimRequest {
	this := ClaimRequest{}
	return &this
}

// NewClaimRequestWithDefaults instantiates a new ClaimRequest object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewClaimRequestWithDefaults() *ClaimRequest {
	this := ClaimRequest{}
	return &this
}

// GetClaimDurationSeconds returns the ClaimDurationSeconds field value if set, zero value otherwise.
func (o *ClaimRequest) GetClaimDurationSeconds() float64 {
	if o == nil || IsNil(o.ClaimDurationSeconds) {
		var ret float64
		return ret
	}
	return *o.ClaimDurationSeconds
}

// GetClaimDurationSecondsOk returns a tuple with the ClaimDurationSeconds field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ClaimRequest) GetClaimDurationSecondsOk() (*float64, bool) {
	if o == nil || IsNil(o.ClaimDurationSeconds) {
		return nil, false
	}
	return o.ClaimDurationSeconds, true
}

// HasClaimDurationSeconds returns a boolean if a field has been set.
func (o *ClaimRequest) HasClaimDurationSeconds() bool {
	if o != nil && !IsNil(o.ClaimDurationSeconds) {
		return true
	}

	return false
}

// SetClaimDurationSeconds gets a reference to the given float64 and assigns it to the ClaimDurationSeconds field.
func (o *ClaimRequest) SetClaimDurationSeconds(v float64) {
	o.ClaimDurationSeconds = &v
}

func (o ClaimRequest) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o ClaimRequest) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	if !IsNil(o.ClaimDurationSeconds) {
		toSerialize["claim_duration_seconds"] = o.ClaimDurationSeconds
	}
	return toSerialize, nil
}

type NullableClaimRequest struct {
	value *ClaimRequest
	isSet bool
}

func (v NullableClaimRequest) Get() *ClaimRequest {
	return v.value
}

func (v *NullableClaimRequest) Set(val *ClaimRequest) {
	v.value = val
	v.isSet = true
}

func (v NullableClaimRequest) IsSet() bool {
	return v.isSet
}

func (v *NullableClaimRequest) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableClaimRequest(val *ClaimRequest) *NullableClaimRequest {
	return &NullableClaimRequest{value: val, isSet: true}
}

func (v NullableClaimRequest) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableClaimRequest) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


