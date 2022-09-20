
# Atributos

## Atributos semanticos
En la implementacion son el retorno de las funciones

expr -> expr + term - expr.t = expr.t || term.t || '+'

## Atributos heredaros
En la implementacion son los atributos de las funciones

expr -> term resto - resto.her = term - expr.t = resto.t
resto -> + term resto1 - resto1.her = resto.her || term.t || '+' - resto.t = resto1.t
resto ->  ∅ - resto.t = resto.her
