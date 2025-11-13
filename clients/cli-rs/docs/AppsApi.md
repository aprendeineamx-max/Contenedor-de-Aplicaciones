# \AppsApi

All URIs are relative to *https://localhost:7443/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_app_id_launch_post**](AppsApi.md#apps_app_id_launch_post) | **POST** /apps/{appId}/launch | Ejecutar una app
[**containers_container_id_apps_get**](AppsApi.md#containers_container_id_apps_get) | **GET** /containers/{containerId}/apps | Listar apps dentro de un contenedor
[**containers_container_id_apps_post**](AppsApi.md#containers_container_id_apps_post) | **POST** /containers/{containerId}/apps | Instalar app dentro de contenedor



## apps_app_id_launch_post

> models::Task apps_app_id_launch_post(app_id, apps_app_id_launch_post_request)
Ejecutar una app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **uuid::Uuid** |  | [required] |
**apps_app_id_launch_post_request** | Option<[**AppsAppIdLaunchPostRequest**](AppsAppIdLaunchPostRequest.md)> |  |  |

### Return type

[**models::Task**](Task.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## containers_container_id_apps_get

> Vec<models::AppInstance> containers_container_id_apps_get(container_id)
Listar apps dentro de un contenedor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::AppInstance>**](AppInstance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## containers_container_id_apps_post

> models::Task containers_container_id_apps_post(container_id, containers_container_id_apps_post_request)
Instalar app dentro de contenedor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** |  | [required] |
**containers_container_id_apps_post_request** | [**ContainersContainerIdAppsPostRequest**](ContainersContainerIdAppsPostRequest.md) |  | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

