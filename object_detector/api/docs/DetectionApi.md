# openapi_client.DetectionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_detection**](DetectionApi.md#delete_detection) | **DELETE** /api/detections/{primaryKey} | 
[**get_detection**](DetectionApi.md#get_detection) | **GET** /api/detections/{primaryKey} | 
[**get_detections**](DetectionApi.md#get_detections) | **GET** /api/detections | 
[**patch_detection**](DetectionApi.md#patch_detection) | **PATCH** /api/detections/{primaryKey} | 
[**post_detections**](DetectionApi.md#post_detections) | **POST** /api/detections | 
[**put_detection**](DetectionApi.md#put_detection) | **PUT** /api/detections/{primaryKey} | 


# **delete_detection**
> delete_detection(primary_key, depth=depth)



### Example


```python
import openapi_client
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.DetectionApi(api_client)
    primary_key = None # object | Primary key for Detection
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_instance.delete_detection(primary_key, depth=depth)
    except Exception as e:
        print("Exception when calling DetectionApi->delete_detection: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **primary_key** | [**object**](.md)| Primary key for Detection | 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**204** | Successful Item Delete for Detections |  -  |
**0** | Failed Item Delete for Detections |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_detection**
> GetDetections200Response get_detection(primary_key, depth=depth)



### Example


```python
import openapi_client
from openapi_client.models.get_detections200_response import GetDetections200Response
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.DetectionApi(api_client)
    primary_key = None # object | Primary key for Detection
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_response = api_instance.get_detection(primary_key, depth=depth)
        print("The response of DetectionApi->get_detection:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DetectionApi->get_detection: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **primary_key** | [**object**](.md)| Primary key for Detection | 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 

### Return type

[**GetDetections200Response**](GetDetections200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful Item Fetch for Detections |  -  |
**0** | Failed Item Fetch for Detections |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_detections**
> GetDetections200Response get_detections(limit=limit, offset=offset, depth=depth, id__eq=id__eq, id__ne=id__ne, id__gt=id__gt, id__gte=id__gte, id__lt=id__lt, id__lte=id__lte, id__in=id__in, id__notin=id__notin, id__isnull=id__isnull, id__isnotnull=id__isnotnull, id__isfalse=id__isfalse, id__istrue=id__istrue, id__like=id__like, id__notlike=id__notlike, id__ilike=id__ilike, id__notilike=id__notilike, id__desc=id__desc, id__asc=id__asc, created_at__eq=created_at__eq, created_at__ne=created_at__ne, created_at__gt=created_at__gt, created_at__gte=created_at__gte, created_at__lt=created_at__lt, created_at__lte=created_at__lte, created_at__in=created_at__in, created_at__notin=created_at__notin, created_at__isnull=created_at__isnull, created_at__isnotnull=created_at__isnotnull, created_at__isfalse=created_at__isfalse, created_at__istrue=created_at__istrue, created_at__like=created_at__like, created_at__notlike=created_at__notlike, created_at__ilike=created_at__ilike, created_at__notilike=created_at__notilike, created_at__desc=created_at__desc, created_at__asc=created_at__asc, updated_at__eq=updated_at__eq, updated_at__ne=updated_at__ne, updated_at__gt=updated_at__gt, updated_at__gte=updated_at__gte, updated_at__lt=updated_at__lt, updated_at__lte=updated_at__lte, updated_at__in=updated_at__in, updated_at__notin=updated_at__notin, updated_at__isnull=updated_at__isnull, updated_at__isnotnull=updated_at__isnotnull, updated_at__isfalse=updated_at__isfalse, updated_at__istrue=updated_at__istrue, updated_at__like=updated_at__like, updated_at__notlike=updated_at__notlike, updated_at__ilike=updated_at__ilike, updated_at__notilike=updated_at__notilike, updated_at__desc=updated_at__desc, updated_at__asc=updated_at__asc, deleted_at__eq=deleted_at__eq, deleted_at__ne=deleted_at__ne, deleted_at__gt=deleted_at__gt, deleted_at__gte=deleted_at__gte, deleted_at__lt=deleted_at__lt, deleted_at__lte=deleted_at__lte, deleted_at__in=deleted_at__in, deleted_at__notin=deleted_at__notin, deleted_at__isnull=deleted_at__isnull, deleted_at__isnotnull=deleted_at__isnotnull, deleted_at__isfalse=deleted_at__isfalse, deleted_at__istrue=deleted_at__istrue, deleted_at__like=deleted_at__like, deleted_at__notlike=deleted_at__notlike, deleted_at__ilike=deleted_at__ilike, deleted_at__notilike=deleted_at__notilike, deleted_at__desc=deleted_at__desc, deleted_at__asc=deleted_at__asc, seen_at__eq=seen_at__eq, seen_at__ne=seen_at__ne, seen_at__gt=seen_at__gt, seen_at__gte=seen_at__gte, seen_at__lt=seen_at__lt, seen_at__lte=seen_at__lte, seen_at__in=seen_at__in, seen_at__notin=seen_at__notin, seen_at__isnull=seen_at__isnull, seen_at__isnotnull=seen_at__isnotnull, seen_at__isfalse=seen_at__isfalse, seen_at__istrue=seen_at__istrue, seen_at__like=seen_at__like, seen_at__notlike=seen_at__notlike, seen_at__ilike=seen_at__ilike, seen_at__notilike=seen_at__notilike, seen_at__desc=seen_at__desc, seen_at__asc=seen_at__asc, class_id__eq=class_id__eq, class_id__ne=class_id__ne, class_id__gt=class_id__gt, class_id__gte=class_id__gte, class_id__lt=class_id__lt, class_id__lte=class_id__lte, class_id__in=class_id__in, class_id__notin=class_id__notin, class_id__isnull=class_id__isnull, class_id__isnotnull=class_id__isnotnull, class_id__isfalse=class_id__isfalse, class_id__istrue=class_id__istrue, class_id__like=class_id__like, class_id__notlike=class_id__notlike, class_id__ilike=class_id__ilike, class_id__notilike=class_id__notilike, class_id__desc=class_id__desc, class_id__asc=class_id__asc, class_name__eq=class_name__eq, class_name__ne=class_name__ne, class_name__gt=class_name__gt, class_name__gte=class_name__gte, class_name__lt=class_name__lt, class_name__lte=class_name__lte, class_name__in=class_name__in, class_name__notin=class_name__notin, class_name__isnull=class_name__isnull, class_name__isnotnull=class_name__isnotnull, class_name__isfalse=class_name__isfalse, class_name__istrue=class_name__istrue, class_name__like=class_name__like, class_name__notlike=class_name__notlike, class_name__ilike=class_name__ilike, class_name__notilike=class_name__notilike, class_name__desc=class_name__desc, class_name__asc=class_name__asc, score__eq=score__eq, score__ne=score__ne, score__gt=score__gt, score__gte=score__gte, score__lt=score__lt, score__lte=score__lte, score__in=score__in, score__notin=score__notin, score__isnull=score__isnull, score__isnotnull=score__isnotnull, score__isfalse=score__isfalse, score__istrue=score__istrue, score__like=score__like, score__notlike=score__notlike, score__ilike=score__ilike, score__notilike=score__notilike, score__desc=score__desc, score__asc=score__asc, video_id__eq=video_id__eq, video_id__ne=video_id__ne, video_id__gt=video_id__gt, video_id__gte=video_id__gte, video_id__lt=video_id__lt, video_id__lte=video_id__lte, video_id__in=video_id__in, video_id__notin=video_id__notin, video_id__isnull=video_id__isnull, video_id__isnotnull=video_id__isnotnull, video_id__isfalse=video_id__isfalse, video_id__istrue=video_id__istrue, video_id__like=video_id__like, video_id__notlike=video_id__notlike, video_id__ilike=video_id__ilike, video_id__notilike=video_id__notilike, video_id__desc=video_id__desc, video_id__asc=video_id__asc, camera_id__eq=camera_id__eq, camera_id__ne=camera_id__ne, camera_id__gt=camera_id__gt, camera_id__gte=camera_id__gte, camera_id__lt=camera_id__lt, camera_id__lte=camera_id__lte, camera_id__in=camera_id__in, camera_id__notin=camera_id__notin, camera_id__isnull=camera_id__isnull, camera_id__isnotnull=camera_id__isnotnull, camera_id__isfalse=camera_id__isfalse, camera_id__istrue=camera_id__istrue, camera_id__like=camera_id__like, camera_id__notlike=camera_id__notlike, camera_id__ilike=camera_id__ilike, camera_id__notilike=camera_id__notilike, camera_id__desc=camera_id__desc, camera_id__asc=camera_id__asc)



### Example


```python
import openapi_client
from openapi_client.models.get_detections200_response import GetDetections200Response
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.DetectionApi(api_client)
    limit = 56 # int | SQL LIMIT operator (optional)
    offset = 56 # int | SQL OFFSET operator (optional)
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)
    id__eq = 'id__eq_example' # str | SQL = comparison (optional)
    id__ne = 'id__ne_example' # str | SQL != comparison (optional)
    id__gt = 'id__gt_example' # str | SQL > comparison, may not work with all column types (optional)
    id__gte = 'id__gte_example' # str | SQL >= comparison, may not work with all column types (optional)
    id__lt = 'id__lt_example' # str | SQL < comparison, may not work with all column types (optional)
    id__lte = 'id__lte_example' # str | SQL <= comparison, may not work with all column types (optional)
    id__in = 'id__in_example' # str | SQL IN comparison, permits comma-separated values (optional)
    id__notin = 'id__notin_example' # str | SQL NOT IN comparison, permits comma-separated values (optional)
    id__isnull = 'id__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    id__isnotnull = 'id__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    id__isfalse = 'id__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    id__istrue = 'id__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    id__like = 'id__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    id__notlike = 'id__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    id__ilike = 'id__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    id__notilike = 'id__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    id__desc = 'id__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    id__asc = 'id__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    created_at__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = comparison (optional)
    created_at__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != comparison (optional)
    created_at__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > comparison, may not work with all column types (optional)
    created_at__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= comparison, may not work with all column types (optional)
    created_at__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < comparison, may not work with all column types (optional)
    created_at__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= comparison, may not work with all column types (optional)
    created_at__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN comparison, permits comma-separated values (optional)
    created_at__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN comparison, permits comma-separated values (optional)
    created_at__isnull = 'created_at__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    created_at__isnotnull = 'created_at__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    created_at__isfalse = 'created_at__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    created_at__istrue = 'created_at__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    created_at__like = 'created_at__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    created_at__notlike = 'created_at__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    created_at__ilike = 'created_at__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    created_at__notilike = 'created_at__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    created_at__desc = 'created_at__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    created_at__asc = 'created_at__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    updated_at__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = comparison (optional)
    updated_at__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != comparison (optional)
    updated_at__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > comparison, may not work with all column types (optional)
    updated_at__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= comparison, may not work with all column types (optional)
    updated_at__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < comparison, may not work with all column types (optional)
    updated_at__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= comparison, may not work with all column types (optional)
    updated_at__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN comparison, permits comma-separated values (optional)
    updated_at__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN comparison, permits comma-separated values (optional)
    updated_at__isnull = 'updated_at__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    updated_at__isnotnull = 'updated_at__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    updated_at__isfalse = 'updated_at__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    updated_at__istrue = 'updated_at__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    updated_at__like = 'updated_at__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    updated_at__notlike = 'updated_at__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    updated_at__ilike = 'updated_at__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    updated_at__notilike = 'updated_at__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    updated_at__desc = 'updated_at__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    updated_at__asc = 'updated_at__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    deleted_at__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = comparison (optional)
    deleted_at__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != comparison (optional)
    deleted_at__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > comparison, may not work with all column types (optional)
    deleted_at__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= comparison, may not work with all column types (optional)
    deleted_at__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < comparison, may not work with all column types (optional)
    deleted_at__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= comparison, may not work with all column types (optional)
    deleted_at__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN comparison, permits comma-separated values (optional)
    deleted_at__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN comparison, permits comma-separated values (optional)
    deleted_at__isnull = 'deleted_at__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    deleted_at__isnotnull = 'deleted_at__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    deleted_at__isfalse = 'deleted_at__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    deleted_at__istrue = 'deleted_at__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    deleted_at__like = 'deleted_at__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__notlike = 'deleted_at__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__ilike = 'deleted_at__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__notilike = 'deleted_at__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__desc = 'deleted_at__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    deleted_at__asc = 'deleted_at__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    seen_at__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = comparison (optional)
    seen_at__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != comparison (optional)
    seen_at__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > comparison, may not work with all column types (optional)
    seen_at__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= comparison, may not work with all column types (optional)
    seen_at__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < comparison, may not work with all column types (optional)
    seen_at__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= comparison, may not work with all column types (optional)
    seen_at__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN comparison, permits comma-separated values (optional)
    seen_at__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN comparison, permits comma-separated values (optional)
    seen_at__isnull = 'seen_at__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    seen_at__isnotnull = 'seen_at__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    seen_at__isfalse = 'seen_at__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    seen_at__istrue = 'seen_at__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    seen_at__like = 'seen_at__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    seen_at__notlike = 'seen_at__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    seen_at__ilike = 'seen_at__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    seen_at__notilike = 'seen_at__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    seen_at__desc = 'seen_at__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    seen_at__asc = 'seen_at__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    class_id__eq = 56 # int | SQL = comparison (optional)
    class_id__ne = 56 # int | SQL != comparison (optional)
    class_id__gt = 56 # int | SQL > comparison, may not work with all column types (optional)
    class_id__gte = 56 # int | SQL >= comparison, may not work with all column types (optional)
    class_id__lt = 56 # int | SQL < comparison, may not work with all column types (optional)
    class_id__lte = 56 # int | SQL <= comparison, may not work with all column types (optional)
    class_id__in = 56 # int | SQL IN comparison, permits comma-separated values (optional)
    class_id__notin = 56 # int | SQL NOT IN comparison, permits comma-separated values (optional)
    class_id__isnull = 'class_id__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    class_id__isnotnull = 'class_id__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    class_id__isfalse = 'class_id__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    class_id__istrue = 'class_id__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    class_id__like = 'class_id__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    class_id__notlike = 'class_id__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    class_id__ilike = 'class_id__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    class_id__notilike = 'class_id__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    class_id__desc = 'class_id__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    class_id__asc = 'class_id__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    class_name__eq = 'class_name__eq_example' # str | SQL = comparison (optional)
    class_name__ne = 'class_name__ne_example' # str | SQL != comparison (optional)
    class_name__gt = 'class_name__gt_example' # str | SQL > comparison, may not work with all column types (optional)
    class_name__gte = 'class_name__gte_example' # str | SQL >= comparison, may not work with all column types (optional)
    class_name__lt = 'class_name__lt_example' # str | SQL < comparison, may not work with all column types (optional)
    class_name__lte = 'class_name__lte_example' # str | SQL <= comparison, may not work with all column types (optional)
    class_name__in = 'class_name__in_example' # str | SQL IN comparison, permits comma-separated values (optional)
    class_name__notin = 'class_name__notin_example' # str | SQL NOT IN comparison, permits comma-separated values (optional)
    class_name__isnull = 'class_name__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    class_name__isnotnull = 'class_name__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    class_name__isfalse = 'class_name__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    class_name__istrue = 'class_name__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    class_name__like = 'class_name__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    class_name__notlike = 'class_name__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    class_name__ilike = 'class_name__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    class_name__notilike = 'class_name__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    class_name__desc = 'class_name__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    class_name__asc = 'class_name__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    score__eq = 3.4 # float | SQL = comparison (optional)
    score__ne = 3.4 # float | SQL != comparison (optional)
    score__gt = 3.4 # float | SQL > comparison, may not work with all column types (optional)
    score__gte = 3.4 # float | SQL >= comparison, may not work with all column types (optional)
    score__lt = 3.4 # float | SQL < comparison, may not work with all column types (optional)
    score__lte = 3.4 # float | SQL <= comparison, may not work with all column types (optional)
    score__in = 3.4 # float | SQL IN comparison, permits comma-separated values (optional)
    score__notin = 3.4 # float | SQL NOT IN comparison, permits comma-separated values (optional)
    score__isnull = 'score__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    score__isnotnull = 'score__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    score__isfalse = 'score__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    score__istrue = 'score__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    score__like = 'score__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    score__notlike = 'score__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    score__ilike = 'score__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    score__notilike = 'score__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    score__desc = 'score__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    score__asc = 'score__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    video_id__eq = 'video_id__eq_example' # str | SQL = comparison (optional)
    video_id__ne = 'video_id__ne_example' # str | SQL != comparison (optional)
    video_id__gt = 'video_id__gt_example' # str | SQL > comparison, may not work with all column types (optional)
    video_id__gte = 'video_id__gte_example' # str | SQL >= comparison, may not work with all column types (optional)
    video_id__lt = 'video_id__lt_example' # str | SQL < comparison, may not work with all column types (optional)
    video_id__lte = 'video_id__lte_example' # str | SQL <= comparison, may not work with all column types (optional)
    video_id__in = 'video_id__in_example' # str | SQL IN comparison, permits comma-separated values (optional)
    video_id__notin = 'video_id__notin_example' # str | SQL NOT IN comparison, permits comma-separated values (optional)
    video_id__isnull = 'video_id__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    video_id__isnotnull = 'video_id__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    video_id__isfalse = 'video_id__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    video_id__istrue = 'video_id__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    video_id__like = 'video_id__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    video_id__notlike = 'video_id__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    video_id__ilike = 'video_id__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    video_id__notilike = 'video_id__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    video_id__desc = 'video_id__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    video_id__asc = 'video_id__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    camera_id__eq = 'camera_id__eq_example' # str | SQL = comparison (optional)
    camera_id__ne = 'camera_id__ne_example' # str | SQL != comparison (optional)
    camera_id__gt = 'camera_id__gt_example' # str | SQL > comparison, may not work with all column types (optional)
    camera_id__gte = 'camera_id__gte_example' # str | SQL >= comparison, may not work with all column types (optional)
    camera_id__lt = 'camera_id__lt_example' # str | SQL < comparison, may not work with all column types (optional)
    camera_id__lte = 'camera_id__lte_example' # str | SQL <= comparison, may not work with all column types (optional)
    camera_id__in = 'camera_id__in_example' # str | SQL IN comparison, permits comma-separated values (optional)
    camera_id__notin = 'camera_id__notin_example' # str | SQL NOT IN comparison, permits comma-separated values (optional)
    camera_id__isnull = 'camera_id__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    camera_id__isnotnull = 'camera_id__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    camera_id__isfalse = 'camera_id__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    camera_id__istrue = 'camera_id__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    camera_id__like = 'camera_id__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    camera_id__notlike = 'camera_id__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    camera_id__ilike = 'camera_id__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    camera_id__notilike = 'camera_id__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    camera_id__desc = 'camera_id__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    camera_id__asc = 'camera_id__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)

    try:
        api_response = api_instance.get_detections(limit=limit, offset=offset, depth=depth, id__eq=id__eq, id__ne=id__ne, id__gt=id__gt, id__gte=id__gte, id__lt=id__lt, id__lte=id__lte, id__in=id__in, id__notin=id__notin, id__isnull=id__isnull, id__isnotnull=id__isnotnull, id__isfalse=id__isfalse, id__istrue=id__istrue, id__like=id__like, id__notlike=id__notlike, id__ilike=id__ilike, id__notilike=id__notilike, id__desc=id__desc, id__asc=id__asc, created_at__eq=created_at__eq, created_at__ne=created_at__ne, created_at__gt=created_at__gt, created_at__gte=created_at__gte, created_at__lt=created_at__lt, created_at__lte=created_at__lte, created_at__in=created_at__in, created_at__notin=created_at__notin, created_at__isnull=created_at__isnull, created_at__isnotnull=created_at__isnotnull, created_at__isfalse=created_at__isfalse, created_at__istrue=created_at__istrue, created_at__like=created_at__like, created_at__notlike=created_at__notlike, created_at__ilike=created_at__ilike, created_at__notilike=created_at__notilike, created_at__desc=created_at__desc, created_at__asc=created_at__asc, updated_at__eq=updated_at__eq, updated_at__ne=updated_at__ne, updated_at__gt=updated_at__gt, updated_at__gte=updated_at__gte, updated_at__lt=updated_at__lt, updated_at__lte=updated_at__lte, updated_at__in=updated_at__in, updated_at__notin=updated_at__notin, updated_at__isnull=updated_at__isnull, updated_at__isnotnull=updated_at__isnotnull, updated_at__isfalse=updated_at__isfalse, updated_at__istrue=updated_at__istrue, updated_at__like=updated_at__like, updated_at__notlike=updated_at__notlike, updated_at__ilike=updated_at__ilike, updated_at__notilike=updated_at__notilike, updated_at__desc=updated_at__desc, updated_at__asc=updated_at__asc, deleted_at__eq=deleted_at__eq, deleted_at__ne=deleted_at__ne, deleted_at__gt=deleted_at__gt, deleted_at__gte=deleted_at__gte, deleted_at__lt=deleted_at__lt, deleted_at__lte=deleted_at__lte, deleted_at__in=deleted_at__in, deleted_at__notin=deleted_at__notin, deleted_at__isnull=deleted_at__isnull, deleted_at__isnotnull=deleted_at__isnotnull, deleted_at__isfalse=deleted_at__isfalse, deleted_at__istrue=deleted_at__istrue, deleted_at__like=deleted_at__like, deleted_at__notlike=deleted_at__notlike, deleted_at__ilike=deleted_at__ilike, deleted_at__notilike=deleted_at__notilike, deleted_at__desc=deleted_at__desc, deleted_at__asc=deleted_at__asc, seen_at__eq=seen_at__eq, seen_at__ne=seen_at__ne, seen_at__gt=seen_at__gt, seen_at__gte=seen_at__gte, seen_at__lt=seen_at__lt, seen_at__lte=seen_at__lte, seen_at__in=seen_at__in, seen_at__notin=seen_at__notin, seen_at__isnull=seen_at__isnull, seen_at__isnotnull=seen_at__isnotnull, seen_at__isfalse=seen_at__isfalse, seen_at__istrue=seen_at__istrue, seen_at__like=seen_at__like, seen_at__notlike=seen_at__notlike, seen_at__ilike=seen_at__ilike, seen_at__notilike=seen_at__notilike, seen_at__desc=seen_at__desc, seen_at__asc=seen_at__asc, class_id__eq=class_id__eq, class_id__ne=class_id__ne, class_id__gt=class_id__gt, class_id__gte=class_id__gte, class_id__lt=class_id__lt, class_id__lte=class_id__lte, class_id__in=class_id__in, class_id__notin=class_id__notin, class_id__isnull=class_id__isnull, class_id__isnotnull=class_id__isnotnull, class_id__isfalse=class_id__isfalse, class_id__istrue=class_id__istrue, class_id__like=class_id__like, class_id__notlike=class_id__notlike, class_id__ilike=class_id__ilike, class_id__notilike=class_id__notilike, class_id__desc=class_id__desc, class_id__asc=class_id__asc, class_name__eq=class_name__eq, class_name__ne=class_name__ne, class_name__gt=class_name__gt, class_name__gte=class_name__gte, class_name__lt=class_name__lt, class_name__lte=class_name__lte, class_name__in=class_name__in, class_name__notin=class_name__notin, class_name__isnull=class_name__isnull, class_name__isnotnull=class_name__isnotnull, class_name__isfalse=class_name__isfalse, class_name__istrue=class_name__istrue, class_name__like=class_name__like, class_name__notlike=class_name__notlike, class_name__ilike=class_name__ilike, class_name__notilike=class_name__notilike, class_name__desc=class_name__desc, class_name__asc=class_name__asc, score__eq=score__eq, score__ne=score__ne, score__gt=score__gt, score__gte=score__gte, score__lt=score__lt, score__lte=score__lte, score__in=score__in, score__notin=score__notin, score__isnull=score__isnull, score__isnotnull=score__isnotnull, score__isfalse=score__isfalse, score__istrue=score__istrue, score__like=score__like, score__notlike=score__notlike, score__ilike=score__ilike, score__notilike=score__notilike, score__desc=score__desc, score__asc=score__asc, video_id__eq=video_id__eq, video_id__ne=video_id__ne, video_id__gt=video_id__gt, video_id__gte=video_id__gte, video_id__lt=video_id__lt, video_id__lte=video_id__lte, video_id__in=video_id__in, video_id__notin=video_id__notin, video_id__isnull=video_id__isnull, video_id__isnotnull=video_id__isnotnull, video_id__isfalse=video_id__isfalse, video_id__istrue=video_id__istrue, video_id__like=video_id__like, video_id__notlike=video_id__notlike, video_id__ilike=video_id__ilike, video_id__notilike=video_id__notilike, video_id__desc=video_id__desc, video_id__asc=video_id__asc, camera_id__eq=camera_id__eq, camera_id__ne=camera_id__ne, camera_id__gt=camera_id__gt, camera_id__gte=camera_id__gte, camera_id__lt=camera_id__lt, camera_id__lte=camera_id__lte, camera_id__in=camera_id__in, camera_id__notin=camera_id__notin, camera_id__isnull=camera_id__isnull, camera_id__isnotnull=camera_id__isnotnull, camera_id__isfalse=camera_id__isfalse, camera_id__istrue=camera_id__istrue, camera_id__like=camera_id__like, camera_id__notlike=camera_id__notlike, camera_id__ilike=camera_id__ilike, camera_id__notilike=camera_id__notilike, camera_id__desc=camera_id__desc, camera_id__asc=camera_id__asc)
        print("The response of DetectionApi->get_detections:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DetectionApi->get_detections: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int**| SQL LIMIT operator | [optional] 
 **offset** | **int**| SQL OFFSET operator | [optional] 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 
 **id__eq** | **str**| SQL &#x3D; comparison | [optional] 
 **id__ne** | **str**| SQL !&#x3D; comparison | [optional] 
 **id__gt** | **str**| SQL &gt; comparison, may not work with all column types | [optional] 
 **id__gte** | **str**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **id__lt** | **str**| SQL &lt; comparison, may not work with all column types | [optional] 
 **id__lte** | **str**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **id__in** | **str**| SQL IN comparison, permits comma-separated values | [optional] 
 **id__notin** | **str**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **id__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **id__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **id__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **id__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **id__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **id__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **id__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **id__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **id__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **id__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **created_at__eq** | **datetime**| SQL &#x3D; comparison | [optional] 
 **created_at__ne** | **datetime**| SQL !&#x3D; comparison | [optional] 
 **created_at__gt** | **datetime**| SQL &gt; comparison, may not work with all column types | [optional] 
 **created_at__gte** | **datetime**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **created_at__lt** | **datetime**| SQL &lt; comparison, may not work with all column types | [optional] 
 **created_at__lte** | **datetime**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **created_at__in** | **datetime**| SQL IN comparison, permits comma-separated values | [optional] 
 **created_at__notin** | **datetime**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **created_at__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **created_at__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **created_at__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **created_at__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **created_at__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **created_at__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **updated_at__eq** | **datetime**| SQL &#x3D; comparison | [optional] 
 **updated_at__ne** | **datetime**| SQL !&#x3D; comparison | [optional] 
 **updated_at__gt** | **datetime**| SQL &gt; comparison, may not work with all column types | [optional] 
 **updated_at__gte** | **datetime**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **updated_at__lt** | **datetime**| SQL &lt; comparison, may not work with all column types | [optional] 
 **updated_at__lte** | **datetime**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **updated_at__in** | **datetime**| SQL IN comparison, permits comma-separated values | [optional] 
 **updated_at__notin** | **datetime**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **updated_at__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **updated_at__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **updated_at__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **updated_at__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **updated_at__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **updated_at__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **deleted_at__eq** | **datetime**| SQL &#x3D; comparison | [optional] 
 **deleted_at__ne** | **datetime**| SQL !&#x3D; comparison | [optional] 
 **deleted_at__gt** | **datetime**| SQL &gt; comparison, may not work with all column types | [optional] 
 **deleted_at__gte** | **datetime**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **deleted_at__lt** | **datetime**| SQL &lt; comparison, may not work with all column types | [optional] 
 **deleted_at__lte** | **datetime**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **deleted_at__in** | **datetime**| SQL IN comparison, permits comma-separated values | [optional] 
 **deleted_at__notin** | **datetime**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **deleted_at__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **deleted_at__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **deleted_at__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **deleted_at__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **deleted_at__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **deleted_at__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **seen_at__eq** | **datetime**| SQL &#x3D; comparison | [optional] 
 **seen_at__ne** | **datetime**| SQL !&#x3D; comparison | [optional] 
 **seen_at__gt** | **datetime**| SQL &gt; comparison, may not work with all column types | [optional] 
 **seen_at__gte** | **datetime**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **seen_at__lt** | **datetime**| SQL &lt; comparison, may not work with all column types | [optional] 
 **seen_at__lte** | **datetime**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **seen_at__in** | **datetime**| SQL IN comparison, permits comma-separated values | [optional] 
 **seen_at__notin** | **datetime**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **seen_at__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **seen_at__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **seen_at__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **seen_at__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **seen_at__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **seen_at__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **seen_at__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **seen_at__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **seen_at__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **seen_at__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **class_id__eq** | **int**| SQL &#x3D; comparison | [optional] 
 **class_id__ne** | **int**| SQL !&#x3D; comparison | [optional] 
 **class_id__gt** | **int**| SQL &gt; comparison, may not work with all column types | [optional] 
 **class_id__gte** | **int**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **class_id__lt** | **int**| SQL &lt; comparison, may not work with all column types | [optional] 
 **class_id__lte** | **int**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **class_id__in** | **int**| SQL IN comparison, permits comma-separated values | [optional] 
 **class_id__notin** | **int**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **class_id__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **class_id__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **class_id__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **class_id__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **class_id__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **class_id__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **class_id__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **class_id__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **class_id__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **class_id__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **class_name__eq** | **str**| SQL &#x3D; comparison | [optional] 
 **class_name__ne** | **str**| SQL !&#x3D; comparison | [optional] 
 **class_name__gt** | **str**| SQL &gt; comparison, may not work with all column types | [optional] 
 **class_name__gte** | **str**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **class_name__lt** | **str**| SQL &lt; comparison, may not work with all column types | [optional] 
 **class_name__lte** | **str**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **class_name__in** | **str**| SQL IN comparison, permits comma-separated values | [optional] 
 **class_name__notin** | **str**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **class_name__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **class_name__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **class_name__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **class_name__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **class_name__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **class_name__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **class_name__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **class_name__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **class_name__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **class_name__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **score__eq** | **float**| SQL &#x3D; comparison | [optional] 
 **score__ne** | **float**| SQL !&#x3D; comparison | [optional] 
 **score__gt** | **float**| SQL &gt; comparison, may not work with all column types | [optional] 
 **score__gte** | **float**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **score__lt** | **float**| SQL &lt; comparison, may not work with all column types | [optional] 
 **score__lte** | **float**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **score__in** | **float**| SQL IN comparison, permits comma-separated values | [optional] 
 **score__notin** | **float**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **score__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **score__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **score__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **score__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **score__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **score__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **score__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **score__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **score__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **score__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **video_id__eq** | **str**| SQL &#x3D; comparison | [optional] 
 **video_id__ne** | **str**| SQL !&#x3D; comparison | [optional] 
 **video_id__gt** | **str**| SQL &gt; comparison, may not work with all column types | [optional] 
 **video_id__gte** | **str**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **video_id__lt** | **str**| SQL &lt; comparison, may not work with all column types | [optional] 
 **video_id__lte** | **str**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **video_id__in** | **str**| SQL IN comparison, permits comma-separated values | [optional] 
 **video_id__notin** | **str**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **video_id__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **video_id__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **video_id__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **video_id__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **video_id__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **video_id__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **video_id__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **video_id__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **video_id__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **video_id__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **camera_id__eq** | **str**| SQL &#x3D; comparison | [optional] 
 **camera_id__ne** | **str**| SQL !&#x3D; comparison | [optional] 
 **camera_id__gt** | **str**| SQL &gt; comparison, may not work with all column types | [optional] 
 **camera_id__gte** | **str**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **camera_id__lt** | **str**| SQL &lt; comparison, may not work with all column types | [optional] 
 **camera_id__lte** | **str**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **camera_id__in** | **str**| SQL IN comparison, permits comma-separated values | [optional] 
 **camera_id__notin** | **str**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **camera_id__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **camera_id__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **camera_id__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **camera_id__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **camera_id__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **camera_id__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **camera_id__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **camera_id__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **camera_id__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **camera_id__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 

### Return type

[**GetDetections200Response**](GetDetections200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful List Fetch for Detections |  -  |
**0** | Failed List Fetch for Detections |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_detection**
> GetDetections200Response patch_detection(primary_key, detection, depth=depth)



### Example


```python
import openapi_client
from openapi_client.models.detection import Detection
from openapi_client.models.get_detections200_response import GetDetections200Response
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.DetectionApi(api_client)
    primary_key = None # object | Primary key for Detection
    detection = openapi_client.Detection() # Detection | 
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_response = api_instance.patch_detection(primary_key, detection, depth=depth)
        print("The response of DetectionApi->patch_detection:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DetectionApi->patch_detection: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **primary_key** | [**object**](.md)| Primary key for Detection | 
 **detection** | [**Detection**](Detection.md)|  | 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 

### Return type

[**GetDetections200Response**](GetDetections200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful Item Update for Detections |  -  |
**0** | Failed Item Update for Detections |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_detections**
> GetDetections200Response post_detections(detection, depth=depth)



### Example


```python
import openapi_client
from openapi_client.models.detection import Detection
from openapi_client.models.get_detections200_response import GetDetections200Response
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.DetectionApi(api_client)
    detection = [openapi_client.Detection()] # List[Detection] | 
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_response = api_instance.post_detections(detection, depth=depth)
        print("The response of DetectionApi->post_detections:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DetectionApi->post_detections: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **detection** | [**List[Detection]**](Detection.md)|  | 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 

### Return type

[**GetDetections200Response**](GetDetections200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful List Create for Detections |  -  |
**0** | Failed List Create for Detections |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_detection**
> GetDetections200Response put_detection(primary_key, detection, depth=depth)



### Example


```python
import openapi_client
from openapi_client.models.detection import Detection
from openapi_client.models.get_detections200_response import GetDetections200Response
from openapi_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = openapi_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with openapi_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = openapi_client.DetectionApi(api_client)
    primary_key = None # object | Primary key for Detection
    detection = openapi_client.Detection() # Detection | 
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_response = api_instance.put_detection(primary_key, detection, depth=depth)
        print("The response of DetectionApi->put_detection:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DetectionApi->put_detection: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **primary_key** | [**object**](.md)| Primary key for Detection | 
 **detection** | [**Detection**](Detection.md)|  | 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 

### Return type

[**GetDetections200Response**](GetDetections200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful Item Replace for Detections |  -  |
**0** | Failed Item Replace for Detections |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

