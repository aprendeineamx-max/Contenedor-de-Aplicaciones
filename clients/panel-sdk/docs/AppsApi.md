# AppsApi

All URIs are relative to *https://localhost:7443/api*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**appsAppIdLaunchPost**](AppsApi.md#appsappidlaunchpostoperation) | **POST** /apps/{appId}/launch | Ejecutar una app |
| [**containersContainerIdAppsGet**](AppsApi.md#containerscontaineridappsget) | **GET** /containers/{containerId}/apps | Listar apps dentro de un contenedor |
| [**containersContainerIdAppsPost**](AppsApi.md#containerscontaineridappspostoperation) | **POST** /containers/{containerId}/apps | Instalar app dentro de contenedor |



## appsAppIdLaunchPost

> Task appsAppIdLaunchPost(appId, appsAppIdLaunchPostRequest)

Ejecutar una app

### Example

```ts
import {
  Configuration,
  AppsApi,
} from '@orbit/panel-sdk';
import type { AppsAppIdLaunchPostOperationRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new AppsApi(config);

  const body = {
    // string
    appId: 38400000-8cf0-11bd-b23e-10b96e4ef00d,
    // AppsAppIdLaunchPostRequest (optional)
    appsAppIdLaunchPostRequest: ...,
  } satisfies AppsAppIdLaunchPostOperationRequest;

  try {
    const data = await api.appsAppIdLaunchPost(body);
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
| **appId** | `string` |  | [Defaults to `undefined`] |
| **appsAppIdLaunchPostRequest** | [AppsAppIdLaunchPostRequest](AppsAppIdLaunchPostRequest.md) |  | [Optional] |

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
| **202** | Tarea de lanzamiento creada |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## containersContainerIdAppsGet

> Array&lt;AppInstance&gt; containersContainerIdAppsGet(containerId)

Listar apps dentro de un contenedor

### Example

```ts
import {
  Configuration,
  AppsApi,
} from '@orbit/panel-sdk';
import type { ContainersContainerIdAppsGetRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new AppsApi(config);

  const body = {
    // string
    containerId: 38400000-8cf0-11bd-b23e-10b96e4ef00d,
  } satisfies ContainersContainerIdAppsGetRequest;

  try {
    const data = await api.containersContainerIdAppsGet(body);
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

[**Array&lt;AppInstance&gt;**](AppInstance.md)

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


## containersContainerIdAppsPost

> Task containersContainerIdAppsPost(containerId, containersContainerIdAppsPostRequest)

Instalar app dentro de contenedor

### Example

```ts
import {
  Configuration,
  AppsApi,
} from '@orbit/panel-sdk';
import type { ContainersContainerIdAppsPostOperationRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new AppsApi(config);

  const body = {
    // string
    containerId: 38400000-8cf0-11bd-b23e-10b96e4ef00d,
    // ContainersContainerIdAppsPostRequest
    containersContainerIdAppsPostRequest: ...,
  } satisfies ContainersContainerIdAppsPostOperationRequest;

  try {
    const data = await api.containersContainerIdAppsPost(body);
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
| **containersContainerIdAppsPostRequest** | [ContainersContainerIdAppsPostRequest](ContainersContainerIdAppsPostRequest.md) |  | |

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
| **202** | Tarea encolada |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

