---
title: Django Architecture Template
sidebar_label: Django
---

# Django Architecture Template

Pre-configured architectural rules for Django projects.

## Quick Start

```bash
architect --template django
```

## Layer Structure

Django follows MTV (Model-Template-View) architecture with clear separation:

- **Models** - Data layer
- **Views** - Business logic and request handling
- **Templates** - Presentation layer
- **Services** - Extracted business logic
- **Utils** - Helper functions
- **Serializers** - Data serialization (DRF)

```
project/
├── app1/
│   ├── models.py
│   ├── views.py
│   ├── serializers.py
│   ├── urls.py
│   ├── services/
│   ├── migrations/
│   └── templates/
├── app2/
├── shared/
│   ├── utils.py
│   ├── decorators.py
│   └── permissions.py
├── manage.py
└── settings.py
```

## Pre-configured Rules

```json
{
  "max_lines_per_function": 50,
  "architecture_pattern": "MVC",
  "forbidden_imports": [
    {
      "from": "project/**/models.py",
      "to": "project/**/views.py",
      "reason": "Models should be independent of views"
    },
    {
      "from": "project/**/templates/**",
      "to": "project/**/models.py",
      "reason": "Templates should not import models directly"
    },
    {
      "from": "project/**/views.py",
      "to": "project/**/templates/**",
      "reason": "Views should render templates, not import them"
    },
    {
      "from": "project/shared/**",
      "to": "project/app*/**",
      "reason": "Shared utilities should not depend on specific apps"
    }
  ]
}
```

## Best Practices

- Keep models focused on data
- Extract business logic to services
- Use ViewSets with Django REST Framework
- Use serializers for data validation
- Organize apps by feature
- Use decorators for cross-cutting concerns
- Keep views/viewsets thin

## Common Issues

### Fat Models

Extract business logic to services:

```python
# services.py
class UserService:
    @staticmethod
    def create_user(email, password):
        # Business logic here
        pass

# views.py
def create_user(request):
    UserService.create_user(...)
```

### Circular Imports

Use late imports or abstract base classes:

```python
# Avoid circular imports with TYPE_CHECKING
from typing import TYPE_CHECKING
if TYPE_CHECKING:
    from .other_module import OtherClass
```
