import { useCallback, useState } from "react";
import { useTranslation } from "react-i18next";
import { Input } from "@nextui-org/input";
import { Button } from "@nextui-org/button";
import { Code } from "@nextui-org/code";
import { Textarea } from "@nextui-org/input";

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
    <div className="flex w-full flex-col">
      <Code color="success" className="text-lg text-center">{`${t('calculator.answer')}: ${calculationResult?.result ?? ""}`}</Code>
      <Textarea
        isReadOnly
        className="h-10 w-full content-center"
        value={calculationResult?.trace?.postfix?.join(" ")}
      />
      <Button color='secondary' onClick={toggleTrace}>
        {showTrace ? t('calculator.hideTrace') : t('calculator.showTrace')}
      </Button>
      {showTrace && (
        <Textarea
          maxRows={30}
          isReadOnly
          defaultValue={JSON.stringify(calculationResult, null, 2)}
        />
      )}
    </div>
  );
};

export const CalculateView = () => {
  const [calculateText, setCalculateText] = useState<string>("1+2");
  const [calculationResult, setCalculationResult] =
    useState<CalculationResponse>();

  const { t } = useTranslation();

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
    <div className="mt-10 w-2/3">
      <div className="flex h-10 w-full">
        <Input
          className="h-full w-full text-center"
          type="text"
          placeholder="1 + 2"
          value={calculateText}
          onChange={(e) => setCalculateText(e.target.value)}
        />
        <Button
          color="primary"
          onClick={calculateRequest}
        >
          {t('calculator.calculate')}
        </Button>
      </div>
      <DisplayCalculation calculationResult={calculationResult} />
    </div >
  );
};
