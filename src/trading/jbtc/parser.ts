//? This script parses the JBTC value list excel
//? DO NOT RUN THE PARSER SCRIPT IF YOU HAVENT INITIALIZED GOOGLE API KEY INSIDE "secrets.json"

import { google } from "googleapis";
import { JBItemDemand, StrToJBDemand, type JBItem } from "../types";
import { writeFileSync } from "node:fs"

const final: JBItem[] = [
    {
        name: "Volt Bike",
        value: 1_000_000,
        demand: JBItemDemand.Mid
    },
    {
        name: "Blackhawk",
        value: 1_000_000,
        demand: JBItemDemand.Mid
    },
    {
        name: "Trailblazer",
        value: 1_000_000,
        demand: JBItemDemand.Mid
    },
    {
        name: "Monster",
        value: 1_000_000,
        demand: JBItemDemand.Low
    },
    {
        name: "Jet",
        value: 1_000_000,
        demand: JBItemDemand.Mid
    },
    {
        name: "Drone",
        value: 1_000_000,
        demand: JBItemDemand.Mid
    }
];

type Response = string[]

async function parse(sheets: any, range: string, hyper?: boolean, format?: (id: string) => string) {
    const result = await sheets.spreadsheets.values.get({
        spreadsheetId: "12aPBmrHP5MLwoht9QcBiERPYUmXTJiTbBF43630togE",
        range: range
    });
    const values = result.data.values;
    values?.forEach(([id, value, duped_value, demand, notes]: Response) => {
        // console.log(id, value, duped_value, demand, notes);
        if (hyper && format) {
            final.push({
                name: id.trim() !== "Hypershift" ? format(id) : "Hypershift",
                value: Number(value.replaceAll(",", "")),
                // duped_value: Number(duped_value.replaceAll(",", "")),
                demand: StrToJBDemand(duped_value),
                notes: demand?.trim().length == 0 ? undefined : demand
            });
        } else {
            final.push({
                name: id,
                value: Number(value.replaceAll(",", "")),
                duped_value: Number(duped_value.replaceAll(",", "")),
                demand: StrToJBDemand(demand),
                notes: notes?.trim().length == 0 ? undefined : notes
            });
        }
    })
    console.log("[JBTC] Completed", range);
}

async function main() {
    const auth = await google.auth.getClient({
        scopes: ["https://www.googleapis.com/auth/spreadsheets.readonly"],
    });
    const sheets = google.sheets({ version: "v4", auth });
    await parse(sheets, "Value List!C20:G67"); // Vehicles
    await parse(sheets, "Value List!C71:G120") // Txts/Colors
    await parse(sheets, "Value List!C124:G183") // Rims
    await parse(sheets, "Value List!C187:G245") // Spoilers
    await parse(sheets, "Value List!C249:G267") // Tires/Horns
    await parse(sheets, "Value List!C249:G267") // Furniture
    await parse(sheets, "Value List!C249:G267") // Gun Skins

    // Hypers
    await parse(sheets, "Hyperchromes!C22:F30", true, (n) => `Hyper ${n} L5`)
    await parse(sheets, "Hyperchromes!C34:F41", true, (n) => `Hyper ${n} L4`)
    await parse(sheets, "Hyperchromes!C45:E52", true, (n) => `Hyper ${n} L3`)
    await parse(sheets, "Hyperchromes!C56:E63", true, (n) => `Hyper ${n} L2`)
    // No hyper lvl 1s!!
    writeFileSync("./src/trading/jbtc/values.json", JSON.stringify(final, null, 2));
    console.log("[JBTC] Saved");
}

main();