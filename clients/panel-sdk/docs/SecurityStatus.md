
# SecurityStatus


## Properties

Name | Type
------------ | -------------
`authEnabled` | boolean
`adminTokenPresent` | boolean
`staticTokenCount` | number
`managedTokenCount` | number
`expiringTokenCount` | number
`scopesCatalog` | Array&lt;string&gt;

## Example

```typescript
import type { SecurityStatus } from '@orbit/panel-sdk'

// TODO: Update the object below with actual values
const example = {
  "authEnabled": null,
  "adminTokenPresent": null,
  "staticTokenCount": null,
  "managedTokenCount": null,
  "expiringTokenCount": null,
  "scopesCatalog": null,
} satisfies SecurityStatus

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SecurityStatus
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


