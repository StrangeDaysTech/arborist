# Estrategia de Ramas en Git

> **Documento de referencia para el flujo de trabajo en Git de este proyecto.**
> Para las reglas rápidas, consulta la sección de Operaciones Git en CLAUDE.md o GEMINI.md.

---

## Regla Crítica

**NUNCA hagas commit directamente a la rama `main`.**

Todos los cambios deben pasar por ramas feature/fix y Pull Requests. Esto asegura revisión de código, validación de CI y un historial de commits limpio.

---

## Convención de Nombres de Ramas

| Prefijo | Propósito | Ejemplo |
| --------- | ----------- | --------- |
| `feature/` | Nuevas funcionalidades o mejoras | `feature/export-excel` |
| `feat/` | Alias para feature | `feat/folio-c5` |
| `fix/` | Corrección de errores | `fix/report-form-tests` |
| `hotfix/` | Correcciones urgentes en producción | `hotfix/auth-bypass` |
| `docs/` | Cambios solo de documentación | `docs/api-reference` |
| `refactor/` | Refactorización de código (sin cambio de comportamiento) | `refactor/catalog-service` |
| `test/` | Cambios solo de pruebas | `test/bunit-coverage` |

---

## Flujo de Trabajo

1. **Antes de comenzar**, verifica la rama actual:

   ```bash
   git branch --show-current
   ```

2. **Crear rama** desde `main` actualizado:

   ```bash
   git checkout main
   git pull origin main
   git checkout -b fix/nombre-descriptivo
   ```

3. **Hacer commits** siguiendo el formato de commits convencionales

4. **Push y crear PR**:

   ```bash
   git push -u origin fix/nombre-descriptivo
   gh pr create --title "fix: descripción" --body "..."
   ```

5. **Merge vía PR** después de que CI pase - Nunca hacer push directamente a `main`

---

## Commits Convencionales

Usa prefijos semánticos en los mensajes de commit:

| Prefijo | Caso de Uso |
| --------- | ------------- |
| `feat:` | Nueva funcionalidad |
| `fix:` | Corrección de error |
| `docs:` | Solo documentación |
| `test:` | Agregar o corregir pruebas |
| `refactor:` | Cambio de código sin cambio de comportamiento |
| `chore:` | Mantenimiento, dependencias, configuración |
| `perf:` | Mejora de rendimiento |

---

## Recuperación: Si Accidentalmente Haces Commit en Main

Si se hicieron commits en `main` que deberían estar en una rama:

```bash
# Crear una nueva rama con los commits
git branch fix/commits-accidentales

# Resetear main para que coincida con origin
git reset --hard origin/main

# Cambiar a la nueva rama y hacer push
git checkout fix/commits-accidentales
git push -u origin fix/commits-accidentales
```

---

*DevTrail v1.0.0 | Última actualización: 2025-01-30*
*[Strange Days Tech](https://strangedays.tech) — Porque cada cambio cuenta una historia.*
