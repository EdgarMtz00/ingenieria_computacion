
# Recursion por la izquierda

A -> Aa|b

## Como eliminarla
A -> bR
R -> aR|∅

## Ejemplo
expr -> expr + term | expr - term | term
### Eliminacion
expr -> term resto
resto -> + term resto | - term resto | ∅

      expr
    term  resto
    4     -   term  resto
              3     +   term  resto
                        6     +   term  resto
                                  9     ∅
