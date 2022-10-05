function isPrime(n) {
    if (n === 1) return false
    if (n === 2) return true

    for (let divisor = 2; divisor <= n/2; divisor += 1)
        if (n % divisor === 0) return false

    return true
}

const number = 600851475143
let n = number
let factors = []

while (!isPrime(n)) {
    for (let divisor = 2; divisor <= n/2; divisor += 1) {
        if (isPrime(divisor) && n % divisor == 0) {
            console.log(divisor, "is prime and divides", n, "evenly")
            factors.push(divisor)
            n = n / divisor
            break
        }
    }
}

factors.push(n)
console.log("Prime factors of", number, ":", factors)

