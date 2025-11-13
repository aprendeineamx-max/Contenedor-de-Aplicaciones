
# ConfigSecurity


## Properties

Name | Type
------------ | -------------
`authEnabled` | boolean
`adminTokenPresent` | boolean
`staticTokens` | number

## Example

```typescript
import type { ConfigSecurity } from '@orbit/panel-sdk'

// TODO: Update the object below with actual values
const example = {
  "authEnabled": null,
  "adminTokenPresent": null,
  "staticTokens": null,
} satisfies ConfigSecurity

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ConfigSecurity
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


