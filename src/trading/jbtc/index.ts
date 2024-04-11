import JBBaseClass from "../base"
import type { JBItem } from "../types";

import CachedValues from "./values.json";

export default class JBTC extends JBBaseClass {
    values: JBItem[] = CachedValues as JBItem[];
    name = "jbtc"
}