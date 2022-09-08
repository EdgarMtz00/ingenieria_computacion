#include <stdio.h>
#include <stdlib.h>

const int CANTIDAD_NUMEROS = 10;

int main()
{
    int vector[CANTIDAD_NUMEROS];
    int vectorResultante[CANTIDAD_NUMEROS];
    int multiplicador;

    for(int i = 0; i < CANTIDAD_NUMEROS; i ++){
        printf("Ingrese el elemento #%d: ", i + 1);
        scanf("%d", &vector[i]);
    }

    printf("Ingrese el multiplicador: ");
    scanf("%d", &multiplicador);

    for(int i = 0; i < CANTIDAD_NUMEROS; 1++){
        vectorResultante[i] = vector[i] * multiplicador;
        printf("El resultado #%d es: %d\n", i + 1, vectorResultante[i]);
    }
    return 0;
}
