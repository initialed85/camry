# openapi_client.CameraApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_camera**](CameraApi.md#delete_camera) | **DELETE** /api/cameras/{primaryKey} | 
[**get_camera**](CameraApi.md#get_camera) | **GET** /api/cameras/{primaryKey} | 
[**get_cameras**](CameraApi.md#get_cameras) | **GET** /api/cameras | 
[**patch_camera**](CameraApi.md#patch_camera) | **PATCH** /api/cameras/{primaryKey} | 
[**post_cameras**](CameraApi.md#post_cameras) | **POST** /api/cameras | 
[**put_camera**](CameraApi.md#put_camera) | **PUT** /api/cameras/{primaryKey} | 


# **delete_camera**
> delete_camera(primary_key, depth=depth)



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
    api_instance = openapi_client.CameraApi(api_client)
    primary_key = None # object | Primary key for Camera
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_instance.delete_camera(primary_key, depth=depth)
    except Exception as e:
        print("Exception when calling CameraApi->delete_camera: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **primary_key** | [**object**](.md)| Primary key for Camera | 
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
**204** | Successful Item Delete for Cameras |  -  |
**0** | Failed Item Delete for Cameras |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_camera**
> GetCameras200Response get_camera(primary_key, depth=depth)



### Example


```python
import openapi_client
from openapi_client.models.get_cameras200_response import GetCameras200Response
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
    api_instance = openapi_client.CameraApi(api_client)
    primary_key = None # object | Primary key for Camera
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_response = api_instance.get_camera(primary_key, depth=depth)
        print("The response of CameraApi->get_camera:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling CameraApi->get_camera: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **primary_key** | [**object**](.md)| Primary key for Camera | 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 

### Return type

[**GetCameras200Response**](GetCameras200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful Item Fetch for Cameras |  -  |
**0** | Failed Item Fetch for Cameras |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cameras**
> GetCameras200Response get_cameras(limit=limit, offset=offset, depth=depth, id__eq=id__eq, id__ne=id__ne, id__gt=id__gt, id__gte=id__gte, id__lt=id__lt, id__lte=id__lte, id__in=id__in, id__nin=id__nin, id__notin=id__notin, id__isnull=id__isnull, id__nisnull=id__nisnull, id__isnotnull=id__isnotnull, id__l=id__l, id__like=id__like, id__nl=id__nl, id__nlike=id__nlike, id__notlike=id__notlike, id__il=id__il, id__ilike=id__ilike, id__nil=id__nil, id__nilike=id__nilike, id__notilike=id__notilike, id__desc=id__desc, id__asc=id__asc, created_at__eq=created_at__eq, created_at__ne=created_at__ne, created_at__gt=created_at__gt, created_at__gte=created_at__gte, created_at__lt=created_at__lt, created_at__lte=created_at__lte, created_at__in=created_at__in, created_at__nin=created_at__nin, created_at__notin=created_at__notin, created_at__isnull=created_at__isnull, created_at__nisnull=created_at__nisnull, created_at__isnotnull=created_at__isnotnull, created_at__l=created_at__l, created_at__like=created_at__like, created_at__nl=created_at__nl, created_at__nlike=created_at__nlike, created_at__notlike=created_at__notlike, created_at__il=created_at__il, created_at__ilike=created_at__ilike, created_at__nil=created_at__nil, created_at__nilike=created_at__nilike, created_at__notilike=created_at__notilike, created_at__desc=created_at__desc, created_at__asc=created_at__asc, updated_at__eq=updated_at__eq, updated_at__ne=updated_at__ne, updated_at__gt=updated_at__gt, updated_at__gte=updated_at__gte, updated_at__lt=updated_at__lt, updated_at__lte=updated_at__lte, updated_at__in=updated_at__in, updated_at__nin=updated_at__nin, updated_at__notin=updated_at__notin, updated_at__isnull=updated_at__isnull, updated_at__nisnull=updated_at__nisnull, updated_at__isnotnull=updated_at__isnotnull, updated_at__l=updated_at__l, updated_at__like=updated_at__like, updated_at__nl=updated_at__nl, updated_at__nlike=updated_at__nlike, updated_at__notlike=updated_at__notlike, updated_at__il=updated_at__il, updated_at__ilike=updated_at__ilike, updated_at__nil=updated_at__nil, updated_at__nilike=updated_at__nilike, updated_at__notilike=updated_at__notilike, updated_at__desc=updated_at__desc, updated_at__asc=updated_at__asc, deleted_at__eq=deleted_at__eq, deleted_at__ne=deleted_at__ne, deleted_at__gt=deleted_at__gt, deleted_at__gte=deleted_at__gte, deleted_at__lt=deleted_at__lt, deleted_at__lte=deleted_at__lte, deleted_at__in=deleted_at__in, deleted_at__nin=deleted_at__nin, deleted_at__notin=deleted_at__notin, deleted_at__isnull=deleted_at__isnull, deleted_at__nisnull=deleted_at__nisnull, deleted_at__isnotnull=deleted_at__isnotnull, deleted_at__l=deleted_at__l, deleted_at__like=deleted_at__like, deleted_at__nl=deleted_at__nl, deleted_at__nlike=deleted_at__nlike, deleted_at__notlike=deleted_at__notlike, deleted_at__il=deleted_at__il, deleted_at__ilike=deleted_at__ilike, deleted_at__nil=deleted_at__nil, deleted_at__nilike=deleted_at__nilike, deleted_at__notilike=deleted_at__notilike, deleted_at__desc=deleted_at__desc, deleted_at__asc=deleted_at__asc, name__eq=name__eq, name__ne=name__ne, name__gt=name__gt, name__gte=name__gte, name__lt=name__lt, name__lte=name__lte, name__in=name__in, name__nin=name__nin, name__notin=name__notin, name__isnull=name__isnull, name__nisnull=name__nisnull, name__isnotnull=name__isnotnull, name__l=name__l, name__like=name__like, name__nl=name__nl, name__nlike=name__nlike, name__notlike=name__notlike, name__il=name__il, name__ilike=name__ilike, name__nil=name__nil, name__nilike=name__nilike, name__notilike=name__notilike, name__desc=name__desc, name__asc=name__asc, stream_url__eq=stream_url__eq, stream_url__ne=stream_url__ne, stream_url__gt=stream_url__gt, stream_url__gte=stream_url__gte, stream_url__lt=stream_url__lt, stream_url__lte=stream_url__lte, stream_url__in=stream_url__in, stream_url__nin=stream_url__nin, stream_url__notin=stream_url__notin, stream_url__isnull=stream_url__isnull, stream_url__nisnull=stream_url__nisnull, stream_url__isnotnull=stream_url__isnotnull, stream_url__l=stream_url__l, stream_url__like=stream_url__like, stream_url__nl=stream_url__nl, stream_url__nlike=stream_url__nlike, stream_url__notlike=stream_url__notlike, stream_url__il=stream_url__il, stream_url__ilike=stream_url__ilike, stream_url__nil=stream_url__nil, stream_url__nilike=stream_url__nilike, stream_url__notilike=stream_url__notilike, stream_url__desc=stream_url__desc, stream_url__asc=stream_url__asc, last_seen__eq=last_seen__eq, last_seen__ne=last_seen__ne, last_seen__gt=last_seen__gt, last_seen__gte=last_seen__gte, last_seen__lt=last_seen__lt, last_seen__lte=last_seen__lte, last_seen__in=last_seen__in, last_seen__nin=last_seen__nin, last_seen__notin=last_seen__notin, last_seen__isnull=last_seen__isnull, last_seen__nisnull=last_seen__nisnull, last_seen__isnotnull=last_seen__isnotnull, last_seen__l=last_seen__l, last_seen__like=last_seen__like, last_seen__nl=last_seen__nl, last_seen__nlike=last_seen__nlike, last_seen__notlike=last_seen__notlike, last_seen__il=last_seen__il, last_seen__ilike=last_seen__ilike, last_seen__nil=last_seen__nil, last_seen__nilike=last_seen__nilike, last_seen__notilike=last_seen__notilike, last_seen__desc=last_seen__desc, last_seen__asc=last_seen__asc, segment_producer_claimed_until__eq=segment_producer_claimed_until__eq, segment_producer_claimed_until__ne=segment_producer_claimed_until__ne, segment_producer_claimed_until__gt=segment_producer_claimed_until__gt, segment_producer_claimed_until__gte=segment_producer_claimed_until__gte, segment_producer_claimed_until__lt=segment_producer_claimed_until__lt, segment_producer_claimed_until__lte=segment_producer_claimed_until__lte, segment_producer_claimed_until__in=segment_producer_claimed_until__in, segment_producer_claimed_until__nin=segment_producer_claimed_until__nin, segment_producer_claimed_until__notin=segment_producer_claimed_until__notin, segment_producer_claimed_until__isnull=segment_producer_claimed_until__isnull, segment_producer_claimed_until__nisnull=segment_producer_claimed_until__nisnull, segment_producer_claimed_until__isnotnull=segment_producer_claimed_until__isnotnull, segment_producer_claimed_until__l=segment_producer_claimed_until__l, segment_producer_claimed_until__like=segment_producer_claimed_until__like, segment_producer_claimed_until__nl=segment_producer_claimed_until__nl, segment_producer_claimed_until__nlike=segment_producer_claimed_until__nlike, segment_producer_claimed_until__notlike=segment_producer_claimed_until__notlike, segment_producer_claimed_until__il=segment_producer_claimed_until__il, segment_producer_claimed_until__ilike=segment_producer_claimed_until__ilike, segment_producer_claimed_until__nil=segment_producer_claimed_until__nil, segment_producer_claimed_until__nilike=segment_producer_claimed_until__nilike, segment_producer_claimed_until__notilike=segment_producer_claimed_until__notilike, segment_producer_claimed_until__desc=segment_producer_claimed_until__desc, segment_producer_claimed_until__asc=segment_producer_claimed_until__asc, stream_producer_claimed_until__eq=stream_producer_claimed_until__eq, stream_producer_claimed_until__ne=stream_producer_claimed_until__ne, stream_producer_claimed_until__gt=stream_producer_claimed_until__gt, stream_producer_claimed_until__gte=stream_producer_claimed_until__gte, stream_producer_claimed_until__lt=stream_producer_claimed_until__lt, stream_producer_claimed_until__lte=stream_producer_claimed_until__lte, stream_producer_claimed_until__in=stream_producer_claimed_until__in, stream_producer_claimed_until__nin=stream_producer_claimed_until__nin, stream_producer_claimed_until__notin=stream_producer_claimed_until__notin, stream_producer_claimed_until__isnull=stream_producer_claimed_until__isnull, stream_producer_claimed_until__nisnull=stream_producer_claimed_until__nisnull, stream_producer_claimed_until__isnotnull=stream_producer_claimed_until__isnotnull, stream_producer_claimed_until__l=stream_producer_claimed_until__l, stream_producer_claimed_until__like=stream_producer_claimed_until__like, stream_producer_claimed_until__nl=stream_producer_claimed_until__nl, stream_producer_claimed_until__nlike=stream_producer_claimed_until__nlike, stream_producer_claimed_until__notlike=stream_producer_claimed_until__notlike, stream_producer_claimed_until__il=stream_producer_claimed_until__il, stream_producer_claimed_until__ilike=stream_producer_claimed_until__ilike, stream_producer_claimed_until__nil=stream_producer_claimed_until__nil, stream_producer_claimed_until__nilike=stream_producer_claimed_until__nilike, stream_producer_claimed_until__notilike=stream_producer_claimed_until__notilike, stream_producer_claimed_until__desc=stream_producer_claimed_until__desc, stream_producer_claimed_until__asc=stream_producer_claimed_until__asc)



### Example


```python
import openapi_client
from openapi_client.models.get_cameras200_response import GetCameras200Response
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
    api_instance = openapi_client.CameraApi(api_client)
    limit = 56 # int | SQL LIMIT operator (optional)
    offset = 56 # int | SQL OFFSET operator (optional)
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)
    id__eq = 'id__eq_example' # str | SQL = operator (optional)
    id__ne = 'id__ne_example' # str | SQL != operator (optional)
    id__gt = 'id__gt_example' # str | SQL > operator, may not work with all column types (optional)
    id__gte = 'id__gte_example' # str | SQL >= operator, may not work with all column types (optional)
    id__lt = 'id__lt_example' # str | SQL < operator, may not work with all column types (optional)
    id__lte = 'id__lte_example' # str | SQL <= operator, may not work with all column types (optional)
    id__in = 'id__in_example' # str | SQL IN operator, permits comma-separated values (optional)
    id__nin = 'id__nin_example' # str | SQL NOT IN operator, permits comma-separated values (optional)
    id__notin = 'id__notin_example' # str | SQL NOT IN operator, permits comma-separated values (optional)
    id__isnull = 'id__isnull_example' # str | SQL IS NULL operator, value is ignored (presence of key is sufficient) (optional)
    id__nisnull = 'id__nisnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    id__isnotnull = 'id__isnotnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    id__l = 'id__l_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    id__like = 'id__like_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    id__nl = 'id__nl_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    id__nlike = 'id__nlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    id__notlike = 'id__notlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    id__il = 'id__il_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    id__ilike = 'id__ilike_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    id__nil = 'id__nil_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    id__nilike = 'id__nilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    id__notilike = 'id__notilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    id__desc = 'id__desc_example' # str | SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) (optional)
    id__asc = 'id__asc_example' # str | SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) (optional)
    created_at__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = operator (optional)
    created_at__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != operator (optional)
    created_at__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > operator, may not work with all column types (optional)
    created_at__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= operator, may not work with all column types (optional)
    created_at__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < operator, may not work with all column types (optional)
    created_at__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= operator, may not work with all column types (optional)
    created_at__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN operator, permits comma-separated values (optional)
    created_at__nin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN operator, permits comma-separated values (optional)
    created_at__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN operator, permits comma-separated values (optional)
    created_at__isnull = 'created_at__isnull_example' # str | SQL IS NULL operator, value is ignored (presence of key is sufficient) (optional)
    created_at__nisnull = 'created_at__nisnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    created_at__isnotnull = 'created_at__isnotnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    created_at__l = 'created_at__l_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    created_at__like = 'created_at__like_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    created_at__nl = 'created_at__nl_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    created_at__nlike = 'created_at__nlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    created_at__notlike = 'created_at__notlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    created_at__il = 'created_at__il_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    created_at__ilike = 'created_at__ilike_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    created_at__nil = 'created_at__nil_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    created_at__nilike = 'created_at__nilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    created_at__notilike = 'created_at__notilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    created_at__desc = 'created_at__desc_example' # str | SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) (optional)
    created_at__asc = 'created_at__asc_example' # str | SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) (optional)
    updated_at__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = operator (optional)
    updated_at__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != operator (optional)
    updated_at__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > operator, may not work with all column types (optional)
    updated_at__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= operator, may not work with all column types (optional)
    updated_at__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < operator, may not work with all column types (optional)
    updated_at__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= operator, may not work with all column types (optional)
    updated_at__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN operator, permits comma-separated values (optional)
    updated_at__nin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN operator, permits comma-separated values (optional)
    updated_at__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN operator, permits comma-separated values (optional)
    updated_at__isnull = 'updated_at__isnull_example' # str | SQL IS NULL operator, value is ignored (presence of key is sufficient) (optional)
    updated_at__nisnull = 'updated_at__nisnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    updated_at__isnotnull = 'updated_at__isnotnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    updated_at__l = 'updated_at__l_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    updated_at__like = 'updated_at__like_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    updated_at__nl = 'updated_at__nl_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    updated_at__nlike = 'updated_at__nlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    updated_at__notlike = 'updated_at__notlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    updated_at__il = 'updated_at__il_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    updated_at__ilike = 'updated_at__ilike_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    updated_at__nil = 'updated_at__nil_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    updated_at__nilike = 'updated_at__nilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    updated_at__notilike = 'updated_at__notilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    updated_at__desc = 'updated_at__desc_example' # str | SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) (optional)
    updated_at__asc = 'updated_at__asc_example' # str | SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) (optional)
    deleted_at__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = operator (optional)
    deleted_at__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != operator (optional)
    deleted_at__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > operator, may not work with all column types (optional)
    deleted_at__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= operator, may not work with all column types (optional)
    deleted_at__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < operator, may not work with all column types (optional)
    deleted_at__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= operator, may not work with all column types (optional)
    deleted_at__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN operator, permits comma-separated values (optional)
    deleted_at__nin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN operator, permits comma-separated values (optional)
    deleted_at__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN operator, permits comma-separated values (optional)
    deleted_at__isnull = 'deleted_at__isnull_example' # str | SQL IS NULL operator, value is ignored (presence of key is sufficient) (optional)
    deleted_at__nisnull = 'deleted_at__nisnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    deleted_at__isnotnull = 'deleted_at__isnotnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    deleted_at__l = 'deleted_at__l_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__like = 'deleted_at__like_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__nl = 'deleted_at__nl_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__nlike = 'deleted_at__nlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__notlike = 'deleted_at__notlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__il = 'deleted_at__il_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__ilike = 'deleted_at__ilike_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__nil = 'deleted_at__nil_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__nilike = 'deleted_at__nilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__notilike = 'deleted_at__notilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    deleted_at__desc = 'deleted_at__desc_example' # str | SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) (optional)
    deleted_at__asc = 'deleted_at__asc_example' # str | SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) (optional)
    name__eq = 'name__eq_example' # str | SQL = operator (optional)
    name__ne = 'name__ne_example' # str | SQL != operator (optional)
    name__gt = 'name__gt_example' # str | SQL > operator, may not work with all column types (optional)
    name__gte = 'name__gte_example' # str | SQL >= operator, may not work with all column types (optional)
    name__lt = 'name__lt_example' # str | SQL < operator, may not work with all column types (optional)
    name__lte = 'name__lte_example' # str | SQL <= operator, may not work with all column types (optional)
    name__in = 'name__in_example' # str | SQL IN operator, permits comma-separated values (optional)
    name__nin = 'name__nin_example' # str | SQL NOT IN operator, permits comma-separated values (optional)
    name__notin = 'name__notin_example' # str | SQL NOT IN operator, permits comma-separated values (optional)
    name__isnull = 'name__isnull_example' # str | SQL IS NULL operator, value is ignored (presence of key is sufficient) (optional)
    name__nisnull = 'name__nisnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    name__isnotnull = 'name__isnotnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    name__l = 'name__l_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    name__like = 'name__like_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    name__nl = 'name__nl_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    name__nlike = 'name__nlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    name__notlike = 'name__notlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    name__il = 'name__il_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    name__ilike = 'name__ilike_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    name__nil = 'name__nil_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    name__nilike = 'name__nilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    name__notilike = 'name__notilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    name__desc = 'name__desc_example' # str | SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) (optional)
    name__asc = 'name__asc_example' # str | SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) (optional)
    stream_url__eq = 'stream_url__eq_example' # str | SQL = operator (optional)
    stream_url__ne = 'stream_url__ne_example' # str | SQL != operator (optional)
    stream_url__gt = 'stream_url__gt_example' # str | SQL > operator, may not work with all column types (optional)
    stream_url__gte = 'stream_url__gte_example' # str | SQL >= operator, may not work with all column types (optional)
    stream_url__lt = 'stream_url__lt_example' # str | SQL < operator, may not work with all column types (optional)
    stream_url__lte = 'stream_url__lte_example' # str | SQL <= operator, may not work with all column types (optional)
    stream_url__in = 'stream_url__in_example' # str | SQL IN operator, permits comma-separated values (optional)
    stream_url__nin = 'stream_url__nin_example' # str | SQL NOT IN operator, permits comma-separated values (optional)
    stream_url__notin = 'stream_url__notin_example' # str | SQL NOT IN operator, permits comma-separated values (optional)
    stream_url__isnull = 'stream_url__isnull_example' # str | SQL IS NULL operator, value is ignored (presence of key is sufficient) (optional)
    stream_url__nisnull = 'stream_url__nisnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    stream_url__isnotnull = 'stream_url__isnotnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    stream_url__l = 'stream_url__l_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_url__like = 'stream_url__like_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_url__nl = 'stream_url__nl_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_url__nlike = 'stream_url__nlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_url__notlike = 'stream_url__notlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_url__il = 'stream_url__il_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_url__ilike = 'stream_url__ilike_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_url__nil = 'stream_url__nil_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_url__nilike = 'stream_url__nilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_url__notilike = 'stream_url__notilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_url__desc = 'stream_url__desc_example' # str | SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) (optional)
    stream_url__asc = 'stream_url__asc_example' # str | SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) (optional)
    last_seen__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = operator (optional)
    last_seen__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != operator (optional)
    last_seen__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > operator, may not work with all column types (optional)
    last_seen__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= operator, may not work with all column types (optional)
    last_seen__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < operator, may not work with all column types (optional)
    last_seen__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= operator, may not work with all column types (optional)
    last_seen__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN operator, permits comma-separated values (optional)
    last_seen__nin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN operator, permits comma-separated values (optional)
    last_seen__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN operator, permits comma-separated values (optional)
    last_seen__isnull = 'last_seen__isnull_example' # str | SQL IS NULL operator, value is ignored (presence of key is sufficient) (optional)
    last_seen__nisnull = 'last_seen__nisnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    last_seen__isnotnull = 'last_seen__isnotnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    last_seen__l = 'last_seen__l_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    last_seen__like = 'last_seen__like_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    last_seen__nl = 'last_seen__nl_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    last_seen__nlike = 'last_seen__nlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    last_seen__notlike = 'last_seen__notlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    last_seen__il = 'last_seen__il_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    last_seen__ilike = 'last_seen__ilike_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    last_seen__nil = 'last_seen__nil_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    last_seen__nilike = 'last_seen__nilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    last_seen__notilike = 'last_seen__notilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    last_seen__desc = 'last_seen__desc_example' # str | SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) (optional)
    last_seen__asc = 'last_seen__asc_example' # str | SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) (optional)
    segment_producer_claimed_until__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = operator (optional)
    segment_producer_claimed_until__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != operator (optional)
    segment_producer_claimed_until__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > operator, may not work with all column types (optional)
    segment_producer_claimed_until__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= operator, may not work with all column types (optional)
    segment_producer_claimed_until__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < operator, may not work with all column types (optional)
    segment_producer_claimed_until__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= operator, may not work with all column types (optional)
    segment_producer_claimed_until__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN operator, permits comma-separated values (optional)
    segment_producer_claimed_until__nin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN operator, permits comma-separated values (optional)
    segment_producer_claimed_until__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN operator, permits comma-separated values (optional)
    segment_producer_claimed_until__isnull = 'segment_producer_claimed_until__isnull_example' # str | SQL IS NULL operator, value is ignored (presence of key is sufficient) (optional)
    segment_producer_claimed_until__nisnull = 'segment_producer_claimed_until__nisnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    segment_producer_claimed_until__isnotnull = 'segment_producer_claimed_until__isnotnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    segment_producer_claimed_until__l = 'segment_producer_claimed_until__l_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    segment_producer_claimed_until__like = 'segment_producer_claimed_until__like_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    segment_producer_claimed_until__nl = 'segment_producer_claimed_until__nl_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    segment_producer_claimed_until__nlike = 'segment_producer_claimed_until__nlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    segment_producer_claimed_until__notlike = 'segment_producer_claimed_until__notlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    segment_producer_claimed_until__il = 'segment_producer_claimed_until__il_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    segment_producer_claimed_until__ilike = 'segment_producer_claimed_until__ilike_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    segment_producer_claimed_until__nil = 'segment_producer_claimed_until__nil_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    segment_producer_claimed_until__nilike = 'segment_producer_claimed_until__nilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    segment_producer_claimed_until__notilike = 'segment_producer_claimed_until__notilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    segment_producer_claimed_until__desc = 'segment_producer_claimed_until__desc_example' # str | SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) (optional)
    segment_producer_claimed_until__asc = 'segment_producer_claimed_until__asc_example' # str | SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) (optional)
    stream_producer_claimed_until__eq = '2013-10-20T19:20:30+01:00' # datetime | SQL = operator (optional)
    stream_producer_claimed_until__ne = '2013-10-20T19:20:30+01:00' # datetime | SQL != operator (optional)
    stream_producer_claimed_until__gt = '2013-10-20T19:20:30+01:00' # datetime | SQL > operator, may not work with all column types (optional)
    stream_producer_claimed_until__gte = '2013-10-20T19:20:30+01:00' # datetime | SQL >= operator, may not work with all column types (optional)
    stream_producer_claimed_until__lt = '2013-10-20T19:20:30+01:00' # datetime | SQL < operator, may not work with all column types (optional)
    stream_producer_claimed_until__lte = '2013-10-20T19:20:30+01:00' # datetime | SQL <= operator, may not work with all column types (optional)
    stream_producer_claimed_until__in = '2013-10-20T19:20:30+01:00' # datetime | SQL IN operator, permits comma-separated values (optional)
    stream_producer_claimed_until__nin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN operator, permits comma-separated values (optional)
    stream_producer_claimed_until__notin = '2013-10-20T19:20:30+01:00' # datetime | SQL NOT IN operator, permits comma-separated values (optional)
    stream_producer_claimed_until__isnull = 'stream_producer_claimed_until__isnull_example' # str | SQL IS NULL operator, value is ignored (presence of key is sufficient) (optional)
    stream_producer_claimed_until__nisnull = 'stream_producer_claimed_until__nisnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    stream_producer_claimed_until__isnotnull = 'stream_producer_claimed_until__isnotnull_example' # str | SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) (optional)
    stream_producer_claimed_until__l = 'stream_producer_claimed_until__l_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_producer_claimed_until__like = 'stream_producer_claimed_until__like_example' # str | SQL LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_producer_claimed_until__nl = 'stream_producer_claimed_until__nl_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_producer_claimed_until__nlike = 'stream_producer_claimed_until__nlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_producer_claimed_until__notlike = 'stream_producer_claimed_until__notlike_example' # str | SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_producer_claimed_until__il = 'stream_producer_claimed_until__il_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_producer_claimed_until__ilike = 'stream_producer_claimed_until__ilike_example' # str | SQL ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_producer_claimed_until__nil = 'stream_producer_claimed_until__nil_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_producer_claimed_until__nilike = 'stream_producer_claimed_until__nilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_producer_claimed_until__notilike = 'stream_producer_claimed_until__notilike_example' # str | SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % (optional)
    stream_producer_claimed_until__desc = 'stream_producer_claimed_until__desc_example' # str | SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) (optional)
    stream_producer_claimed_until__asc = 'stream_producer_claimed_until__asc_example' # str | SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) (optional)

    try:
        api_response = api_instance.get_cameras(limit=limit, offset=offset, depth=depth, id__eq=id__eq, id__ne=id__ne, id__gt=id__gt, id__gte=id__gte, id__lt=id__lt, id__lte=id__lte, id__in=id__in, id__nin=id__nin, id__notin=id__notin, id__isnull=id__isnull, id__nisnull=id__nisnull, id__isnotnull=id__isnotnull, id__l=id__l, id__like=id__like, id__nl=id__nl, id__nlike=id__nlike, id__notlike=id__notlike, id__il=id__il, id__ilike=id__ilike, id__nil=id__nil, id__nilike=id__nilike, id__notilike=id__notilike, id__desc=id__desc, id__asc=id__asc, created_at__eq=created_at__eq, created_at__ne=created_at__ne, created_at__gt=created_at__gt, created_at__gte=created_at__gte, created_at__lt=created_at__lt, created_at__lte=created_at__lte, created_at__in=created_at__in, created_at__nin=created_at__nin, created_at__notin=created_at__notin, created_at__isnull=created_at__isnull, created_at__nisnull=created_at__nisnull, created_at__isnotnull=created_at__isnotnull, created_at__l=created_at__l, created_at__like=created_at__like, created_at__nl=created_at__nl, created_at__nlike=created_at__nlike, created_at__notlike=created_at__notlike, created_at__il=created_at__il, created_at__ilike=created_at__ilike, created_at__nil=created_at__nil, created_at__nilike=created_at__nilike, created_at__notilike=created_at__notilike, created_at__desc=created_at__desc, created_at__asc=created_at__asc, updated_at__eq=updated_at__eq, updated_at__ne=updated_at__ne, updated_at__gt=updated_at__gt, updated_at__gte=updated_at__gte, updated_at__lt=updated_at__lt, updated_at__lte=updated_at__lte, updated_at__in=updated_at__in, updated_at__nin=updated_at__nin, updated_at__notin=updated_at__notin, updated_at__isnull=updated_at__isnull, updated_at__nisnull=updated_at__nisnull, updated_at__isnotnull=updated_at__isnotnull, updated_at__l=updated_at__l, updated_at__like=updated_at__like, updated_at__nl=updated_at__nl, updated_at__nlike=updated_at__nlike, updated_at__notlike=updated_at__notlike, updated_at__il=updated_at__il, updated_at__ilike=updated_at__ilike, updated_at__nil=updated_at__nil, updated_at__nilike=updated_at__nilike, updated_at__notilike=updated_at__notilike, updated_at__desc=updated_at__desc, updated_at__asc=updated_at__asc, deleted_at__eq=deleted_at__eq, deleted_at__ne=deleted_at__ne, deleted_at__gt=deleted_at__gt, deleted_at__gte=deleted_at__gte, deleted_at__lt=deleted_at__lt, deleted_at__lte=deleted_at__lte, deleted_at__in=deleted_at__in, deleted_at__nin=deleted_at__nin, deleted_at__notin=deleted_at__notin, deleted_at__isnull=deleted_at__isnull, deleted_at__nisnull=deleted_at__nisnull, deleted_at__isnotnull=deleted_at__isnotnull, deleted_at__l=deleted_at__l, deleted_at__like=deleted_at__like, deleted_at__nl=deleted_at__nl, deleted_at__nlike=deleted_at__nlike, deleted_at__notlike=deleted_at__notlike, deleted_at__il=deleted_at__il, deleted_at__ilike=deleted_at__ilike, deleted_at__nil=deleted_at__nil, deleted_at__nilike=deleted_at__nilike, deleted_at__notilike=deleted_at__notilike, deleted_at__desc=deleted_at__desc, deleted_at__asc=deleted_at__asc, name__eq=name__eq, name__ne=name__ne, name__gt=name__gt, name__gte=name__gte, name__lt=name__lt, name__lte=name__lte, name__in=name__in, name__nin=name__nin, name__notin=name__notin, name__isnull=name__isnull, name__nisnull=name__nisnull, name__isnotnull=name__isnotnull, name__l=name__l, name__like=name__like, name__nl=name__nl, name__nlike=name__nlike, name__notlike=name__notlike, name__il=name__il, name__ilike=name__ilike, name__nil=name__nil, name__nilike=name__nilike, name__notilike=name__notilike, name__desc=name__desc, name__asc=name__asc, stream_url__eq=stream_url__eq, stream_url__ne=stream_url__ne, stream_url__gt=stream_url__gt, stream_url__gte=stream_url__gte, stream_url__lt=stream_url__lt, stream_url__lte=stream_url__lte, stream_url__in=stream_url__in, stream_url__nin=stream_url__nin, stream_url__notin=stream_url__notin, stream_url__isnull=stream_url__isnull, stream_url__nisnull=stream_url__nisnull, stream_url__isnotnull=stream_url__isnotnull, stream_url__l=stream_url__l, stream_url__like=stream_url__like, stream_url__nl=stream_url__nl, stream_url__nlike=stream_url__nlike, stream_url__notlike=stream_url__notlike, stream_url__il=stream_url__il, stream_url__ilike=stream_url__ilike, stream_url__nil=stream_url__nil, stream_url__nilike=stream_url__nilike, stream_url__notilike=stream_url__notilike, stream_url__desc=stream_url__desc, stream_url__asc=stream_url__asc, last_seen__eq=last_seen__eq, last_seen__ne=last_seen__ne, last_seen__gt=last_seen__gt, last_seen__gte=last_seen__gte, last_seen__lt=last_seen__lt, last_seen__lte=last_seen__lte, last_seen__in=last_seen__in, last_seen__nin=last_seen__nin, last_seen__notin=last_seen__notin, last_seen__isnull=last_seen__isnull, last_seen__nisnull=last_seen__nisnull, last_seen__isnotnull=last_seen__isnotnull, last_seen__l=last_seen__l, last_seen__like=last_seen__like, last_seen__nl=last_seen__nl, last_seen__nlike=last_seen__nlike, last_seen__notlike=last_seen__notlike, last_seen__il=last_seen__il, last_seen__ilike=last_seen__ilike, last_seen__nil=last_seen__nil, last_seen__nilike=last_seen__nilike, last_seen__notilike=last_seen__notilike, last_seen__desc=last_seen__desc, last_seen__asc=last_seen__asc, segment_producer_claimed_until__eq=segment_producer_claimed_until__eq, segment_producer_claimed_until__ne=segment_producer_claimed_until__ne, segment_producer_claimed_until__gt=segment_producer_claimed_until__gt, segment_producer_claimed_until__gte=segment_producer_claimed_until__gte, segment_producer_claimed_until__lt=segment_producer_claimed_until__lt, segment_producer_claimed_until__lte=segment_producer_claimed_until__lte, segment_producer_claimed_until__in=segment_producer_claimed_until__in, segment_producer_claimed_until__nin=segment_producer_claimed_until__nin, segment_producer_claimed_until__notin=segment_producer_claimed_until__notin, segment_producer_claimed_until__isnull=segment_producer_claimed_until__isnull, segment_producer_claimed_until__nisnull=segment_producer_claimed_until__nisnull, segment_producer_claimed_until__isnotnull=segment_producer_claimed_until__isnotnull, segment_producer_claimed_until__l=segment_producer_claimed_until__l, segment_producer_claimed_until__like=segment_producer_claimed_until__like, segment_producer_claimed_until__nl=segment_producer_claimed_until__nl, segment_producer_claimed_until__nlike=segment_producer_claimed_until__nlike, segment_producer_claimed_until__notlike=segment_producer_claimed_until__notlike, segment_producer_claimed_until__il=segment_producer_claimed_until__il, segment_producer_claimed_until__ilike=segment_producer_claimed_until__ilike, segment_producer_claimed_until__nil=segment_producer_claimed_until__nil, segment_producer_claimed_until__nilike=segment_producer_claimed_until__nilike, segment_producer_claimed_until__notilike=segment_producer_claimed_until__notilike, segment_producer_claimed_until__desc=segment_producer_claimed_until__desc, segment_producer_claimed_until__asc=segment_producer_claimed_until__asc, stream_producer_claimed_until__eq=stream_producer_claimed_until__eq, stream_producer_claimed_until__ne=stream_producer_claimed_until__ne, stream_producer_claimed_until__gt=stream_producer_claimed_until__gt, stream_producer_claimed_until__gte=stream_producer_claimed_until__gte, stream_producer_claimed_until__lt=stream_producer_claimed_until__lt, stream_producer_claimed_until__lte=stream_producer_claimed_until__lte, stream_producer_claimed_until__in=stream_producer_claimed_until__in, stream_producer_claimed_until__nin=stream_producer_claimed_until__nin, stream_producer_claimed_until__notin=stream_producer_claimed_until__notin, stream_producer_claimed_until__isnull=stream_producer_claimed_until__isnull, stream_producer_claimed_until__nisnull=stream_producer_claimed_until__nisnull, stream_producer_claimed_until__isnotnull=stream_producer_claimed_until__isnotnull, stream_producer_claimed_until__l=stream_producer_claimed_until__l, stream_producer_claimed_until__like=stream_producer_claimed_until__like, stream_producer_claimed_until__nl=stream_producer_claimed_until__nl, stream_producer_claimed_until__nlike=stream_producer_claimed_until__nlike, stream_producer_claimed_until__notlike=stream_producer_claimed_until__notlike, stream_producer_claimed_until__il=stream_producer_claimed_until__il, stream_producer_claimed_until__ilike=stream_producer_claimed_until__ilike, stream_producer_claimed_until__nil=stream_producer_claimed_until__nil, stream_producer_claimed_until__nilike=stream_producer_claimed_until__nilike, stream_producer_claimed_until__notilike=stream_producer_claimed_until__notilike, stream_producer_claimed_until__desc=stream_producer_claimed_until__desc, stream_producer_claimed_until__asc=stream_producer_claimed_until__asc)
        print("The response of CameraApi->get_cameras:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling CameraApi->get_cameras: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int**| SQL LIMIT operator | [optional] 
 **offset** | **int**| SQL OFFSET operator | [optional] 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 
 **id__eq** | **str**| SQL &#x3D; operator | [optional] 
 **id__ne** | **str**| SQL !&#x3D; operator | [optional] 
 **id__gt** | **str**| SQL &gt; operator, may not work with all column types | [optional] 
 **id__gte** | **str**| SQL &gt;&#x3D; operator, may not work with all column types | [optional] 
 **id__lt** | **str**| SQL &lt; operator, may not work with all column types | [optional] 
 **id__lte** | **str**| SQL &lt;&#x3D; operator, may not work with all column types | [optional] 
 **id__in** | **str**| SQL IN operator, permits comma-separated values | [optional] 
 **id__nin** | **str**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **id__notin** | **str**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **id__isnull** | **str**| SQL IS NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **id__nisnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **id__isnotnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **id__l** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **id__like** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **id__nl** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **id__nlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **id__notlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **id__il** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **id__ilike** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **id__nil** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **id__nilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **id__notilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **id__desc** | **str**| SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) | [optional] 
 **id__asc** | **str**| SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) | [optional] 
 **created_at__eq** | **datetime**| SQL &#x3D; operator | [optional] 
 **created_at__ne** | **datetime**| SQL !&#x3D; operator | [optional] 
 **created_at__gt** | **datetime**| SQL &gt; operator, may not work with all column types | [optional] 
 **created_at__gte** | **datetime**| SQL &gt;&#x3D; operator, may not work with all column types | [optional] 
 **created_at__lt** | **datetime**| SQL &lt; operator, may not work with all column types | [optional] 
 **created_at__lte** | **datetime**| SQL &lt;&#x3D; operator, may not work with all column types | [optional] 
 **created_at__in** | **datetime**| SQL IN operator, permits comma-separated values | [optional] 
 **created_at__nin** | **datetime**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **created_at__notin** | **datetime**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **created_at__isnull** | **str**| SQL IS NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **created_at__nisnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **created_at__isnotnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **created_at__l** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__like** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__nl** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__nlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__notlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__il** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__ilike** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__nil** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__nilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__notilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **created_at__desc** | **str**| SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) | [optional] 
 **created_at__asc** | **str**| SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) | [optional] 
 **updated_at__eq** | **datetime**| SQL &#x3D; operator | [optional] 
 **updated_at__ne** | **datetime**| SQL !&#x3D; operator | [optional] 
 **updated_at__gt** | **datetime**| SQL &gt; operator, may not work with all column types | [optional] 
 **updated_at__gte** | **datetime**| SQL &gt;&#x3D; operator, may not work with all column types | [optional] 
 **updated_at__lt** | **datetime**| SQL &lt; operator, may not work with all column types | [optional] 
 **updated_at__lte** | **datetime**| SQL &lt;&#x3D; operator, may not work with all column types | [optional] 
 **updated_at__in** | **datetime**| SQL IN operator, permits comma-separated values | [optional] 
 **updated_at__nin** | **datetime**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **updated_at__notin** | **datetime**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **updated_at__isnull** | **str**| SQL IS NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **updated_at__nisnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **updated_at__isnotnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **updated_at__l** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__like** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__nl** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__nlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__notlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__il** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__ilike** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__nil** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__nilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__notilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **updated_at__desc** | **str**| SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) | [optional] 
 **updated_at__asc** | **str**| SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) | [optional] 
 **deleted_at__eq** | **datetime**| SQL &#x3D; operator | [optional] 
 **deleted_at__ne** | **datetime**| SQL !&#x3D; operator | [optional] 
 **deleted_at__gt** | **datetime**| SQL &gt; operator, may not work with all column types | [optional] 
 **deleted_at__gte** | **datetime**| SQL &gt;&#x3D; operator, may not work with all column types | [optional] 
 **deleted_at__lt** | **datetime**| SQL &lt; operator, may not work with all column types | [optional] 
 **deleted_at__lte** | **datetime**| SQL &lt;&#x3D; operator, may not work with all column types | [optional] 
 **deleted_at__in** | **datetime**| SQL IN operator, permits comma-separated values | [optional] 
 **deleted_at__nin** | **datetime**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **deleted_at__notin** | **datetime**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **deleted_at__isnull** | **str**| SQL IS NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **deleted_at__nisnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **deleted_at__isnotnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **deleted_at__l** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__like** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__nl** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__nlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__notlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__il** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__ilike** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__nil** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__nilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__notilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **deleted_at__desc** | **str**| SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) | [optional] 
 **deleted_at__asc** | **str**| SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) | [optional] 
 **name__eq** | **str**| SQL &#x3D; operator | [optional] 
 **name__ne** | **str**| SQL !&#x3D; operator | [optional] 
 **name__gt** | **str**| SQL &gt; operator, may not work with all column types | [optional] 
 **name__gte** | **str**| SQL &gt;&#x3D; operator, may not work with all column types | [optional] 
 **name__lt** | **str**| SQL &lt; operator, may not work with all column types | [optional] 
 **name__lte** | **str**| SQL &lt;&#x3D; operator, may not work with all column types | [optional] 
 **name__in** | **str**| SQL IN operator, permits comma-separated values | [optional] 
 **name__nin** | **str**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **name__notin** | **str**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **name__isnull** | **str**| SQL IS NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **name__nisnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **name__isnotnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **name__l** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **name__like** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **name__nl** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **name__nlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **name__notlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **name__il** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **name__ilike** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **name__nil** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **name__nilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **name__notilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **name__desc** | **str**| SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) | [optional] 
 **name__asc** | **str**| SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) | [optional] 
 **stream_url__eq** | **str**| SQL &#x3D; operator | [optional] 
 **stream_url__ne** | **str**| SQL !&#x3D; operator | [optional] 
 **stream_url__gt** | **str**| SQL &gt; operator, may not work with all column types | [optional] 
 **stream_url__gte** | **str**| SQL &gt;&#x3D; operator, may not work with all column types | [optional] 
 **stream_url__lt** | **str**| SQL &lt; operator, may not work with all column types | [optional] 
 **stream_url__lte** | **str**| SQL &lt;&#x3D; operator, may not work with all column types | [optional] 
 **stream_url__in** | **str**| SQL IN operator, permits comma-separated values | [optional] 
 **stream_url__nin** | **str**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **stream_url__notin** | **str**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **stream_url__isnull** | **str**| SQL IS NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **stream_url__nisnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **stream_url__isnotnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **stream_url__l** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_url__like** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_url__nl** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_url__nlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_url__notlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_url__il** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_url__ilike** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_url__nil** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_url__nilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_url__notilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_url__desc** | **str**| SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) | [optional] 
 **stream_url__asc** | **str**| SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) | [optional] 
 **last_seen__eq** | **datetime**| SQL &#x3D; operator | [optional] 
 **last_seen__ne** | **datetime**| SQL !&#x3D; operator | [optional] 
 **last_seen__gt** | **datetime**| SQL &gt; operator, may not work with all column types | [optional] 
 **last_seen__gte** | **datetime**| SQL &gt;&#x3D; operator, may not work with all column types | [optional] 
 **last_seen__lt** | **datetime**| SQL &lt; operator, may not work with all column types | [optional] 
 **last_seen__lte** | **datetime**| SQL &lt;&#x3D; operator, may not work with all column types | [optional] 
 **last_seen__in** | **datetime**| SQL IN operator, permits comma-separated values | [optional] 
 **last_seen__nin** | **datetime**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **last_seen__notin** | **datetime**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **last_seen__isnull** | **str**| SQL IS NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **last_seen__nisnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **last_seen__isnotnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **last_seen__l** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **last_seen__like** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **last_seen__nl** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **last_seen__nlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **last_seen__notlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **last_seen__il** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **last_seen__ilike** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **last_seen__nil** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **last_seen__nilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **last_seen__notilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **last_seen__desc** | **str**| SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) | [optional] 
 **last_seen__asc** | **str**| SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) | [optional] 
 **segment_producer_claimed_until__eq** | **datetime**| SQL &#x3D; operator | [optional] 
 **segment_producer_claimed_until__ne** | **datetime**| SQL !&#x3D; operator | [optional] 
 **segment_producer_claimed_until__gt** | **datetime**| SQL &gt; operator, may not work with all column types | [optional] 
 **segment_producer_claimed_until__gte** | **datetime**| SQL &gt;&#x3D; operator, may not work with all column types | [optional] 
 **segment_producer_claimed_until__lt** | **datetime**| SQL &lt; operator, may not work with all column types | [optional] 
 **segment_producer_claimed_until__lte** | **datetime**| SQL &lt;&#x3D; operator, may not work with all column types | [optional] 
 **segment_producer_claimed_until__in** | **datetime**| SQL IN operator, permits comma-separated values | [optional] 
 **segment_producer_claimed_until__nin** | **datetime**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **segment_producer_claimed_until__notin** | **datetime**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **segment_producer_claimed_until__isnull** | **str**| SQL IS NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **segment_producer_claimed_until__nisnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **segment_producer_claimed_until__isnotnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **segment_producer_claimed_until__l** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **segment_producer_claimed_until__like** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **segment_producer_claimed_until__nl** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **segment_producer_claimed_until__nlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **segment_producer_claimed_until__notlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **segment_producer_claimed_until__il** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **segment_producer_claimed_until__ilike** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **segment_producer_claimed_until__nil** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **segment_producer_claimed_until__nilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **segment_producer_claimed_until__notilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **segment_producer_claimed_until__desc** | **str**| SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) | [optional] 
 **segment_producer_claimed_until__asc** | **str**| SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) | [optional] 
 **stream_producer_claimed_until__eq** | **datetime**| SQL &#x3D; operator | [optional] 
 **stream_producer_claimed_until__ne** | **datetime**| SQL !&#x3D; operator | [optional] 
 **stream_producer_claimed_until__gt** | **datetime**| SQL &gt; operator, may not work with all column types | [optional] 
 **stream_producer_claimed_until__gte** | **datetime**| SQL &gt;&#x3D; operator, may not work with all column types | [optional] 
 **stream_producer_claimed_until__lt** | **datetime**| SQL &lt; operator, may not work with all column types | [optional] 
 **stream_producer_claimed_until__lte** | **datetime**| SQL &lt;&#x3D; operator, may not work with all column types | [optional] 
 **stream_producer_claimed_until__in** | **datetime**| SQL IN operator, permits comma-separated values | [optional] 
 **stream_producer_claimed_until__nin** | **datetime**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **stream_producer_claimed_until__notin** | **datetime**| SQL NOT IN operator, permits comma-separated values | [optional] 
 **stream_producer_claimed_until__isnull** | **str**| SQL IS NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **stream_producer_claimed_until__nisnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **stream_producer_claimed_until__isnotnull** | **str**| SQL IS NOT NULL operator, value is ignored (presence of key is sufficient) | [optional] 
 **stream_producer_claimed_until__l** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_producer_claimed_until__like** | **str**| SQL LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_producer_claimed_until__nl** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_producer_claimed_until__nlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_producer_claimed_until__notlike** | **str**| SQL NOT LIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_producer_claimed_until__il** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_producer_claimed_until__ilike** | **str**| SQL ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_producer_claimed_until__nil** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_producer_claimed_until__nilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_producer_claimed_until__notilike** | **str**| SQL NOT ILIKE operator, value is implicitly prefixed and suffixed with % | [optional] 
 **stream_producer_claimed_until__desc** | **str**| SQL ORDER BY _ DESC operator, value is ignored (presence of key is sufficient) | [optional] 
 **stream_producer_claimed_until__asc** | **str**| SQL ORDER BY _ ASC operator, value is ignored (presence of key is sufficient) | [optional] 

### Return type

[**GetCameras200Response**](GetCameras200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful List Fetch for Cameras |  -  |
**0** | Failed List Fetch for Cameras |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_camera**
> GetCameras200Response patch_camera(primary_key, camera, depth=depth)



### Example


```python
import openapi_client
from openapi_client.models.camera import Camera
from openapi_client.models.get_cameras200_response import GetCameras200Response
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
    api_instance = openapi_client.CameraApi(api_client)
    primary_key = None # object | Primary key for Camera
    camera = openapi_client.Camera() # Camera | 
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_response = api_instance.patch_camera(primary_key, camera, depth=depth)
        print("The response of CameraApi->patch_camera:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling CameraApi->patch_camera: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **primary_key** | [**object**](.md)| Primary key for Camera | 
 **camera** | [**Camera**](Camera.md)|  | 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 

### Return type

[**GetCameras200Response**](GetCameras200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful Item Update for Cameras |  -  |
**0** | Failed Item Update for Cameras |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **post_cameras**
> GetCameras200Response post_cameras(camera, depth=depth)



### Example


```python
import openapi_client
from openapi_client.models.camera import Camera
from openapi_client.models.get_cameras200_response import GetCameras200Response
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
    api_instance = openapi_client.CameraApi(api_client)
    camera = [openapi_client.Camera()] # List[Camera] | 
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_response = api_instance.post_cameras(camera, depth=depth)
        print("The response of CameraApi->post_cameras:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling CameraApi->post_cameras: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **camera** | [**List[Camera]**](Camera.md)|  | 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 

### Return type

[**GetCameras200Response**](GetCameras200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful List Create for Cameras |  -  |
**0** | Failed List Create for Cameras |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **put_camera**
> GetCameras200Response put_camera(primary_key, camera, depth=depth)



### Example


```python
import openapi_client
from openapi_client.models.camera import Camera
from openapi_client.models.get_cameras200_response import GetCameras200Response
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
    api_instance = openapi_client.CameraApi(api_client)
    primary_key = None # object | Primary key for Camera
    camera = openapi_client.Camera() # Camera | 
    depth = 56 # int | Max recursion depth for loading foreign objects; default = 1  (0 = recurse until graph cycle detected, 1 = this object only, 2 = this object + neighbours, 3 = this object + neighbours + their neighbours... etc) (optional)

    try:
        api_response = api_instance.put_camera(primary_key, camera, depth=depth)
        print("The response of CameraApi->put_camera:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling CameraApi->put_camera: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **primary_key** | [**object**](.md)| Primary key for Camera | 
 **camera** | [**Camera**](Camera.md)|  | 
 **depth** | **int**| Max recursion depth for loading foreign objects; default &#x3D; 1  (0 &#x3D; recurse until graph cycle detected, 1 &#x3D; this object only, 2 &#x3D; this object + neighbours, 3 &#x3D; this object + neighbours + their neighbours... etc) | [optional] 

### Return type

[**GetCameras200Response**](GetCameras200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Successful Item Replace for Cameras |  -  |
**0** | Failed Item Replace for Cameras |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

