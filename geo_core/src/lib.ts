// Calculates the measure of a missing angle in a polygon given the number of sides and the known angles.
export function findMissingAngle(numSides: number, knownAngles: number[]): number {
    // Precompute the total sum of interior angles
    const totalSum = (numSides - 2) * 180;

    // Use a for loop for summing up the known angles (slightly faster than reduce)
    let knownSum = 0;
    for (let i = 0; i < knownAngles.length; i++) {
        knownSum += knownAngles[i];
    }

    // Calculate the missing angle
    return totalSum - knownSum;
}

// Rounds a number to a specified number of decimal places
export function roundTo(num: number, decimals: number): number {
    const factor = 10 ** decimals; // Use exponentiation operator for better readability
    return Math.round(num * factor) / factor;
}

// Calculates the number of sides of a polygon given the total interior angle sum
export function calculatePolygonSides(totalInteriorAngleSum: number): number {
    return totalInteriorAngleSum / 180 + 2;
}

// Example usage
const knownAngles = [108.0, 121.0, 59.0];
const numSides = 4;
const missingAngle = findMissingAngle(numSides, knownAngles);
console.log(`The missing angle is: ${missingAngle}`); // Output: The missing angle is: 72.0