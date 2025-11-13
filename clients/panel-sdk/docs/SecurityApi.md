# SecurityApi

All URIs are relative to *https://localhost:7443/api*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**securityTokensGet**](SecurityApi.md#securitytokensget) | **GET** /security/tokens | Listar tokens de servicio emitidos |
| [**securityTokensPost**](SecurityApi.md#securitytokenspostoperation) | **POST** /security/tokens | Emitir un token de servicio |
| [**securityTokensTokenIdDelete**](SecurityApi.md#securitytokenstokeniddelete) | **DELETE** /security/tokens/{tokenId} | Revocar token |
| [**systemSecurityReloadPost**](SecurityApi.md#systemsecurityreloadpost) | **POST** /system/security/reload | Recargar configuración y tokens estáticos desde variables de entorno |



## securityTokensGet

> Array&lt;ApiToken&gt; securityTokensGet()

Listar tokens de servicio emitidos

### Example

```ts
import {
  Configuration,
  SecurityApi,
} from '@orbit/panel-sdk';
import type { SecurityTokensGetRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new SecurityApi(config);

  try {
    const data = await api.securityTokensGet();
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**Array&lt;ApiToken&gt;**](ApiToken.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | OK |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## securityTokensPost

> ApiTokenCreated securityTokensPost(securityTokensPostRequest)

Emitir un token de servicio

### Example

```ts
import {
  Configuration,
  SecurityApi,
} from '@orbit/panel-sdk';
import type { SecurityTokensPostOperationRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new SecurityApi(config);

  const body = {
    // SecurityTokensPostRequest
    securityTokensPostRequest: ...,
  } satisfies SecurityTokensPostOperationRequest;

  try {
    const data = await api.securityTokensPost(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **securityTokensPostRequest** | [SecurityTokensPostRequest](SecurityTokensPostRequest.md) |  | |

### Return type

[**ApiTokenCreated**](ApiTokenCreated.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Token emitido |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## securityTokensTokenIdDelete

> securityTokensTokenIdDelete(tokenId)

Revocar token

### Example

```ts
import {
  Configuration,
  SecurityApi,
} from '@orbit/panel-sdk';
import type { SecurityTokensTokenIdDeleteRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new SecurityApi(config);

  const body = {
    // string
    tokenId: 38400000-8cf0-11bd-b23e-10b96e4ef00d,
  } satisfies SecurityTokensTokenIdDeleteRequest;

  try {
    const data = await api.securityTokensTokenIdDelete(body);
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters


| Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **tokenId** | `string` |  | [Defaults to `undefined`] |

### Return type

`void` (Empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **204** | Token revocado |  -  |
| **404** | Token inexistente |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## systemSecurityReloadPost

> SecurityStatus systemSecurityReloadPost()

Recargar configuración y tokens estáticos desde variables de entorno

### Example

```ts
import {
  Configuration,
  SecurityApi,
} from '@orbit/panel-sdk';
import type { SystemSecurityReloadPostRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new SecurityApi(config);

  try {
    const data = await api.systemSecurityReloadPost();
    console.log(data);
  } catch (error) {
    console.error(error);
  }
}

// Run the test
example().catch(console.error);
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**SecurityStatus**](SecurityStatus.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Estado de seguridad actualizado |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

