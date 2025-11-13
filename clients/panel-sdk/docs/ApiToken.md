
# ApiToken


## Properties

Name | Type
------------ | -------------
`id` | string
`name` | string
`prefix` | string
`createdAt` | Date
`revokedAt` | Date

## Example

```typescript
import type { ApiToken } from '@orbit/panel-sdk'

// TODO: Update the object below with actual values
const example = {
  "id": null,
  "name": null,
  "prefix": null,
  "createdAt": null,
  "revokedAt": null,
} satisfies ApiToken

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ApiToken
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


