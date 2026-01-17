import type { Value } from "../types/scenario";

export type ValueType = "string" | "number" | "boolean";

export const ValueTypeUtils = {
  detect(value: Value | undefined): ValueType {
    if (value === undefined) {return "string";}
    if (typeof value === "boolean") {return "boolean";}
    if (typeof value === "number") {return "number";}
    return "string";
  },

  convert(raw: string, type: ValueType): Value {
    switch (type) {
      case "boolean":
        return raw === "true";
      case "number":
        return parseFloat(raw) || 0;
      default:
        return raw;
    }
  },

  toString(value: Value | undefined): string {
    if (value === undefined) {return "";}
    return String(value);
  },
} as const;
