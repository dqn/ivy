import { useState } from "react";
import type { SetVar, IfCondition } from "../../types/scenario";
import { ValueTypeUtils, type ValueType } from "../../lib";

interface VariableEditorProps {
  setVar: SetVar | undefined;
  ifCondition: IfCondition | undefined;
  labels: string[];
  onSetChange: (value: SetVar | undefined) => void;
  onIfChange: (value: IfCondition | undefined) => void;
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
    ValueTypeUtils.detect(setVar?.value)
  );
  const [ifValueType, setIfValueType] = useState<ValueType>(
    ValueTypeUtils.detect(ifCondition?.is)
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
    const current: SetVar = setVar ?? { name: "", value: "" };
    onSetChange({ ...current, ...updates });
  };

  const updateIfCondition = (updates: Partial<IfCondition>) => {
    const current: IfCondition = ifCondition ?? { var: "", is: "", jump: "" };
    onIfChange({ ...current, ...updates });
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
                    const newType = e.target.value;
                    if (newType === "string" || newType === "number" || newType === "boolean") {
                      setSetValueType(newType);
                      const currentValue = ValueTypeUtils.toString(setVar?.value);
                      updateSetVar({ value: ValueTypeUtils.convert(currentValue, newType) });
                    }
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
                    value={ValueTypeUtils.toString(setVar?.value)}
                    onChange={(e) =>
                      updateSetVar({ value: parseFloat(e.target.value) || 0 })
                    }
                    placeholder="0"
                  />
                ) : (
                  <input
                    type="text"
                    value={ValueTypeUtils.toString(setVar?.value)}
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
                    const newType = e.target.value;
                    if (newType === "string" || newType === "number" || newType === "boolean") {
                      setIfValueType(newType);
                      const currentValue = ValueTypeUtils.toString(ifCondition?.is);
                      updateIfCondition({ is: ValueTypeUtils.convert(currentValue, newType) });
                    }
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
                    value={ValueTypeUtils.toString(ifCondition?.is)}
                    onChange={(e) =>
                      updateIfCondition({ is: parseFloat(e.target.value) || 0 })
                    }
                    placeholder="0"
                  />
                ) : (
                  <input
                    type="text"
                    value={ValueTypeUtils.toString(ifCondition?.is)}
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
