let multiplesOfThree = []
let multiplesOfFive = []

// calculate multiples of 3 under 1000
for (let i = 3; i < 1000; i += 3) {
    multiplesOfThree.push(i)
}

// calculate multiples of 5 under 1000
for (let i = 5; i < 1000; i += 5) {
    multiplesOfFive.push(i)
}

// merge the arrays
// (sets contain no duplicates)
let mOfThreeAndFive = new Set([...multiplesOfThree, ...multiplesOfFive])

// sum the elements
const sum = [...mOfThreeAndFive].reduce((all, item) => all + item, 0)

console.log("Sum of all the multiples of 3 or 5 below 1000:", sum)
