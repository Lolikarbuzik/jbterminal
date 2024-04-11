import JBBaseClass from "../base"
import type { JBItem } from "../types";

import CachedValues from "./values.json";

export default class JBTrading extends JBBaseClass {
    values: JBItem[] = CachedValues as JBItem[];
    name = "jbtr"
}