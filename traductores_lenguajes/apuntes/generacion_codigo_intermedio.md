
# Generacion de codigo intermedio

## Comprobador estatico
Despues de realizar el analisis sintactico se pasa a esta fase.
En esta fase se comprueba que los tipos de datos declarados y/o devueltos por funciones sean correctos, tambien que estructuras como el switch tengan todas sus sentencias necesarias

## Generador de codigo intermedio

### Grafo aciclico dirigido
Es una representacion intermedia similar a los arboles sintacticos

Ej. a + a * (b - c) + (b - c) * d 
Produccion      | Semanticas 
E -> E + T        E.nodo = new Nodo('+', E.nodo, T.nodo) 
E -> E - T        E.nodo = new Nodo('-', E.nodo, T.nodo)
E -> T            E.nodo = T.nodo
T -> (E)          T.nodo = E.nodo
T -> id           T.nodo = new Hoja(id, id.entrada)
T -> num          T.nodo = new Hoja(num, num.val)

