
# Compiladores

## Front End
### Analizador lexico
Recibe el flujo de caracteres del archivo fuente
- asegura reglas de nombramiento de simbolos
- crea un flujo de tokens
###  Analizador sintactico 
- revisa sintaxis
- une los tokens en una estructura que represente al programa
- genera un arbol sintactico
### Analizador semantico
Busca errores que no rompan la sintaxis pero que no sean coherentes con el lenguaje
- regresa el mismo arbol sintactico que el analizador sintactico
### Generador de codigo intermedio
Crea una representacion intermedia (Grafos asicliclos definidos, Codigo de 3 caminos)

## Back End
### Optimizador independiente de la maquina 
Reduce ciclos, duplicaciones, etc.
### Generador de codigo 
En este punto ya se obtiene el codigo maquina destinio, es posible terminar la compilacion en este punto 
### Optimizador de codigo dependiente de la maquina
Opcionalmente se pueden aprovechar detalles de la arquitectura, cache o de mas hardware al que se esta compilando
