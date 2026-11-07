# Aprendiendo Rust 🦀

Este repositorio es mi **diario de aprendizaje de Rust**. Aquí guardo los pequeños programas que voy escribiendo mientras aprendo el lenguaje, ordenados por tema.

No es un proyecto terminado ni una librería: es un registro de mis progresos. El código va creciendo a medida que avanzo, así que algunos ejemplos son muy básicos y otros pueden tener cosas mejorables.

## Contenido

| Tema | Programas | Qué practico |
|---|---|---|
| [PrimerPrograma](PrimerPrograma/) | `HelloWorld`, `Prints`, `PrintConVariable` | Primer programa, macros `println!` / `print!` e impresión con variables |
| [VariablesMutabilidad](VariablesMutabilidad/) | `Constante`, `bloques`, `shadowing`, `vidas` | Constantes, `let` y `let mut`, ámbitos de bloque y *shadowing* |
| [TiposDatos](TiposDatos/) | `as`, `Desestructura`, `ErrorTipoDato`, `Indices` | Conversiones con `as`, desestructuración de tuplas, tipos numéricos e índices |

## Estructura

Cada programa tiene su propia carpeta con tres archivos:

```
Tema/Programa/
├── programa.rs    # código fuente
├── programa.exe   # ejecutable compilado
└── programa.pdb   # símbolos de depuración (Windows)
```

## Cómo compilar y ejecutar

Solo hace falta tener [Rust instalado](https://www.rust-lang.org/tools/install). Desde la carpeta del programa:

```sh
rustc programa.rs
./programa.exe
```

## Progreso

- [x] Primer programa e impresión por pantalla
- [x] Variables, mutabilidad y *shadowing*
- [x] Tipos de datos básicos
- [ ] Funciones
- [ ] Control de flujo
- [ ] *Ownership* y *borrowing*
- [ ] Structs y enums
- [ ] Cargo y proyectos

---

*Repositorio personal de aprendizaje. Se irá actualizando a medida que avance.*
