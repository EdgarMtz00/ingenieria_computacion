
# Analisis sintactico predictivo descendente

Cada simbolo no terminal se convierte en una funcion que recibe atributos heredaros y retorna los atributos del simbolo

## Ejemplo (siguiendo [Ejemplo de recursion izquierda](./recursion_izquierda.md))

```python
def expr():
  term()
  resto()

def resto():
  if preanalisis == '-' or preanalisis == '+':
    coincidir(preanalisis)
    term()
    resto()
 
def term():
  if preanalisis == '0':
    coincidir(0)
  elif preanalisis == '1':
    coincidir(1)
  ###
  # . . .
  ###
  elif preanalisis == '9':
    coincidir(9)
  else:
    # Lanzar excepcion
```
