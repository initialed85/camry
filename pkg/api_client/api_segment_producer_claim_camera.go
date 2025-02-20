/*
Djangolang

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

API version: 1.0
*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package api_client

import (
	"bytes"
	"context"
	"io"
	"net/http"
	"net/url"
)


// SegmentProducerClaimCameraAPIService SegmentProducerClaimCameraAPI service
type SegmentProducerClaimCameraAPIService service

type ApiPostSegmentProducerClaimCamerasRequest struct {
	ctx context.Context
	ApiService *SegmentProducerClaimCameraAPIService
	cameraSegmentProducerClaimRequest *CameraSegmentProducerClaimRequest
}

func (r ApiPostSegmentProducerClaimCamerasRequest) CameraSegmentProducerClaimRequest(cameraSegmentProducerClaimRequest CameraSegmentProducerClaimRequest) ApiPostSegmentProducerClaimCamerasRequest {
	r.cameraSegmentProducerClaimRequest = &cameraSegmentProducerClaimRequest
	return r
}

func (r ApiPostSegmentProducerClaimCamerasRequest) Execute() (*ResponseWithGenericOfCamera, *http.Response, error) {
	return r.ApiService.PostSegmentProducerClaimCamerasExecute(r)
}

/*
PostSegmentProducerClaimCameras Method for PostSegmentProducerClaimCameras

 @param ctx context.Context - for authentication, logging, cancellation, deadlines, tracing, etc. Passed from http.Request or context.Background().
 @return ApiPostSegmentProducerClaimCamerasRequest
*/
func (a *SegmentProducerClaimCameraAPIService) PostSegmentProducerClaimCameras(ctx context.Context) ApiPostSegmentProducerClaimCamerasRequest {
	return ApiPostSegmentProducerClaimCamerasRequest{
		ApiService: a,
		ctx: ctx,
	}
}

// Execute executes the request
//  @return ResponseWithGenericOfCamera
func (a *SegmentProducerClaimCameraAPIService) PostSegmentProducerClaimCamerasExecute(r ApiPostSegmentProducerClaimCamerasRequest) (*ResponseWithGenericOfCamera, *http.Response, error) {
	var (
		localVarHTTPMethod   = http.MethodPost
		localVarPostBody     interface{}
		formFiles            []formFile
		localVarReturnValue  *ResponseWithGenericOfCamera
	)

	localBasePath, err := a.client.cfg.ServerURLWithContext(r.ctx, "SegmentProducerClaimCameraAPIService.PostSegmentProducerClaimCameras")
	if err != nil {
		return localVarReturnValue, nil, &GenericOpenAPIError{error: err.Error()}
	}

	localVarPath := localBasePath + "/api/segment-producer-claim-camera"

	localVarHeaderParams := make(map[string]string)
	localVarQueryParams := url.Values{}
	localVarFormParams := url.Values{}
	if r.cameraSegmentProducerClaimRequest == nil {
		return localVarReturnValue, nil, reportError("cameraSegmentProducerClaimRequest is required and must be specified")
	}

	// to determine the Content-Type header
	localVarHTTPContentTypes := []string{"application/json"}

	// set Content-Type header
	localVarHTTPContentType := selectHeaderContentType(localVarHTTPContentTypes)
	if localVarHTTPContentType != "" {
		localVarHeaderParams["Content-Type"] = localVarHTTPContentType
	}

	// to determine the Accept header
	localVarHTTPHeaderAccepts := []string{"application/json"}

	// set Accept header
	localVarHTTPHeaderAccept := selectHeaderAccept(localVarHTTPHeaderAccepts)
	if localVarHTTPHeaderAccept != "" {
		localVarHeaderParams["Accept"] = localVarHTTPHeaderAccept
	}
	// body params
	localVarPostBody = r.cameraSegmentProducerClaimRequest
	req, err := a.client.prepareRequest(r.ctx, localVarPath, localVarHTTPMethod, localVarPostBody, localVarHeaderParams, localVarQueryParams, localVarFormParams, formFiles)
	if err != nil {
		return localVarReturnValue, nil, err
	}

	localVarHTTPResponse, err := a.client.callAPI(req)
	if err != nil || localVarHTTPResponse == nil {
		return localVarReturnValue, localVarHTTPResponse, err
	}

	localVarBody, err := io.ReadAll(localVarHTTPResponse.Body)
	localVarHTTPResponse.Body.Close()
	localVarHTTPResponse.Body = io.NopCloser(bytes.NewBuffer(localVarBody))
	if err != nil {
		return localVarReturnValue, localVarHTTPResponse, err
	}

	if localVarHTTPResponse.StatusCode >= 300 {
		newErr := &GenericOpenAPIError{
			body:  localVarBody,
			error: localVarHTTPResponse.Status,
		}
			var v GetCamerasDefaultResponse
			err = a.client.decode(&v, localVarBody, localVarHTTPResponse.Header.Get("Content-Type"))
			if err != nil {
				newErr.error = err.Error()
				return localVarReturnValue, localVarHTTPResponse, newErr
			}
					newErr.error = formatErrorMessage(localVarHTTPResponse.Status, &v)
					newErr.model = v
		return localVarReturnValue, localVarHTTPResponse, newErr
	}

	err = a.client.decode(&localVarReturnValue, localVarBody, localVarHTTPResponse.Header.Get("Content-Type"))
	if err != nil {
		newErr := &GenericOpenAPIError{
			body:  localVarBody,
			error: err.Error(),
		}
		return localVarReturnValue, localVarHTTPResponse, newErr
	}

	return localVarReturnValue, localVarHTTPResponse, nil
}
