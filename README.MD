This tool is intended to be a buffed `touch` for React projects. Upon creating a file, it also creates boilerplate for a Typescript module. 

How it works:

```typescript
// touchsx dopeFunc.tsx
// touchsx dopeFunc.jsx
// touchsx dopeFunc.arbitrary
// touchsx dopeFunc
export default function DopeFunc() {
	return (

	)
}
```

```typescript
// touchsx dopeFunc.ts
// touchsx dopeFunc.js

export default function dopeFunc() {
	return (

	)
}
```

As with `touch`, multiple filenames can be passed in: `touchsx DopeFunc CoolFunc RadFunc`

Open for PRs and for suggestions. 