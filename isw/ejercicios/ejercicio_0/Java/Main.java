class Main {
    public static void main(String[] args) {
        System.out.println("Edgar");
        System.out.println("Ingenieria en Computacion");
        System.out.println("Ceti Colomos");
        System.out.println("Me apasiona la computacion");
        System.out.println("C, C++, Java, Javascript, Python, Rust");
        fibonacciPrimes();
    }

    //print fibonacci numbers that are primes
    public static void fibonacciPrimes() {
        int a = 0;
        int b = 1;
        int c = 0;
        while (c < 100) {
            c = a + b;
            a = b;
            b = c;
            if (isPrime(c)) {
                System.out.println(c);
            }
        }
    }

    // print fibonacci series
    public static void fibonacci() {
        int a = 0;
        int b = 1;
        int c = 0;
        System.out.println(a);
        System.out.println(b);
        while (c < 100) {
            c = a + b;
            System.out.println(c);
            a = b;
            b = c;
        }
    }

    // print prime numbers
    public static void prime() {
        int n = 0;
        int i = 0;
        int j = 0;
        while (n < 100) {
            n = n + 1;
            i = 2;
            while (i < n) {
                j = n % i;
                if (j == 0) {
                    break;
                }
                i = i + 1;
            }
            if (j != 0) {
                System.out.println(n);
            }
        }
    }
}
