
# ConfigResponse


## Properties

Name | Type
------------ | -------------
`config` | [ConfigSnapshot](ConfigSnapshot.md)
`security` | [SecurityStatus](SecurityStatus.md)
`sources` | [ConfigSources](ConfigSources.md)

## Example

```typescript
import type { ConfigResponse } from '@orbit/panel-sdk'

// TODO: Update the object below with actual values
const example = {
  "config": null,
  "security": null,
  "sources": null,
} satisfies ConfigResponse

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ConfigResponse
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


