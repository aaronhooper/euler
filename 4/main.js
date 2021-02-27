function isPalindrome(n) {
    const nStr = n.toString()
    let a = ''
    let b = ''
    
    if (nStr.length % 2 !== 0) {
        a = nStr.substring(0, Math.floor(nStr.length / 2))
        b = nStr.substring(Math.ceil(nStr.length / 2))
    } else {
        a = nStr.substring(0, nStr.length / 2)
        b = nStr.substring(nStr.length / 2)
    }

    if (a === reverse(b)) return true
    return false
}

function reverse(str) {
    return str.split('').reverse().join('')
}

const numbers = [
    2002,
    30003,
    100,
    9876
]

numbers.forEach(number => {
    console.log('Is number', number, 'a palindrome?', isPalindrome(number))
})

let largestPalindrome = 0

for (let i = 100; i <= 999; i++)
    for (let j = 100; j <= 999; j++)
        if (isPalindrome(i*j) && i*j > largestPalindrome)
            largestPalindrome = i*j

console.log("The largest palindrome made from the product of two 3-digit numbers:", largestPalindrome)
