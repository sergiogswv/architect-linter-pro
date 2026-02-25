---
title: React Architecture Template
sidebar_label: React
---

# React Architecture Template

Pre-configured architectural rules for React projects.

## Quick Start

```bash
architect --template react
```

## Layer Structure

React projects benefit from feature-based organization:

- **Components** - UI components (presentational)
- **Containers** - Smart/connected components
- **Services** - API and business logic
- **Hooks** - Custom React hooks
- **Utils** - Helper functions
- **Styles** - Component styling

```
src/
├── features/
│   ├── auth/
│   │   ├── components/
│   │   ├── hooks/
│   │   ├── services/
│   │   ├── types/
│   │   └── index.ts
│   ├── users/
│   ├── products/
│   └── shared/
├── common/
│   ├── components/
│   ├── hooks/
│   └── utils/
├── styles/
└── App.tsx
```

## Pre-configured Rules

```json
{
  "max_lines_per_function": 50,
  "architecture_pattern": "Ninguno",
  "forbidden_imports": [
    {
      "from": "src/features/**/components/**",
      "to": "src/features/**/services/**",
      "reason": "Components should use hooks, not call services directly"
    },
    {
      "from": "src/features/**",
      "to": "src/features/*/components/**",
      "reason": "Features should only import from their own feature or common"
    },
    {
      "from": "src/common/**",
      "to": "src/features/**",
      "reason": "Common utilities should not depend on specific features"
    }
  ]
}
```

## Best Practices

- Keep components small and focused
- Use custom hooks for reusable logic
- Extract services for API calls
- Use TypeScript for type safety
- Organize by feature, not file type
- Keep styling co-located with components

## Common Issues

### State Management

Avoid prop drilling with context or state management libraries:

```typescript
// Use context for shared state
const UserContext = createContext();
```

### Component Re-renders

Optimize with React.memo and useMemo:

```typescript
export const MyComponent = React.memo(({ data }) => {
  // Component won't re-render if props don't change
});
```
