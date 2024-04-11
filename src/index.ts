import { shortenNumber, JBTC, JBTrading, DemandToValMul, JBItemDemand } from "./trading"
import readline from "node:readline"
const jbtc = new JBTC();
const jbtr = new JBTrading();
let tval = jbtc;
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});
let trade2 = [""];
// const items = ["beam"]
console.log("Using:", tval.name);
function ask() {
    rl.question("Items -> ", (str) => {
        let items = str.split(" ").map(v => v.replaceAll("_", " "));
        if (items[0] == "q") {
            console.log("quitting")
            rl.close();
            process.exit(0)
        } else if (items[0] == "@") {
            // if (items.length == 1) {
            //     trade2 = [];
            // }
            items.shift();
            console.log("Set new counter trade ->", items.join(","));
            trade2 = items;
            return ask()
        } else if (items[0] == "$") {
            if (tval == jbtr) {
                tval = jbtc;
            } else tval = jbtr
            console.log("Now using:", tval.name);
            items.shift();
            return ask()
        }
        items = items.filter(v => {
            if (v.startsWith("/")) {
                const uid = v.replace("/", "");
                console.log(`User "${uid}" is ${jbtc.isDuped(uid) ? "duped" : "clean"}`);
                return false;
            }
            return true;
        })
        console.log("JBTC ->", shortenNumber(jbtc.calcValueFromArr(items)));
        console.log("JBTR ->", shortenNumber(jbtr.calcValueFromArr(items)));
        if (trade2.length !== 0) {
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