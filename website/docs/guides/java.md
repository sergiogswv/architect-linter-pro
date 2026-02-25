---
title: Java (Spring Boot) Guide
sidebar_position: 5
---

# Java & Spring Boot Integration Guide

Architect Linter Pro brings enterprise-grade architecture linting to Java applications, with specialized support for Spring Boot.

## Spring Boot Architecture

Spring Boot applications typically follow a layered architecture: **Controller** → **Service** → **Repository** → **Entity**.

### 1. Enforcing Layered Architecture

The most common requirement is ensuring that the Repository layer doesn't leak into the Controller layer, and that entities remain pure.

**Forbidden Rules for Spring Boot:**

```json
{
  "forbidden_imports": [
    {
      "from": "com.example.controller",
      "to": "com.example.repository",
      "reason": "Controllers must use services to interact with data."
    },
    {
      "from": "com.example.service",
      "to": "com.example.controller",
      "reason": "Services should never depend on controllers (Circular Dependency)."
    }
  ]
}
```

### 2. Hexagonal Architecture in Java

Java is a popular choice for Hexagonal Architecture (Ports and Adapters).

**Rules for Hexagonal Java:**

```json
{
  "forbidden_imports": [
    {
      "from": ".domain.",
      "to": ".infrastructure.",
      "reason": "The domain core must be independent of database or external API implementations."
    },
    {
      "from": ".domain.",
      "to": ".application.",
      "reason": "Business logic should not depend on specific use case orchestration."
    }
  ]
}
```

## Tree-sitter for Java

The Java parser supports:
- `import package.name.Class;`
- Wildcard imports: `import package.name.*;`
- Static imports
- Accurate package path resolution based on file location

## Maven & Gradle Integration

You can run Architect Linter Pro as a step in your build lifecycle.

**Maven Example:**
```xml
<plugin>
  <groupId>org.codehaus.mojo</groupId>
  <artifactId>exec-maven-plugin</artifactId>
  <executions>
    <execution>
      <id>architect-lint</id>
      <phase>test</phase>
      <goals>
        <goal>exec</goal>
      </goals>
      <configuration>
        <executable>architect-linter-pro</executable>
        <arguments>
          <argument>.</argument>
        </arguments>
      </configuration>
    </execution>
  </executions>
</plugin>
```

## Health Score for Java

Java codebases can become verbose. Use the **Health Score** to keep an eye on:
- **Complexity**: Identify massive Service or Controller classes.
- **Layer Isolation**: Ensure Spring's `@Autowired` doesn't lead to architectural drift.

## Best Practices

- **Use DTOs**: Transfer data between layers using simple POJOs.
- **Dependency Inversion**: Use Spring interfaces to decouple your domain from implementation details.
- **Default Scoping**: Use package-private visibility where possible to enforce isolation beyond what the linter detects.
