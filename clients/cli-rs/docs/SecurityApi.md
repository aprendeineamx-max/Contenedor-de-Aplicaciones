# \SecurityApi

All URIs are relative to *https://localhost:7443/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**security_tokens_get**](SecurityApi.md#security_tokens_get) | **GET** /security/tokens | Listar tokens de servicio emitidos
[**security_tokens_post**](SecurityApi.md#security_tokens_post) | **POST** /security/tokens | Emitir un token de servicio
[**security_tokens_token_id_delete**](SecurityApi.md#security_tokens_token_id_delete) | **DELETE** /security/tokens/{tokenId} | Revocar token
[**system_security_reload_post**](SecurityApi.md#system_security_reload_post) | **POST** /system/security/reload | Recargar configuración y tokens estáticos desde variables de entorno



## security_tokens_get

> Vec<models::ApiToken> security_tokens_get()
Listar tokens de servicio emitidos

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApiToken>**](ApiToken.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_tokens_post

> models::ApiTokenCreated security_tokens_post(security_tokens_post_request)
Emitir un token de servicio

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**security_tokens_post_request** | [**SecurityTokensPostRequest**](SecurityTokensPostRequest.md) |  | [required] |

### Return type

[**models::ApiTokenCreated**](ApiTokenCreated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_tokens_token_id_delete

> security_tokens_token_id_delete(token_id)
Revocar token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_security_reload_post

> models::SecurityStatus system_security_reload_post()
Recargar configuración y tokens estáticos desde variables de entorno

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SecurityStatus**](SecurityStatus.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

