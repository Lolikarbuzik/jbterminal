// Use this to parse jbtrading values and demands
import { JBItemDemand, StrToJBDemand, type JBItem } from "../types";
import { writeFileSync } from "node:fs"
import { parse } from 'node-html-parser';

const final: JBItem[] = [];

const MIN_VALUE = 100_000;

async function check(section: string) {
    const url = "https://www.jailbreaktrading.net/" + section;
    const body = await (await fetch(url)).text();
    const root = parse(body);
    root.querySelectorAll("div.tyJCtd.mGzaTb.Depvyb.baZpAe").forEach(el => {
        if (el.innerText.includes("Copyright")) return;
        const texts = el.getElementsByTagName("p").map(v => v.text);
        if (texts.length < 8) {
            return;
        }
        // Keep the replaces in the order
        let name = texts[0].replace("Level", "L").replace(" V", "5").replace(" IV", "4").replace(" III", "3").replace(" II", "2");
        if (name.indexOf("I") === name.length - 1) {
            name = name.replace(" I", "1");
        }
        const demand = StrToJBDemand(texts[2].replace("Demand: ⭐ ", ""));
        const duped_value = Number(texts[6].replace("Duped Item Value: ⚠️", "").replaceAll(",", "").replace("$", ""));
        const value = Number(texts[7].replace("JailbreakTrading.net: 💸 ", "").replace("FREE!", "").replaceAll(",", "").replace("$", ""));
        if (value < MIN_VALUE) return;
        final.push({
            name,
            demand,
            duped_value,
            value
        });
        // console.log(name, demand, duped_value, value);
    })
    console.log("[JBTR] Completed", section);
}

async function main() {
    await check("vehicles");
    await check("textures");
    await check("colors");
    await check("spoilers");
    await check("rims");
    await check("other-values/furniture");
    await check("other-values/gun-textures");
    await check("other-values/vehicle-horns");
    await check("other-values/drift-particles");

    writeFileSync("src/trading/jbtrading/values.json", JSON.stringify(final, null, 2));
    console.log("[JBTR] Saved");
}

main()