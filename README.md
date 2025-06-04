# Envite Project

[Español](#español) | [English](#english)

## Español

# Proyecto Envite - Sistema de Pencas

## ¿Qué es Envite?

Envite es una plataforma para la gestión de "pencas" (quinielas o pronósticos deportivos), donde los participantes pueden competir prediciendo resultados de partidos y siguiendo a sus equipos y jugadores favoritos.

## Estado Actual del Proyecto

### Desarrollado hasta ahora:

- **Modelo de dominio**: Se ha implementado la estructura básica del modelo de dominio con los siguientes componentes:
  - `Penca`: Entidad principal que gestiona participantes, partidos y reglas de puntuación.
  - `Participant`: Representa a los usuarios que participan en la penca.
  - `Match`: Representa los partidos con sus equipos, resultados y tipos.
  - `Team`: Representa los equipos deportivos.
  - `Player`: Representa a los jugadores deportivos.
  - `League`: Representa las ligas o competiciones.
  - `ScoringRules`: Sistema de reglas para la puntuación según diferentes eventos.

- **Formatos de Penca**:
  - Modo Tradicional
  - Modo Club
  - Modo Mixto

- **Sistema de puntuación**: Implementación de reglas para diferentes escenarios:
  - Resultados de partidos de clubes
  - Eventos de jugadores amigos
  - Resultados de partidos predichos
  - Eventos de jugadores seleccionados

- **Tests de integración**: Prueba básica end-to-end que demuestra el flujo completo.

### Pendiente por desarrollar:

- **Backend**:
  - API REST o GraphQL para interactuar con el modelo de dominio
  - Persistencia de datos (base de datos)
  - Autenticación y autorización de usuarios
  - Lógica de negocio para la gestión de pencas en tiempo real

- **Frontend**:
  - Interfaz de usuario completa
  - Páginas para creación y gestión de pencas
  - Visualización de clasificaciones y resultados
  - Perfil de usuario y configuraciones

- **Funcionalidades adicionales**:
  - Notificaciones en tiempo real
  - Integración con APIs externas para obtener resultados deportivos
  - Sistema de invitaciones y compartir
  - Estadísticas avanzadas y análisis

- **Infraestructura**:
  - Despliegue en producción
  - CI/CD
  - Monitorización y logging

## Tecnologías

El proyecto está desarrollado en Rust, con una estructura de workspace que incluye:
- `shared`: Biblioteca compartida con el modelo de dominio
- `backend`: Servidor backend (por implementar)
- `frontend`: Cliente frontend (por implementar)

---

## English

# Envite Project - Sports Prediction System

## What is Envite?

Envite is a platform for managing "pencas" (sports prediction pools), where participants can compete by predicting match results and following their favorite teams and players.

## Current Project Status

### Developed so far:

- **Domain model**: The basic structure of the domain model has been implemented with the following components:
  - `Penca`: Main entity that manages participants, matches, and scoring rules.
  - `Participant`: Represents users participating in the prediction pool.
  - `Match`: Represents matches with their teams, results, and types.
  - `Team`: Represents sports teams.
  - `Player`: Represents sports players.
  - `League`: Represents leagues or competitions.
  - `ScoringRules`: System of rules for scoring based on different events.

- **Penca Formats**:
  - Traditional Mode
  - Club Mode
  - Mixed Mode

- **Scoring system**: Implementation of rules for different scenarios:
  - Club match results
  - Friend player events
  - Predicted match results
  - Selected player events

- **Integration tests**: Basic end-to-end test demonstrating the complete flow.

### Pending development:

- **Backend**:
  - REST or GraphQL API to interact with the domain model
  - Data persistence (database)
  - User authentication and authorization
  - Business logic for real-time penca management

- **Frontend**:
  - Complete user interface
  - Pages for creating and managing pencas
  - Display of rankings and results
  - User profile and settings

- **Additional features**:
  - Real-time notifications
  - Integration with external APIs for sports results
  - Invitation and sharing system
  - Advanced statistics and analysis

- **Infrastructure**:
  - Production deployment
  - CI/CD
  - Monitoring and logging

## Technologies

The project is developed in Rust, with a workspace structure that includes:
- `shared`: Shared library with the domain model
- `backend`: Backend server (to be implemented)
- `frontend`: Frontend client (to be implemented)
