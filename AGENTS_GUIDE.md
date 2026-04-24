# 🤖 Guía Maestra de Agentes en Vibe-AI

Esta documentación detalla la arquitectura, creación y operación de **Agentes Autónomos Especializados** dentro del ecosistema `vibe-ai`. Un agente en este framework no es simplemente un chat con un prompt; es una unidad operativa con capacidad de cómputo, acceso a sistemas de archivos y colaboración inter-agente.

---

## 🏗️ 1. Arquitectura de un Agente Profesional

En `vibe-ai`, la inteligencia de un agente se desacopla en tres capas concurrentes:

1.  **Capa de Razonamiento (The Brain):** Definida en `index.yaml`. Utiliza modelos de lenguaje (LLM) para procesar lógica compleja y toma de decisiones.
2.  **Capa de Ejecución (The Muscle):** Ubicada en el directorio `bin/`. Son scripts (Bash, Python, Rust, etc.) que ejecutan las acciones reales en el sistema operativo.
3.  **Capa de Datos y Contexto (The Memory):** Gestionada a través de variables de entorno, RAG (Retrieval Augmented Generation) y archivos de configuración.

### Estructura de Archivos Estándar
Cada agente reside en su propio espacio de nombres dentro de la configuración de `vibe-ai`:
`~/.config/vibe-ai/functions/agents/<nombre-agente>/`

```text
.
├── index.yaml       # Manifiesto, metadatos e instrucciones (Prompt Engineering)
├── functions.json   # Definición de capacidades (Herramientas / Function Calling)
├── config.yaml      # (Opcional) Overrides de modelo y parámetros por defecto
├── bin/             # Scripts ejecutables que implementan las herramientas
│   └── <nombre>     # Script principal (el "Handler")
└── rag/             # (Opcional) Documentación técnica indexable (NIST, ISO, etc.)
```

---

## 🛡️ 2. Caso de Estudio: `vibe-arq`

Este agente combina un rol de **Cybersecurity Architect** con la capacidad de persistir y gestionar artefactos técnicos (ADRs, Políticas, Diagramas).

### A. El Manifiesto (`index.yaml`)
Aquí definimos cómo el agente debe comportarse y qué variables necesita.

```yaml
name: vibe-arq
version: 1.1.0
description: Arquitecto Senior con capacidades de gestión documental y diseño de seguridad.

# El prompt utiliza interpolación de variables para ser flexible
instructions: |
  Eres un Cybersecurity Architect Senior (10+ años de experiencia).
  Tu misión es diseñar sistemas seguros por defecto.
  
  CONTEXTO OPERATIVO:
  - Estás operando en el directorio: {{WORK_DIR}}
  - Debes basar tus recomendaciones en el Rol Maestro de Seguridad proporcionado.
  
  FLUJO DE TRABAJO OBLIGATORIO:
  1. Identificar el requerimiento del usuario.
  2. Consultar el estado del proyecto (usar herramienta 'fs_manager' con op='list').
  3. Generar el artefacto técnico (ADR, Markdown, Mermaid).
  4. Persistir el resultado (usar 'fs_manager' con op='write').

  {{__tools__}}

variables:
  - name: WORK_DIR
    description: "Ruta absoluta al directorio de trabajo del proyecto."
    default: "./cyber-workspace"
  - name: SECURITY_LEVEL
    description: "Nivel de criticidad (LOW, MEDIUM, HIGH, MISSION_CRITICAL)."
    default: "HIGH"
```

### B. Definición de Capacidades (`functions.json`)
Definimos el "Contrato de Interfaz". El LLM usará este JSON para saber cómo invocar tus scripts.

```json
[
  {
    "name": "fs_manager",
    "description": "Gestiona archivos de arquitectura en el almacenamiento local.",
    "parameters": {
      "type": "object",
      "properties": {
        "op": { 
          "type": "string", 
          "enum": ["list", "read", "write", "delete"],
          "description": "Operación de sistema de archivos a realizar."
        },
        "path": { "type": "string", "description": "Nombre del archivo (ej. ADR-001.md)" },
        "data": { "type": "string", "description": "Contenido a escribir (requerido para 'write')" }
      },
      "required": ["op"]
    },
    "agent": true
  }
]
```

### C. El Ejecutor Logístico (`bin/vibe-arq`)
Este script es invocado por `vibe-ai` cada vez que el LLM decide usar una herramienta.

```bash
#!/bin/bash
# Los agentes reciben dos argumentos: el nombre de la función y un JSON con los parámetros.
FUNCTION_NAME=$1
JSON_ARGS=$2

# Recuperar variables del agente desde el entorno (Prefijo: LLM_AGENT_VAR_)
BASE_DIR="${LLM_AGENT_VAR_WORK_DIR:-./workspace}"
mkdir -p "$BASE_DIR"

# Procesar la función 'fs_manager'
if [ "$FUNCTION_NAME" == "fs_manager" ]; then
    OP=$(echo "$JSON_ARGS" | jq -r .op)
    FILE_PATH=$(echo "$JSON_ARGS" | jq -r .path)
    
    case "$OP" in
        list)
            # Retornar lista de archivos al LLM mediante el archivo temporal $LLM_OUTPUT
            ls -1 "$BASE_DIR" | jq -R . | jq -s . > "$LLM_OUTPUT"
            ;;
        write)
            CONTENT=$(echo "$JSON_ARGS" | jq -r .data)
            echo "$CONTENT" > "$BASE_DIR/$FILE_PATH"
            echo "{\"status\": \"success\", \"message\": \"Archivo '$FILE_PATH' creado en $BASE_DIR\"}" > "$LLM_OUTPUT"
            ;;
        read)
            if [ -f "$BASE_DIR/$FILE_PATH" ]; then
                cat "$BASE_DIR/$FILE_PATH" > "$LLM_OUTPUT"
            else
                echo "{\"error\": \"Archivo no encontrado\"}" > "$LLM_OUTPUT"
            fi
            ;;
    esac
fi
```

---

## 🚀 3. Optimización Avanzada

### Instrucciones Dinámicas (`dynamic_instructions`)
Puedes hacer que el prompt de tu agente cambie en tiempo real.
1. En `index.yaml`, establece `dynamic_instructions: true`.
2. En tu script `bin/`, implementa el comando `_instructions`.
3. `vibe-ai` ejecutará `mi-agente _instructions` antes de cada sesión para obtener el prompt fresco (útil para inyectar la hora actual, estado de servidores o logs recientes).

### Inyección de Variables Globales
Puedes usar variables del sistema dentro de tus prompts:
- `{{current_shelf}}`: Referencia al almacenamiento actual.
- `{{__tools__}}`: Se expande automáticamente con la lista de funciones de `functions.json`.

### Seguridad (Sandboxing)
Aunque los agentes corren con los permisos de tu usuario en Termux/Linux, es buena práctica:
- Validar los nombres de archivo en los scripts para evitar ataques de *Path Traversal* (ej. `../etc/passwd`).
- Usar rutas absolutas definidas en variables de entorno.

---

## 🤝 4. Colaboración Multi-Agente

La verdadera potencia de `vibe-ai` surge cuando los agentes colaboran.

**Estrategia de Orquestación:**
Puedes crear un agente "Líder de Proyecto" que, dentro de sus herramientas, tenga scripts que ejecuten:
`vibe-ai --agent vibe-arq "Genera un reporte de seguridad para la red X"`

Esto permite:
1.  **Arquitecto:** Diseña la solución.
2.  **Auditor:** Revisa el diseño del arquitecto.
3.  **Implementador:** Crea los scripts de despliegue basados en el diseño aprobado.

---

## 🛠️ 5. Guía de Operación (Workflow Profesional)

Para iniciar una sesión de arquitectura de alta fidelidad:

1.  **Cargar el Rol Maestro:**
    ```bash
    SECURITY_ROLE=$(cat ~/SyntheticMind/assets/roles/%code%.md)
    ```

2.  **Lanzar el Agente con Inyección de Contexto:**
    ```bash
    vibe-ai --agent vibe-arq \
            ROLE_PROMPT="$SECURITY_ROLE" \
            WORK_DIR="/sdcard/Documents/Proyectos/Fintech-App" \
            SECURITY_LEVEL="MISSION_CRITICAL"
    ```

3.  **Interacción de Ejemplo:**
    *   *Usuario:* "Arquitecto, revisa si tenemos una política de contraseñas y, si no, crea una alineada a NIST 800-63B."
    *   *Agente:* (Internamente lista archivos -> no encuentra la política -> redacta el documento -> llama a `fs_manager` -> confirma la creación).

---

## 📂 6. Resumen de Mejores Prácticas

| Característica | Recomendación |
| :--- | :--- |
| **Variables** | Usa valores por defecto (default) en `index.yaml` para evitar errores de inicio. |
| **Scripts** | Usa `jq` para parsear los argumentos JSON; es más seguro que `grep` o `sed`. |
| **Documentos** | Usa la carpeta `rag/` para manuales pesados en lugar de ponerlos en el prompt. |
| **Salida** | Siempre escribe la respuesta de la función en `$LLM_OUTPUT`. |
| **Modularidad** | Crea un agente por especialidad en lugar de uno solo "que haga todo". |

---
*Documentación generada para el ecosistema Vibe-AI - 2026*
