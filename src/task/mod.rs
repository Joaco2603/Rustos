//! Etapa 7 — tareas cooperativas.
//!
//! Cada tarea guarda su stack y cede la CPU de forma explícita.
//! La preempción llega cuando el handler del timer llama al mismo cambio de contexto.
