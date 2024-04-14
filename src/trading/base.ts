
import { type JBItem } from "./types";
import { name_count } from "./util";
import DupeList from "./dupes/values.json"

export default class JBBaseClass {
    values: JBItem[] = [];
    dupers: string[] = DupeList;
    name = "base"

    isDuped(name: string): boolean {
        name = name.toLowerCase();
        if (name.length < 8) {

        }
        return this.dupers.map(v => v.toLowerCase()).includes(name);
    }

    calcAvgDemand(items: JBItem[]): number {
        return items.reduce((dem, i) => dem + i.demand as number, 0) / items.length;
    }

    calcAvgDemandFromArr(searches: string[]): number {
        return searches.reduce((dem, search) => {
            search = search.replace("duped", "").replace("dupe", "")
            const item = this.itemSearch(search)[0];
            if (!item) {
                console.log(`Cant find item of '${search}'`);
                return dem;
            }
            return dem + item.demand as number;
        }, 0) / searches.length;
    }

    calcValue(items: JBItem[]): number {
        return items.reduce((sum, item) => {
            if (item.og && this.isDuped(item.og)) {
                return sum + (item.duped_value ?? item.value);
            } else return sum + item.value;
        }, 0);
    }

    calcValueFromArr(searches: string[]): number {
        return searches.reduce((sum, search) => {
            let duped = false;
            if (search.includes("dupe") || search.includes("duped")) {
                search = search.replace("duped", "").replace("dupe", "")
                duped = true;
            }
            const item = this.itemSearch(search)[0];
            if (!item) {
                console.log(`Cant find item of '${search}'`);
                return sum;
            }
            // console.log(`Found item of ${item.name} for duped: ${duped}, val: ${(duped ? item.duped_value ?? item.value : item.value)}`);
            return sum + (duped ? item.duped_value ?? item.value : item.value);
        }, 0)
    }

    itemSearch(search: string): JBItem[] {
        if (search === "") return [];
        search = search.toLowerCase().replaceAll(" ", "").replace("level", "l").replace("lvl", "l");
        if (!search.includes("radiant")) {
            search = search.replace("rad", "radiant");
        }
        const items: JBItem[] = [];
        const available_items: [string, number][] = this.values.map((item, i) => [item.name.toLowerCase().replaceAll(" ", ""), i]);

        // Loop through each element and check how good the `search` matches the item name and sort the items array by best match
        for (const [name, i] of available_items) {
            if (name.includes(search)) {
                items.push(this.values[i]);
            }
        }

        // Sort the items array by how many times `search` matches to item name 

        items.sort((a, b) => {
            const a_count = name_count(a.name, search);
            const b_count = name_count(b.name, search);
            return b_count - a_count;
        });

        // console.log(items);
        return items;
    }

    itemSearchFromArr(searches: string[]): JBItem[] {
        const items = [];
        for (const search of searches) {
            console.log(search.trim().length)
            if (search.trim().length === 0) continue;
            const item = this.itemSearch(search)[0];
            if (!item) {
                console.log(`Cant find item of '${search}'`);
                continue;
            }
            items.push(item);
        }

        return items;
    }
}