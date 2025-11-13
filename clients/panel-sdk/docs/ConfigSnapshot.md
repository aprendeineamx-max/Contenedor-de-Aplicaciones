
# ConfigSnapshot


## Properties

Name | Type
------------ | -------------
`containersRoot` | string
`telemetryLevel` | string
`apiBind` | string
`databasePath` | string
`security` | [ConfigSecurity](ConfigSecurity.md)

## Example

```typescript
import type { ConfigSnapshot } from '@orbit/panel-sdk'

// TODO: Update the object below with actual values
const example = {
  "containersRoot": null,
  "telemetryLevel": null,
  "apiBind": null,
  "databasePath": null,
  "security": null,
} satisfies ConfigSnapshot

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ConfigSnapshot
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


