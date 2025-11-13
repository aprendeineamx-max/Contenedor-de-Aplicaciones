
# ConfigSources


## Properties

Name | Type
------------ | -------------
`defaultsFile` | string
`localFile` | string
`envOverrides` | Array&lt;string&gt;

## Example

```typescript
import type { ConfigSources } from '@orbit/panel-sdk'

// TODO: Update the object below with actual values
const example = {
  "defaultsFile": null,
  "localFile": null,
  "envOverrides": null,
} satisfies ConfigSources

console.log(example)

// Convert the instance to a JSON string
const exampleJSON: string = JSON.stringify(example)
console.log(exampleJSON)

// Parse the JSON string back to an object
const exampleParsed = JSON.parse(exampleJSON) as ConfigSources
console.log(exampleParsed)
```

[[Back to top]](#) [[Back to API list]](../README.md#api-endpoints) [[Back to Model list]](../README.md#models) [[Back to README]](../README.md)


