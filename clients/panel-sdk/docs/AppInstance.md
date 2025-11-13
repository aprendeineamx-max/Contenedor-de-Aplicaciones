
# AppInstance


## Properties

Name | Type
------------ | -------------
`id` | string
`containerId` | string
`name` | string
`version` | string
`status` | string
`entryPoints` | [Array&lt;AppInstanceEntryPointsInner&gt;](AppInstanceEntryPointsInner.md)
`createdAt` | Date

## Example

```typescript
import type { AppInstance } from '@orbit/panel-sdk'

// TODO: Update the object below with actual values
const example = {
  "id": null,
  "containerId": null,
  "name": null,
  "version": null,
  "status": null,
  "entryPoints": null,
  "createdAt": null,
} satisfies AppInstance

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as AppInstance
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


