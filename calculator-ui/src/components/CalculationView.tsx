import { useCallback, useState } from "react";
import { useTranslation } from "react-i18next";

type CalculationResponse = {
  result: string;
  status: string;
  trace: {
    calculation_trace: Record<string, unknown>;
    infix: string[];
    postfix: string[];
  };
};

const DisplayCalculation = ({
  calculationResult,
}: {
  calculationResult?: CalculationResponse;
}) => {
  const [showTrace, setShowTrace] = useState(false);
  const toggleTrace = useCallback(() => {
    setShowTrace(!showTrace);
  }, [showTrace]);

  const { t } = useTranslation();
  return (
    <div className="flex h-auto w-full flex-col">
      <div className="mt-1 mb-1 h-10 content-center rounded-r rounded-l bg-green-700/75 text-center text-xl text-white">{`${t('calculator.answer')}: ${calculationResult?.result ?? ""}`}</div>
      <textarea
        className="h-10 w-full border-2 content-center"
        value={calculationResult?.trace?.postfix?.join(" ")}
      />
      <button className="h-10 w-full border-2 bg-blue-50" onClick={toggleTrace}>
        {showTrace ? t('calculator.hideTrace') : t('calculator.showTrace')}
      </button>
      {showTrace && (
        <textarea
          className="h-screen w-full border-2"
          value={JSON.stringify(calculationResult, null, 2)}
        />
      )}
    </div>
  );
};

export const CalculateView = () => {
  const [calculateText, setCalculateText] = useState<string>("1+2");
  const [calculationResult, setCalculationResult] =
    useState<CalculationResponse>();

  const calculateRequest = useCallback(async () => {
    const payload = {
      infix: calculateText,
    };

    const response = await fetch("/calculate", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: JSON.stringify(payload),
    });
    const result = await response.json();
    setCalculationResult(result);
  }, [calculateText]);

  return (
    <div className="mt-10 h-auto w-2/3">
      <div className="flex h-10 w-full">
        <input
          className="h-full w-full rounded-l-2xl border-2 text-center"
          type="text"
          placeholder="1 + 2"
          value={calculateText}
          onChange={(e) => setCalculateText(e.target.value)}
        />
        <button
          className="rounded-r-2xl border-2 bg-sky-600 pr-4 pl-2 text-white hover:border-blue-950"
          onClick={calculateRequest}
        >
          Calculate
        </button>
      </div>
      <DisplayCalculation calculationResult={calculationResult} />
    </div>
  );
};
