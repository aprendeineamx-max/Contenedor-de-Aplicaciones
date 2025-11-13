
# ContainersPostRequest


## Properties

Name | Type
------------ | -------------
`name` | string
`description` | string
`platform` | string
`settings` | object

## Example

```typescript
import type { ContainersPostRequest } from '@orbit/panel-sdk'

// TODO: Update the object below with actual values
const example = {
  "name": null,
  "description": null,
  "platform": null,
  "settings": null,
} satisfies ContainersPostRequest

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ContainersPostRequest
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


