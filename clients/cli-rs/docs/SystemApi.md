# \SystemApi

All URIs are relative to *https://localhost:7443/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**system_config_get**](SystemApi.md#system_config_get) | **GET** /system/config | Snapshot de configuraciA3n efectiva (solo admins)
[**system_info_get**](SystemApi.md#system_info_get) | **GET** /system/info | Información del agente



## system_config_get

> models::ConfigResponse system_config_get()
Snapshot de configuraciA3n efectiva (solo admins)

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ConfigResponse**](ConfigResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_info_get

> models::SystemInfoGet200Response system_info_get()
Información del agente

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SystemInfoGet200Response**](_system_info_get_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

