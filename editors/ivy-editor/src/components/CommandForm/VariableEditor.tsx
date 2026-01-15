import { useState } from "react";
import type { SetVar, IfCondition, Value } from "../../types/scenario";

interface VariableEditorProps {
  setVar: SetVar | undefined;
  ifCondition: IfCondition | undefined;
  labels: string[];
  onSetChange: (value: SetVar | undefined) => void;
  onIfChange: (value: IfCondition | undefined) => void;
}

type ValueType = "string" | "number" | "boolean";

function getValueType(value: Value | undefined): ValueType {
  if (value === undefined) return "string";
  if (typeof value === "boolean") return "boolean";
  if (typeof value === "number") return "number";
  return "string";
}

function convertValue(value: string, type: ValueType): Value {
  switch (type) {
    case "boolean":
      return value === "true";
    case "number":
      return parseFloat(value) || 0;
    default:
      return value;
  }
}

function valueToString(value: Value | undefined): string {
  if (value === undefined) return "";
  return String(value);
}

export const VariableEditor: React.FC<VariableEditorProps> = ({
  setVar,
  ifCondition,
  labels,
  onSetChange,
  onIfChange,
}) => {
  const [setEnabled, setSetEnabled] = useState(!!setVar);
  const [ifEnabled, setIfEnabled] = useState(!!ifCondition);
  const [setValueType, setSetValueType] = useState<ValueType>(
    getValueType(setVar?.value)
  );
  const [ifValueType, setIfValueType] = useState<ValueType>(
    getValueType(ifCondition?.is)
  );

  const handleSetToggle = (enabled: boolean) => {
    setSetEnabled(enabled);
    if (!enabled) {
      onSetChange(undefined);
    } else {
      onSetChange({ name: "", value: "" });
    }
  };

  const handleIfToggle = (enabled: boolean) => {
    setIfEnabled(enabled);
    if (!enabled) {
      onIfChange(undefined);
    } else {
      onIfChange({ var: "", is: "", jump: "" });
    }
  };

  const updateSetVar = (updates: Partial<SetVar>) => {
    onSetChange({ ...setVar, ...updates } as SetVar);
  };

  const updateIfCondition = (updates: Partial<IfCondition>) => {
    onIfChange({ ...ifCondition, ...updates } as IfCondition);
  };

  return (
    <div className="variable-editor">
      {/* Set Variable Section */}
      <div className="variable-section">
        <div className="variable-header">
          <label>
            <input
              type="checkbox"
              checked={setEnabled}
              onChange={(e) => handleSetToggle(e.target.checked)}
            />
            Set Variable
          </label>
        </div>

        {setEnabled && (
          <div className="variable-options">
            <div className="variable-row">
              <div className="variable-field">
                <label>Name</label>
                <input
                  type="text"
                  value={setVar?.name || ""}
                  onChange={(e) => updateSetVar({ name: e.target.value })}
                  placeholder="variable_name"
                />
              </div>
            </div>

            <div className="variable-row">
              <div className="variable-field type-field">
                <label>Type</label>
                <select
                  value={setValueType}
                  onChange={(e) => {
                    const newType = e.target.value as ValueType;
                    setSetValueType(newType);
                    const currentValue = valueToString(setVar?.value);
                    updateSetVar({ value: convertValue(currentValue, newType) });
                  }}
                >
                  <option value="string">String</option>
                  <option value="number">Number</option>
                  <option value="boolean">Boolean</option>
                </select>
              </div>

              <div className="variable-field value-field">
                <label>Value</label>
                {setValueType === "boolean" ? (
                  <select
                    value={String(setVar?.value ?? "false")}
                    onChange={(e) =>
                      updateSetVar({ value: e.target.value === "true" })
                    }
                  >
                    <option value="true">true</option>
                    <option value="false">false</option>
                  </select>
                ) : setValueType === "number" ? (
                  <input
                    type="number"
                    value={valueToString(setVar?.value)}
                    onChange={(e) =>
                      updateSetVar({ value: parseFloat(e.target.value) || 0 })
                    }
                    placeholder="0"
                  />
                ) : (
                  <input
                    type="text"
                    value={valueToString(setVar?.value)}
                    onChange={(e) => updateSetVar({ value: e.target.value })}
                    placeholder="value"
                  />
                )}
              </div>
            </div>

            {/* Preview */}
            <div className="variable-preview">
              <code>
                {setVar?.name || "var"} = {JSON.stringify(setVar?.value ?? "")}
              </code>
            </div>
          </div>
        )}
      </div>

      {/* If Condition Section */}
      <div className="variable-section">
        <div className="variable-header">
          <label>
            <input
              type="checkbox"
              checked={ifEnabled}
              onChange={(e) => handleIfToggle(e.target.checked)}
            />
            Conditional Jump
          </label>
        </div>

        {ifEnabled && (
          <div className="variable-options">
            <div className="variable-row">
              <div className="variable-field">
                <label>Variable</label>
                <input
                  type="text"
                  value={ifCondition?.var || ""}
                  onChange={(e) => updateIfCondition({ var: e.target.value })}
                  placeholder="variable_name"
                />
              </div>
            </div>

            <div className="variable-row">
              <div className="variable-field type-field">
                <label>Type</label>
                <select
                  value={ifValueType}
                  onChange={(e) => {
                    const newType = e.target.value as ValueType;
                    setIfValueType(newType);
                    const currentValue = valueToString(ifCondition?.is);
                    updateIfCondition({ is: convertValue(currentValue, newType) });
                  }}
                >
                  <option value="string">String</option>
                  <option value="number">Number</option>
                  <option value="boolean">Boolean</option>
                </select>
              </div>

              <div className="variable-field value-field">
                <label>Equals</label>
                {ifValueType === "boolean" ? (
                  <select
                    value={String(ifCondition?.is ?? "true")}
                    onChange={(e) =>
                      updateIfCondition({ is: e.target.value === "true" })
                    }
                  >
                    <option value="true">true</option>
                    <option value="false">false</option>
                  </select>
                ) : ifValueType === "number" ? (
                  <input
                    type="number"
                    value={valueToString(ifCondition?.is)}
                    onChange={(e) =>
                      updateIfCondition({ is: parseFloat(e.target.value) || 0 })
                    }
                    placeholder="0"
                  />
                ) : (
                  <input
                    type="text"
                    value={valueToString(ifCondition?.is)}
                    onChange={(e) => updateIfCondition({ is: e.target.value })}
                    placeholder="value"
                  />
                )}
              </div>
            </div>

            <div className="variable-row">
              <div className="variable-field">
                <label>Jump to</label>
                <select
                  value={ifCondition?.jump || ""}
                  onChange={(e) => updateIfCondition({ jump: e.target.value })}
                >
                  <option value="">-- Select label --</option>
                  {labels.map((label) => (
                    <option key={label} value={label}>
                      {label}
                    </option>
                  ))}
                </select>
              </div>
            </div>

            {/* Preview */}
            <div className="variable-preview">
              <code>
                if {ifCondition?.var || "var"} == {JSON.stringify(ifCondition?.is ?? "")}{" "}
                → {ifCondition?.jump || "?"}
              </code>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
