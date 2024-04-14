import { shortenNumber, JBTC, JBTrading, DemandToValMul, JBItemDemand, DemandTostr } from "./trading"
import readline from "node:readline"
const jbtc = new JBTC();
const jbtr = new JBTrading();
let tval = jbtc;
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});
let trade2 = [""];
let prev_items: string[] = [];

function removeAt<V>(arr: V[], index: number) {
    if (index > arr.length - 1 || index < 0) return arr;
    for (let i = 0; i < arr.length; i++) {
        if (i >= index) {
            arr[i] = arr[i + 1];
        }
    }
    arr.pop();
    return arr;
}

// const items = ["beam"]
console.log("Using:", tval.name);
function ask() {
    rl.question("Items -> ", (str) => {
        let items = str.split(" ").map(v => v.replaceAll("_", " "));
        items = items.filter((v, i) => {
            switch (v) {
                case "q":
                    console.log("quitting")
                    rl.close();
                    process.exit(0)
                case "!":
                    removeAt(items, i)
                    if (items.length == 0) {
                        console.log("Cleared counter trade");
                    } else {
                        console.log("Set new counter trade ->", items.join(","));
                    }
                    trade2 = items;
                    return ask()
                case "@":
                    if (tval == jbtr) {
                        tval = jbtc;
                    } else tval = jbtr
                    console.log("Swapped to:", tval.name);
                    items[i] = ""
                    return ask()
                case "?":
                    items[i] = ""
                    console.log("Info about ->", items.join(","));
                    const jbitems = tval.itemSearchFromArr(items);
                    jbitems.forEach(v => {
                        console.log(v.name, shortenNumber(v.value), DemandTostr[v.demand], v.duped_value ?? `none`);
                    })
                    return false;
                case "^":
                    // console.log("cooo");
                    items = items.concat(prev_items);
                    console.log("New items ->", items.join(","))
                    return false;
            }

            if (v.startsWith("/")) {
                const uid = v.replace("/", "");
                console.log(`User "${uid}" is ${jbtc.isDuped(uid) ? "duped" : "clean"}`);
                return false;
            }
        })

        // items = items.filter(v => {
        //     if (v.startsWith("/")) {
        //         const uid = v.replace("/", "");
        //         console.log(`User "${uid}" is ${jbtc.isDuped(uid) ? "duped" : "clean"}`);
        //         return false;
        //     }

        //     return true;
        // })
        items.forEach((v, i) => {
            const start = v.split(" ")[0]
            if (start.endsWith("x") && Number(start.replace("x", ""))) {
                const count = Number(start.replace("x", ""));
                const a = v.split(" ");
                a.shift()
                items[i] = a.join(" ");
                for (let i = 1; i < count; i++) {
                    items.push(a.join(" "));
                }
            }
        })
        prev_items = items;
        console.log("JBTC ->", shortenNumber(jbtc.calcValueFromArr(items)));
        console.log("JBTR ->", shortenNumber(jbtr.calcValueFromArr(items)));
        if (trade2.length !== 0 && items.length !== 0) {
            const margin = 0.1;
            const trade1 = items

            const sum1 = tval.calcValueFromArr(trade1)
            const sum2 = tval.calcValueFromArr(trade2)
            const dem1 = tval.calcAvgDemandFromArr(trade1);
            const dem2 = tval.calcAvgDemandFromArr(trade2)
            const avg1 = sum1 * (4 + DemandToValMul[Math.floor(dem1) as JBItemDemand]);
            const avg2 = sum2 * (4 + DemandToValMul[Math.floor(dem2) as JBItemDemand]);
            let result = Number(((avg2 - avg1) / avg1).toFixed(2));
            console.log(result, result > margin ? (result > 2 * margin ? "Flip" : "Would") : result < -margin ? result <= -2 * margin ? "Big L" : "L" : "Fair", shortenNumber(sum2 - sum1), "profit", shortenNumber(sum1 * result));
        }

        ask()
    })
}



ask()