# \SnapshotsApi

All URIs are relative to *https://localhost:7443/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**containers_container_id_snapshots_get**](SnapshotsApi.md#containers_container_id_snapshots_get) | **GET** /containers/{containerId}/snapshots | Listar snapshots
[**containers_container_id_snapshots_post**](SnapshotsApi.md#containers_container_id_snapshots_post) | **POST** /containers/{containerId}/snapshots | Crear snapshot
[**snapshots_snapshot_id_restore_post**](SnapshotsApi.md#snapshots_snapshot_id_restore_post) | **POST** /snapshots/{snapshotId}/restore | Restaurar snapshot



## containers_container_id_snapshots_get

> Vec<models::Snapshot> containers_container_id_snapshots_get(container_id)
Listar snapshots

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::Snapshot>**](Snapshot.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## containers_container_id_snapshots_post

> models::Task containers_container_id_snapshots_post(container_id, containers_container_id_snapshots_post_request)
Crear snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**container_id** | **uuid::Uuid** |  | [required] |
**containers_container_id_snapshots_post_request** | Option<[**ContainersContainerIdSnapshotsPostRequest**](ContainersContainerIdSnapshotsPostRequest.md)> |  |  |

### Return type

[**models::Task**](Task.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshots_snapshot_id_restore_post

> models::Task snapshots_snapshot_id_restore_post(snapshot_id)
Restaurar snapshot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

