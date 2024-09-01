# openapi_client.VideoApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_video**](VideoApi.md#delete_video) | **DELETE** /api/videos/{primaryKey} | 
[**get_video**](VideoApi.md#get_video) | **GET** /api/videos/{primaryKey} | 
[**get_videos**](VideoApi.md#get_videos) | **GET** /api/videos | 
[**patch_video**](VideoApi.md#patch_video) | **PATCH** /api/videos/{primaryKey} | 
[**post_videos**](VideoApi.md#post_videos) | **POST** /api/videos | 
[**put_video**](VideoApi.md#put_video) | **PUT** /api/videos/{primaryKey} | 


# **delete_video**
> delete_video(primary_key, depth=depth)



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
    api_instance = openapi_client.VideoApi(api_client)
    primary_key = None # object | Primary key for Video
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_instance.delete_video(primary_key, depth=depth)
    except Exception as e:
        print("Exception when calling VideoApi->delete_video: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **primary_key** | [**object**](.md)| Primary key for Video | 
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
**204** | Successful Item Delete for Videos |  -  |
**0** | Failed Item Delete for Videos |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_video**
> GetVideos200Response get_video(primary_key, depth=depth)



### Example


```python
import openapi_client
from openapi_client.models.get_videos200_response import GetVideos200Response
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
    api_instance = openapi_client.VideoApi(api_client)
    primary_key = None # object | Primary key for Video
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_response = api_instance.get_video(primary_key, depth=depth)
        print("The response of VideoApi->get_video:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling VideoApi->get_video: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **primary_key** | [**object**](.md)| Primary key for Video | 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 

### Return type

[**GetVideos200Response**](GetVideos200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful Item Fetch for Videos |  -  |
**0** | Failed Item Fetch for Videos |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_videos**
> GetVideos200Response get_videos(limit=limit, offset=offset, depth=depth, id__eq=id__eq, id__ne=id__ne, id__gt=id__gt, id__gte=id__gte, id__lt=id__lt, id__lte=id__lte, id__in=id__in, id__notin=id__notin, id__isnull=id__isnull, id__isnotnull=id__isnotnull, id__isfalse=id__isfalse, id__istrue=id__istrue, id__like=id__like, id__notlike=id__notlike, id__ilike=id__ilike, id__notilike=id__notilike, id__desc=id__desc, id__asc=id__asc, created_at__eq=created_at__eq, created_at__ne=created_at__ne, created_at__gt=created_at__gt, created_at__gte=created_at__gte, created_at__lt=created_at__lt, created_at__lte=created_at__lte, created_at__in=created_at__in, created_at__notin=created_at__notin, created_at__isnull=created_at__isnull, created_at__isnotnull=created_at__isnotnull, created_at__isfalse=created_at__isfalse, created_at__istrue=created_at__istrue, created_at__like=created_at__like, created_at__notlike=created_at__notlike, created_at__ilike=created_at__ilike, created_at__notilike=created_at__notilike, created_at__desc=created_at__desc, created_at__asc=created_at__asc, updated_at__eq=updated_at__eq, updated_at__ne=updated_at__ne, updated_at__gt=updated_at__gt, updated_at__gte=updated_at__gte, updated_at__lt=updated_at__lt, updated_at__lte=updated_at__lte, updated_at__in=updated_at__in, updated_at__notin=updated_at__notin, updated_at__isnull=updated_at__isnull, updated_at__isnotnull=updated_at__isnotnull, updated_at__isfalse=updated_at__isfalse, updated_at__istrue=updated_at__istrue, updated_at__like=updated_at__like, updated_at__notlike=updated_at__notlike, updated_at__ilike=updated_at__ilike, updated_at__notilike=updated_at__notilike, updated_at__desc=updated_at__desc, updated_at__asc=updated_at__asc, deleted_at__eq=deleted_at__eq, deleted_at__ne=deleted_at__ne, deleted_at__gt=deleted_at__gt, deleted_at__gte=deleted_at__gte, deleted_at__lt=deleted_at__lt, deleted_at__lte=deleted_at__lte, deleted_at__in=deleted_at__in, deleted_at__notin=deleted_at__notin, deleted_at__isnull=deleted_at__isnull, deleted_at__isnotnull=deleted_at__isnotnull, deleted_at__isfalse=deleted_at__isfalse, deleted_at__istrue=deleted_at__istrue, deleted_at__like=deleted_at__like, deleted_at__notlike=deleted_at__notlike, deleted_at__ilike=deleted_at__ilike, deleted_at__notilike=deleted_at__notilike, deleted_at__desc=deleted_at__desc, deleted_at__asc=deleted_at__asc, file_name__eq=file_name__eq, file_name__ne=file_name__ne, file_name__gt=file_name__gt, file_name__gte=file_name__gte, file_name__lt=file_name__lt, file_name__lte=file_name__lte, file_name__in=file_name__in, file_name__notin=file_name__notin, file_name__isnull=file_name__isnull, file_name__isnotnull=file_name__isnotnull, file_name__isfalse=file_name__isfalse, file_name__istrue=file_name__istrue, file_name__like=file_name__like, file_name__notlike=file_name__notlike, file_name__ilike=file_name__ilike, file_name__notilike=file_name__notilike, file_name__desc=file_name__desc, file_name__asc=file_name__asc, started_at__eq=started_at__eq, started_at__ne=started_at__ne, started_at__gt=started_at__gt, started_at__gte=started_at__gte, started_at__lt=started_at__lt, started_at__lte=started_at__lte, started_at__in=started_at__in, started_at__notin=started_at__notin, started_at__isnull=started_at__isnull, started_at__isnotnull=started_at__isnotnull, started_at__isfalse=started_at__isfalse, started_at__istrue=started_at__istrue, started_at__like=started_at__like, started_at__notlike=started_at__notlike, started_at__ilike=started_at__ilike, started_at__notilike=started_at__notilike, started_at__desc=started_at__desc, started_at__asc=started_at__asc, ended_at__eq=ended_at__eq, ended_at__ne=ended_at__ne, ended_at__gt=ended_at__gt, ended_at__gte=ended_at__gte, ended_at__lt=ended_at__lt, ended_at__lte=ended_at__lte, ended_at__in=ended_at__in, ended_at__notin=ended_at__notin, ended_at__isnull=ended_at__isnull, ended_at__isnotnull=ended_at__isnotnull, ended_at__isfalse=ended_at__isfalse, ended_at__istrue=ended_at__istrue, ended_at__like=ended_at__like, ended_at__notlike=ended_at__notlike, ended_at__ilike=ended_at__ilike, ended_at__notilike=ended_at__notilike, ended_at__desc=ended_at__desc, ended_at__asc=ended_at__asc, duration__eq=duration__eq, duration__ne=duration__ne, duration__gt=duration__gt, duration__gte=duration__gte, duration__lt=duration__lt, duration__lte=duration__lte, duration__in=duration__in, duration__notin=duration__notin, duration__isnull=duration__isnull, duration__isnotnull=duration__isnotnull, duration__isfalse=duration__isfalse, duration__istrue=duration__istrue, duration__like=duration__like, duration__notlike=duration__notlike, duration__ilike=duration__ilike, duration__notilike=duration__notilike, duration__desc=duration__desc, duration__asc=duration__asc, file_size__eq=file_size__eq, file_size__ne=file_size__ne, file_size__gt=file_size__gt, file_size__gte=file_size__gte, file_size__lt=file_size__lt, file_size__lte=file_size__lte, file_size__in=file_size__in, file_size__notin=file_size__notin, file_size__isnull=file_size__isnull, file_size__isnotnull=file_size__isnotnull, file_size__isfalse=file_size__isfalse, file_size__istrue=file_size__istrue, file_size__like=file_size__like, file_size__notlike=file_size__notlike, file_size__ilike=file_size__ilike, file_size__notilike=file_size__notilike, file_size__desc=file_size__desc, file_size__asc=file_size__asc, thumbnail_name__eq=thumbnail_name__eq, thumbnail_name__ne=thumbnail_name__ne, thumbnail_name__gt=thumbnail_name__gt, thumbnail_name__gte=thumbnail_name__gte, thumbnail_name__lt=thumbnail_name__lt, thumbnail_name__lte=thumbnail_name__lte, thumbnail_name__in=thumbnail_name__in, thumbnail_name__notin=thumbnail_name__notin, thumbnail_name__isnull=thumbnail_name__isnull, thumbnail_name__isnotnull=thumbnail_name__isnotnull, thumbnail_name__isfalse=thumbnail_name__isfalse, thumbnail_name__istrue=thumbnail_name__istrue, thumbnail_name__like=thumbnail_name__like, thumbnail_name__notlike=thumbnail_name__notlike, thumbnail_name__ilike=thumbnail_name__ilike, thumbnail_name__notilike=thumbnail_name__notilike, thumbnail_name__desc=thumbnail_name__desc, thumbnail_name__asc=thumbnail_name__asc, status__eq=status__eq, status__ne=status__ne, status__gt=status__gt, status__gte=status__gte, status__lt=status__lt, status__lte=status__lte, status__in=status__in, status__notin=status__notin, status__isnull=status__isnull, status__isnotnull=status__isnotnull, status__isfalse=status__isfalse, status__istrue=status__istrue, status__like=status__like, status__notlike=status__notlike, status__ilike=status__ilike, status__notilike=status__notilike, status__desc=status__desc, status__asc=status__asc, object_detector_claimed_until__eq=object_detector_claimed_until__eq, object_detector_claimed_until__ne=object_detector_claimed_until__ne, object_detector_claimed_until__gt=object_detector_claimed_until__gt, object_detector_claimed_until__gte=object_detector_claimed_until__gte, object_detector_claimed_until__lt=object_detector_claimed_until__lt, object_detector_claimed_until__lte=object_detector_claimed_until__lte, object_detector_claimed_until__in=object_detector_claimed_until__in, object_detector_claimed_until__notin=object_detector_claimed_until__notin, object_detector_claimed_until__isnull=object_detector_claimed_until__isnull, object_detector_claimed_until__isnotnull=object_detector_claimed_until__isnotnull, object_detector_claimed_until__isfalse=object_detector_claimed_until__isfalse, object_detector_claimed_until__istrue=object_detector_claimed_until__istrue, object_detector_claimed_until__like=object_detector_claimed_until__like, object_detector_claimed_until__notlike=object_detector_claimed_until__notlike, object_detector_claimed_until__ilike=object_detector_claimed_until__ilike, object_detector_claimed_until__notilike=object_detector_claimed_until__notilike, object_detector_claimed_until__desc=object_detector_claimed_until__desc, object_detector_claimed_until__asc=object_detector_claimed_until__asc, object_tracker_claimed_until__eq=object_tracker_claimed_until__eq, object_tracker_claimed_until__ne=object_tracker_claimed_until__ne, object_tracker_claimed_until__gt=object_tracker_claimed_until__gt, object_tracker_claimed_until__gte=object_tracker_claimed_until__gte, object_tracker_claimed_until__lt=object_tracker_claimed_until__lt, object_tracker_claimed_until__lte=object_tracker_claimed_until__lte, object_tracker_claimed_until__in=object_tracker_claimed_until__in, object_tracker_claimed_until__notin=object_tracker_claimed_until__notin, object_tracker_claimed_until__isnull=object_tracker_claimed_until__isnull, object_tracker_claimed_until__isnotnull=object_tracker_claimed_until__isnotnull, object_tracker_claimed_until__isfalse=object_tracker_claimed_until__isfalse, object_tracker_claimed_until__istrue=object_tracker_claimed_until__istrue, object_tracker_claimed_until__like=object_tracker_claimed_until__like, object_tracker_claimed_until__notlike=object_tracker_claimed_until__notlike, object_tracker_claimed_until__ilike=object_tracker_claimed_until__ilike, object_tracker_claimed_until__notilike=object_tracker_claimed_until__notilike, object_tracker_claimed_until__desc=object_tracker_claimed_until__desc, object_tracker_claimed_until__asc=object_tracker_claimed_until__asc, camera_id__eq=camera_id__eq, camera_id__ne=camera_id__ne, camera_id__gt=camera_id__gt, camera_id__gte=camera_id__gte, camera_id__lt=camera_id__lt, camera_id__lte=camera_id__lte, camera_id__in=camera_id__in, camera_id__notin=camera_id__notin, camera_id__isnull=camera_id__isnull, camera_id__isnotnull=camera_id__isnotnull, camera_id__isfalse=camera_id__isfalse, camera_id__istrue=camera_id__istrue, camera_id__like=camera_id__like, camera_id__notlike=camera_id__notlike, camera_id__ilike=camera_id__ilike, camera_id__notilike=camera_id__notilike, camera_id__desc=camera_id__desc, camera_id__asc=camera_id__asc)



### Example


```python
import openapi_client
from openapi_client.models.get_videos200_response import GetVideos200Response
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
    api_instance = openapi_client.VideoApi(api_client)
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
    file_name__eq = 'file_name__eq_example' # str | SQL = comparison (optional)
    file_name__ne = 'file_name__ne_example' # str | SQL != comparison (optional)
    file_name__gt = 'file_name__gt_example' # str | SQL > comparison, may not work with all column types (optional)
    file_name__gte = 'file_name__gte_example' # str | SQL >= comparison, may not work with all column types (optional)
    file_name__lt = 'file_name__lt_example' # str | SQL < comparison, may not work with all column types (optional)
    file_name__lte = 'file_name__lte_example' # str | SQL <= comparison, may not work with all column types (optional)
    file_name__in = 'file_name__in_example' # str | SQL IN comparison, permits comma-separated values (optional)
    file_name__notin = 'file_name__notin_example' # str | SQL NOT IN comparison, permits comma-separated values (optional)
    file_name__isnull = 'file_name__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    file_name__isnotnull = 'file_name__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    file_name__isfalse = 'file_name__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    file_name__istrue = 'file_name__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    file_name__like = 'file_name__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    file_name__notlike = 'file_name__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    file_name__ilike = 'file_name__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    file_name__notilike = 'file_name__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    file_name__desc = 'file_name__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    file_name__asc = 'file_name__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    started_at__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = comparison (optional)
    started_at__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != comparison (optional)
    started_at__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > comparison, may not work with all column types (optional)
    started_at__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= comparison, may not work with all column types (optional)
    started_at__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < comparison, may not work with all column types (optional)
    started_at__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= comparison, may not work with all column types (optional)
    started_at__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN comparison, permits comma-separated values (optional)
    started_at__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN comparison, permits comma-separated values (optional)
    started_at__isnull = 'started_at__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    started_at__isnotnull = 'started_at__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    started_at__isfalse = 'started_at__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    started_at__istrue = 'started_at__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    started_at__like = 'started_at__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    started_at__notlike = 'started_at__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    started_at__ilike = 'started_at__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    started_at__notilike = 'started_at__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    started_at__desc = 'started_at__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    started_at__asc = 'started_at__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    ended_at__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = comparison (optional)
    ended_at__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != comparison (optional)
    ended_at__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > comparison, may not work with all column types (optional)
    ended_at__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= comparison, may not work with all column types (optional)
    ended_at__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < comparison, may not work with all column types (optional)
    ended_at__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= comparison, may not work with all column types (optional)
    ended_at__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN comparison, permits comma-separated values (optional)
    ended_at__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN comparison, permits comma-separated values (optional)
    ended_at__isnull = 'ended_at__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    ended_at__isnotnull = 'ended_at__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    ended_at__isfalse = 'ended_at__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    ended_at__istrue = 'ended_at__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    ended_at__like = 'ended_at__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    ended_at__notlike = 'ended_at__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    ended_at__ilike = 'ended_at__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    ended_at__notilike = 'ended_at__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    ended_at__desc = 'ended_at__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    ended_at__asc = 'ended_at__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    duration__eq = 56 # int | SQL = comparison (optional)
    duration__ne = 56 # int | SQL != comparison (optional)
    duration__gt = 56 # int | SQL > comparison, may not work with all column types (optional)
    duration__gte = 56 # int | SQL >= comparison, may not work with all column types (optional)
    duration__lt = 56 # int | SQL < comparison, may not work with all column types (optional)
    duration__lte = 56 # int | SQL <= comparison, may not work with all column types (optional)
    duration__in = 56 # int | SQL IN comparison, permits comma-separated values (optional)
    duration__notin = 56 # int | SQL NOT IN comparison, permits comma-separated values (optional)
    duration__isnull = 'duration__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    duration__isnotnull = 'duration__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    duration__isfalse = 'duration__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    duration__istrue = 'duration__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    duration__like = 'duration__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    duration__notlike = 'duration__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    duration__ilike = 'duration__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    duration__notilike = 'duration__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    duration__desc = 'duration__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    duration__asc = 'duration__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    file_size__eq = 3.4 # float | SQL = comparison (optional)
    file_size__ne = 3.4 # float | SQL != comparison (optional)
    file_size__gt = 3.4 # float | SQL > comparison, may not work with all column types (optional)
    file_size__gte = 3.4 # float | SQL >= comparison, may not work with all column types (optional)
    file_size__lt = 3.4 # float | SQL < comparison, may not work with all column types (optional)
    file_size__lte = 3.4 # float | SQL <= comparison, may not work with all column types (optional)
    file_size__in = 3.4 # float | SQL IN comparison, permits comma-separated values (optional)
    file_size__notin = 3.4 # float | SQL NOT IN comparison, permits comma-separated values (optional)
    file_size__isnull = 'file_size__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    file_size__isnotnull = 'file_size__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    file_size__isfalse = 'file_size__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    file_size__istrue = 'file_size__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    file_size__like = 'file_size__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    file_size__notlike = 'file_size__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    file_size__ilike = 'file_size__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    file_size__notilike = 'file_size__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    file_size__desc = 'file_size__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    file_size__asc = 'file_size__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    thumbnail_name__eq = 'thumbnail_name__eq_example' # str | SQL = comparison (optional)
    thumbnail_name__ne = 'thumbnail_name__ne_example' # str | SQL != comparison (optional)
    thumbnail_name__gt = 'thumbnail_name__gt_example' # str | SQL > comparison, may not work with all column types (optional)
    thumbnail_name__gte = 'thumbnail_name__gte_example' # str | SQL >= comparison, may not work with all column types (optional)
    thumbnail_name__lt = 'thumbnail_name__lt_example' # str | SQL < comparison, may not work with all column types (optional)
    thumbnail_name__lte = 'thumbnail_name__lte_example' # str | SQL <= comparison, may not work with all column types (optional)
    thumbnail_name__in = 'thumbnail_name__in_example' # str | SQL IN comparison, permits comma-separated values (optional)
    thumbnail_name__notin = 'thumbnail_name__notin_example' # str | SQL NOT IN comparison, permits comma-separated values (optional)
    thumbnail_name__isnull = 'thumbnail_name__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    thumbnail_name__isnotnull = 'thumbnail_name__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    thumbnail_name__isfalse = 'thumbnail_name__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    thumbnail_name__istrue = 'thumbnail_name__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    thumbnail_name__like = 'thumbnail_name__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    thumbnail_name__notlike = 'thumbnail_name__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    thumbnail_name__ilike = 'thumbnail_name__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    thumbnail_name__notilike = 'thumbnail_name__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    thumbnail_name__desc = 'thumbnail_name__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    thumbnail_name__asc = 'thumbnail_name__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    status__eq = 'status__eq_example' # str | SQL = comparison (optional)
    status__ne = 'status__ne_example' # str | SQL != comparison (optional)
    status__gt = 'status__gt_example' # str | SQL > comparison, may not work with all column types (optional)
    status__gte = 'status__gte_example' # str | SQL >= comparison, may not work with all column types (optional)
    status__lt = 'status__lt_example' # str | SQL < comparison, may not work with all column types (optional)
    status__lte = 'status__lte_example' # str | SQL <= comparison, may not work with all column types (optional)
    status__in = 'status__in_example' # str | SQL IN comparison, permits comma-separated values (optional)
    status__notin = 'status__notin_example' # str | SQL NOT IN comparison, permits comma-separated values (optional)
    status__isnull = 'status__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    status__isnotnull = 'status__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    status__isfalse = 'status__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    status__istrue = 'status__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    status__like = 'status__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    status__notlike = 'status__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    status__ilike = 'status__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    status__notilike = 'status__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    status__desc = 'status__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    status__asc = 'status__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    object_detector_claimed_until__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = comparison (optional)
    object_detector_claimed_until__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != comparison (optional)
    object_detector_claimed_until__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > comparison, may not work with all column types (optional)
    object_detector_claimed_until__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= comparison, may not work with all column types (optional)
    object_detector_claimed_until__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < comparison, may not work with all column types (optional)
    object_detector_claimed_until__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= comparison, may not work with all column types (optional)
    object_detector_claimed_until__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN comparison, permits comma-separated values (optional)
    object_detector_claimed_until__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN comparison, permits comma-separated values (optional)
    object_detector_claimed_until__isnull = 'object_detector_claimed_until__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    object_detector_claimed_until__isnotnull = 'object_detector_claimed_until__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    object_detector_claimed_until__isfalse = 'object_detector_claimed_until__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    object_detector_claimed_until__istrue = 'object_detector_claimed_until__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    object_detector_claimed_until__like = 'object_detector_claimed_until__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    object_detector_claimed_until__notlike = 'object_detector_claimed_until__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    object_detector_claimed_until__ilike = 'object_detector_claimed_until__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    object_detector_claimed_until__notilike = 'object_detector_claimed_until__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    object_detector_claimed_until__desc = 'object_detector_claimed_until__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    object_detector_claimed_until__asc = 'object_detector_claimed_until__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
    object_tracker_claimed_until__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = comparison (optional)
    object_tracker_claimed_until__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != comparison (optional)
    object_tracker_claimed_until__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > comparison, may not work with all column types (optional)
    object_tracker_claimed_until__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= comparison, may not work with all column types (optional)
    object_tracker_claimed_until__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < comparison, may not work with all column types (optional)
    object_tracker_claimed_until__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= comparison, may not work with all column types (optional)
    object_tracker_claimed_until__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN comparison, permits comma-separated values (optional)
    object_tracker_claimed_until__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN comparison, permits comma-separated values (optional)
    object_tracker_claimed_until__isnull = 'object_tracker_claimed_until__isnull_example' # str | SQL IS null comparison, value is ignored (presence of key is sufficient) (optional)
    object_tracker_claimed_until__isnotnull = 'object_tracker_claimed_until__isnotnull_example' # str | SQL IS NOT null comparison, value is ignored (presence of key is sufficient) (optional)
    object_tracker_claimed_until__isfalse = 'object_tracker_claimed_until__isfalse_example' # str | SQL IS false comparison, value is ignored (presence of key is sufficient) (optional)
    object_tracker_claimed_until__istrue = 'object_tracker_claimed_until__istrue_example' # str | SQL IS true comparison, value is ignored (presence of key is sufficient) (optional)
    object_tracker_claimed_until__like = 'object_tracker_claimed_until__like_example' # str | SQL LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    object_tracker_claimed_until__notlike = 'object_tracker_claimed_until__notlike_example' # str | SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    object_tracker_claimed_until__ilike = 'object_tracker_claimed_until__ilike_example' # str | SQL ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    object_tracker_claimed_until__notilike = 'object_tracker_claimed_until__notilike_example' # str | SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % (optional)
    object_tracker_claimed_until__desc = 'object_tracker_claimed_until__desc_example' # str | SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) (optional)
    object_tracker_claimed_until__asc = 'object_tracker_claimed_until__asc_example' # str | SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) (optional)
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
        api_response = api_instance.get_videos(limit=limit, offset=offset, depth=depth, id__eq=id__eq, id__ne=id__ne, id__gt=id__gt, id__gte=id__gte, id__lt=id__lt, id__lte=id__lte, id__in=id__in, id__notin=id__notin, id__isnull=id__isnull, id__isnotnull=id__isnotnull, id__isfalse=id__isfalse, id__istrue=id__istrue, id__like=id__like, id__notlike=id__notlike, id__ilike=id__ilike, id__notilike=id__notilike, id__desc=id__desc, id__asc=id__asc, created_at__eq=created_at__eq, created_at__ne=created_at__ne, created_at__gt=created_at__gt, created_at__gte=created_at__gte, created_at__lt=created_at__lt, created_at__lte=created_at__lte, created_at__in=created_at__in, created_at__notin=created_at__notin, created_at__isnull=created_at__isnull, created_at__isnotnull=created_at__isnotnull, created_at__isfalse=created_at__isfalse, created_at__istrue=created_at__istrue, created_at__like=created_at__like, created_at__notlike=created_at__notlike, created_at__ilike=created_at__ilike, created_at__notilike=created_at__notilike, created_at__desc=created_at__desc, created_at__asc=created_at__asc, updated_at__eq=updated_at__eq, updated_at__ne=updated_at__ne, updated_at__gt=updated_at__gt, updated_at__gte=updated_at__gte, updated_at__lt=updated_at__lt, updated_at__lte=updated_at__lte, updated_at__in=updated_at__in, updated_at__notin=updated_at__notin, updated_at__isnull=updated_at__isnull, updated_at__isnotnull=updated_at__isnotnull, updated_at__isfalse=updated_at__isfalse, updated_at__istrue=updated_at__istrue, updated_at__like=updated_at__like, updated_at__notlike=updated_at__notlike, updated_at__ilike=updated_at__ilike, updated_at__notilike=updated_at__notilike, updated_at__desc=updated_at__desc, updated_at__asc=updated_at__asc, deleted_at__eq=deleted_at__eq, deleted_at__ne=deleted_at__ne, deleted_at__gt=deleted_at__gt, deleted_at__gte=deleted_at__gte, deleted_at__lt=deleted_at__lt, deleted_at__lte=deleted_at__lte, deleted_at__in=deleted_at__in, deleted_at__notin=deleted_at__notin, deleted_at__isnull=deleted_at__isnull, deleted_at__isnotnull=deleted_at__isnotnull, deleted_at__isfalse=deleted_at__isfalse, deleted_at__istrue=deleted_at__istrue, deleted_at__like=deleted_at__like, deleted_at__notlike=deleted_at__notlike, deleted_at__ilike=deleted_at__ilike, deleted_at__notilike=deleted_at__notilike, deleted_at__desc=deleted_at__desc, deleted_at__asc=deleted_at__asc, file_name__eq=file_name__eq, file_name__ne=file_name__ne, file_name__gt=file_name__gt, file_name__gte=file_name__gte, file_name__lt=file_name__lt, file_name__lte=file_name__lte, file_name__in=file_name__in, file_name__notin=file_name__notin, file_name__isnull=file_name__isnull, file_name__isnotnull=file_name__isnotnull, file_name__isfalse=file_name__isfalse, file_name__istrue=file_name__istrue, file_name__like=file_name__like, file_name__notlike=file_name__notlike, file_name__ilike=file_name__ilike, file_name__notilike=file_name__notilike, file_name__desc=file_name__desc, file_name__asc=file_name__asc, started_at__eq=started_at__eq, started_at__ne=started_at__ne, started_at__gt=started_at__gt, started_at__gte=started_at__gte, started_at__lt=started_at__lt, started_at__lte=started_at__lte, started_at__in=started_at__in, started_at__notin=started_at__notin, started_at__isnull=started_at__isnull, started_at__isnotnull=started_at__isnotnull, started_at__isfalse=started_at__isfalse, started_at__istrue=started_at__istrue, started_at__like=started_at__like, started_at__notlike=started_at__notlike, started_at__ilike=started_at__ilike, started_at__notilike=started_at__notilike, started_at__desc=started_at__desc, started_at__asc=started_at__asc, ended_at__eq=ended_at__eq, ended_at__ne=ended_at__ne, ended_at__gt=ended_at__gt, ended_at__gte=ended_at__gte, ended_at__lt=ended_at__lt, ended_at__lte=ended_at__lte, ended_at__in=ended_at__in, ended_at__notin=ended_at__notin, ended_at__isnull=ended_at__isnull, ended_at__isnotnull=ended_at__isnotnull, ended_at__isfalse=ended_at__isfalse, ended_at__istrue=ended_at__istrue, ended_at__like=ended_at__like, ended_at__notlike=ended_at__notlike, ended_at__ilike=ended_at__ilike, ended_at__notilike=ended_at__notilike, ended_at__desc=ended_at__desc, ended_at__asc=ended_at__asc, duration__eq=duration__eq, duration__ne=duration__ne, duration__gt=duration__gt, duration__gte=duration__gte, duration__lt=duration__lt, duration__lte=duration__lte, duration__in=duration__in, duration__notin=duration__notin, duration__isnull=duration__isnull, duration__isnotnull=duration__isnotnull, duration__isfalse=duration__isfalse, duration__istrue=duration__istrue, duration__like=duration__like, duration__notlike=duration__notlike, duration__ilike=duration__ilike, duration__notilike=duration__notilike, duration__desc=duration__desc, duration__asc=duration__asc, file_size__eq=file_size__eq, file_size__ne=file_size__ne, file_size__gt=file_size__gt, file_size__gte=file_size__gte, file_size__lt=file_size__lt, file_size__lte=file_size__lte, file_size__in=file_size__in, file_size__notin=file_size__notin, file_size__isnull=file_size__isnull, file_size__isnotnull=file_size__isnotnull, file_size__isfalse=file_size__isfalse, file_size__istrue=file_size__istrue, file_size__like=file_size__like, file_size__notlike=file_size__notlike, file_size__ilike=file_size__ilike, file_size__notilike=file_size__notilike, file_size__desc=file_size__desc, file_size__asc=file_size__asc, thumbnail_name__eq=thumbnail_name__eq, thumbnail_name__ne=thumbnail_name__ne, thumbnail_name__gt=thumbnail_name__gt, thumbnail_name__gte=thumbnail_name__gte, thumbnail_name__lt=thumbnail_name__lt, thumbnail_name__lte=thumbnail_name__lte, thumbnail_name__in=thumbnail_name__in, thumbnail_name__notin=thumbnail_name__notin, thumbnail_name__isnull=thumbnail_name__isnull, thumbnail_name__isnotnull=thumbnail_name__isnotnull, thumbnail_name__isfalse=thumbnail_name__isfalse, thumbnail_name__istrue=thumbnail_name__istrue, thumbnail_name__like=thumbnail_name__like, thumbnail_name__notlike=thumbnail_name__notlike, thumbnail_name__ilike=thumbnail_name__ilike, thumbnail_name__notilike=thumbnail_name__notilike, thumbnail_name__desc=thumbnail_name__desc, thumbnail_name__asc=thumbnail_name__asc, status__eq=status__eq, status__ne=status__ne, status__gt=status__gt, status__gte=status__gte, status__lt=status__lt, status__lte=status__lte, status__in=status__in, status__notin=status__notin, status__isnull=status__isnull, status__isnotnull=status__isnotnull, status__isfalse=status__isfalse, status__istrue=status__istrue, status__like=status__like, status__notlike=status__notlike, status__ilike=status__ilike, status__notilike=status__notilike, status__desc=status__desc, status__asc=status__asc, object_detector_claimed_until__eq=object_detector_claimed_until__eq, object_detector_claimed_until__ne=object_detector_claimed_until__ne, object_detector_claimed_until__gt=object_detector_claimed_until__gt, object_detector_claimed_until__gte=object_detector_claimed_until__gte, object_detector_claimed_until__lt=object_detector_claimed_until__lt, object_detector_claimed_until__lte=object_detector_claimed_until__lte, object_detector_claimed_until__in=object_detector_claimed_until__in, object_detector_claimed_until__notin=object_detector_claimed_until__notin, object_detector_claimed_until__isnull=object_detector_claimed_until__isnull, object_detector_claimed_until__isnotnull=object_detector_claimed_until__isnotnull, object_detector_claimed_until__isfalse=object_detector_claimed_until__isfalse, object_detector_claimed_until__istrue=object_detector_claimed_until__istrue, object_detector_claimed_until__like=object_detector_claimed_until__like, object_detector_claimed_until__notlike=object_detector_claimed_until__notlike, object_detector_claimed_until__ilike=object_detector_claimed_until__ilike, object_detector_claimed_until__notilike=object_detector_claimed_until__notilike, object_detector_claimed_until__desc=object_detector_claimed_until__desc, object_detector_claimed_until__asc=object_detector_claimed_until__asc, object_tracker_claimed_until__eq=object_tracker_claimed_until__eq, object_tracker_claimed_until__ne=object_tracker_claimed_until__ne, object_tracker_claimed_until__gt=object_tracker_claimed_until__gt, object_tracker_claimed_until__gte=object_tracker_claimed_until__gte, object_tracker_claimed_until__lt=object_tracker_claimed_until__lt, object_tracker_claimed_until__lte=object_tracker_claimed_until__lte, object_tracker_claimed_until__in=object_tracker_claimed_until__in, object_tracker_claimed_until__notin=object_tracker_claimed_until__notin, object_tracker_claimed_until__isnull=object_tracker_claimed_until__isnull, object_tracker_claimed_until__isnotnull=object_tracker_claimed_until__isnotnull, object_tracker_claimed_until__isfalse=object_tracker_claimed_until__isfalse, object_tracker_claimed_until__istrue=object_tracker_claimed_until__istrue, object_tracker_claimed_until__like=object_tracker_claimed_until__like, object_tracker_claimed_until__notlike=object_tracker_claimed_until__notlike, object_tracker_claimed_until__ilike=object_tracker_claimed_until__ilike, object_tracker_claimed_until__notilike=object_tracker_claimed_until__notilike, object_tracker_claimed_until__desc=object_tracker_claimed_until__desc, object_tracker_claimed_until__asc=object_tracker_claimed_until__asc, camera_id__eq=camera_id__eq, camera_id__ne=camera_id__ne, camera_id__gt=camera_id__gt, camera_id__gte=camera_id__gte, camera_id__lt=camera_id__lt, camera_id__lte=camera_id__lte, camera_id__in=camera_id__in, camera_id__notin=camera_id__notin, camera_id__isnull=camera_id__isnull, camera_id__isnotnull=camera_id__isnotnull, camera_id__isfalse=camera_id__isfalse, camera_id__istrue=camera_id__istrue, camera_id__like=camera_id__like, camera_id__notlike=camera_id__notlike, camera_id__ilike=camera_id__ilike, camera_id__notilike=camera_id__notilike, camera_id__desc=camera_id__desc, camera_id__asc=camera_id__asc)
        print("The response of VideoApi->get_videos:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling VideoApi->get_videos: %s\n" % e)
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
 **file_name__eq** | **str**| SQL &#x3D; comparison | [optional] 
 **file_name__ne** | **str**| SQL !&#x3D; comparison | [optional] 
 **file_name__gt** | **str**| SQL &gt; comparison, may not work with all column types | [optional] 
 **file_name__gte** | **str**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **file_name__lt** | **str**| SQL &lt; comparison, may not work with all column types | [optional] 
 **file_name__lte** | **str**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **file_name__in** | **str**| SQL IN comparison, permits comma-separated values | [optional] 
 **file_name__notin** | **str**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **file_name__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **file_name__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **file_name__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **file_name__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **file_name__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **file_name__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **file_name__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **file_name__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **file_name__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **file_name__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **started_at__eq** | **datetime**| SQL &#x3D; comparison | [optional] 
 **started_at__ne** | **datetime**| SQL !&#x3D; comparison | [optional] 
 **started_at__gt** | **datetime**| SQL &gt; comparison, may not work with all column types | [optional] 
 **started_at__gte** | **datetime**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **started_at__lt** | **datetime**| SQL &lt; comparison, may not work with all column types | [optional] 
 **started_at__lte** | **datetime**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **started_at__in** | **datetime**| SQL IN comparison, permits comma-separated values | [optional] 
 **started_at__notin** | **datetime**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **started_at__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **started_at__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **started_at__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **started_at__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **started_at__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **started_at__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **started_at__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **started_at__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **started_at__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **started_at__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **ended_at__eq** | **datetime**| SQL &#x3D; comparison | [optional] 
 **ended_at__ne** | **datetime**| SQL !&#x3D; comparison | [optional] 
 **ended_at__gt** | **datetime**| SQL &gt; comparison, may not work with all column types | [optional] 
 **ended_at__gte** | **datetime**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **ended_at__lt** | **datetime**| SQL &lt; comparison, may not work with all column types | [optional] 
 **ended_at__lte** | **datetime**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **ended_at__in** | **datetime**| SQL IN comparison, permits comma-separated values | [optional] 
 **ended_at__notin** | **datetime**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **ended_at__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **ended_at__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **ended_at__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **ended_at__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **ended_at__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **ended_at__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **ended_at__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **ended_at__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **ended_at__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **ended_at__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **duration__eq** | **int**| SQL &#x3D; comparison | [optional] 
 **duration__ne** | **int**| SQL !&#x3D; comparison | [optional] 
 **duration__gt** | **int**| SQL &gt; comparison, may not work with all column types | [optional] 
 **duration__gte** | **int**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **duration__lt** | **int**| SQL &lt; comparison, may not work with all column types | [optional] 
 **duration__lte** | **int**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **duration__in** | **int**| SQL IN comparison, permits comma-separated values | [optional] 
 **duration__notin** | **int**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **duration__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **duration__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **duration__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **duration__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **duration__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **duration__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **duration__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **duration__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **duration__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **duration__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **file_size__eq** | **float**| SQL &#x3D; comparison | [optional] 
 **file_size__ne** | **float**| SQL !&#x3D; comparison | [optional] 
 **file_size__gt** | **float**| SQL &gt; comparison, may not work with all column types | [optional] 
 **file_size__gte** | **float**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **file_size__lt** | **float**| SQL &lt; comparison, may not work with all column types | [optional] 
 **file_size__lte** | **float**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **file_size__in** | **float**| SQL IN comparison, permits comma-separated values | [optional] 
 **file_size__notin** | **float**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **file_size__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **file_size__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **file_size__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **file_size__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **file_size__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **file_size__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **file_size__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **file_size__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **file_size__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **file_size__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **thumbnail_name__eq** | **str**| SQL &#x3D; comparison | [optional] 
 **thumbnail_name__ne** | **str**| SQL !&#x3D; comparison | [optional] 
 **thumbnail_name__gt** | **str**| SQL &gt; comparison, may not work with all column types | [optional] 
 **thumbnail_name__gte** | **str**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **thumbnail_name__lt** | **str**| SQL &lt; comparison, may not work with all column types | [optional] 
 **thumbnail_name__lte** | **str**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **thumbnail_name__in** | **str**| SQL IN comparison, permits comma-separated values | [optional] 
 **thumbnail_name__notin** | **str**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **thumbnail_name__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **thumbnail_name__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **thumbnail_name__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **thumbnail_name__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **thumbnail_name__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **thumbnail_name__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **thumbnail_name__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **thumbnail_name__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **thumbnail_name__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **thumbnail_name__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **status__eq** | **str**| SQL &#x3D; comparison | [optional] 
 **status__ne** | **str**| SQL !&#x3D; comparison | [optional] 
 **status__gt** | **str**| SQL &gt; comparison, may not work with all column types | [optional] 
 **status__gte** | **str**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **status__lt** | **str**| SQL &lt; comparison, may not work with all column types | [optional] 
 **status__lte** | **str**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **status__in** | **str**| SQL IN comparison, permits comma-separated values | [optional] 
 **status__notin** | **str**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **status__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **status__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **status__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **status__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **status__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **status__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **status__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **status__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **status__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **status__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **object_detector_claimed_until__eq** | **datetime**| SQL &#x3D; comparison | [optional] 
 **object_detector_claimed_until__ne** | **datetime**| SQL !&#x3D; comparison | [optional] 
 **object_detector_claimed_until__gt** | **datetime**| SQL &gt; comparison, may not work with all column types | [optional] 
 **object_detector_claimed_until__gte** | **datetime**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **object_detector_claimed_until__lt** | **datetime**| SQL &lt; comparison, may not work with all column types | [optional] 
 **object_detector_claimed_until__lte** | **datetime**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **object_detector_claimed_until__in** | **datetime**| SQL IN comparison, permits comma-separated values | [optional] 
 **object_detector_claimed_until__notin** | **datetime**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **object_detector_claimed_until__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **object_detector_claimed_until__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **object_detector_claimed_until__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **object_detector_claimed_until__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **object_detector_claimed_until__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **object_detector_claimed_until__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **object_detector_claimed_until__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **object_detector_claimed_until__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **object_detector_claimed_until__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **object_detector_claimed_until__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
 **object_tracker_claimed_until__eq** | **datetime**| SQL &#x3D; comparison | [optional] 
 **object_tracker_claimed_until__ne** | **datetime**| SQL !&#x3D; comparison | [optional] 
 **object_tracker_claimed_until__gt** | **datetime**| SQL &gt; comparison, may not work with all column types | [optional] 
 **object_tracker_claimed_until__gte** | **datetime**| SQL &gt;&#x3D; comparison, may not work with all column types | [optional] 
 **object_tracker_claimed_until__lt** | **datetime**| SQL &lt; comparison, may not work with all column types | [optional] 
 **object_tracker_claimed_until__lte** | **datetime**| SQL &lt;&#x3D; comparison, may not work with all column types | [optional] 
 **object_tracker_claimed_until__in** | **datetime**| SQL IN comparison, permits comma-separated values | [optional] 
 **object_tracker_claimed_until__notin** | **datetime**| SQL NOT IN comparison, permits comma-separated values | [optional] 
 **object_tracker_claimed_until__isnull** | **str**| SQL IS null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **object_tracker_claimed_until__isnotnull** | **str**| SQL IS NOT null comparison, value is ignored (presence of key is sufficient) | [optional] 
 **object_tracker_claimed_until__isfalse** | **str**| SQL IS false comparison, value is ignored (presence of key is sufficient) | [optional] 
 **object_tracker_claimed_until__istrue** | **str**| SQL IS true comparison, value is ignored (presence of key is sufficient) | [optional] 
 **object_tracker_claimed_until__like** | **str**| SQL LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **object_tracker_claimed_until__notlike** | **str**| SQL NOT LIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **object_tracker_claimed_until__ilike** | **str**| SQL ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **object_tracker_claimed_until__notilike** | **str**| SQL NOT ILIKE comparison, value is implicitly prefixed and suffixed with % | [optional] 
 **object_tracker_claimed_until__desc** | **str**| SQL ORDER BY _ DESC clause, value is ignored (presence of key is sufficient) | [optional] 
 **object_tracker_claimed_until__asc** | **str**| SQL ORDER BY _ ASC clause, value is ignored (presence of key is sufficient) | [optional] 
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

[**GetVideos200Response**](GetVideos200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful List Fetch for Videos |  -  |
**0** | Failed List Fetch for Videos |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_video**
> GetVideos200Response patch_video(primary_key, video, depth=depth)



### Example


```python
import openapi_client
from openapi_client.models.get_videos200_response import GetVideos200Response
from openapi_client.models.video import Video
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
    api_instance = openapi_client.VideoApi(api_client)
    primary_key = None # object | Primary key for Video
    video = openapi_client.Video() # Video | 
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_response = api_instance.patch_video(primary_key, video, depth=depth)
        print("The response of VideoApi->patch_video:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling VideoApi->patch_video: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **primary_key** | [**object**](.md)| Primary key for Video | 
 **video** | [**Video**](Video.md)|  | 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 

### Return type

[**GetVideos200Response**](GetVideos200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful Item Update for Videos |  -  |
**0** | Failed Item Update for Videos |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_videos**
> GetVideos200Response post_videos(video, depth=depth)



### Example


```python
import openapi_client
from openapi_client.models.get_videos200_response import GetVideos200Response
from openapi_client.models.video import Video
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
    api_instance = openapi_client.VideoApi(api_client)
    video = [openapi_client.Video()] # List[Video] | 
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_response = api_instance.post_videos(video, depth=depth)
        print("The response of VideoApi->post_videos:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling VideoApi->post_videos: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **video** | [**List[Video]**](Video.md)|  | 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 

### Return type

[**GetVideos200Response**](GetVideos200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful List Create for Videos |  -  |
**0** | Failed List Create for Videos |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_video**
> GetVideos200Response put_video(primary_key, video, depth=depth)



### Example


```python
import openapi_client
from openapi_client.models.get_videos200_response import GetVideos200Response
from openapi_client.models.video import Video
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
    api_instance = openapi_client.VideoApi(api_client)
    primary_key = None # object | Primary key for Video
    video = openapi_client.Video() # Video | 
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_response = api_instance.put_video(primary_key, video, depth=depth)
        print("The response of VideoApi->put_video:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling VideoApi->put_video: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **primary_key** | [**object**](.md)| Primary key for Video | 
 **video** | [**Video**](Video.md)|  | 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 

### Return type

[**GetVideos200Response**](GetVideos200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful Item Replace for Videos |  -  |
**0** | Failed Item Replace for Videos |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

