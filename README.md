This tool is intended to be a buffed `touch` for React projects. Upon creating a file, it also creates boilerplate for a Typescript module.

```
USAGE:
    touchsx [names]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <names>...    Name of file
```

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

Open for PRs and for suggestions!
