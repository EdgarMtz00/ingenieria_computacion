class Main{
    public static void main(String[] args) {
        int a = 1;
        int b = 2;
        int c = 3;
        int temporal;

        imprimirVariables(a, b, c);

        temporal = a;
        a = c;
        c = b;
        b = temporal;

        imprimirVariables(a, b, c);
    }

    public static void imprimirVariables(int a, int b, int c){
        System.out.printf("El valor de A es: %d\n", a);
        System.out.printf("El valor de B es: %d\n", b);
        System.out.printf("El valor de C es: %d\n\n", c);
    }
}