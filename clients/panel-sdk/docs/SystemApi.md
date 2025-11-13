# SystemApi

All URIs are relative to *https://localhost:7443/api*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**systemConfigGet**](SystemApi.md#systemconfigget) | **GET** /system/config | Snapshot de configuraciA3n efectiva (solo admins) |
| [**systemInfoGet**](SystemApi.md#systeminfoget) | **GET** /system/info | Información del agente |



## systemConfigGet

> ConfigResponse systemConfigGet()

Snapshot de configuraciA3n efectiva (solo admins)

### Example

```ts
import {
  Configuration,
  SystemApi,
} from '@orbit/panel-sdk';
import type { SystemConfigGetRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new SystemApi(config);

  try {
    const data = await api.systemConfigGet();
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

[**ConfigResponse**](ConfigResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | ConfiguraciA3n resultante y fuentes |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## systemInfoGet

> SystemInfoGet200Response systemInfoGet()

Información del agente

### Example

```ts
import {
  Configuration,
  SystemApi,
} from '@orbit/panel-sdk';
import type { SystemInfoGetRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new SystemApi(config);

  try {
    const data = await api.systemInfoGet();
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

[**SystemInfoGet200Response**](SystemInfoGet200Response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Info y métricas |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

