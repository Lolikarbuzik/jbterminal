async function main() {
    await import("./jbtc/parser")
    await import("./jbtrading/parser")
    await import("./dupes/parser")
    console.log("Parsing...")
}

main()