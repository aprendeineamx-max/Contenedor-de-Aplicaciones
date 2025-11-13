
# Task


## Properties

Name | Type
------------ | -------------
`id` | string
`type` | string
`status` | string
`progress` | number
`payload` | object
`result` | object
`createdAt` | Date
`updatedAt` | Date

## Example

```typescript
import type { Task } from '@orbit/panel-sdk'

// TODO: Update the object below with actual values
const example = {
  "id": null,
  "type": null,
  "status": null,
  "progress": null,
  "payload": null,
  "result": null,
  "createdAt": null,
  "updatedAt": null,
} satisfies Task

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Task
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


