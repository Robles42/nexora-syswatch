# Nexora-Syswatch | High-Performance TUI Monitor

Nexora-Syswatch es un monitor de recursos del sistema desarrollado en Rust, diseñado para ofrecer métricas en tiempo real con un consumo mínimo de recursos. Forma parte del ecosistema de herramientas de alto rendimiento Nexora.

## Caracteristicas Principales

* Monitoreo en Tiempo Real: Visualizacion de carga de CPU y uso de memoria RAM mediante indicadores dinamicos.
* Deteccion de Procesos Criticos: Listado automatico de los 5 procesos que mas memoria RAM consumen en el sistema.
* Alertas Visuales: Cambio dinamico de colores cuando el uso de CPU supera el 80%.
* Arquitectura de Bajo Nivel: Construido sobre Rust para garantizar seguridad de memoria y velocidad de ejecucion.
* Interfaz TUI: Interfaz de usuario basada en terminal utilizando la libreria Ratatui.

## Stack Tecnologico

* Lenguaje: Rust (v1.70+)
* Librerias:
    * ratatui: Renderizado de la interfaz de terminal.
    * sysinfo: Extraccion de metricas de hardware y procesos.
    * crossterm: Manejo de eventos de teclado.

## Instalacion y Uso

### Requisitos Previos
Es necesario contar con el toolchain de Rust instalado:
```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh