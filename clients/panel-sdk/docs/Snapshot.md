
# Snapshot


## Properties

Name | Type
------------ | -------------
`id` | string
`containerId` | string
`label` | string
`type` | string
`baseSnapshotId` | string
`sizeBytes` | number
`createdAt` | Date

## Example

```typescript
import type { Snapshot } from '@orbit/panel-sdk'

// TODO: Update the object below with actual values
const example = {
  "id": null,
  "containerId": null,
  "label": null,
  "type": null,
  "baseSnapshotId": null,
  "sizeBytes": null,
  "createdAt": null,
} satisfies Snapshot

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Snapshot
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


