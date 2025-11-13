
# SystemInfoGet200Response


## Properties

Name | Type
------------ | -------------
`version` | string
`build` | string
`uptimeSeconds` | number
`driverStatus` | string

## Example

```typescript
import type { SystemInfoGet200Response } from '@orbit/panel-sdk'

// TODO: Update the object below with actual values
const example = {
  "version": null,
  "build": null,
  "uptimeSeconds": null,
  "driverStatus": null,
} satisfies SystemInfoGet200Response

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as SystemInfoGet200Response
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


