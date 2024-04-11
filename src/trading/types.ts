export enum JBItemDemand {
    None,
    Very_Low,
    Low,
    Mid,
    Decent,
    High,
}

export const DemandToValMul = {
    [JBItemDemand.None]: 1.0,
    [JBItemDemand.Very_Low]: 0.86,
    [JBItemDemand.Low]: 0.9,
    [JBItemDemand.Mid]: 0.95,
    [JBItemDemand.Decent]: 1.0,
    [JBItemDemand.High]: 1.2
}

export interface JBItem {
    name: string;
    value: number;
    demand: JBItemDemand,
    duped_value?: number;
    notes?: string,
    og?: string;
}

export function StrToJBDemand(str: string): JBItemDemand {
    switch (str) {
        case "Extremely Low":
            return JBItemDemand.Very_Low;
        case "Below Average":
            return JBItemDemand.Mid;
        case "Very Low":
            return JBItemDemand.Very_Low;
        case "Low":
            return JBItemDemand.Low;
        case "Decent":
            return JBItemDemand.Decent;
        case "Mainly Average":
            return JBItemDemand.Decent;
        case "Above Average":
            return JBItemDemand.Decent;
        case "Mid":
            return JBItemDemand.Mid;
        case "High":
            return JBItemDemand.High;
        default:
            return JBItemDemand.None;
    }
}