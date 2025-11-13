# SecurityTokensPostRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Etiqueta descriptiva del token | 
**scopes** | Option<**Vec<String>**> | Lista de scopes (p.e. `containers:read`, `tasks:write`). VacA�o = sin privilegios adicionales. | [optional]
**expires_at** | Option<**String**> | Fecha de expiraciA3n en formato RFC3339 (opcional) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


