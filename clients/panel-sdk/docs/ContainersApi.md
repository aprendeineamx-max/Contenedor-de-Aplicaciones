# ContainersApi

All URIs are relative to *https://localhost:7443/api*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**containersContainerIdDelete**](ContainersApi.md#containerscontaineriddelete) | **DELETE** /containers/{containerId} | Eliminar contenedor |
| [**containersContainerIdGet**](ContainersApi.md#containerscontaineridget) | **GET** /containers/{containerId} | Obtener contenedor |
| [**containersGet**](ContainersApi.md#containersget) | **GET** /containers | Listar contenedores |
| [**containersPost**](ContainersApi.md#containerspostoperation) | **POST** /containers | Crear contenedor |



## containersContainerIdDelete

> Task containersContainerIdDelete(containerId)

Eliminar contenedor

### Example

```ts
import {
  Configuration,
  ContainersApi,
} from '@orbit/panel-sdk';
import type { ContainersContainerIdDeleteRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new ContainersApi(config);

  const body = {
    // string
    containerId: 38400000-8cf0-11bd-b23e-10b96e4ef00d,
  } satisfies ContainersContainerIdDeleteRequest;

  try {
    const data = await api.containersContainerIdDelete(body);
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
| **containerId** | `string` |  | [Defaults to `undefined`] |

### Return type

[**Task**](Task.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Tarea disparada |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## containersContainerIdGet

> Container containersContainerIdGet(containerId)

Obtener contenedor

### Example

```ts
import {
  Configuration,
  ContainersApi,
} from '@orbit/panel-sdk';
import type { ContainersContainerIdGetRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new ContainersApi(config);

  const body = {
    // string
    containerId: 38400000-8cf0-11bd-b23e-10b96e4ef00d,
  } satisfies ContainersContainerIdGetRequest;

  try {
    const data = await api.containersContainerIdGet(body);
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
| **containerId** | `string` |  | [Defaults to `undefined`] |

### Return type

[**Container**](Container.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | OK |  -  |
| **404** | No encontrado |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## containersGet

> Array&lt;Container&gt; containersGet(status)

Listar contenedores

### Example

```ts
import {
  Configuration,
  ContainersApi,
} from '@orbit/panel-sdk';
import type { ContainersGetRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new ContainersApi(config);

  const body = {
    // string (optional)
    status: status_example,
  } satisfies ContainersGetRequest;

  try {
    const data = await api.containersGet(body);
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
| **status** | `string` |  | [Optional] [Defaults to `undefined`] |

### Return type

[**Array&lt;Container&gt;**](Container.md)

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


## containersPost

> Task containersPost(containersPostRequest)

Crear contenedor

### Example

```ts
import {
  Configuration,
  ContainersApi,
} from '@orbit/panel-sdk';
import type { ContainersPostOperationRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new ContainersApi(config);

  const body = {
    // ContainersPostRequest
    containersPostRequest: ...,
  } satisfies ContainersPostOperationRequest;

  try {
    const data = await api.containersPost(body);
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
| **containersPostRequest** | [ContainersPostRequest](ContainersPostRequest.md) |  | |

### Return type

[**Task**](Task.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Tarea creada |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

