// create array of fibonacci numbers below 4 million
const LIMIT = 4000000
let fibonacci = [1, 2]
let limitReached = false

while (!limitReached) {
    const last = fibonacci[fibonacci.length - 1]
    const secondLast = fibonacci[fibonacci.length - 2]
    const next = last + secondLast
    
    if (next <= LIMIT) {
        fibonacci.push(next)
    } else {
        limitReached = true
    }
}

console.log(fibonacci)

// sum each even number in array

const sum = fibonacci.reduce((all, item) => {
    if (item % 2 == 0) {
        return all + item
    }

    return all
}, 0)

console.log("The sum of the even-valued fibonacci terms:", sum)

