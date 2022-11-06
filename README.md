# Introducción a Rust 

## Instalación

<br>

### Rust
Para instalar [Rust](https://www.rust-lang.org/learn/get-started) es necesario ingresar a la página oficial y abrir la terminar y ejecutar el siguiente comando.

<code>
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
</code>

<br>  

### Visual Studio Code + Rust Analyzer

Instala Visual Studio Code usandro <b>brew</b> o desde la página [oficial](https://code.visualstudio.com/). 

<code>
brew install visual-studio-code
</code>

<br>  

Acontinuación es aconsejable instalar la extensión [analyzer](
https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) descargandola desde el gestor de extensiones de Visual Studio Code o desde el el siguiente [link](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

<br>

## Crear el primer proyecto

Usaremos la herramienta <b>cargo</b>, similar al <b>npm</b> de nodejs, es una herramienta para manejar package y ejecutar el código, viene en la instalación de <b>Rust</b> y no es necesario una instalación adicional.

<code>
cargo init suma
</code>

<br>

La estructura del proyecto quedará de esta manera. 

```
suma
├── src         // acá estarán todos los fuentes
│   ├── main.rs // archivo main 
├── target      // carpeta con varias cosas que no tocaremos
├── .gitignore
├── Cargo.lock 
└── Cargo.toml  // archivo con las dependencias del proyecto
```

## Ejecutar el Hola Mundo

Al crear el proyecto, en el archivo <b>main.rs</b> se genera un hola mundo, como template, para ejecutarlo solo hay que posicionarse en el proyecto con la terminal (dentro de la carpeta suma) y ejecutar el siguiente comando. 

<code>
cargo run
</code>

<br>

## Manos a la obra

1.- Lo primero que haremos es crear 2 funciones que sumarán 2 números, ¿por qué? solo para mostrar las 2 formas que tiene <b>Rust</b> de retornar datos.
<br>
2.- El segundo paso es separ las funciones en un archivo aparte, creando un <b>módulo</b> y llamarlo desde el <b>main.rs</b> 
<br>
3.- Lo Siguiente que vamos a hacer es agregar una dependencia, para mostrar la hora UTC y la hora local con chrono. 


```
suma
├── src                          // root folder
│   ├── main.rs                  // main start the program
│   ├── controller 
│   │   ├── mod.rs 
│   │   ├── hello_controller.rs 
│   │   ├── time_controller.rs 
│   │   └── sum_controller.rs 
│   ├── models 
│   │   ├── mod.rs 
│   │   ├── sum_request.rs 
│   │   └── sum_response.rs 
│   ├── service 
│   │   ├── mod.rs 
│   │   ├── timenow_service.rs 
│   │   └── sum_service.rs 
├── target                      // carpeta con varias cosas que no tocaremos
├── .gitignore
├── Cargo.lock 
└── Cargo.toml                  // archivo con las dependencias del proyecto
```