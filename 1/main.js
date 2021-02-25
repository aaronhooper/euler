// calculate multiples of 3 under 1000
let multiplesOfThree = []

for (let i = 3; i < 1000; i += 3) {
    multiplesOfThree.push(i)
}

// calculate multiples of 5 under 1000
let multiplesOfFive = []

for (let i = 5; i < 1000; i += 5) {
    multiplesOfFive.push(i)
}

// merge the two arrays
let mOfThreeAndFive = new Set([...multiplesOfThree, ...multiplesOfFive])

// add them together
const sum = [...mOfThreeAndFive].reduce((all, item) => all + item, 0)

console.log("Sum of all the multiples of 3 or 5 below 1000:", sum)
