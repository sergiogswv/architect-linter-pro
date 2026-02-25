# Architect Linter Pro

<p align="center">
  <img src="./public/architect-linter-banner.png" alt="Banner Architect Linter Pro" width="100%">
</p>

<p align="center">
  <img src="https://img.shields.io/badge/versión-4.3.0-blue.svg" alt="Versión">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Edición Rust">
  <img src="https://img.shields.io/badge/licencia-MIT-green.svg" alt="Licencia">
  <img src="https://img.shields.io/badge/plataforma-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg" alt="Plataforma">
</p>

Un linter de arquitectura de software multi-lenguaje escrito en Rust que valida reglas arquitectónicas mediante un motor de reglas dinámicas. Soporta **4 lenguajes de producción: TypeScript, JavaScript, Python y PHP** usando Tree-sitter para análisis rápido y preciso.

## 📚 Documentación Completa

👉 **[Lee la documentación completa](https://architect-linter-pro.dev)**

### Enlaces Rápidos
- [Guía de Instalación](/docs/installation)
- [Primeros Pasos](/docs/getting-started)
- [Referencia de API](/docs/api-reference)
- [Plantillas](/docs/templates)
- [Solución de Problemas](/docs/troubleshooting)

## 🚀 Instalación Rápida

```bash
cargo install architect-linter-pro
architect --init
architect --check
```

## ✨ Características Principales

- 🌐 **Soporte Multi-Lenguaje**: TypeScript, JavaScript, Python, PHP
- 🔧 **Motor de Reglas Dinámico**: Define restricciones personalizadas via architect.json
- 🔍 **Detección de Dependencias Cíclicas**: Detección automática de ciclos
- 📦 **Validación de Importaciones**: Bloquea violaciones de arquitectura
- ⚡ **Procesamiento Paralelo**: Análisis ultrarrápido con Rayon
- 🏆 **Sistema de Health Score**: Métricas de calidad (Calificación A-F)
- 🤖 **Auto-Fix con IA**: Sugerencias y correcciones automáticas
- 👁️ **Modo Watch**: Monitoreo en tiempo real con notificaciones
- 👻 **Modo Daemon**: Monitoreo en segundo plano continuo

## 🤝 Contribuir

¡Las contribuciones son bienvenidas! Consulta nuestra [guía de contribución](https://architect-linter-pro.dev/docs/contributing).

## 📄 Licencia

MIT

---

**Idiomas:** [English](README.md) | Español
