export function shortenNumber(number: number): string {
    const suffixes = ["", "k", "m", "b", "t"]; // Add more suffixes as needed
    const tier = Math.log10(Math.abs(number)) / 3 | 0;

    if (tier === 0) return number.toString();

    const suffix = suffixes[tier];
    const scale = Math.pow(10, tier * 3);

    const scaledNumber = number / scale;

    // Check if the scaled number has no decimal part
    if (scaledNumber % 1 === 0) {
        return scaledNumber.toString() + suffix;
    } else {
        // Format the number with up to 1 decimal places
        const formattedNumber = scaledNumber.toFixed(1);
        return formattedNumber + suffix;
    }
}

export function name_count(name: string, search: string): number {
    name = name.toLowerCase();
    if (search.length > name.length) {
        return -1;
    }
    let count = 0;
    let str = "";
    for (let i = 0; i < search.length; i++) {
        if (name.startsWith(str)) {
            count += 2;
        } else if (name.includes(str)) {
            count++;
        }
        str += search[i];
    }
    // console.log(`Score ${count} for '${name}' with ${search}`)
    return count;
}