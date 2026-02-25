---
title: NestJS Integration Guide
sidebar_position: 1
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# NestJS Integration Guide

NestJS is one of the most popular frameworks for building scalable server-side applications with Node.js. Its modular architecture makes it a perfect fit for **Architect Linter Pro**.

## Recommended Architectural Patterns

Most NestJS projects follow either a standard **MVC** (Model-View-Controller) pattern or a more advanced **Hexagonal/Clean Architecture**.

<Tabs>
  <TabItem value="mvc" label="MVC Pattern" default>
    In a standard NestJS project, you want to ensure that controllers only talk to services, and services handle the business logic and database interactions.

    **Forbidden Rules for MVC:**

    ```json
    {
      "forbidden_imports": [
        {
          "from": ".controller.ts",
          "to": ".repository.ts",
          "reason": "Controllers should use services, not repositories directly."
        },
        {
          "from": ".service.ts",
          "to": ".controller.ts",
          "reason": "Services should never depend on controllers (Circular dependency)."
        }
      ]
    }
    ```
  </TabItem>
  <TabItem value="hex" label="Hexagonal Architecture">
    If you are using Hexagonal Architecture, you want to protect your `domain` layer from any `infrastructure` details (like TypeORM or Axios).

    **Forbidden Rules for Hexagonal:**

    ```json
    {
      "forbidden_imports": [
        {
          "from": "/domain/",
          "to": "/infrastructure/",
          "reason": "Domain layer must be agnostic of implementation details."
        },
        {
          "from": "/domain/",
          "to": "/application/",
          "reason": "Entities should not depend on use cases."
        }
      ]
    }
    ```
  </TabItem>
</Tabs>

## Automatic Framework Detection

When you run `architect-linter-pro` for the first time in a NestJS project, it will:
1. Detect `package.json` with `@nestjs/core`.
2. Suggest the **MVC** or **Hexagonal** pattern.
3. Automatically configure `max_lines_per_function` (recommended: 40-50).

## Pre-commit Integration with Husky

To ensure no architectural violations reach your repository, we recommend using Husky.

1. **Install Husky:**
   ```bash
   npx husky-init && npm install
   ```

2. **Configure Hook:**
   Edit `.husky/pre-commit`:
   ```bash
   #!/bin/sh
   . "$(dirname "$0")/_/husky.sh"

   echo "🏗️ Running Architect Linter..."
   architect-linter-pro . --staged
   ```

## Best Practices

- **Use DTOs**: Ensure your domain doesn't leak into your controllers via raw entities.
- **Interface Segregation**: Use interfaces in your domain/application layer and implement them in the infrastructure layer.
- **Health Score**: Aim for a Health Score of **A (90+)** by keeping your modules decoupled and your functions small.
