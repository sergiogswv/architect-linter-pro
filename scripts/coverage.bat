@echo off
REM Coverage report generation script for Windows

echo Running coverage report...
cargo tarpaulin --verbose --all-features --timeout 300 --out xml

echo.
echo Coverage by module:
echo   - Run: cargo tarpaulin --exclude-files -- --test
echo   - Report: cobertura.xml generated
echo.
echo To upload to Codecov (if configured):
echo   - Run: codecov -f cobertura.xml
