#!/bin/bash
# Generate a large fixture project with 50 files for stress testing

FIXTURE_DIR="./tests/fixtures/large-project"
mkdir -p "$FIXTURE_DIR"

# Create architect.json
cat > "$FIXTURE_DIR/architect.json" << 'EOF'
{
  "version": "1.0",
  "rules": [
    {
      "from": "services",
      "to": "controllers",
      "message": "Services should not import from controllers"
    }
  ]
}
EOF

# Generate 50 service files
for i in {1..50}; do
  mkdir -p "$FIXTURE_DIR/services"
  cat > "$FIXTURE_DIR/services/service_$i.ts" << EOF
export class Service$i {
  doWork() {
    return 'work from service $i';
  }
}
EOF
done

# Generate 50 controller files
for i in {1..50}; do
  mkdir -p "$FIXTURE_DIR/controllers"
  cat > "$FIXTURE_DIR/controllers/controller_$i.ts" << EOF
import { Service$i } from '../services/service_$i';

export class Controller$i {
  private service = new Service$i();

  handle() {
    return this.service.doWork();
  }
}
EOF
done

echo "Generated large project fixture with 100+ files"
