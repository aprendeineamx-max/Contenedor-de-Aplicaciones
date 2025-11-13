# SnapshotsApi

All URIs are relative to *https://localhost:7443/api*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**containersContainerIdSnapshotsGet**](SnapshotsApi.md#containerscontaineridsnapshotsget) | **GET** /containers/{containerId}/snapshots | Listar snapshots |
| [**containersContainerIdSnapshotsPost**](SnapshotsApi.md#containerscontaineridsnapshotspostoperation) | **POST** /containers/{containerId}/snapshots | Crear snapshot |
| [**snapshotsSnapshotIdRestorePost**](SnapshotsApi.md#snapshotssnapshotidrestorepost) | **POST** /snapshots/{snapshotId}/restore | Restaurar snapshot |



## containersContainerIdSnapshotsGet

> Array&lt;Snapshot&gt; containersContainerIdSnapshotsGet(containerId)

Listar snapshots

### Example

```ts
import {
  Configuration,
  SnapshotsApi,
} from '@orbit/panel-sdk';
import type { ContainersContainerIdSnapshotsGetRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new SnapshotsApi(config);

  const body = {
    // string
    containerId: 38400000-8cf0-11bd-b23e-10b96e4ef00d,
  } satisfies ContainersContainerIdSnapshotsGetRequest;

  try {
    const data = await api.containersContainerIdSnapshotsGet(body);
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

[**Array&lt;Snapshot&gt;**](Snapshot.md)

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


## containersContainerIdSnapshotsPost

> Task containersContainerIdSnapshotsPost(containerId, containersContainerIdSnapshotsPostRequest)

Crear snapshot

### Example

```ts
import {
  Configuration,
  SnapshotsApi,
} from '@orbit/panel-sdk';
import type { ContainersContainerIdSnapshotsPostOperationRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new SnapshotsApi(config);

  const body = {
    // string
    containerId: 38400000-8cf0-11bd-b23e-10b96e4ef00d,
    // ContainersContainerIdSnapshotsPostRequest (optional)
    containersContainerIdSnapshotsPostRequest: ...,
  } satisfies ContainersContainerIdSnapshotsPostOperationRequest;

  try {
    const data = await api.containersContainerIdSnapshotsPost(body);
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
| **containersContainerIdSnapshotsPostRequest** | [ContainersContainerIdSnapshotsPostRequest](ContainersContainerIdSnapshotsPostRequest.md) |  | [Optional] |

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
| **202** | Tarea de snapshot |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


## snapshotsSnapshotIdRestorePost

> Task snapshotsSnapshotIdRestorePost(snapshotId)

Restaurar snapshot

### Example

```ts
import {
  Configuration,
  SnapshotsApi,
} from '@orbit/panel-sdk';
import type { SnapshotsSnapshotIdRestorePostRequest } from '@orbit/panel-sdk';

async function example() {
  console.log("🚀 Testing @orbit/panel-sdk SDK...");
  const config = new Configuration({ 
    // Configure HTTP bearer authorization: bearerAuth
    accessToken: "YOUR BEARER TOKEN",
  });
  const api = new SnapshotsApi(config);

  const body = {
    // string
    snapshotId: 38400000-8cf0-11bd-b23e-10b96e4ef00d,
  } satisfies SnapshotsSnapshotIdRestorePostRequest;

  try {
    const data = await api.snapshotsSnapshotIdRestorePost(body);
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
| **snapshotId** | `string` |  | [Defaults to `undefined`] |

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
| **202** | Restauración en progreso |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)

