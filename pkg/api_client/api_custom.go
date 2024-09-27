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


// CustomAPIService CustomAPI service
type CustomAPIService service

type ApiPatchCustomClaimVideoForObjectDetectorRequest struct {
	ctx context.Context
	ApiService *CustomAPIService
	claimRequest *ClaimRequest
}

func (r ApiPatchCustomClaimVideoForObjectDetectorRequest) ClaimRequest(claimRequest ClaimRequest) ApiPatchCustomClaimVideoForObjectDetectorRequest {
	r.claimRequest = &claimRequest
	return r
}

func (r ApiPatchCustomClaimVideoForObjectDetectorRequest) Execute() (*Video, *http.Response, error) {
	return r.ApiService.PatchCustomClaimVideoForObjectDetectorExecute(r)
}

/*
PatchCustomClaimVideoForObjectDetector Method for PatchCustomClaimVideoForObjectDetector

 @param ctx context.Context - for authentication, logging, cancellation, deadlines, tracing, etc. Passed from http.Request or context.Background().
 @return ApiPatchCustomClaimVideoForObjectDetectorRequest
*/
func (a *CustomAPIService) PatchCustomClaimVideoForObjectDetector(ctx context.Context) ApiPatchCustomClaimVideoForObjectDetectorRequest {
	return ApiPatchCustomClaimVideoForObjectDetectorRequest{
		ApiService: a,
		ctx: ctx,
	}
}

// Execute executes the request
//  @return Video
func (a *CustomAPIService) PatchCustomClaimVideoForObjectDetectorExecute(r ApiPatchCustomClaimVideoForObjectDetectorRequest) (*Video, *http.Response, error) {
	var (
		localVarHTTPMethod   = http.MethodPatch
		localVarPostBody     interface{}
		formFiles            []formFile
		localVarReturnValue  *Video
	)

	localBasePath, err := a.client.cfg.ServerURLWithContext(r.ctx, "CustomAPIService.PatchCustomClaimVideoForObjectDetector")
	if err != nil {
		return localVarReturnValue, nil, &GenericOpenAPIError{error: err.Error()}
	}

	localVarPath := localBasePath + "/api/custom/claim-video-for-object-detector"

	localVarHeaderParams := make(map[string]string)
	localVarQueryParams := url.Values{}
	localVarFormParams := url.Values{}
	if r.claimRequest == nil {
		return localVarReturnValue, nil, reportError("claimRequest is required and must be specified")
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
	localVarPostBody = r.claimRequest
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