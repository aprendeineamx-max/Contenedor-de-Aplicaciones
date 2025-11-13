# \ContainersApi

All URIs are relative to *https://localhost:7443/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**containers_container_id_delete**](ContainersApi.md#containers_container_id_delete) | **DELETE** /containers/{containerId} | Eliminar contenedor
[**containers_container_id_get**](ContainersApi.md#containers_container_id_get) | **GET** /containers/{containerId} | Obtener contenedor
[**containers_get**](ContainersApi.md#containers_get) | **GET** /containers | Listar contenedores
[**containers_post**](ContainersApi.md#containers_post) | **POST** /containers | Crear contenedor



## containers_container_id_delete

> models::Task containers_container_id_delete(container_id)
Eliminar contenedor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## containers_container_id_get

> models::Container containers_container_id_get(container_id)
Obtener contenedor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Container**](Container.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## containers_get

> Vec<models::Container> containers_get(status)
Listar contenedores

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> |  |  |

### Return type

[**Vec<models::Container>**](Container.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## containers_post

> models::Task containers_post(containers_post_request)
Crear contenedor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**containers_post_request** | [**ContainersPostRequest**](ContainersPostRequest.md) |  | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

