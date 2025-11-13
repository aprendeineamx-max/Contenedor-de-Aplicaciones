
# Container


## Properties

Name | Type
------------ | -------------
`id` | string
`name` | string
`description` | string
`status` | string
`platform` | string
`tags` | Array&lt;string&gt;
`sizeBytes` | number
`settings` | object
`createdAt` | Date
`updatedAt` | Date

## Example

```typescript
import type { Container } from '@orbit/panel-sdk'

// TODO: Update the object below with actual values
const example = {
  "id": null,
  "name": null,
  "description": null,
  "status": null,
  "platform": null,
  "tags": null,
  "sizeBytes": null,
  "settings": null,
  "createdAt": null,
  "updatedAt": null,
} satisfies Container

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as Container
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


